use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Rewarder {
    pub mint_wrapper: Pubkey,
    pub minter: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub emissions_per_second: u128,
    pub growth_global: u128,
}
impl Rewarder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint_wrapper: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let minter: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second: u128 = crate::borsh_de_or_default(&mut reader)?;
        let growth_global: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint_wrapper,
            minter,
            mint,
            authority,
            emissions_per_second,
            growth_global,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PositionReward {
    pub growth_inside: u128,
    pub amount_owed: u64,
}
impl PositionReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let growth_inside: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { growth_inside, amount_owed })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Tick {
    pub is_initialized: bool,
    pub index: i32,
    pub sqrt_price: u128,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growth_outside: [u128; 3],
}
impl Tick {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_net: i128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_gross: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_growth_outside: [u128; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_initialized,
            index,
            sqrt_price,
            liquidity_net,
            liquidity_gross,
            fee_growth_outside_a,
            fee_growth_outside_b,
            reward_growth_outside,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum ErrorCode {
    #[default]
    Lok,
    NumberCastError,
    ZeroMintAmount,
    IntegerDowncastOverflow,
    MultiplicationOverflow,
    DivisorIsZero,
    TokenAmountMinSubceeded,
    TokenAmountMaxExceeded,
    SqrtPriceOutOfBounds,
    ProtocolFeeRateIllegal,
    FeeRateIllegal,
    TokenMintPairIllgal,
    TickArrayStartIndexIllegal,
    InvalidTickSpacing,
    InvalidTickIndex,
    InvalidTickArrayAccount,
    PositionIsNotEmpty,
    InvalidTokenAccountOwner,
    TickNotFound,
    TickNotInArray,
    InvalidTokenAccount,
    InvalidMint,
    InvalidAuthority,
    PositionAndClmmpoolNotMatch,
    PositionIllegal,
    InvalidDeltaLiquidity,
    ConfigAndPoolNotMatch,
    WrongSqrtPriceLimit,
    TickArrayNotFound,
    InvalidTickArrayIndex,
    NextTickNotFound,
    FeeGrowthIllegal,
    LiquidityOverflow,
    LiquidityUnderflow,
    RemainerAmountUnderflow,
    SwapAmountInOverflow,
    SwapAmountOutOverflow,
    SwapFeeAmountOverflow,
    InvalidTime,
    AmountInAboveMaximumLimit,
    AmountOutBelowMaximumLimit,
    InvalidFixedTokenType,
    InvalidRewarderIndex,
    InvalidPartner,
    InvalidClmmpoolStatus,
    InvalidClmmpoolMetadataAccount,
}
