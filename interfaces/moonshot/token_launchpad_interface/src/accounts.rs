use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CONFIG_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [189, 255, 97, 70, 186, 189, 24, 102];
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
pub struct ConfigAccount {
    pub migration_authority: Pubkey,
    pub backend_authority: Pubkey,
    pub config_authority: Pubkey,
    pub helio_fee: Pubkey,
    pub dex_fee: Pubkey,
    pub fee_bps: u16,
    pub dex_fee_share: u8,
    pub migration_fee: u64,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub min_supported_decimal_places: u8,
    pub max_supported_decimal_places: u8,
    pub min_supported_token_supply: u64,
    pub max_supported_token_supply: u64,
    pub bump: u8,
    pub coef_b: u32,
}
impl ConfigAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let migration_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let backend_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let helio_fee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dex_fee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let dex_fee_share: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let marketcap_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let marketcap_currency: Currency = crate::borsh_de_or_default(&mut reader)?;
        let min_supported_decimal_places: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_supported_decimal_places: u8 = crate::borsh_de_or_default(&mut reader)?;
        let min_supported_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_supported_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let coef_b: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            migration_authority,
            backend_authority,
            config_authority,
            helio_fee,
            dex_fee,
            fee_bps,
            dex_fee_share,
            migration_fee,
            marketcap_threshold,
            marketcap_currency,
            min_supported_decimal_places,
            max_supported_decimal_places,
            min_supported_token_supply,
            max_supported_token_supply,
            bump,
            coef_b,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.migration_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.backend_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.helio_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dex_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dex_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.marketcap_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.marketcap_currency, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_supported_decimal_places,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_supported_decimal_places,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_supported_token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supported_token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coef_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccountAccount(pub ConfigAccount);
impl ConfigAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ConfigAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CURVE_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [8, 91, 83, 28, 132, 216, 248, 22];
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
pub struct CurveAccount {
    pub total_supply: u64,
    pub curve_amount: u64,
    pub mint: Pubkey,
    pub decimals: u8,
    pub collateral_currency: Currency,
    pub curve_type: CurveType,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub migration_fee: u64,
    pub coef_b: u32,
    pub bump: u8,
}
impl CurveAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_currency: Currency = crate::borsh_de_or_default(&mut reader)?;
        let curve_type: CurveType = crate::borsh_de_or_default(&mut reader)?;
        let marketcap_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let marketcap_currency: Currency = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coef_b: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_supply,
            curve_amount,
            mint,
            decimals,
            collateral_currency,
            curve_type,
            marketcap_threshold,
            marketcap_currency,
            migration_fee,
            coef_b,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_currency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.marketcap_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.marketcap_currency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coef_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CurveAccountAccount(pub CurveAccount);
impl CurveAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CURVE_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CurveAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CURVE_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
