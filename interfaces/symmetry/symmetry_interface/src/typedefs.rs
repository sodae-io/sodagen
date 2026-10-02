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
pub struct Rule {
    pub filter_by: u64,
    pub filter_days: u64,
    pub sort_by: u64,
    pub total_weight: u64,
    pub fixed_asset: u64,
    pub num_assets: u64,
    pub weight_by: u64,
    pub weight_days: u64,
    pub weight_expo: i64,
    pub exclude_num: u64,
    pub exclude_assets: [u64; 10],
    pub rule_assets: [u64; 20],
}
impl Rule {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let filter_by: u64 = crate::borsh_de_or_default(&mut reader)?;
        let filter_days: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sort_by: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_weight: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_asset: u64 = crate::borsh_de_or_default(&mut reader)?;
        let num_assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let weight_by: u64 = crate::borsh_de_or_default(&mut reader)?;
        let weight_days: u64 = crate::borsh_de_or_default(&mut reader)?;
        let weight_expo: i64 = crate::borsh_de_or_default(&mut reader)?;
        let exclude_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exclude_assets: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let rule_assets: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            filter_by,
            filter_days,
            sort_by,
            total_weight,
            fixed_asset,
            num_assets,
            weight_by,
            weight_days,
            weight_expo,
            exclude_num,
            exclude_assets,
            rule_assets,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenSettings {
    pub token_mint: Pubkey,
    pub decimals: u8,
    pub coingecko_id: [u8; 30],
    pub pda_token_account: Pubkey,
    pub oracle_type: u8,
    pub oracle_account: Pubkey,
    pub oracle_index: u8,
    pub oracle_confidence_pct: u8,
    pub fixed_confidence_bps: u8,
    pub token_swap_fee_after_tw_bps: u8,
    pub token_swap_fee_before_tw_bps: u8,
    pub is_live: u8,
    pub lp_on: u8,
    pub use_curve_data: u8,
    #[serde(with = "crate::big_array_serde")]
    pub additional_data: [u8; 63],
}
impl TokenSettings {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let coingecko_id: [u8; 30] = crate::borsh_de_or_default(&mut reader)?;
        let pda_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_confidence_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_confidence_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_swap_fee_after_tw_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_swap_fee_before_tw_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_live: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_on: u8 = crate::borsh_de_or_default(&mut reader)?;
        let use_curve_data: u8 = crate::borsh_de_or_default(&mut reader)?;
        let additional_data = <[u8; 63] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            token_mint,
            decimals,
            coingecko_id,
            pda_token_account,
            oracle_type,
            oracle_account,
            oracle_index,
            oracle_confidence_pct,
            fixed_confidence_bps,
            token_swap_fee_after_tw_bps,
            token_swap_fee_before_tw_bps,
            is_live,
            lp_on,
            use_curve_data,
            additional_data,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenData {
    #[serde(with = "crate::big_array_serde")]
    pub price: [u64; 460],
    #[serde(with = "crate::big_array_serde")]
    pub circulating_supply: [u64; 460],
    #[serde(with = "crate::big_array_serde")]
    pub volume: [u64; 460],
    #[serde(with = "crate::big_array_serde")]
    pub timestamp: [u64; 460],
    pub index: u64,
}
impl TokenData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price = <[u64; 460] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let circulating_supply = <[u64; 460] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let volume = <[u64; 460] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let timestamp = <[u64; 460] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price,
            circulating_supply,
            volume,
            timestamp,
            index,
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
pub struct Stats {
    pub days: u64,
    pub performance: u64,
    pub volume: u64,
    pub mcap: u64,
}
impl Stats {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let days: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mcap: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            days,
            performance,
            volume,
            mcap,
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
pub struct TokenPriceData {
    pub amount: [u64; 10],
    pub price: [u64; 10],
}
impl TokenPriceData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let price: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount, price })
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
pub struct UpdateMetadataParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl UpdateMetadataParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol, uri })
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
pub enum PriceStatus {
    #[default]
    Unknown,
    Trading,
    Halted,
    Auction,
}
impl TryFrom<u8> for PriceStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Unknown),
            1u8 => Ok(Self::Trading),
            2u8 => Ok(Self::Halted),
            3u8 => Ok(Self::Auction),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum CorpAction {
    #[default]
    NoCorpAct,
}
impl TryFrom<u8> for CorpAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::NoCorpAct),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum PriceType {
    #[default]
    Unknown,
    Price,
    Twap,
    Volatility,
}
impl TryFrom<u8> for PriceType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Unknown),
            1u8 => Ok(Self::Price),
            2u8 => Ok(Self::Twap),
            3u8 => Ok(Self::Volatility),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
