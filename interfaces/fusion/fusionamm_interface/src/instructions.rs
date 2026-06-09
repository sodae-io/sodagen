use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum FusionammProgramIx {
    CloseBundledPosition(CloseBundledPositionIxArgs),
    CloseLimitOrder,
    ClosePosition,
    CollectFees(CollectFeesIxArgs),
    CollectProtocolFees(CollectProtocolFeesIxArgs),
    DecreaseLimitOrder(DecreaseLimitOrderIxArgs),
    DecreaseLiquidity(DecreaseLiquidityIxArgs),
    DeletePositionBundle,
    DeleteTokenBadge,
    IncreaseLimitOrder(IncreaseLimitOrderIxArgs),
    IncreaseLiquidity(IncreaseLiquidityIxArgs),
    InitializeConfig(InitializeConfigIxArgs),
    InitializePool(InitializePoolIxArgs),
    InitializePositionBundle,
    InitializePositionBundleWithMetadata,
    InitializeTickArray(InitializeTickArrayIxArgs),
    InitializeTokenBadge,
    LockPosition(LockPositionIxArgs),
    OpenBundledPosition(OpenBundledPositionIxArgs),
    OpenLimitOrder(OpenLimitOrderIxArgs),
    OpenPosition(OpenPositionIxArgs),
    ResetPoolPrice(ResetPoolPriceIxArgs),
    SetCollectProtocolFeesAuthority,
    SetDefaultProtocolFeeRate(SetDefaultProtocolFeeRateIxArgs),
    SetFeeAuthority,
    SetFeeRate(SetFeeRateIxArgs),
    SetPositionRange(SetPositionRangeIxArgs),
    SetProtocolFeeRate(SetProtocolFeeRateIxArgs),
    SetTokenBadgeAuthority,
    Swap(SwapIxArgs),
    TwoHopSwap(TwoHopSwapIxArgs),
    UpdateFees,
}
impl FusionammProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CLOSE_BUNDLED_POSITION_IX_DISCM) {
            let mut reader = &buf[CLOSE_BUNDLED_POSITION_IX_DISCM.len()..];
            let bundle_index: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CloseBundledPosition(CloseBundledPositionIxArgs {
                    bundle_index,
                }),
            );
        }
        if buf.starts_with(&CLOSE_LIMIT_ORDER_IX_DISCM) {
            return Ok(Self::CloseLimitOrder);
        }
        if buf.starts_with(&CLOSE_POSITION_IX_DISCM) {
            return Ok(Self::ClosePosition);
        }
        if buf.starts_with(&COLLECT_FEES_IX_DISCM) {
            let mut reader = &buf[COLLECT_FEES_IX_DISCM.len()..];
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::CollectFees(CollectFeesIxArgs {
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&COLLECT_PROTOCOL_FEES_IX_DISCM) {
            let mut reader = &buf[COLLECT_PROTOCOL_FEES_IX_DISCM.len()..];
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::CollectProtocolFees(CollectProtocolFeesIxArgs {
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&DECREASE_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[DECREASE_LIMIT_ORDER_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::DecreaseLimitOrder(DecreaseLimitOrderIxArgs {
                    amount,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&DECREASE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[DECREASE_LIQUIDITY_IX_DISCM.len()..];
            let liquidity_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            let token_min_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_min_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::DecreaseLiquidity(DecreaseLiquidityIxArgs {
                    liquidity_amount,
                    token_min_a,
                    token_min_b,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&DELETE_POSITION_BUNDLE_IX_DISCM) {
            return Ok(Self::DeletePositionBundle);
        }
        if buf.starts_with(&DELETE_TOKEN_BADGE_IX_DISCM) {
            return Ok(Self::DeleteTokenBadge);
        }
        if buf.starts_with(&INCREASE_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[INCREASE_LIMIT_ORDER_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::IncreaseLimitOrder(IncreaseLimitOrderIxArgs {
                    amount,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&INCREASE_LIQUIDITY_IX_DISCM) {
            let mut reader = &buf[INCREASE_LIQUIDITY_IX_DISCM.len()..];
            let liquidity_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
            let token_max_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_max_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::IncreaseLiquidity(IncreaseLiquidityIxArgs {
                    liquidity_amount,
                    token_max_a,
                    token_max_b,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CONFIG_IX_DISCM.len()..];
            let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let collect_protocol_fees_authority: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_badge_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let default_protocol_fee_rate: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::InitializeConfig(InitializeConfigIxArgs {
                    fee_authority,
                    collect_protocol_fees_authority,
                    token_badge_authority,
                    default_protocol_fee_rate,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POOL_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_POOL_IX_DISCM.len()..];
            let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
            let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            let initial_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializePool(InitializePoolIxArgs {
                    tick_spacing,
                    fee_rate,
                    initial_sqrt_price,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POSITION_BUNDLE_IX_DISCM) {
            return Ok(Self::InitializePositionBundle);
        }
        if buf.starts_with(&INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM) {
            return Ok(Self::InitializePositionBundleWithMetadata);
        }
        if buf.starts_with(&INITIALIZE_TICK_ARRAY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_TICK_ARRAY_IX_DISCM.len()..];
            let start_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeTickArray(InitializeTickArrayIxArgs {
                    start_tick_index,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_TOKEN_BADGE_IX_DISCM) {
            return Ok(Self::InitializeTokenBadge);
        }
        if buf.starts_with(&LOCK_POSITION_IX_DISCM) {
            let mut reader = &buf[LOCK_POSITION_IX_DISCM.len()..];
            let lock_type: PositionLockType = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::LockPosition(LockPositionIxArgs { lock_type }));
        }
        if buf.starts_with(&OPEN_BUNDLED_POSITION_IX_DISCM) {
            let mut reader = &buf[OPEN_BUNDLED_POSITION_IX_DISCM.len()..];
            let bundle_index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenBundledPosition(OpenBundledPositionIxArgs {
                    bundle_index,
                    tick_lower_index,
                    tick_upper_index,
                }),
            );
        }
        if buf.starts_with(&OPEN_LIMIT_ORDER_IX_DISCM) {
            let mut reader = &buf[OPEN_LIMIT_ORDER_IX_DISCM.len()..];
            let tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            let with_token_metadata_extension: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::OpenLimitOrder(OpenLimitOrderIxArgs {
                    tick_index,
                    a_to_b,
                    with_token_metadata_extension,
                }),
            );
        }
        if buf.starts_with(&OPEN_POSITION_IX_DISCM) {
            let mut reader = &buf[OPEN_POSITION_IX_DISCM.len()..];
            let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let with_token_metadata_extension: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::OpenPosition(OpenPositionIxArgs {
                    tick_lower_index,
                    tick_upper_index,
                    with_token_metadata_extension,
                }),
            );
        }
        if buf.starts_with(&RESET_POOL_PRICE_IX_DISCM) {
            let mut reader = &buf[RESET_POOL_PRICE_IX_DISCM.len()..];
            let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ResetPoolPrice(ResetPoolPriceIxArgs { sqrt_price }));
        }
        if buf.starts_with(&SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM) {
            return Ok(Self::SetCollectProtocolFeesAuthority);
        }
        if buf.starts_with(&SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM) {
            let mut reader = &buf[SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM.len()..];
            let default_protocol_fee_rate: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetDefaultProtocolFeeRate(SetDefaultProtocolFeeRateIxArgs {
                    default_protocol_fee_rate,
                }),
            );
        }
        if buf.starts_with(&SET_FEE_AUTHORITY_IX_DISCM) {
            return Ok(Self::SetFeeAuthority);
        }
        if buf.starts_with(&SET_FEE_RATE_IX_DISCM) {
            let mut reader = &buf[SET_FEE_RATE_IX_DISCM.len()..];
            let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetFeeRate(SetFeeRateIxArgs { fee_rate }));
        }
        if buf.starts_with(&SET_POSITION_RANGE_IX_DISCM) {
            let mut reader = &buf[SET_POSITION_RANGE_IX_DISCM.len()..];
            let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetPositionRange(SetPositionRangeIxArgs {
                    tick_lower_index,
                    tick_upper_index,
                }),
            );
        }
        if buf.starts_with(&SET_PROTOCOL_FEE_RATE_IX_DISCM) {
            let mut reader = &buf[SET_PROTOCOL_FEE_RATE_IX_DISCM.len()..];
            let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetProtocolFeeRate(SetProtocolFeeRateIxArgs {
                    protocol_fee_rate,
                }),
            );
        }
        if buf.starts_with(&SET_TOKEN_BADGE_AUTHORITY_IX_DISCM) {
            return Ok(Self::SetTokenBadgeAuthority);
        }
        if buf.starts_with(&SWAP_IX_DISCM) {
            let mut reader = &buf[SWAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
            let amount_specified_is_input: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::Swap(SwapIxArgs {
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit,
                    amount_specified_is_input,
                    a_to_b,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&TWO_HOP_SWAP_IX_DISCM) {
            let mut reader = &buf[TWO_HOP_SWAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_specified_is_input: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let a_to_b_one: bool = crate::borsh_de_or_default(&mut reader)?;
            let a_to_b_two: bool = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit_one: u128 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit_two: u128 = crate::borsh_de_or_default(&mut reader)?;
            let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::TwoHopSwap(TwoHopSwapIxArgs {
                    amount,
                    other_amount_threshold,
                    amount_specified_is_input,
                    a_to_b_one,
                    a_to_b_two,
                    sqrt_price_limit_one,
                    sqrt_price_limit_two,
                    remaining_accounts_info,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEES_IX_DISCM) {
            return Ok(Self::UpdateFees);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CloseBundledPosition(args) => {
                writer.write_all(&CLOSE_BUNDLED_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bundle_index, &mut writer)?;
                Ok(())
            }
            Self::CloseLimitOrder => writer.write_all(&CLOSE_LIMIT_ORDER_IX_DISCM),
            Self::ClosePosition => writer.write_all(&CLOSE_POSITION_IX_DISCM),
            Self::CollectFees(args) => {
                writer.write_all(&COLLECT_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CollectProtocolFees(args) => {
                writer.write_all(&COLLECT_PROTOCOL_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DecreaseLimitOrder(args) => {
                writer.write_all(&DECREASE_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DecreaseLiquidity(args) => {
                writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_min_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_min_b, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DeletePositionBundle => {
                writer.write_all(&DELETE_POSITION_BUNDLE_IX_DISCM)
            }
            Self::DeleteTokenBadge => writer.write_all(&DELETE_TOKEN_BADGE_IX_DISCM),
            Self::IncreaseLimitOrder(args) => {
                writer.write_all(&INCREASE_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::IncreaseLiquidity(args) => {
                writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_max_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_max_b, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeConfig(args) => {
                writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_authority, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.collect_protocol_fees_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_badge_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.default_protocol_fee_rate,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializePool(args) => {
                writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_spacing, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_rate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.initial_sqrt_price, &mut writer)?;
                Ok(())
            }
            Self::InitializePositionBundle => {
                writer.write_all(&INITIALIZE_POSITION_BUNDLE_IX_DISCM)
            }
            Self::InitializePositionBundleWithMetadata => {
                writer.write_all(&INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM)
            }
            Self::InitializeTickArray(args) => {
                writer.write_all(&INITIALIZE_TICK_ARRAY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start_tick_index, &mut writer)?;
                Ok(())
            }
            Self::InitializeTokenBadge => {
                writer.write_all(&INITIALIZE_TOKEN_BADGE_IX_DISCM)
            }
            Self::LockPosition(args) => {
                writer.write_all(&LOCK_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.lock_type, &mut writer)?;
                Ok(())
            }
            Self::OpenBundledPosition(args) => {
                writer.write_all(&OPEN_BUNDLED_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bundle_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_lower_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_upper_index, &mut writer)?;
                Ok(())
            }
            Self::OpenLimitOrder(args) => {
                writer.write_all(&OPEN_LIMIT_ORDER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.with_token_metadata_extension,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::OpenPosition(args) => {
                writer.write_all(&OPEN_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_lower_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_upper_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.with_token_metadata_extension,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::ResetPoolPrice(args) => {
                writer.write_all(&RESET_POOL_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.sqrt_price, &mut writer)?;
                Ok(())
            }
            Self::SetCollectProtocolFeesAuthority => {
                writer.write_all(&SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM)
            }
            Self::SetDefaultProtocolFeeRate(args) => {
                writer.write_all(&SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.default_protocol_fee_rate,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetFeeAuthority => writer.write_all(&SET_FEE_AUTHORITY_IX_DISCM),
            Self::SetFeeRate(args) => {
                writer.write_all(&SET_FEE_RATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee_rate, &mut writer)?;
                Ok(())
            }
            Self::SetPositionRange(args) => {
                writer.write_all(&SET_POSITION_RANGE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_lower_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_upper_index, &mut writer)?;
                Ok(())
            }
            Self::SetProtocolFeeRate(args) => {
                writer.write_all(&SET_PROTOCOL_FEE_RATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.protocol_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::SetTokenBadgeAuthority => {
                writer.write_all(&SET_TOKEN_BADGE_AUTHORITY_IX_DISCM)
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.other_amount_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.amount_specified_is_input,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::TwoHopSwap(args) => {
                writer.write_all(&TWO_HOP_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.other_amount_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.amount_specified_is_input,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.a_to_b_one, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.a_to_b_two, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.sqrt_price_limit_one,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.sqrt_price_limit_two,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.remaining_accounts_info,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateFees => writer.write_all(&UPDATE_FEES_IX_DISCM),
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
pub const CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseBundledPositionAccounts<'me, 'info> {
    pub bundled_position: &'me AccountInfo<'info>,
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_authority: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseBundledPositionKeys {
    pub bundled_position: Pubkey,
    pub position_bundle: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_authority: Pubkey,
    pub receiver: Pubkey,
}
impl From<CloseBundledPositionAccounts<'_, '_>> for CloseBundledPositionKeys {
    fn from(accounts: CloseBundledPositionAccounts) -> Self {
        Self {
            bundled_position: *accounts.bundled_position.key,
            position_bundle: *accounts.position_bundle.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_authority: *accounts.position_bundle_authority.key,
            receiver: *accounts.receiver.key,
        }
    }
}
impl From<CloseBundledPositionKeys>
for [AccountMeta; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseBundledPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bundled_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_bundle_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN]>
for CloseBundledPositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bundled_position: pubkeys[0],
            position_bundle: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_authority: pubkeys[3],
            receiver: pubkeys[4],
        }
    }
}
impl<'info> From<CloseBundledPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseBundledPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.bundled_position.clone(),
            accounts.position_bundle.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_authority.clone(),
            accounts.receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN]>
for CloseBundledPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bundled_position: &arr[0],
            position_bundle: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_authority: &arr[3],
            receiver: &arr[4],
        }
    }
}
pub const CLOSE_BUNDLED_POSITION_IX_DISCM: [u8; 8usize] = [
    41, 36, 216, 245, 27, 85, 103, 67,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseBundledPositionIxArgs {
    pub bundle_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseBundledPositionIxData(pub CloseBundledPositionIxArgs);
impl From<CloseBundledPositionIxArgs> for CloseBundledPositionIxData {
    fn from(args: CloseBundledPositionIxArgs) -> Self {
        Self(args)
    }
}
impl CloseBundledPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_BUNDLED_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bundle_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CloseBundledPositionIxArgs {
                bundle_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_BUNDLED_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bundle_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_bundled_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseBundledPositionKeys,
    args: CloseBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_BUNDLED_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseBundledPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_bundled_position_ix(
    keys: CloseBundledPositionKeys,
    args: CloseBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    close_bundled_position_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn close_bundled_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
) -> ProgramResult {
    let keys: CloseBundledPositionKeys = accounts.into();
    let ix = close_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_bundled_position_invoke(
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
) -> ProgramResult {
    close_bundled_position_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn close_bundled_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseBundledPositionKeys = accounts.into();
    let ix = close_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_bundled_position_invoke_signed(
    accounts: CloseBundledPositionAccounts<'_, '_>,
    args: CloseBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_bundled_position_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn close_bundled_position_verify_account_keys(
    accounts: CloseBundledPositionAccounts<'_, '_>,
    keys: CloseBundledPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bundled_position.key, keys.bundled_position),
        (*accounts.position_bundle.key, keys.position_bundle),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_authority.key, keys.position_bundle_authority),
        (*accounts.receiver.key, keys.receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_bundled_position_verify_writable_privileges<'me, 'info>(
    accounts: CloseBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bundled_position,
        accounts.position_bundle,
        accounts.receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_bundled_position_verify_signer_privileges<'me, 'info>(
    accounts: CloseBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_bundled_position_verify_account_privileges<'me, 'info>(
    accounts: CloseBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_bundled_position_verify_writable_privileges(accounts)?;
    close_bundled_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CloseLimitOrderAccounts<'me, 'info> {
    pub limit_order_authority: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub limit_order: &'me AccountInfo<'info>,
    pub limit_order_mint: &'me AccountInfo<'info>,
    pub limit_order_token_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseLimitOrderKeys {
    pub limit_order_authority: Pubkey,
    pub receiver: Pubkey,
    pub limit_order: Pubkey,
    pub limit_order_mint: Pubkey,
    pub limit_order_token_account: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<CloseLimitOrderAccounts<'_, '_>> for CloseLimitOrderKeys {
    fn from(accounts: CloseLimitOrderAccounts) -> Self {
        Self {
            limit_order_authority: *accounts.limit_order_authority.key,
            receiver: *accounts.receiver.key,
            limit_order: *accounts.limit_order.key,
            limit_order_mint: *accounts.limit_order_mint.key,
            limit_order_token_account: *accounts.limit_order_token_account.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<CloseLimitOrderKeys> for [AccountMeta; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.limit_order_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for CloseLimitOrderKeys {
    fn from(pubkeys: [Pubkey; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            limit_order_authority: pubkeys[0],
            receiver: pubkeys[1],
            limit_order: pubkeys[2],
            limit_order_mint: pubkeys[3],
            limit_order_token_account: pubkeys[4],
            token_2022_program: pubkeys[5],
        }
    }
}
impl<'info> From<CloseLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.limit_order_authority.clone(),
            accounts.receiver.clone(),
            accounts.limit_order.clone(),
            accounts.limit_order_mint.clone(),
            accounts.limit_order_token_account.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for CloseLimitOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            limit_order_authority: &arr[0],
            receiver: &arr[1],
            limit_order: &arr[2],
            limit_order_mint: &arr[3],
            limit_order_token_account: &arr[4],
            token_2022_program: &arr[5],
        }
    }
}
pub const CLOSE_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    76, 124, 128, 15, 213, 87, 37, 250,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseLimitOrderIxData;
impl CloseLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_LIMIT_ORDER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseLimitOrderKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseLimitOrderIxData.try_to_vec()?,
    })
}
pub fn close_limit_order_ix(keys: CloseLimitOrderKeys) -> std::io::Result<Instruction> {
    close_limit_order_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn close_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseLimitOrderAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseLimitOrderKeys = accounts.into();
    let ix = close_limit_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_limit_order_invoke(
    accounts: CloseLimitOrderAccounts<'_, '_>,
) -> ProgramResult {
    close_limit_order_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn close_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseLimitOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseLimitOrderKeys = accounts.into();
    let ix = close_limit_order_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_limit_order_invoke_signed(
    accounts: CloseLimitOrderAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_limit_order_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_limit_order_verify_account_keys(
    accounts: CloseLimitOrderAccounts<'_, '_>,
    keys: CloseLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.limit_order_authority.key, keys.limit_order_authority),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.limit_order_mint.key, keys.limit_order_mint),
        (*accounts.limit_order_token_account.key, keys.limit_order_token_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: CloseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.receiver,
        accounts.limit_order,
        accounts.limit_order_mint,
        accounts.limit_order_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: CloseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.limit_order_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_limit_order_verify_account_privileges<'me, 'info>(
    accounts: CloseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_limit_order_verify_writable_privileges(accounts)?;
    close_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub position_authority: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub position_authority: Pubkey,
    pub receiver: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub token_2022_program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            position_authority: *accounts.position_authority.key,
            receiver: *accounts.receiver.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            token_2022_program: *accounts.token_2022_program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_authority: pubkeys[0],
            receiver: pubkeys[1],
            position: pubkeys[2],
            position_mint: pubkeys[3],
            position_token_account: pubkeys[4],
            token_2022_program: pubkeys[5],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.position_authority.clone(),
            accounts.receiver.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.token_2022_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]>
for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_authority: &arr[0],
            receiver: &arr[1],
            position: &arr[2],
            position_mint: &arr[3],
            position_token_account: &arr[4],
            token_2022_program: &arr[5],
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
    close_position_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
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
    close_position_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
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
    close_position_invoke_signed_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, seeds)
}
pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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
        accounts.receiver,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
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
    for should_be_signer in [accounts.position_authority] {
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
pub const COLLECT_FEES_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeesAccounts<'me, 'info> {
    pub fusion_pool: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeesKeys {
    pub fusion_pool: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
}
impl From<CollectFeesAccounts<'_, '_>> for CollectFeesKeys {
    fn from(accounts: CollectFeesAccounts) -> Self {
        Self {
            fusion_pool: *accounts.fusion_pool.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<CollectFeesKeys> for [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]> for CollectFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: pubkeys[0],
            position_authority: pubkeys[1],
            position: pubkeys[2],
            position_token_account: pubkeys[3],
            token_mint_a: pubkeys[4],
            token_mint_b: pubkeys[5],
            token_owner_account_a: pubkeys[6],
            token_owner_account_b: pubkeys[7],
            token_vault_a: pubkeys[8],
            token_vault_b: pubkeys[9],
            token_program_a: pubkeys[10],
            token_program_b: pubkeys[11],
            memo_program: pubkeys[12],
        }
    }
}
impl<'info> From<CollectFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pool.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]>
for CollectFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: &arr[0],
            position_authority: &arr[1],
            position: &arr[2],
            position_token_account: &arr[3],
            token_mint_a: &arr[4],
            token_mint_b: &arr[5],
            token_owner_account_a: &arr[6],
            token_owner_account_b: &arr[7],
            token_vault_a: &arr[8],
            token_vault_b: &arr[9],
            token_program_a: &arr[10],
            token_program_b: &arr[11],
            memo_program: &arr[12],
        }
    }
}
pub const COLLECT_FEES_IX_DISCM: [u8; 8usize] = [164, 152, 207, 99, 30, 186, 19, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectFeesIxArgs {
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesIxData(pub CollectFeesIxArgs);
impl From<CollectFeesIxArgs> for CollectFeesIxData {
    fn from(args: CollectFeesIxArgs) -> Self {
        Self(args)
    }
}
impl CollectFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(CollectFeesIxArgs {
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeesKeys,
    args: CollectFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_fees_ix(
    keys: CollectFeesKeys,
    args: CollectFeesIxArgs,
) -> std::io::Result<Instruction> {
    collect_fees_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn collect_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
    args: CollectFeesIxArgs,
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fees_invoke(
    accounts: CollectFeesAccounts<'_, '_>,
    args: CollectFeesIxArgs,
) -> ProgramResult {
    collect_fees_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn collect_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
    args: CollectFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fees_invoke_signed(
    accounts: CollectFeesAccounts<'_, '_>,
    args: CollectFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fees_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_fees_verify_account_keys(
    accounts: CollectFeesAccounts<'_, '_>,
    keys: CollectFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fees_verify_writable_privileges(accounts)?;
    collect_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolFeesAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub collect_protocol_fees_authority: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub token_destination_a: &'me AccountInfo<'info>,
    pub token_destination_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolFeesKeys {
    pub fusion_pools_config: Pubkey,
    pub fusion_pool: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub token_destination_a: Pubkey,
    pub token_destination_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
}
impl From<CollectProtocolFeesAccounts<'_, '_>> for CollectProtocolFeesKeys {
    fn from(accounts: CollectProtocolFeesAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            fusion_pool: *accounts.fusion_pool.key,
            collect_protocol_fees_authority: *accounts
                .collect_protocol_fees_authority
                .key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            token_destination_a: *accounts.token_destination_a.key,
            token_destination_b: *accounts.token_destination_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<CollectProtocolFeesKeys>
for [AccountMeta; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collect_protocol_fees_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_destination_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_destination_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN]> for CollectProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            fusion_pool: pubkeys[1],
            collect_protocol_fees_authority: pubkeys[2],
            token_mint_a: pubkeys[3],
            token_mint_b: pubkeys[4],
            token_vault_a: pubkeys[5],
            token_vault_b: pubkeys[6],
            token_destination_a: pubkeys[7],
            token_destination_b: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            memo_program: pubkeys[11],
        }
    }
}
impl<'info> From<CollectProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.fusion_pool.clone(),
            accounts.collect_protocol_fees_authority.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.token_destination_a.clone(),
            accounts.token_destination_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for CollectProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            fusion_pool: &arr[1],
            collect_protocol_fees_authority: &arr[2],
            token_mint_a: &arr[3],
            token_mint_b: &arr[4],
            token_vault_a: &arr[5],
            token_vault_b: &arr[6],
            token_destination_a: &arr[7],
            token_destination_b: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            memo_program: &arr[11],
        }
    }
}
pub const COLLECT_PROTOCOL_FEES_IX_DISCM: [u8; 8usize] = [
    22, 67, 23, 98, 150, 178, 70, 220,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeesIxArgs {
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeesIxData(pub CollectProtocolFeesIxArgs);
impl From<CollectProtocolFeesIxArgs> for CollectProtocolFeesIxData {
    fn from(args: CollectProtocolFeesIxArgs) -> Self {
        Self(args)
    }
}
impl CollectProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(CollectProtocolFeesIxArgs {
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectProtocolFeesKeys,
    args: CollectProtocolFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectProtocolFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_protocol_fees_ix(
    keys: CollectProtocolFeesKeys,
    args: CollectProtocolFeesIxArgs,
) -> std::io::Result<Instruction> {
    collect_protocol_fees_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn collect_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    args: CollectProtocolFeesIxArgs,
) -> ProgramResult {
    let keys: CollectProtocolFeesKeys = accounts.into();
    let ix = collect_protocol_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_protocol_fees_invoke(
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    args: CollectProtocolFeesIxArgs,
) -> ProgramResult {
    collect_protocol_fees_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn collect_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    args: CollectProtocolFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectProtocolFeesKeys = accounts.into();
    let ix = collect_protocol_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_protocol_fees_invoke_signed(
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    args: CollectProtocolFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_protocol_fees_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn collect_protocol_fees_verify_account_keys(
    accounts: CollectProtocolFeesAccounts<'_, '_>,
    keys: CollectProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (
            *accounts.collect_protocol_fees_authority.key,
            keys.collect_protocol_fees_authority,
        ),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.token_destination_a.key, keys.token_destination_a),
        (*accounts.token_destination_b.key, keys.token_destination_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fusion_pool,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.token_destination_a,
        accounts.token_destination_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_protocol_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_protocol_fees_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_protocol_fees_verify_writable_privileges(accounts)?;
    collect_protocol_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseLimitOrderAccounts<'me, 'info> {
    pub limit_order_authority: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub limit_order: &'me AccountInfo<'info>,
    pub limit_order_token_account: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLimitOrderKeys {
    pub limit_order_authority: Pubkey,
    pub fusion_pool: Pubkey,
    pub limit_order: Pubkey,
    pub limit_order_token_account: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
}
impl From<DecreaseLimitOrderAccounts<'_, '_>> for DecreaseLimitOrderKeys {
    fn from(accounts: DecreaseLimitOrderAccounts) -> Self {
        Self {
            limit_order_authority: *accounts.limit_order_authority.key,
            fusion_pool: *accounts.fusion_pool.key,
            limit_order: *accounts.limit_order.key,
            limit_order_token_account: *accounts.limit_order_token_account.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array: *accounts.tick_array.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<DecreaseLimitOrderKeys>
for [AccountMeta; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.limit_order_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for DecreaseLimitOrderKeys {
    fn from(pubkeys: [Pubkey; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            limit_order_authority: pubkeys[0],
            fusion_pool: pubkeys[1],
            limit_order: pubkeys[2],
            limit_order_token_account: pubkeys[3],
            token_mint_a: pubkeys[4],
            token_mint_b: pubkeys[5],
            token_owner_account_a: pubkeys[6],
            token_owner_account_b: pubkeys[7],
            token_vault_a: pubkeys[8],
            token_vault_b: pubkeys[9],
            tick_array: pubkeys[10],
            token_program_a: pubkeys[11],
            token_program_b: pubkeys[12],
            memo_program: pubkeys[13],
        }
    }
}
impl<'info> From<DecreaseLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.limit_order_authority.clone(),
            accounts.fusion_pool.clone(),
            accounts.limit_order.clone(),
            accounts.limit_order_token_account.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for DecreaseLimitOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            limit_order_authority: &arr[0],
            fusion_pool: &arr[1],
            limit_order: &arr[2],
            limit_order_token_account: &arr[3],
            token_mint_a: &arr[4],
            token_mint_b: &arr[5],
            token_owner_account_a: &arr[6],
            token_owner_account_b: &arr[7],
            token_vault_a: &arr[8],
            token_vault_b: &arr[9],
            tick_array: &arr[10],
            token_program_a: &arr[11],
            token_program_b: &arr[12],
            memo_program: &arr[13],
        }
    }
}
pub const DECREASE_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    117, 157, 60, 103, 66, 49, 163, 0,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLimitOrderIxArgs {
    pub amount: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLimitOrderIxData(pub DecreaseLimitOrderIxArgs);
impl From<DecreaseLimitOrderIxArgs> for DecreaseLimitOrderIxData {
    fn from(args: DecreaseLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(DecreaseLimitOrderIxArgs {
                amount,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLimitOrderKeys,
    args: DecreaseLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_limit_order_ix(
    keys: DecreaseLimitOrderKeys,
    args: DecreaseLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    decrease_limit_order_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn decrease_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLimitOrderAccounts<'_, '_>,
    args: DecreaseLimitOrderIxArgs,
) -> ProgramResult {
    let keys: DecreaseLimitOrderKeys = accounts.into();
    let ix = decrease_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_limit_order_invoke(
    accounts: DecreaseLimitOrderAccounts<'_, '_>,
    args: DecreaseLimitOrderIxArgs,
) -> ProgramResult {
    decrease_limit_order_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn decrease_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLimitOrderAccounts<'_, '_>,
    args: DecreaseLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLimitOrderKeys = accounts.into();
    let ix = decrease_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_limit_order_invoke_signed(
    accounts: DecreaseLimitOrderAccounts<'_, '_>,
    args: DecreaseLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_limit_order_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_limit_order_verify_account_keys(
    accounts: DecreaseLimitOrderAccounts<'_, '_>,
    keys: DecreaseLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.limit_order_authority.key, keys.limit_order_authority),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.limit_order_token_account.key, keys.limit_order_token_account),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array.key, keys.tick_array),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fusion_pool,
        accounts.limit_order,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.tick_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.limit_order_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_limit_order_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_limit_order_verify_writable_privileges(accounts)?;
    decrease_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityAccounts<'me, 'info> {
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityKeys {
    pub fusion_pool: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
impl From<DecreaseLiquidityAccounts<'_, '_>> for DecreaseLiquidityKeys {
    fn from(accounts: DecreaseLiquidityAccounts) -> Self {
        Self {
            fusion_pool: *accounts.fusion_pool.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
        }
    }
}
impl From<DecreaseLiquidityKeys> for [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for DecreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: pubkeys[0],
            token_program_a: pubkeys[1],
            token_program_b: pubkeys[2],
            memo_program: pubkeys[3],
            position_authority: pubkeys[4],
            position: pubkeys[5],
            position_token_account: pubkeys[6],
            token_mint_a: pubkeys[7],
            token_mint_b: pubkeys[8],
            token_owner_account_a: pubkeys[9],
            token_owner_account_b: pubkeys[10],
            token_vault_a: pubkeys[11],
            token_vault_b: pubkeys[12],
            tick_array_lower: pubkeys[13],
            tick_array_upper: pubkeys[14],
        }
    }
}
impl<'info> From<DecreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pool.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DecreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: &arr[0],
            token_program_a: &arr[1],
            token_program_b: &arr[2],
            memo_program: &arr[3],
            position_authority: &arr[4],
            position: &arr[5],
            position_token_account: &arr[6],
            token_mint_a: &arr[7],
            token_mint_b: &arr[8],
            token_owner_account_a: &arr[9],
            token_owner_account_b: &arr[10],
            token_vault_a: &arr[11],
            token_vault_b: &arr[12],
            tick_array_lower: &arr[13],
            tick_array_upper: &arr[14],
        }
    }
}
pub const DECREASE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    160, 38, 208, 111, 104, 91, 44, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityIxArgs {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityIxData(pub DecreaseLiquidityIxArgs);
impl From<DecreaseLiquidityIxArgs> for DecreaseLiquidityIxData {
    fn from(args: DecreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_min_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_min_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(DecreaseLiquidityIxArgs {
                liquidity_amount,
                token_min_a,
                token_min_b,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_min_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_min_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_liquidity_ix(
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    decrease_liquidity_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn decrease_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_liquidity_invoke(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    decrease_liquidity_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn decrease_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_liquidity_invoke_signed(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_liquidity_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_liquidity_verify_account_keys(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    keys: DecreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fusion_pool,
        accounts.position,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_liquidity_verify_writable_privileges(accounts)?;
    decrease_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DeletePositionBundleAccounts<'me, 'info> {
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_mint: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_owner: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeletePositionBundleKeys {
    pub position_bundle: Pubkey,
    pub position_bundle_mint: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_owner: Pubkey,
    pub receiver: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeletePositionBundleAccounts<'_, '_>> for DeletePositionBundleKeys {
    fn from(accounts: DeletePositionBundleAccounts) -> Self {
        Self {
            position_bundle: *accounts.position_bundle.key,
            position_bundle_mint: *accounts.position_bundle_mint.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_owner: *accounts.position_bundle_owner.key,
            receiver: *accounts.receiver.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeletePositionBundleKeys>
for [AccountMeta; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(keys: DeletePositionBundleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver,
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
impl From<[Pubkey; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for DeletePositionBundleKeys {
    fn from(pubkeys: [Pubkey; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_bundle: pubkeys[0],
            position_bundle_mint: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_owner: pubkeys[3],
            receiver: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<DeletePositionBundleAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeletePositionBundleAccounts<'_, 'info>) -> Self {
        [
            accounts.position_bundle.clone(),
            accounts.position_bundle_mint.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_owner.clone(),
            accounts.receiver.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for DeletePositionBundleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: &arr[0],
            position_bundle_mint: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_owner: &arr[3],
            receiver: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const DELETE_POSITION_BUNDLE_IX_DISCM: [u8; 8usize] = [
    100, 25, 99, 2, 217, 239, 124, 173,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DeletePositionBundleIxData;
impl DeletePositionBundleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_POSITION_BUNDLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_POSITION_BUNDLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_position_bundle_ix_with_program_id(
    program_id: Pubkey,
    keys: DeletePositionBundleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeletePositionBundleIxData.try_to_vec()?,
    })
}
pub fn delete_position_bundle_ix(
    keys: DeletePositionBundleKeys,
) -> std::io::Result<Instruction> {
    delete_position_bundle_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn delete_position_bundle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeletePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeletePositionBundleKeys = accounts.into();
    let ix = delete_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_position_bundle_invoke(
    accounts: DeletePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    delete_position_bundle_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn delete_position_bundle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeletePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeletePositionBundleKeys = accounts.into();
    let ix = delete_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_position_bundle_invoke_signed(
    accounts: DeletePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_position_bundle_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn delete_position_bundle_verify_account_keys(
    accounts: DeletePositionBundleAccounts<'_, '_>,
    keys: DeletePositionBundleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_bundle.key, keys.position_bundle),
        (*accounts.position_bundle_mint.key, keys.position_bundle_mint),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_owner.key, keys.position_bundle_owner),
        (*accounts.receiver.key, keys.receiver),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_position_bundle_verify_writable_privileges<'me, 'info>(
    accounts: DeletePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_bundle,
        accounts.position_bundle_mint,
        accounts.position_bundle_token_account,
        accounts.receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_position_bundle_verify_signer_privileges<'me, 'info>(
    accounts: DeletePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_position_bundle_verify_account_privileges<'me, 'info>(
    accounts: DeletePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_position_bundle_verify_writable_privileges(accounts)?;
    delete_position_bundle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DeleteTokenBadgeAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub token_badge_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_badge: &'me AccountInfo<'info>,
    pub receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteTokenBadgeKeys {
    pub fusion_pools_config: Pubkey,
    pub token_badge_authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_badge: Pubkey,
    pub receiver: Pubkey,
}
impl From<DeleteTokenBadgeAccounts<'_, '_>> for DeleteTokenBadgeKeys {
    fn from(accounts: DeleteTokenBadgeAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            token_badge_authority: *accounts.token_badge_authority.key,
            token_mint: *accounts.token_mint.key,
            token_badge: *accounts.token_badge.key,
            receiver: *accounts.receiver.key,
        }
    }
}
impl From<DeleteTokenBadgeKeys> for [AccountMeta; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: DeleteTokenBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_authority,
                is_signer: true,
                is_writable: false,
            },
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
                pubkey: keys.receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN]> for DeleteTokenBadgeKeys {
    fn from(pubkeys: [Pubkey; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            token_badge_authority: pubkeys[1],
            token_mint: pubkeys[2],
            token_badge: pubkeys[3],
            receiver: pubkeys[4],
        }
    }
}
impl<'info> From<DeleteTokenBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeleteTokenBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.token_badge_authority.clone(),
            accounts.token_mint.clone(),
            accounts.token_badge.clone(),
            accounts.receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN]>
for DeleteTokenBadgeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            token_badge_authority: &arr[1],
            token_mint: &arr[2],
            token_badge: &arr[3],
            receiver: &arr[4],
        }
    }
}
pub const DELETE_TOKEN_BADGE_IX_DISCM: [u8; 8usize] = [53, 146, 68, 8, 18, 117, 17, 185];
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTokenBadgeIxData;
impl DeleteTokenBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_TOKEN_BADGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_TOKEN_BADGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_token_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: DeleteTokenBadgeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_TOKEN_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeleteTokenBadgeIxData.try_to_vec()?,
    })
}
pub fn delete_token_badge_ix(
    keys: DeleteTokenBadgeKeys,
) -> std::io::Result<Instruction> {
    delete_token_badge_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn delete_token_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeleteTokenBadgeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeleteTokenBadgeKeys = accounts.into();
    let ix = delete_token_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_token_badge_invoke(
    accounts: DeleteTokenBadgeAccounts<'_, '_>,
) -> ProgramResult {
    delete_token_badge_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn delete_token_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeleteTokenBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeleteTokenBadgeKeys = accounts.into();
    let ix = delete_token_badge_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_token_badge_invoke_signed(
    accounts: DeleteTokenBadgeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_token_badge_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn delete_token_badge_verify_account_keys(
    accounts: DeleteTokenBadgeAccounts<'_, '_>,
    keys: DeleteTokenBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.token_badge_authority.key, keys.token_badge_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_badge.key, keys.token_badge),
        (*accounts.receiver.key, keys.receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_token_badge_verify_writable_privileges<'me, 'info>(
    accounts: DeleteTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_badge, accounts.receiver] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_token_badge_verify_signer_privileges<'me, 'info>(
    accounts: DeleteTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_badge_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_token_badge_verify_account_privileges<'me, 'info>(
    accounts: DeleteTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_token_badge_verify_writable_privileges(accounts)?;
    delete_token_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLimitOrderAccounts<'me, 'info> {
    pub limit_order_authority: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub limit_order: &'me AccountInfo<'info>,
    pub limit_order_token_account: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_owner_account: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLimitOrderKeys {
    pub limit_order_authority: Pubkey,
    pub fusion_pool: Pubkey,
    pub limit_order: Pubkey,
    pub limit_order_token_account: Pubkey,
    pub token_mint: Pubkey,
    pub token_owner_account: Pubkey,
    pub token_vault: Pubkey,
    pub tick_array: Pubkey,
    pub token_program: Pubkey,
    pub memo_program: Pubkey,
}
impl From<IncreaseLimitOrderAccounts<'_, '_>> for IncreaseLimitOrderKeys {
    fn from(accounts: IncreaseLimitOrderAccounts) -> Self {
        Self {
            limit_order_authority: *accounts.limit_order_authority.key,
            fusion_pool: *accounts.fusion_pool.key,
            limit_order: *accounts.limit_order.key,
            limit_order_token_account: *accounts.limit_order_token_account.key,
            token_mint: *accounts.token_mint.key,
            token_owner_account: *accounts.token_owner_account.key,
            token_vault: *accounts.token_vault.key,
            tick_array: *accounts.tick_array.key,
            token_program: *accounts.token_program.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<IncreaseLimitOrderKeys>
for [AccountMeta; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.limit_order_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array,
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
        ]
    }
}
impl From<[Pubkey; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for IncreaseLimitOrderKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            limit_order_authority: pubkeys[0],
            fusion_pool: pubkeys[1],
            limit_order: pubkeys[2],
            limit_order_token_account: pubkeys[3],
            token_mint: pubkeys[4],
            token_owner_account: pubkeys[5],
            token_vault: pubkeys[6],
            tick_array: pubkeys[7],
            token_program: pubkeys[8],
            memo_program: pubkeys[9],
        }
    }
}
impl<'info> From<IncreaseLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.limit_order_authority.clone(),
            accounts.fusion_pool.clone(),
            accounts.limit_order.clone(),
            accounts.limit_order_token_account.clone(),
            accounts.token_mint.clone(),
            accounts.token_owner_account.clone(),
            accounts.token_vault.clone(),
            accounts.tick_array.clone(),
            accounts.token_program.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for IncreaseLimitOrderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            limit_order_authority: &arr[0],
            fusion_pool: &arr[1],
            limit_order: &arr[2],
            limit_order_token_account: &arr[3],
            token_mint: &arr[4],
            token_owner_account: &arr[5],
            token_vault: &arr[6],
            tick_array: &arr[7],
            token_program: &arr[8],
            memo_program: &arr[9],
        }
    }
}
pub const INCREASE_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [
    177, 144, 89, 236, 250, 186, 125, 99,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLimitOrderIxArgs {
    pub amount: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLimitOrderIxData(pub IncreaseLimitOrderIxArgs);
impl From<IncreaseLimitOrderIxArgs> for IncreaseLimitOrderIxData {
    fn from(args: IncreaseLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(IncreaseLimitOrderIxArgs {
                amount,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLimitOrderKeys,
    args: IncreaseLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_limit_order_ix(
    keys: IncreaseLimitOrderKeys,
    args: IncreaseLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    increase_limit_order_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn increase_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLimitOrderAccounts<'_, '_>,
    args: IncreaseLimitOrderIxArgs,
) -> ProgramResult {
    let keys: IncreaseLimitOrderKeys = accounts.into();
    let ix = increase_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_limit_order_invoke(
    accounts: IncreaseLimitOrderAccounts<'_, '_>,
    args: IncreaseLimitOrderIxArgs,
) -> ProgramResult {
    increase_limit_order_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn increase_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLimitOrderAccounts<'_, '_>,
    args: IncreaseLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLimitOrderKeys = accounts.into();
    let ix = increase_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_limit_order_invoke_signed(
    accounts: IncreaseLimitOrderAccounts<'_, '_>,
    args: IncreaseLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_limit_order_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_limit_order_verify_account_keys(
    accounts: IncreaseLimitOrderAccounts<'_, '_>,
    keys: IncreaseLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.limit_order_authority.key, keys.limit_order_authority),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.limit_order_token_account.key, keys.limit_order_token_account),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_owner_account.key, keys.token_owner_account),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.tick_array.key, keys.tick_array),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fusion_pool,
        accounts.limit_order,
        accounts.token_owner_account,
        accounts.token_vault,
        accounts.tick_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.limit_order_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_limit_order_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_limit_order_verify_writable_privileges(accounts)?;
    increase_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityAccounts<'me, 'info> {
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityKeys {
    pub fusion_pool: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
impl From<IncreaseLiquidityAccounts<'_, '_>> for IncreaseLiquidityKeys {
    fn from(accounts: IncreaseLiquidityAccounts) -> Self {
        Self {
            fusion_pool: *accounts.fusion_pool.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
        }
    }
}
impl From<IncreaseLiquidityKeys> for [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for IncreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: pubkeys[0],
            token_program_a: pubkeys[1],
            token_program_b: pubkeys[2],
            memo_program: pubkeys[3],
            position_authority: pubkeys[4],
            position: pubkeys[5],
            position_token_account: pubkeys[6],
            token_mint_a: pubkeys[7],
            token_mint_b: pubkeys[8],
            token_owner_account_a: pubkeys[9],
            token_owner_account_b: pubkeys[10],
            token_vault_a: pubkeys[11],
            token_vault_b: pubkeys[12],
            tick_array_lower: pubkeys[13],
            tick_array_upper: pubkeys[14],
        }
    }
}
impl<'info> From<IncreaseLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pool.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for IncreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: &arr[0],
            token_program_a: &arr[1],
            token_program_b: &arr[2],
            memo_program: &arr[3],
            position_authority: &arr[4],
            position: &arr[5],
            position_token_account: &arr[6],
            token_mint_a: &arr[7],
            token_mint_b: &arr[8],
            token_owner_account_a: &arr[9],
            token_owner_account_b: &arr[10],
            token_vault_a: &arr[11],
            token_vault_b: &arr[12],
            tick_array_lower: &arr[13],
            tick_array_upper: &arr[14],
        }
    }
}
pub const INCREASE_LIQUIDITY_IX_DISCM: [u8; 8usize] = [
    46, 156, 243, 118, 13, 205, 251, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityIxArgs {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityIxData(pub IncreaseLiquidityIxArgs);
impl From<IncreaseLiquidityIxArgs> for IncreaseLiquidityIxData {
    fn from(args: IncreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_max_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_max_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(IncreaseLiquidityIxArgs {
                liquidity_amount,
                token_max_a,
                token_max_b,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_max_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_max_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_liquidity_ix(
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    increase_liquidity_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn increase_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_liquidity_invoke(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    increase_liquidity_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn increase_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_liquidity_invoke_signed(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_liquidity_verify_account_keys(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    keys: IncreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fusion_pool,
        accounts.position,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_liquidity_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_verify_writable_privileges(accounts)?;
    increase_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeConfigKeys {
    pub fusion_pools_config: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeConfigAccounts<'_, '_>> for InitializeConfigKeys {
    fn from(accounts: InitializeConfigAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeConfigKeys> for [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
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
impl From<[Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]> for InitializeConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            funder: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            funder: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_CONFIG_IX_DISCM: [u8; 8usize] = [
    208, 127, 21, 1, 194, 190, 196, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeConfigIxArgs {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub token_badge_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigIxData(pub InitializeConfigIxArgs);
impl From<InitializeConfigIxArgs> for InitializeConfigIxData {
    fn from(args: InitializeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collect_protocol_fees_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_badge_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let default_protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeConfigIxArgs {
                fee_authority,
                collect_protocol_fees_authority,
                token_badge_authority,
                default_protocol_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.collect_protocol_fees_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.token_badge_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.default_protocol_fee_rate,
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
pub fn initialize_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_config_ix(
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_config_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn initialize_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_config_invoke(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    initialize_config_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_config_invoke_signed(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_config_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_config_verify_account_keys(
    accounts: InitializeConfigAccounts<'_, '_>,
    keys: InitializeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pools_config, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_config_verify_writable_privileges(accounts)?;
    initialize_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_badge_a: &'me AccountInfo<'info>,
    pub token_badge_b: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolKeys {
    pub fusion_pools_config: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_badge_a: Pubkey,
    pub token_badge_b: Pubkey,
    pub funder: Pubkey,
    pub fusion_pool: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializePoolAccounts<'_, '_>> for InitializePoolKeys {
    fn from(accounts: InitializePoolAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_badge_a: *accounts.token_badge_a.key,
            token_badge_b: *accounts.token_badge_b.key,
            funder: *accounts.funder.key,
            fusion_pool: *accounts.fusion_pool.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializePoolKeys> for [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
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
impl From<[Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            token_mint_a: pubkeys[1],
            token_mint_b: pubkeys[2],
            token_badge_a: pubkeys[3],
            token_badge_b: pubkeys[4],
            funder: pubkeys[5],
            fusion_pool: pubkeys[6],
            token_vault_a: pubkeys[7],
            token_vault_b: pubkeys[8],
            token_program_a: pubkeys[9],
            token_program_b: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<InitializePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_badge_a.clone(),
            accounts.token_badge_b.clone(),
            accounts.funder.clone(),
            accounts.fusion_pool.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]>
for InitializePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            token_mint_a: &arr[1],
            token_mint_b: &arr[2],
            token_badge_a: &arr[3],
            token_badge_b: &arr[4],
            funder: &arr[5],
            fusion_pool: &arr[6],
            token_vault_a: &arr[7],
            token_vault_b: &arr[8],
            token_program_a: &arr[9],
            token_program_b: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const INITIALIZE_POOL_IX_DISCM: [u8; 8usize] = [95, 180, 10, 172, 84, 174, 232, 40];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolIxArgs {
    pub tick_spacing: u16,
    pub fee_rate: u16,
    pub initial_sqrt_price: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolIxData(pub InitializePoolIxArgs);
impl From<InitializePoolIxArgs> for InitializePoolIxData {
    fn from(args: InitializePoolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let initial_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializePoolIxArgs {
                tick_spacing,
                fee_rate,
                initial_sqrt_price,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.initial_sqrt_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_pool_ix(
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pool_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn initialize_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_invoke(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    initialize_pool_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_invoke_signed(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_pool_verify_account_keys(
    accounts: InitializePoolAccounts<'_, '_>,
    keys: InitializePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_badge_a.key, keys.token_badge_a),
        (*accounts.token_badge_b.key, keys.token_badge_b),
        (*accounts.funder.key, keys.funder),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.fusion_pool,
        accounts.token_vault_a,
        accounts.token_vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.funder,
        accounts.token_vault_a,
        accounts.token_vault_b,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_verify_writable_privileges(accounts)?;
    initialize_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionBundleAccounts<'me, 'info> {
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_mint: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_owner: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionBundleKeys {
    pub position_bundle: Pubkey,
    pub position_bundle_mint: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_owner: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitializePositionBundleAccounts<'_, '_>> for InitializePositionBundleKeys {
    fn from(accounts: InitializePositionBundleAccounts) -> Self {
        Self {
            position_bundle: *accounts.position_bundle.key,
            position_bundle_mint: *accounts.position_bundle_mint.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_owner: *accounts.position_bundle_owner.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitializePositionBundleKeys>
for [AccountMeta; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionBundleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_owner,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for InitializePositionBundleKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_bundle: pubkeys[0],
            position_bundle_mint: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_owner: pubkeys[3],
            funder: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializePositionBundleAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionBundleAccounts<'_, 'info>) -> Self {
        [
            accounts.position_bundle.clone(),
            accounts.position_bundle_mint.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_owner.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN]>
for InitializePositionBundleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: &arr[0],
            position_bundle_mint: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_owner: &arr[3],
            funder: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}
pub const INITIALIZE_POSITION_BUNDLE_IX_DISCM: [u8; 8usize] = [
    117, 45, 241, 149, 24, 18, 194, 65,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionBundleIxData;
impl InitializePositionBundleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_BUNDLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_BUNDLE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_bundle_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionBundleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_BUNDLE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePositionBundleIxData.try_to_vec()?,
    })
}
pub fn initialize_position_bundle_ix(
    keys: InitializePositionBundleKeys,
) -> std::io::Result<Instruction> {
    initialize_position_bundle_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn initialize_position_bundle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePositionBundleKeys = accounts.into();
    let ix = initialize_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_bundle_invoke(
    accounts: InitializePositionBundleAccounts<'_, '_>,
) -> ProgramResult {
    initialize_position_bundle_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn initialize_position_bundle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionBundleKeys = accounts.into();
    let ix = initialize_position_bundle_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_bundle_invoke_signed(
    accounts: InitializePositionBundleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_bundle_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_position_bundle_verify_account_keys(
    accounts: InitializePositionBundleAccounts<'_, '_>,
    keys: InitializePositionBundleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_bundle.key, keys.position_bundle),
        (*accounts.position_bundle_mint.key, keys.position_bundle_mint),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_owner.key, keys.position_bundle_owner),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_bundle,
        accounts.position_bundle_mint,
        accounts.position_bundle_token_account,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_mint, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionBundleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_bundle_verify_writable_privileges(accounts)?;
    initialize_position_bundle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionBundleWithMetadataAccounts<'me, 'info> {
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_mint: &'me AccountInfo<'info>,
    pub position_bundle_metadata: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_owner: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub metadata_update_auth: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionBundleWithMetadataKeys {
    pub position_bundle: Pubkey,
    pub position_bundle_mint: Pubkey,
    pub position_bundle_metadata: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_owner: Pubkey,
    pub funder: Pubkey,
    pub metadata_update_auth: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<InitializePositionBundleWithMetadataAccounts<'_, '_>>
for InitializePositionBundleWithMetadataKeys {
    fn from(accounts: InitializePositionBundleWithMetadataAccounts) -> Self {
        Self {
            position_bundle: *accounts.position_bundle.key,
            position_bundle_mint: *accounts.position_bundle_mint.key,
            position_bundle_metadata: *accounts.position_bundle_metadata.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_owner: *accounts.position_bundle_owner.key,
            funder: *accounts.funder.key,
            metadata_update_auth: *accounts.metadata_update_auth.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<InitializePositionBundleWithMetadataKeys>
for [AccountMeta; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionBundleWithMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_update_auth,
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
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN]>
for InitializePositionBundleWithMetadataKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: pubkeys[0],
            position_bundle_mint: pubkeys[1],
            position_bundle_metadata: pubkeys[2],
            position_bundle_token_account: pubkeys[3],
            position_bundle_owner: pubkeys[4],
            funder: pubkeys[5],
            metadata_update_auth: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            associated_token_program: pubkeys[10],
            metadata_program: pubkeys[11],
        }
    }
}
impl<'info> From<InitializePositionBundleWithMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionBundleWithMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.position_bundle.clone(),
            accounts.position_bundle_mint.clone(),
            accounts.position_bundle_metadata.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_owner.clone(),
            accounts.funder.clone(),
            accounts.metadata_update_auth.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN],
> for InitializePositionBundleWithMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position_bundle: &arr[0],
            position_bundle_mint: &arr[1],
            position_bundle_metadata: &arr[2],
            position_bundle_token_account: &arr[3],
            position_bundle_owner: &arr[4],
            funder: &arr[5],
            metadata_update_auth: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            associated_token_program: &arr[10],
            metadata_program: &arr[11],
        }
    }
}
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM: [u8; 8usize] = [
    93, 124, 16, 179, 249, 131, 115, 245,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionBundleWithMetadataIxData;
impl InitializePositionBundleWithMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_bundle_with_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionBundleWithMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_BUNDLE_WITH_METADATA_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePositionBundleWithMetadataIxData.try_to_vec()?,
    })
}
pub fn initialize_position_bundle_with_metadata_ix(
    keys: InitializePositionBundleWithMetadataKeys,
) -> std::io::Result<Instruction> {
    initialize_position_bundle_with_metadata_ix_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        keys,
    )
}
pub fn initialize_position_bundle_with_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePositionBundleWithMetadataKeys = accounts.into();
    let ix = initialize_position_bundle_with_metadata_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_bundle_with_metadata_invoke(
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
) -> ProgramResult {
    initialize_position_bundle_with_metadata_invoke_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_position_bundle_with_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionBundleWithMetadataKeys = accounts.into();
    let ix = initialize_position_bundle_with_metadata_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_bundle_with_metadata_invoke_signed(
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_bundle_with_metadata_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_position_bundle_with_metadata_verify_account_keys(
    accounts: InitializePositionBundleWithMetadataAccounts<'_, '_>,
    keys: InitializePositionBundleWithMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_bundle.key, keys.position_bundle),
        (*accounts.position_bundle_mint.key, keys.position_bundle_mint),
        (*accounts.position_bundle_metadata.key, keys.position_bundle_metadata),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_owner.key, keys.position_bundle_owner),
        (*accounts.funder.key, keys.funder),
        (*accounts.metadata_update_auth.key, keys.metadata_update_auth),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_with_metadata_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionBundleWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_bundle,
        accounts.position_bundle_mint,
        accounts.position_bundle_metadata,
        accounts.position_bundle_token_account,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_with_metadata_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionBundleWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_mint, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_bundle_with_metadata_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionBundleWithMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_bundle_with_metadata_verify_writable_privileges(accounts)?;
    initialize_position_bundle_with_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTickArrayAccounts<'me, 'info> {
    pub fusion_pool: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTickArrayKeys {
    pub fusion_pool: Pubkey,
    pub funder: Pubkey,
    pub tick_array: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTickArrayAccounts<'_, '_>> for InitializeTickArrayKeys {
    fn from(accounts: InitializeTickArrayAccounts) -> Self {
        Self {
            fusion_pool: *accounts.fusion_pool.key,
            funder: *accounts.funder.key,
            tick_array: *accounts.tick_array.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTickArrayKeys>
for [AccountMeta; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTickArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array,
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
impl From<[Pubkey; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN]> for InitializeTickArrayKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: pubkeys[0],
            funder: pubkeys[1],
            tick_array: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeTickArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTickArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pool.clone(),
            accounts.funder.clone(),
            accounts.tick_array.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN]>
for InitializeTickArrayAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pool: &arr[0],
            funder: &arr[1],
            tick_array: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_TICK_ARRAY_IX_DISCM: [u8; 8usize] = [
    11, 188, 193, 214, 141, 91, 149, 184,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeTickArrayIxArgs {
    pub start_tick_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTickArrayIxData(pub InitializeTickArrayIxArgs);
impl From<InitializeTickArrayIxArgs> for InitializeTickArrayIxData {
    fn from(args: InitializeTickArrayIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeTickArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TICK_ARRAY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeTickArrayIxArgs {
                start_tick_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TICK_ARRAY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start_tick_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_tick_array_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTickArrayKeys,
    args: InitializeTickArrayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TICK_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeTickArrayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_tick_array_ix(
    keys: InitializeTickArrayKeys,
    args: InitializeTickArrayIxArgs,
) -> std::io::Result<Instruction> {
    initialize_tick_array_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn initialize_tick_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
) -> ProgramResult {
    let keys: InitializeTickArrayKeys = accounts.into();
    let ix = initialize_tick_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_tick_array_invoke(
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
) -> ProgramResult {
    initialize_tick_array_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn initialize_tick_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTickArrayKeys = accounts.into();
    let ix = initialize_tick_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_tick_array_invoke_signed(
    accounts: InitializeTickArrayAccounts<'_, '_>,
    args: InitializeTickArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_tick_array_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_tick_array_verify_account_keys(
    accounts: InitializeTickArrayAccounts<'_, '_>,
    keys: InitializeTickArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.funder.key, keys.funder),
        (*accounts.tick_array.key, keys.tick_array),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_tick_array_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funder, accounts.tick_array] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_tick_array_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_tick_array_verify_account_privileges<'me, 'info>(
    accounts: InitializeTickArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_tick_array_verify_writable_privileges(accounts)?;
    initialize_tick_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTokenBadgeAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub token_badge_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub token_badge: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTokenBadgeKeys {
    pub fusion_pools_config: Pubkey,
    pub token_badge_authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_badge: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTokenBadgeAccounts<'_, '_>> for InitializeTokenBadgeKeys {
    fn from(accounts: InitializeTokenBadgeAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            token_badge_authority: *accounts.token_badge_authority.key,
            token_mint: *accounts.token_mint.key,
            token_badge: *accounts.token_badge.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTokenBadgeKeys>
for [AccountMeta; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTokenBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_badge_authority,
                is_signer: true,
                is_writable: false,
            },
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
impl From<[Pubkey; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN]>
for InitializeTokenBadgeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            token_badge_authority: pubkeys[1],
            token_mint: pubkeys[2],
            token_badge: pubkeys[3],
            funder: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeTokenBadgeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTokenBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.token_badge_authority.clone(),
            accounts.token_mint.clone(),
            accounts.token_badge.clone(),
            accounts.funder.clone(),
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
            fusion_pools_config: &arr[0],
            token_badge_authority: &arr[1],
            token_mint: &arr[2],
            token_badge: &arr[3],
            funder: &arr[4],
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
    initialize_token_badge_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
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
    initialize_token_badge_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
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
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_token_badge_verify_account_keys(
    accounts: InitializeTokenBadgeAccounts<'_, '_>,
    keys: InitializeTokenBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.token_badge_authority.key, keys.token_badge_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.token_badge.key, keys.token_badge),
        (*accounts.funder.key, keys.funder),
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
    for should_be_writable in [accounts.token_badge, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_token_badge_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_badge_authority, accounts.funder] {
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
pub const LOCK_POSITION_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct LockPositionAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub position_lock: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LockPositionKeys {
    pub funder: Pubkey,
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub position_lock: Pubkey,
    pub fusion_pool: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<LockPositionAccounts<'_, '_>> for LockPositionKeys {
    fn from(accounts: LockPositionAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            position_lock: *accounts.position_lock.key,
            fusion_pool: *accounts.fusion_pool.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<LockPositionKeys> for [AccountMeta; LOCK_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: LockPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_lock,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; LOCK_POSITION_IX_ACCOUNTS_LEN]> for LockPositionKeys {
    fn from(pubkeys: [Pubkey; LOCK_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            position_authority: pubkeys[1],
            position: pubkeys[2],
            position_mint: pubkeys[3],
            position_token_account: pubkeys[4],
            position_lock: pubkeys[5],
            fusion_pool: pubkeys[6],
            token_2022_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<LockPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; LOCK_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: LockPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.position_lock.clone(),
            accounts.fusion_pool.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LOCK_POSITION_IX_ACCOUNTS_LEN]>
for LockPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LOCK_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: &arr[0],
            position_authority: &arr[1],
            position: &arr[2],
            position_mint: &arr[3],
            position_token_account: &arr[4],
            position_lock: &arr[5],
            fusion_pool: &arr[6],
            token_2022_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const LOCK_POSITION_IX_DISCM: [u8; 8usize] = [227, 62, 2, 252, 247, 10, 171, 185];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LockPositionIxArgs {
    pub lock_type: PositionLockType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockPositionIxData(pub LockPositionIxArgs);
impl From<LockPositionIxArgs> for LockPositionIxData {
    fn from(args: LockPositionIxArgs) -> Self {
        Self(args)
    }
}
impl LockPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let lock_type: PositionLockType = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(LockPositionIxArgs { lock_type }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.lock_type, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lock_position_ix_with_program_id(
    program_id: Pubkey,
    keys: LockPositionKeys,
    args: LockPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOCK_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: LockPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lock_position_ix(
    keys: LockPositionKeys,
    args: LockPositionIxArgs,
) -> std::io::Result<Instruction> {
    lock_position_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn lock_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
) -> ProgramResult {
    let keys: LockPositionKeys = accounts.into();
    let ix = lock_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lock_position_invoke(
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
) -> ProgramResult {
    lock_position_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn lock_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LockPositionKeys = accounts.into();
    let ix = lock_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lock_position_invoke_signed(
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lock_position_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn lock_position_verify_account_keys(
    accounts: LockPositionAccounts<'_, '_>,
    keys: LockPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.position_lock.key, keys.position_lock),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lock_position_verify_writable_privileges<'me, 'info>(
    accounts: LockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.position_token_account,
        accounts.position_lock,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lock_position_verify_signer_privileges<'me, 'info>(
    accounts: LockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lock_position_verify_account_privileges<'me, 'info>(
    accounts: LockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lock_position_verify_writable_privileges(accounts)?;
    lock_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct OpenBundledPositionAccounts<'me, 'info> {
    pub bundled_position: &'me AccountInfo<'info>,
    pub position_bundle: &'me AccountInfo<'info>,
    pub position_bundle_token_account: &'me AccountInfo<'info>,
    pub position_bundle_authority: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenBundledPositionKeys {
    pub bundled_position: Pubkey,
    pub position_bundle: Pubkey,
    pub position_bundle_token_account: Pubkey,
    pub position_bundle_authority: Pubkey,
    pub fusion_pool: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<OpenBundledPositionAccounts<'_, '_>> for OpenBundledPositionKeys {
    fn from(accounts: OpenBundledPositionAccounts) -> Self {
        Self {
            bundled_position: *accounts.bundled_position.key,
            position_bundle: *accounts.position_bundle.key,
            position_bundle_token_account: *accounts.position_bundle_token_account.key,
            position_bundle_authority: *accounts.position_bundle_authority.key,
            fusion_pool: *accounts.fusion_pool.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<OpenBundledPositionKeys>
for [AccountMeta; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenBundledPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.bundled_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_bundle_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_bundle_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN]> for OpenBundledPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            bundled_position: pubkeys[0],
            position_bundle: pubkeys[1],
            position_bundle_token_account: pubkeys[2],
            position_bundle_authority: pubkeys[3],
            fusion_pool: pubkeys[4],
            funder: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<OpenBundledPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenBundledPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.bundled_position.clone(),
            accounts.position_bundle.clone(),
            accounts.position_bundle_token_account.clone(),
            accounts.position_bundle_authority.clone(),
            accounts.fusion_pool.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN]>
for OpenBundledPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            bundled_position: &arr[0],
            position_bundle: &arr[1],
            position_bundle_token_account: &arr[2],
            position_bundle_authority: &arr[3],
            fusion_pool: &arr[4],
            funder: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const OPEN_BUNDLED_POSITION_IX_DISCM: [u8; 8usize] = [
    169, 113, 126, 171, 213, 172, 212, 49,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenBundledPositionIxArgs {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenBundledPositionIxData(pub OpenBundledPositionIxArgs);
impl From<OpenBundledPositionIxArgs> for OpenBundledPositionIxData {
    fn from(args: OpenBundledPositionIxArgs) -> Self {
        Self(args)
    }
}
impl OpenBundledPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_BUNDLED_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bundle_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenBundledPositionIxArgs {
                bundle_index,
                tick_lower_index,
                tick_upper_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_BUNDLED_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bundle_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_upper_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_bundled_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenBundledPositionKeys,
    args: OpenBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_BUNDLED_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenBundledPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_bundled_position_ix(
    keys: OpenBundledPositionKeys,
    args: OpenBundledPositionIxArgs,
) -> std::io::Result<Instruction> {
    open_bundled_position_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn open_bundled_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
) -> ProgramResult {
    let keys: OpenBundledPositionKeys = accounts.into();
    let ix = open_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_bundled_position_invoke(
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
) -> ProgramResult {
    open_bundled_position_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn open_bundled_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenBundledPositionKeys = accounts.into();
    let ix = open_bundled_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_bundled_position_invoke_signed(
    accounts: OpenBundledPositionAccounts<'_, '_>,
    args: OpenBundledPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_bundled_position_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_bundled_position_verify_account_keys(
    accounts: OpenBundledPositionAccounts<'_, '_>,
    keys: OpenBundledPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.bundled_position.key, keys.bundled_position),
        (*accounts.position_bundle.key, keys.position_bundle),
        (
            *accounts.position_bundle_token_account.key,
            keys.position_bundle_token_account,
        ),
        (*accounts.position_bundle_authority.key, keys.position_bundle_authority),
        (*accounts.fusion_pool.key, keys.fusion_pool),
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
pub fn open_bundled_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.bundled_position,
        accounts.position_bundle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_bundled_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_bundle_authority, accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_bundled_position_verify_account_privileges<'me, 'info>(
    accounts: OpenBundledPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_bundled_position_verify_writable_privileges(accounts)?;
    open_bundled_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct OpenLimitOrderAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub limit_order: &'me AccountInfo<'info>,
    pub limit_order_mint: &'me AccountInfo<'info>,
    pub limit_order_token_account: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_update_auth: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenLimitOrderKeys {
    pub funder: Pubkey,
    pub owner: Pubkey,
    pub limit_order: Pubkey,
    pub limit_order_mint: Pubkey,
    pub limit_order_token_account: Pubkey,
    pub fusion_pool: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_update_auth: Pubkey,
}
impl From<OpenLimitOrderAccounts<'_, '_>> for OpenLimitOrderKeys {
    fn from(accounts: OpenLimitOrderAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            owner: *accounts.owner.key,
            limit_order: *accounts.limit_order.key,
            limit_order_mint: *accounts.limit_order_mint.key,
            limit_order_token_account: *accounts.limit_order_token_account.key,
            fusion_pool: *accounts.fusion_pool.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_update_auth: *accounts.metadata_update_auth.key,
        }
    }
}
impl From<OpenLimitOrderKeys> for [AccountMeta; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.limit_order,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.limit_order_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
                pubkey: keys.metadata_update_auth,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for OpenLimitOrderKeys {
    fn from(pubkeys: [Pubkey; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            owner: pubkeys[1],
            limit_order: pubkeys[2],
            limit_order_mint: pubkeys[3],
            limit_order_token_account: pubkeys[4],
            fusion_pool: pubkeys[5],
            token_2022_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            metadata_update_auth: pubkeys[9],
        }
    }
}
impl<'info> From<OpenLimitOrderAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.owner.clone(),
            accounts.limit_order.clone(),
            accounts.limit_order_mint.clone(),
            accounts.limit_order_token_account.clone(),
            accounts.fusion_pool.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_update_auth.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
for OpenLimitOrderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: &arr[0],
            owner: &arr[1],
            limit_order: &arr[2],
            limit_order_mint: &arr[3],
            limit_order_token_account: &arr[4],
            fusion_pool: &arr[5],
            token_2022_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
            metadata_update_auth: &arr[9],
        }
    }
}
pub const OPEN_LIMIT_ORDER_IX_DISCM: [u8; 8usize] = [157, 32, 218, 183, 71, 29, 18, 147];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenLimitOrderIxArgs {
    pub tick_index: i32,
    pub a_to_b: bool,
    pub with_token_metadata_extension: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenLimitOrderIxData(pub OpenLimitOrderIxArgs);
impl From<OpenLimitOrderIxArgs> for OpenLimitOrderIxData {
    fn from(args: OpenLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl OpenLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let with_token_metadata_extension: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(OpenLimitOrderIxArgs {
                tick_index,
                a_to_b,
                with_token_metadata_extension,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_LIMIT_ORDER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.with_token_metadata_extension,
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
pub fn open_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenLimitOrderKeys,
    args: OpenLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_limit_order_ix(
    keys: OpenLimitOrderKeys,
    args: OpenLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    open_limit_order_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn open_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenLimitOrderAccounts<'_, '_>,
    args: OpenLimitOrderIxArgs,
) -> ProgramResult {
    let keys: OpenLimitOrderKeys = accounts.into();
    let ix = open_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_limit_order_invoke(
    accounts: OpenLimitOrderAccounts<'_, '_>,
    args: OpenLimitOrderIxArgs,
) -> ProgramResult {
    open_limit_order_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn open_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenLimitOrderAccounts<'_, '_>,
    args: OpenLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenLimitOrderKeys = accounts.into();
    let ix = open_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_limit_order_invoke_signed(
    accounts: OpenLimitOrderAccounts<'_, '_>,
    args: OpenLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_limit_order_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_limit_order_verify_account_keys(
    accounts: OpenLimitOrderAccounts<'_, '_>,
    keys: OpenLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.owner.key, keys.owner),
        (*accounts.limit_order.key, keys.limit_order),
        (*accounts.limit_order_mint.key, keys.limit_order_mint),
        (*accounts.limit_order_token_account.key, keys.limit_order_token_account),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_update_auth.key, keys.metadata_update_auth),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: OpenLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.limit_order,
        accounts.limit_order_mint,
        accounts.limit_order_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: OpenLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.limit_order_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_limit_order_verify_account_privileges<'me, 'info>(
    accounts: OpenLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_limit_order_verify_writable_privileges(accounts)?;
    open_limit_order_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct OpenPositionAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_update_auth: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionKeys {
    pub funder: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_token_account: Pubkey,
    pub fusion_pool: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_update_auth: Pubkey,
}
impl From<OpenPositionAccounts<'_, '_>> for OpenPositionKeys {
    fn from(accounts: OpenPositionAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            owner: *accounts.owner.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_token_account: *accounts.position_token_account.key,
            fusion_pool: *accounts.fusion_pool.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_update_auth: *accounts.metadata_update_auth.key,
        }
    }
}
impl From<OpenPositionKeys> for [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
                pubkey: keys.metadata_update_auth,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]> for OpenPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            owner: pubkeys[1],
            position: pubkeys[2],
            position_mint: pubkeys[3],
            position_token_account: pubkeys[4],
            fusion_pool: pubkeys[5],
            token_2022_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            metadata_update_auth: pubkeys[9],
        }
    }
}
impl<'info> From<OpenPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.owner.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_token_account.clone(),
            accounts.fusion_pool.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_update_auth.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]>
for OpenPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: &arr[0],
            owner: &arr[1],
            position: &arr[2],
            position_mint: &arr[3],
            position_token_account: &arr[4],
            fusion_pool: &arr[5],
            token_2022_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
            metadata_update_auth: &arr[9],
        }
    }
}
pub const OPEN_POSITION_IX_DISCM: [u8; 8usize] = [135, 128, 47, 77, 15, 152, 240, 49];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionIxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata_extension: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionIxData(pub OpenPositionIxArgs);
impl From<OpenPositionIxArgs> for OpenPositionIxData {
    fn from(args: OpenPositionIxArgs) -> Self {
        Self(args)
    }
}
impl OpenPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let with_token_metadata_extension: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(OpenPositionIxArgs {
                tick_lower_index,
                tick_upper_index,
                with_token_metadata_extension,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.with_token_metadata_extension,
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
pub fn open_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_position_ix(
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) -> std::io::Result<Instruction> {
    open_position_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn open_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_position_invoke(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    open_position_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn open_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_position_invoke_signed(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_position_verify_account_keys(
    accounts: OpenPositionAccounts<'_, '_>,
    keys: OpenPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.owner.key, keys.owner),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_update_auth.key, keys.metadata_update_auth),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder,
        accounts.position,
        accounts.position_mint,
        accounts.position_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder, accounts.position_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_position_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_verify_writable_privileges(accounts)?;
    open_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESET_POOL_PRICE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ResetPoolPriceAccounts<'me, 'info> {
    pub fee_authority: &'me AccountInfo<'info>,
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResetPoolPriceKeys {
    pub fee_authority: Pubkey,
    pub fusion_pools_config: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub fusion_pool: Pubkey,
}
impl From<ResetPoolPriceAccounts<'_, '_>> for ResetPoolPriceKeys {
    fn from(accounts: ResetPoolPriceAccounts) -> Self {
        Self {
            fee_authority: *accounts.fee_authority.key,
            fusion_pools_config: *accounts.fusion_pools_config.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            fusion_pool: *accounts.fusion_pool.key,
        }
    }
}
impl From<ResetPoolPriceKeys> for [AccountMeta; RESET_POOL_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: ResetPoolPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESET_POOL_PRICE_IX_ACCOUNTS_LEN]> for ResetPoolPriceKeys {
    fn from(pubkeys: [Pubkey; RESET_POOL_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_authority: pubkeys[0],
            fusion_pools_config: pubkeys[1],
            token_vault_a: pubkeys[2],
            token_vault_b: pubkeys[3],
            fusion_pool: pubkeys[4],
        }
    }
}
impl<'info> From<ResetPoolPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; RESET_POOL_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResetPoolPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_authority.clone(),
            accounts.fusion_pools_config.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.fusion_pool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESET_POOL_PRICE_IX_ACCOUNTS_LEN]>
for ResetPoolPriceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RESET_POOL_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_authority: &arr[0],
            fusion_pools_config: &arr[1],
            token_vault_a: &arr[2],
            token_vault_b: &arr[3],
            fusion_pool: &arr[4],
        }
    }
}
pub const RESET_POOL_PRICE_IX_DISCM: [u8; 8usize] = [93, 158, 158, 189, 131, 42, 15, 22];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResetPoolPriceIxArgs {
    pub sqrt_price: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResetPoolPriceIxData(pub ResetPoolPriceIxArgs);
impl From<ResetPoolPriceIxArgs> for ResetPoolPriceIxData {
    fn from(args: ResetPoolPriceIxArgs) -> Self {
        Self(args)
    }
}
impl ResetPoolPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESET_POOL_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ResetPoolPriceIxArgs { sqrt_price }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESET_POOL_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reset_pool_price_ix_with_program_id(
    program_id: Pubkey,
    keys: ResetPoolPriceKeys,
    args: ResetPoolPriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESET_POOL_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ResetPoolPriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reset_pool_price_ix(
    keys: ResetPoolPriceKeys,
    args: ResetPoolPriceIxArgs,
) -> std::io::Result<Instruction> {
    reset_pool_price_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn reset_pool_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResetPoolPriceAccounts<'_, '_>,
    args: ResetPoolPriceIxArgs,
) -> ProgramResult {
    let keys: ResetPoolPriceKeys = accounts.into();
    let ix = reset_pool_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn reset_pool_price_invoke(
    accounts: ResetPoolPriceAccounts<'_, '_>,
    args: ResetPoolPriceIxArgs,
) -> ProgramResult {
    reset_pool_price_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn reset_pool_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResetPoolPriceAccounts<'_, '_>,
    args: ResetPoolPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResetPoolPriceKeys = accounts.into();
    let ix = reset_pool_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reset_pool_price_invoke_signed(
    accounts: ResetPoolPriceAccounts<'_, '_>,
    args: ResetPoolPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reset_pool_price_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn reset_pool_price_verify_account_keys(
    accounts: ResetPoolPriceAccounts<'_, '_>,
    keys: ResetPoolPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.fusion_pool.key, keys.fusion_pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn reset_pool_price_verify_writable_privileges<'me, 'info>(
    accounts: ResetPoolPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.fusion_pool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reset_pool_price_verify_signer_privileges<'me, 'info>(
    accounts: ResetPoolPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reset_pool_price_verify_account_privileges<'me, 'info>(
    accounts: ResetPoolPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reset_pool_price_verify_writable_privileges(accounts)?;
    reset_pool_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetCollectProtocolFeesAuthorityAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub collect_protocol_fees_authority: &'me AccountInfo<'info>,
    pub new_collect_protocol_fees_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCollectProtocolFeesAuthorityKeys {
    pub fusion_pools_config: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub new_collect_protocol_fees_authority: Pubkey,
}
impl From<SetCollectProtocolFeesAuthorityAccounts<'_, '_>>
for SetCollectProtocolFeesAuthorityKeys {
    fn from(accounts: SetCollectProtocolFeesAuthorityAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            collect_protocol_fees_authority: *accounts
                .collect_protocol_fees_authority
                .key,
            new_collect_protocol_fees_authority: *accounts
                .new_collect_protocol_fees_authority
                .key,
        }
    }
}
impl From<SetCollectProtocolFeesAuthorityKeys>
for [AccountMeta; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCollectProtocolFeesAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collect_protocol_fees_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_collect_protocol_fees_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetCollectProtocolFeesAuthorityKeys {
    fn from(
        pubkeys: [Pubkey; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            collect_protocol_fees_authority: pubkeys[1],
            new_collect_protocol_fees_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetCollectProtocolFeesAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCollectProtocolFeesAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.collect_protocol_fees_authority.clone(),
            accounts.new_collect_protocol_fees_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetCollectProtocolFeesAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            collect_protocol_fees_authority: &arr[1],
            new_collect_protocol_fees_authority: &arr[2],
        }
    }
}
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    34, 150, 93, 244, 139, 225, 233, 67,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetCollectProtocolFeesAuthorityIxData;
impl SetCollectProtocolFeesAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_collect_protocol_fees_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCollectProtocolFeesAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_COLLECT_PROTOCOL_FEES_AUTHORITY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetCollectProtocolFeesAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_collect_protocol_fees_authority_ix(
    keys: SetCollectProtocolFeesAuthorityKeys,
) -> std::io::Result<Instruction> {
    set_collect_protocol_fees_authority_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn set_collect_protocol_fees_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetCollectProtocolFeesAuthorityKeys = accounts.into();
    let ix = set_collect_protocol_fees_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_collect_protocol_fees_authority_invoke(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_collect_protocol_fees_authority_invoke_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
    )
}
pub fn set_collect_protocol_fees_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCollectProtocolFeesAuthorityKeys = accounts.into();
    let ix = set_collect_protocol_fees_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_collect_protocol_fees_authority_invoke_signed(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_collect_protocol_fees_authority_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_collect_protocol_fees_authority_verify_account_keys(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'_, '_>,
    keys: SetCollectProtocolFeesAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (
            *accounts.collect_protocol_fees_authority.key,
            keys.collect_protocol_fees_authority,
        ),
        (
            *accounts.new_collect_protocol_fees_authority.key,
            keys.new_collect_protocol_fees_authority,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_collect_protocol_fees_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_collect_protocol_fees_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_protocol_fees_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_collect_protocol_fees_authority_verify_account_privileges<'me, 'info>(
    accounts: SetCollectProtocolFeesAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_collect_protocol_fees_authority_verify_writable_privileges(accounts)?;
    set_collect_protocol_fees_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetDefaultProtocolFeeRateAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetDefaultProtocolFeeRateKeys {
    pub fusion_pools_config: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetDefaultProtocolFeeRateAccounts<'_, '_>> for SetDefaultProtocolFeeRateKeys {
    fn from(accounts: SetDefaultProtocolFeeRateAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetDefaultProtocolFeeRateKeys>
for [AccountMeta; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetDefaultProtocolFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetDefaultProtocolFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            fee_authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetDefaultProtocolFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetDefaultProtocolFeeRateAccounts<'_, 'info>) -> Self {
        [accounts.fusion_pools_config.clone(), accounts.fee_authority.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetDefaultProtocolFeeRateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            fee_authority: &arr[1],
        }
    }
}
pub const SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM: [u8; 8usize] = [
    107, 205, 249, 226, 151, 35, 86, 0,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetDefaultProtocolFeeRateIxArgs {
    pub default_protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDefaultProtocolFeeRateIxData(pub SetDefaultProtocolFeeRateIxArgs);
impl From<SetDefaultProtocolFeeRateIxArgs> for SetDefaultProtocolFeeRateIxData {
    fn from(args: SetDefaultProtocolFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetDefaultProtocolFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let default_protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetDefaultProtocolFeeRateIxArgs {
                default_protocol_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DEFAULT_PROTOCOL_FEE_RATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.default_protocol_fee_rate,
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
pub fn set_default_protocol_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetDefaultProtocolFeeRateKeys,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_DEFAULT_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetDefaultProtocolFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_default_protocol_fee_rate_ix(
    keys: SetDefaultProtocolFeeRateKeys,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_default_protocol_fee_rate_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn set_default_protocol_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetDefaultProtocolFeeRateKeys = accounts.into();
    let ix = set_default_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_default_protocol_fee_rate_invoke(
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
) -> ProgramResult {
    set_default_protocol_fee_rate_invoke_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_default_protocol_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetDefaultProtocolFeeRateKeys = accounts.into();
    let ix = set_default_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_default_protocol_fee_rate_invoke_signed(
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    args: SetDefaultProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_default_protocol_fee_rate_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_default_protocol_fee_rate_verify_account_keys(
    accounts: SetDefaultProtocolFeeRateAccounts<'_, '_>,
    keys: SetDefaultProtocolFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_default_protocol_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetDefaultProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_default_protocol_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetDefaultProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_default_protocol_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetDefaultProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_default_protocol_fee_rate_verify_writable_privileges(accounts)?;
    set_default_protocol_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAuthorityAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub new_fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeAuthorityKeys {
    pub fusion_pools_config: Pubkey,
    pub fee_authority: Pubkey,
    pub new_fee_authority: Pubkey,
}
impl From<SetFeeAuthorityAccounts<'_, '_>> for SetFeeAuthorityKeys {
    fn from(accounts: SetFeeAuthorityAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            fee_authority: *accounts.fee_authority.key,
            new_fee_authority: *accounts.new_fee_authority.key,
        }
    }
}
impl From<SetFeeAuthorityKeys> for [AccountMeta; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_fee_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]> for SetFeeAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            fee_authority: pubkeys[1],
            new_fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetFeeAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.fee_authority.clone(),
            accounts.new_fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetFeeAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            fee_authority: &arr[1],
            new_fee_authority: &arr[2],
        }
    }
}
pub const SET_FEE_AUTHORITY_IX_DISCM: [u8; 8usize] = [31, 1, 50, 87, 237, 101, 97, 132];
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeAuthorityIxData;
impl SetFeeAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetFeeAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_fee_authority_ix(keys: SetFeeAuthorityKeys) -> std::io::Result<Instruction> {
    set_fee_authority_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn set_fee_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetFeeAuthorityKeys = accounts.into();
    let ix = set_fee_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_authority_invoke(
    accounts: SetFeeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_fee_authority_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn set_fee_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeAuthorityKeys = accounts.into();
    let ix = set_fee_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_authority_invoke_signed(
    accounts: SetFeeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_authority_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_fee_authority_verify_account_keys(
    accounts: SetFeeAuthorityAccounts<'_, '_>,
    keys: SetFeeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.new_fee_authority.key, keys.new_fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_authority_verify_account_privileges<'me, 'info>(
    accounts: SetFeeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_authority_verify_writable_privileges(accounts)?;
    set_fee_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeRateAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetFeeRateKeys {
    pub fusion_pools_config: Pubkey,
    pub fusion_pool: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetFeeRateAccounts<'_, '_>> for SetFeeRateKeys {
    fn from(accounts: SetFeeRateAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            fusion_pool: *accounts.fusion_pool.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetFeeRateKeys> for [AccountMeta; SET_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_RATE_IX_ACCOUNTS_LEN]> for SetFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            fusion_pool: pubkeys[1],
            fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.fusion_pool.clone(),
            accounts.fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetFeeRateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            fusion_pool: &arr[1],
            fee_authority: &arr[2],
        }
    }
}
pub const SET_FEE_RATE_IX_DISCM: [u8; 8usize] = [53, 243, 137, 65, 8, 140, 158, 6];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeRateIxArgs {
    pub fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeRateIxData(pub SetFeeRateIxArgs);
impl From<SetFeeRateIxArgs> for SetFeeRateIxData {
    fn from(args: SetFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetFeeRateIxArgs { fee_rate }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_RATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeRateKeys,
    args: SetFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_rate_ix(
    keys: SetFeeRateKeys,
    args: SetFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_fee_rate_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn set_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetFeeRateKeys = accounts.into();
    let ix = set_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_rate_invoke(
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
) -> ProgramResult {
    set_fee_rate_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn set_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeRateKeys = accounts.into();
    let ix = set_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_rate_invoke_signed(
    accounts: SetFeeRateAccounts<'_, '_>,
    args: SetFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_rate_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_fee_rate_verify_account_keys(
    accounts: SetFeeRateAccounts<'_, '_>,
    keys: SetFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_rate_verify_writable_privileges(accounts)?;
    set_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POSITION_RANGE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetPositionRangeAccounts<'me, 'info> {
    pub position_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPositionRangeKeys {
    pub position_authority: Pubkey,
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub fusion_pool: Pubkey,
}
impl From<SetPositionRangeAccounts<'_, '_>> for SetPositionRangeKeys {
    fn from(accounts: SetPositionRangeAccounts) -> Self {
        Self {
            position_authority: *accounts.position_authority.key,
            position: *accounts.position.key,
            position_token_account: *accounts.position_token_account.key,
            fusion_pool: *accounts.fusion_pool.key,
        }
    }
}
impl From<SetPositionRangeKeys> for [AccountMeta; SET_POSITION_RANGE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPositionRangeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POSITION_RANGE_IX_ACCOUNTS_LEN]> for SetPositionRangeKeys {
    fn from(pubkeys: [Pubkey; SET_POSITION_RANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_authority: pubkeys[0],
            position: pubkeys[1],
            position_token_account: pubkeys[2],
            fusion_pool: pubkeys[3],
        }
    }
}
impl<'info> From<SetPositionRangeAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POSITION_RANGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPositionRangeAccounts<'_, 'info>) -> Self {
        [
            accounts.position_authority.clone(),
            accounts.position.clone(),
            accounts.position_token_account.clone(),
            accounts.fusion_pool.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POSITION_RANGE_IX_ACCOUNTS_LEN]>
for SetPositionRangeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POSITION_RANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_authority: &arr[0],
            position: &arr[1],
            position_token_account: &arr[2],
            fusion_pool: &arr[3],
        }
    }
}
pub const SET_POSITION_RANGE_IX_DISCM: [u8; 8usize] = [
    192, 22, 176, 176, 155, 49, 153, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPositionRangeIxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPositionRangeIxData(pub SetPositionRangeIxArgs);
impl From<SetPositionRangeIxArgs> for SetPositionRangeIxData {
    fn from(args: SetPositionRangeIxArgs) -> Self {
        Self(args)
    }
}
impl SetPositionRangeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POSITION_RANGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetPositionRangeIxArgs {
                tick_lower_index,
                tick_upper_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POSITION_RANGE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_upper_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_position_range_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPositionRangeKeys,
    args: SetPositionRangeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POSITION_RANGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPositionRangeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_position_range_ix(
    keys: SetPositionRangeKeys,
    args: SetPositionRangeIxArgs,
) -> std::io::Result<Instruction> {
    set_position_range_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn set_position_range_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPositionRangeAccounts<'_, '_>,
    args: SetPositionRangeIxArgs,
) -> ProgramResult {
    let keys: SetPositionRangeKeys = accounts.into();
    let ix = set_position_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_position_range_invoke(
    accounts: SetPositionRangeAccounts<'_, '_>,
    args: SetPositionRangeIxArgs,
) -> ProgramResult {
    set_position_range_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn set_position_range_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPositionRangeAccounts<'_, '_>,
    args: SetPositionRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPositionRangeKeys = accounts.into();
    let ix = set_position_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_position_range_invoke_signed(
    accounts: SetPositionRangeAccounts<'_, '_>,
    args: SetPositionRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_position_range_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_position_range_verify_account_keys(
    accounts: SetPositionRangeAccounts<'_, '_>,
    keys: SetPositionRangeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_authority.key, keys.position_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.fusion_pool.key, keys.fusion_pool),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_position_range_verify_writable_privileges<'me, 'info>(
    accounts: SetPositionRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position, accounts.position_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_position_range_verify_signer_privileges<'me, 'info>(
    accounts: SetPositionRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_position_range_verify_account_privileges<'me, 'info>(
    accounts: SetPositionRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_position_range_verify_writable_privileges(accounts)?;
    set_position_range_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeRateAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetProtocolFeeRateKeys {
    pub fusion_pools_config: Pubkey,
    pub fusion_pool: Pubkey,
    pub fee_authority: Pubkey,
}
impl From<SetProtocolFeeRateAccounts<'_, '_>> for SetProtocolFeeRateKeys {
    fn from(accounts: SetProtocolFeeRateAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            fusion_pool: *accounts.fusion_pool.key,
            fee_authority: *accounts.fee_authority.key,
        }
    }
}
impl From<SetProtocolFeeRateKeys>
for [AccountMeta; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetProtocolFeeRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]> for SetProtocolFeeRateKeys {
    fn from(pubkeys: [Pubkey; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            fusion_pool: pubkeys[1],
            fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetProtocolFeeRateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetProtocolFeeRateAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.fusion_pool.clone(),
            accounts.fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN]>
for SetProtocolFeeRateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            fusion_pool: &arr[1],
            fee_authority: &arr[2],
        }
    }
}
pub const SET_PROTOCOL_FEE_RATE_IX_DISCM: [u8; 8usize] = [
    95, 7, 4, 50, 154, 79, 156, 131,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetProtocolFeeRateIxArgs {
    pub protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetProtocolFeeRateIxData(pub SetProtocolFeeRateIxArgs);
impl From<SetProtocolFeeRateIxArgs> for SetProtocolFeeRateIxData {
    fn from(args: SetProtocolFeeRateIxArgs) -> Self {
        Self(args)
    }
}
impl SetProtocolFeeRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PROTOCOL_FEE_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetProtocolFeeRateIxArgs {
                protocol_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PROTOCOL_FEE_RATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.protocol_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_protocol_fee_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetProtocolFeeRateKeys,
    args: SetProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PROTOCOL_FEE_RATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetProtocolFeeRateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_protocol_fee_rate_ix(
    keys: SetProtocolFeeRateKeys,
    args: SetProtocolFeeRateIxArgs,
) -> std::io::Result<Instruction> {
    set_protocol_fee_rate_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn set_protocol_fee_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
) -> ProgramResult {
    let keys: SetProtocolFeeRateKeys = accounts.into();
    let ix = set_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_protocol_fee_rate_invoke(
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
) -> ProgramResult {
    set_protocol_fee_rate_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn set_protocol_fee_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetProtocolFeeRateKeys = accounts.into();
    let ix = set_protocol_fee_rate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_protocol_fee_rate_invoke_signed(
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    args: SetProtocolFeeRateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_protocol_fee_rate_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_protocol_fee_rate_verify_account_keys(
    accounts: SetProtocolFeeRateAccounts<'_, '_>,
    keys: SetProtocolFeeRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.fee_authority.key, keys.fee_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_rate_verify_writable_privileges<'me, 'info>(
    accounts: SetProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_rate_verify_signer_privileges<'me, 'info>(
    accounts: SetProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_rate_verify_account_privileges<'me, 'info>(
    accounts: SetProtocolFeeRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_protocol_fee_rate_verify_writable_privileges(accounts)?;
    set_protocol_fee_rate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetTokenBadgeAuthorityAccounts<'me, 'info> {
    pub fusion_pools_config: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub new_token_badge_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetTokenBadgeAuthorityKeys {
    pub fusion_pools_config: Pubkey,
    pub fee_authority: Pubkey,
    pub new_token_badge_authority: Pubkey,
}
impl From<SetTokenBadgeAuthorityAccounts<'_, '_>> for SetTokenBadgeAuthorityKeys {
    fn from(accounts: SetTokenBadgeAuthorityAccounts) -> Self {
        Self {
            fusion_pools_config: *accounts.fusion_pools_config.key,
            fee_authority: *accounts.fee_authority.key,
            new_token_badge_authority: *accounts.new_token_badge_authority.key,
        }
    }
}
impl From<SetTokenBadgeAuthorityKeys>
for [AccountMeta; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetTokenBadgeAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pools_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_token_badge_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetTokenBadgeAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pools_config: pubkeys[0],
            fee_authority: pubkeys[1],
            new_token_badge_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetTokenBadgeAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetTokenBadgeAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pools_config.clone(),
            accounts.fee_authority.clone(),
            accounts.new_token_badge_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN]>
for SetTokenBadgeAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fusion_pools_config: &arr[0],
            fee_authority: &arr[1],
            new_token_badge_authority: &arr[2],
        }
    }
}
pub const SET_TOKEN_BADGE_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    207, 202, 4, 32, 205, 79, 13, 178,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetTokenBadgeAuthorityIxData;
impl SetTokenBadgeAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_TOKEN_BADGE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_TOKEN_BADGE_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_token_badge_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetTokenBadgeAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_TOKEN_BADGE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetTokenBadgeAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_token_badge_authority_ix(
    keys: SetTokenBadgeAuthorityKeys,
) -> std::io::Result<Instruction> {
    set_token_badge_authority_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn set_token_badge_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenBadgeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetTokenBadgeAuthorityKeys = accounts.into();
    let ix = set_token_badge_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_token_badge_authority_invoke(
    accounts: SetTokenBadgeAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_token_badge_authority_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn set_token_badge_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenBadgeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetTokenBadgeAuthorityKeys = accounts.into();
    let ix = set_token_badge_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_token_badge_authority_invoke_signed(
    accounts: SetTokenBadgeAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_token_badge_authority_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_token_badge_authority_verify_account_keys(
    accounts: SetTokenBadgeAuthorityAccounts<'_, '_>,
    keys: SetTokenBadgeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pools_config.key, keys.fusion_pools_config),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.new_token_badge_authority.key, keys.new_token_badge_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_token_badge_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetTokenBadgeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pools_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_token_badge_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetTokenBadgeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_token_badge_authority_verify_account_privileges<'me, 'info>(
    accounts: SetTokenBadgeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_token_badge_authority_verify_writable_privileges(accounts)?;
    set_token_badge_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub fusion_pool: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_0: &'me AccountInfo<'info>,
    pub tick_array_1: &'me AccountInfo<'info>,
    pub tick_array_2: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
    pub token_authority: Pubkey,
    pub fusion_pool: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array_0: Pubkey,
    pub tick_array_1: Pubkey,
    pub tick_array_2: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
            token_authority: *accounts.token_authority.key,
            fusion_pool: *accounts.fusion_pool.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array_0: *accounts.tick_array_0.key,
            tick_array_1: *accounts.tick_array_1.key,
            tick_array_2: *accounts.tick_array_2.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_2,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program_a: pubkeys[0],
            token_program_b: pubkeys[1],
            memo_program: pubkeys[2],
            token_authority: pubkeys[3],
            fusion_pool: pubkeys[4],
            token_mint_a: pubkeys[5],
            token_mint_b: pubkeys[6],
            token_owner_account_a: pubkeys[7],
            token_owner_account_b: pubkeys[8],
            token_vault_a: pubkeys[9],
            token_vault_b: pubkeys[10],
            tick_array_0: pubkeys[11],
            tick_array_1: pubkeys[12],
            tick_array_2: pubkeys[13],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
            accounts.token_authority.clone(),
            accounts.fusion_pool.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array_0.clone(),
            accounts.tick_array_1.clone(),
            accounts.tick_array_2.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program_a: &arr[0],
            token_program_b: &arr[1],
            memo_program: &arr[2],
            token_authority: &arr[3],
            fusion_pool: &arr[4],
            token_mint_a: &arr[5],
            token_mint_b: &arr[6],
            token_owner_account_a: &arr[7],
            token_owner_account_b: &arr[8],
            token_vault_a: &arr[9],
            token_vault_b: &arr[10],
            tick_array_0: &arr[11],
            tick_array_1: &arr[12],
            tick_array_2: &arr[13],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8usize] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
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
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_specified_is_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapIxArgs {
                amount,
                other_amount_threshold,
                sqrt_price_limit,
                amount_specified_is_input,
                a_to_b,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.other_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.amount_specified_is_input,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
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
    swap_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
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
    swap_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
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
    swap_invoke_signed_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array_0.key, keys.tick_array_0),
        (*accounts.tick_array_1.key, keys.tick_array_1),
        (*accounts.tick_array_2.key, keys.tick_array_2),
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
        accounts.fusion_pool,
        accounts.token_owner_account_a,
        accounts.token_owner_account_b,
        accounts.token_vault_a,
        accounts.token_vault_b,
        accounts.tick_array_0,
        accounts.tick_array_1,
        accounts.tick_array_2,
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
    for should_be_signer in [accounts.token_authority] {
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
pub const TWO_HOP_SWAP_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct TwoHopSwapAccounts<'me, 'info> {
    pub fusion_pool_one: &'me AccountInfo<'info>,
    pub fusion_pool_two: &'me AccountInfo<'info>,
    pub token_mint_input: &'me AccountInfo<'info>,
    pub token_mint_intermediate: &'me AccountInfo<'info>,
    pub token_mint_output: &'me AccountInfo<'info>,
    pub token_program_input: &'me AccountInfo<'info>,
    pub token_program_intermediate: &'me AccountInfo<'info>,
    pub token_program_output: &'me AccountInfo<'info>,
    pub token_owner_account_input: &'me AccountInfo<'info>,
    pub token_vault_one_input: &'me AccountInfo<'info>,
    pub token_vault_one_intermediate: &'me AccountInfo<'info>,
    pub token_vault_two_intermediate: &'me AccountInfo<'info>,
    pub token_vault_two_output: &'me AccountInfo<'info>,
    pub token_owner_account_output: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub tick_array_one_0: &'me AccountInfo<'info>,
    pub tick_array_one_1: &'me AccountInfo<'info>,
    pub tick_array_one_2: &'me AccountInfo<'info>,
    pub tick_array_two_0: &'me AccountInfo<'info>,
    pub tick_array_two_1: &'me AccountInfo<'info>,
    pub tick_array_two_2: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TwoHopSwapKeys {
    pub fusion_pool_one: Pubkey,
    pub fusion_pool_two: Pubkey,
    pub token_mint_input: Pubkey,
    pub token_mint_intermediate: Pubkey,
    pub token_mint_output: Pubkey,
    pub token_program_input: Pubkey,
    pub token_program_intermediate: Pubkey,
    pub token_program_output: Pubkey,
    pub token_owner_account_input: Pubkey,
    pub token_vault_one_input: Pubkey,
    pub token_vault_one_intermediate: Pubkey,
    pub token_vault_two_intermediate: Pubkey,
    pub token_vault_two_output: Pubkey,
    pub token_owner_account_output: Pubkey,
    pub token_authority: Pubkey,
    pub tick_array_one_0: Pubkey,
    pub tick_array_one_1: Pubkey,
    pub tick_array_one_2: Pubkey,
    pub tick_array_two_0: Pubkey,
    pub tick_array_two_1: Pubkey,
    pub tick_array_two_2: Pubkey,
    pub memo_program: Pubkey,
}
impl From<TwoHopSwapAccounts<'_, '_>> for TwoHopSwapKeys {
    fn from(accounts: TwoHopSwapAccounts) -> Self {
        Self {
            fusion_pool_one: *accounts.fusion_pool_one.key,
            fusion_pool_two: *accounts.fusion_pool_two.key,
            token_mint_input: *accounts.token_mint_input.key,
            token_mint_intermediate: *accounts.token_mint_intermediate.key,
            token_mint_output: *accounts.token_mint_output.key,
            token_program_input: *accounts.token_program_input.key,
            token_program_intermediate: *accounts.token_program_intermediate.key,
            token_program_output: *accounts.token_program_output.key,
            token_owner_account_input: *accounts.token_owner_account_input.key,
            token_vault_one_input: *accounts.token_vault_one_input.key,
            token_vault_one_intermediate: *accounts.token_vault_one_intermediate.key,
            token_vault_two_intermediate: *accounts.token_vault_two_intermediate.key,
            token_vault_two_output: *accounts.token_vault_two_output.key,
            token_owner_account_output: *accounts.token_owner_account_output.key,
            token_authority: *accounts.token_authority.key,
            tick_array_one_0: *accounts.tick_array_one_0.key,
            tick_array_one_1: *accounts.tick_array_one_1.key,
            tick_array_one_2: *accounts.tick_array_one_2.key,
            tick_array_two_0: *accounts.tick_array_two_0.key,
            tick_array_two_1: *accounts.tick_array_two_1.key,
            tick_array_two_2: *accounts.tick_array_two_2.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<TwoHopSwapKeys> for [AccountMeta; TWO_HOP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: TwoHopSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pool_one,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fusion_pool_two,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_input,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_intermediate,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_output,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_input,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_intermediate,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_output,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_input,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_one_input,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_one_intermediate,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_two_intermediate,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_two_output,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_output,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_one_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_one_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_one_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_two_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_two_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_two_2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]> for TwoHopSwapKeys {
    fn from(pubkeys: [Pubkey; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool_one: pubkeys[0],
            fusion_pool_two: pubkeys[1],
            token_mint_input: pubkeys[2],
            token_mint_intermediate: pubkeys[3],
            token_mint_output: pubkeys[4],
            token_program_input: pubkeys[5],
            token_program_intermediate: pubkeys[6],
            token_program_output: pubkeys[7],
            token_owner_account_input: pubkeys[8],
            token_vault_one_input: pubkeys[9],
            token_vault_one_intermediate: pubkeys[10],
            token_vault_two_intermediate: pubkeys[11],
            token_vault_two_output: pubkeys[12],
            token_owner_account_output: pubkeys[13],
            token_authority: pubkeys[14],
            tick_array_one_0: pubkeys[15],
            tick_array_one_1: pubkeys[16],
            tick_array_one_2: pubkeys[17],
            tick_array_two_0: pubkeys[18],
            tick_array_two_1: pubkeys[19],
            tick_array_two_2: pubkeys[20],
            memo_program: pubkeys[21],
        }
    }
}
impl<'info> From<TwoHopSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; TWO_HOP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: TwoHopSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pool_one.clone(),
            accounts.fusion_pool_two.clone(),
            accounts.token_mint_input.clone(),
            accounts.token_mint_intermediate.clone(),
            accounts.token_mint_output.clone(),
            accounts.token_program_input.clone(),
            accounts.token_program_intermediate.clone(),
            accounts.token_program_output.clone(),
            accounts.token_owner_account_input.clone(),
            accounts.token_vault_one_input.clone(),
            accounts.token_vault_one_intermediate.clone(),
            accounts.token_vault_two_intermediate.clone(),
            accounts.token_vault_two_output.clone(),
            accounts.token_owner_account_output.clone(),
            accounts.token_authority.clone(),
            accounts.tick_array_one_0.clone(),
            accounts.tick_array_one_1.clone(),
            accounts.tick_array_one_2.clone(),
            accounts.tick_array_two_0.clone(),
            accounts.tick_array_two_1.clone(),
            accounts.tick_array_two_2.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]>
for TwoHopSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TWO_HOP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool_one: &arr[0],
            fusion_pool_two: &arr[1],
            token_mint_input: &arr[2],
            token_mint_intermediate: &arr[3],
            token_mint_output: &arr[4],
            token_program_input: &arr[5],
            token_program_intermediate: &arr[6],
            token_program_output: &arr[7],
            token_owner_account_input: &arr[8],
            token_vault_one_input: &arr[9],
            token_vault_one_intermediate: &arr[10],
            token_vault_two_intermediate: &arr[11],
            token_vault_two_output: &arr[12],
            token_owner_account_output: &arr[13],
            token_authority: &arr[14],
            tick_array_one_0: &arr[15],
            tick_array_one_1: &arr[16],
            tick_array_one_2: &arr[17],
            tick_array_two_0: &arr[18],
            tick_array_two_1: &arr[19],
            tick_array_two_2: &arr[20],
            memo_program: &arr[21],
        }
    }
}
pub const TWO_HOP_SWAP_IX_DISCM: [u8; 8usize] = [195, 96, 237, 108, 68, 162, 219, 230];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TwoHopSwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TwoHopSwapIxData(pub TwoHopSwapIxArgs);
impl From<TwoHopSwapIxArgs> for TwoHopSwapIxData {
    fn from(args: TwoHopSwapIxArgs) -> Self {
        Self(args)
    }
}
impl TwoHopSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TWO_HOP_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_specified_is_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b_one: bool = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b_two: bool = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit_one: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit_two: u128 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_accounts_info: Option<RemainingAccountsInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(TwoHopSwapIxArgs {
                amount,
                other_amount_threshold,
                amount_specified_is_input,
                a_to_b_one,
                a_to_b_two,
                sqrt_price_limit_one,
                sqrt_price_limit_two,
                remaining_accounts_info,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TWO_HOP_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.other_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.amount_specified_is_input,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b_two, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit_two, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.remaining_accounts_info, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn two_hop_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: TwoHopSwapKeys,
    args: TwoHopSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TWO_HOP_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: TwoHopSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn two_hop_swap_ix(
    keys: TwoHopSwapKeys,
    args: TwoHopSwapIxArgs,
) -> std::io::Result<Instruction> {
    two_hop_swap_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys, args)
}
pub fn two_hop_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
) -> ProgramResult {
    let keys: TwoHopSwapKeys = accounts.into();
    let ix = two_hop_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn two_hop_swap_invoke(
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
) -> ProgramResult {
    two_hop_swap_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, args)
}
pub fn two_hop_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TwoHopSwapKeys = accounts.into();
    let ix = two_hop_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn two_hop_swap_invoke_signed(
    accounts: TwoHopSwapAccounts<'_, '_>,
    args: TwoHopSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    two_hop_swap_invoke_signed_with_program_id(
        FUSIONAMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn two_hop_swap_verify_account_keys(
    accounts: TwoHopSwapAccounts<'_, '_>,
    keys: TwoHopSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pool_one.key, keys.fusion_pool_one),
        (*accounts.fusion_pool_two.key, keys.fusion_pool_two),
        (*accounts.token_mint_input.key, keys.token_mint_input),
        (*accounts.token_mint_intermediate.key, keys.token_mint_intermediate),
        (*accounts.token_mint_output.key, keys.token_mint_output),
        (*accounts.token_program_input.key, keys.token_program_input),
        (*accounts.token_program_intermediate.key, keys.token_program_intermediate),
        (*accounts.token_program_output.key, keys.token_program_output),
        (*accounts.token_owner_account_input.key, keys.token_owner_account_input),
        (*accounts.token_vault_one_input.key, keys.token_vault_one_input),
        (*accounts.token_vault_one_intermediate.key, keys.token_vault_one_intermediate),
        (*accounts.token_vault_two_intermediate.key, keys.token_vault_two_intermediate),
        (*accounts.token_vault_two_output.key, keys.token_vault_two_output),
        (*accounts.token_owner_account_output.key, keys.token_owner_account_output),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.tick_array_one_0.key, keys.tick_array_one_0),
        (*accounts.tick_array_one_1.key, keys.tick_array_one_1),
        (*accounts.tick_array_one_2.key, keys.tick_array_one_2),
        (*accounts.tick_array_two_0.key, keys.tick_array_two_0),
        (*accounts.tick_array_two_1.key, keys.tick_array_two_1),
        (*accounts.tick_array_two_2.key, keys.tick_array_two_2),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn two_hop_swap_verify_writable_privileges<'me, 'info>(
    accounts: TwoHopSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fusion_pool_one,
        accounts.fusion_pool_two,
        accounts.token_owner_account_input,
        accounts.token_vault_one_input,
        accounts.token_vault_one_intermediate,
        accounts.token_vault_two_intermediate,
        accounts.token_vault_two_output,
        accounts.token_owner_account_output,
        accounts.tick_array_one_0,
        accounts.tick_array_one_1,
        accounts.tick_array_one_2,
        accounts.tick_array_two_0,
        accounts.tick_array_two_1,
        accounts.tick_array_two_2,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn two_hop_swap_verify_signer_privileges<'me, 'info>(
    accounts: TwoHopSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn two_hop_swap_verify_account_privileges<'me, 'info>(
    accounts: TwoHopSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    two_hop_swap_verify_writable_privileges(accounts)?;
    two_hop_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeesAccounts<'me, 'info> {
    pub fusion_pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeesKeys {
    pub fusion_pool: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
impl From<UpdateFeesAccounts<'_, '_>> for UpdateFeesKeys {
    fn from(accounts: UpdateFeesAccounts) -> Self {
        Self {
            fusion_pool: *accounts.fusion_pool.key,
            position: *accounts.position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
        }
    }
}
impl From<UpdateFeesKeys> for [AccountMeta; UPDATE_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fusion_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEES_IX_ACCOUNTS_LEN]> for UpdateFeesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: pubkeys[0],
            position: pubkeys[1],
            tick_array_lower: pubkeys[2],
            tick_array_upper: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.fusion_pool.clone(),
            accounts.position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEES_IX_ACCOUNTS_LEN]>
for UpdateFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fusion_pool: &arr[0],
            position: &arr[1],
            tick_array_lower: &arr[2],
            tick_array_upper: &arr[3],
        }
    }
}
pub const UPDATE_FEES_IX_DISCM: [u8; 8usize] = [225, 27, 13, 6, 69, 84, 172, 191];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeesIxData;
impl UpdateFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateFeesIxData.try_to_vec()?,
    })
}
pub fn update_fees_ix(keys: UpdateFeesKeys) -> std::io::Result<Instruction> {
    update_fees_ix_with_program_id(FUSIONAMM_PROGRAM_ID, keys)
}
pub fn update_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateFeesKeys = accounts.into();
    let ix = update_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fees_invoke(accounts: UpdateFeesAccounts<'_, '_>) -> ProgramResult {
    update_fees_invoke_with_program_id(FUSIONAMM_PROGRAM_ID, accounts)
}
pub fn update_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeesKeys = accounts.into();
    let ix = update_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fees_invoke_signed(
    accounts: UpdateFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fees_invoke_signed_with_program_id(FUSIONAMM_PROGRAM_ID, accounts, seeds)
}
pub fn update_fees_verify_account_keys(
    accounts: UpdateFeesAccounts<'_, '_>,
    keys: UpdateFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fusion_pool.key, keys.fusion_pool),
        (*accounts.position.key, keys.position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fees_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fusion_pool, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fees_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
