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
impl TryFrom<u8> for ErrorCode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Lok),
            1u8 => Ok(Self::NumberCastError),
            2u8 => Ok(Self::ZeroMintAmount),
            3u8 => Ok(Self::IntegerDowncastOverflow),
            4u8 => Ok(Self::MultiplicationOverflow),
            5u8 => Ok(Self::DivisorIsZero),
            6u8 => Ok(Self::TokenAmountMinSubceeded),
            7u8 => Ok(Self::TokenAmountMaxExceeded),
            8u8 => Ok(Self::SqrtPriceOutOfBounds),
            9u8 => Ok(Self::ProtocolFeeRateIllegal),
            10u8 => Ok(Self::FeeRateIllegal),
            11u8 => Ok(Self::TokenMintPairIllgal),
            12u8 => Ok(Self::TickArrayStartIndexIllegal),
            13u8 => Ok(Self::InvalidTickSpacing),
            14u8 => Ok(Self::InvalidTickIndex),
            15u8 => Ok(Self::InvalidTickArrayAccount),
            16u8 => Ok(Self::PositionIsNotEmpty),
            17u8 => Ok(Self::InvalidTokenAccountOwner),
            18u8 => Ok(Self::TickNotFound),
            19u8 => Ok(Self::TickNotInArray),
            20u8 => Ok(Self::InvalidTokenAccount),
            21u8 => Ok(Self::InvalidMint),
            22u8 => Ok(Self::InvalidAuthority),
            23u8 => Ok(Self::PositionAndClmmpoolNotMatch),
            24u8 => Ok(Self::PositionIllegal),
            25u8 => Ok(Self::InvalidDeltaLiquidity),
            26u8 => Ok(Self::ConfigAndPoolNotMatch),
            27u8 => Ok(Self::WrongSqrtPriceLimit),
            28u8 => Ok(Self::TickArrayNotFound),
            29u8 => Ok(Self::InvalidTickArrayIndex),
            30u8 => Ok(Self::NextTickNotFound),
            31u8 => Ok(Self::FeeGrowthIllegal),
            32u8 => Ok(Self::LiquidityOverflow),
            33u8 => Ok(Self::LiquidityUnderflow),
            34u8 => Ok(Self::RemainerAmountUnderflow),
            35u8 => Ok(Self::SwapAmountInOverflow),
            36u8 => Ok(Self::SwapAmountOutOverflow),
            37u8 => Ok(Self::SwapFeeAmountOverflow),
            38u8 => Ok(Self::InvalidTime),
            39u8 => Ok(Self::AmountInAboveMaximumLimit),
            40u8 => Ok(Self::AmountOutBelowMaximumLimit),
            41u8 => Ok(Self::InvalidFixedTokenType),
            42u8 => Ok(Self::InvalidRewarderIndex),
            43u8 => Ok(Self::InvalidPartner),
            44u8 => Ok(Self::InvalidClmmpoolStatus),
            45u8 => Ok(Self::InvalidClmmpoolMetadataAccount),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
