use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const MARKET_ACCOUNT_DISCM: [u8; 8] = [219, 190, 213, 55, 0, 227, 198, 154];
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
pub struct Market {
    pub config: Pubkey,
    pub creator: Pubkey,
    pub swap_authority: Option<Pubkey>,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub reserve_0: Pubkey,
    pub reserve_1: Pubkey,
    pub fee_reserve: Option<Pubkey>,
    pub fee_reserve_last_update: i64,
    pub settings: MarketSettings,
    pub sqrt_price_x96: u128,
    pub bump: [u8; 1],
}
impl Market {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_reserve: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let fee_reserve_last_update: i64 = crate::borsh_de_or_default(&mut reader)?;
        let settings = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSettings>::deserialize(&mut reader)?
        };
        let sqrt_price_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            creator,
            swap_authority,
            token_mint_0,
            token_mint_1,
            reserve_0,
            reserve_1,
            fee_reserve,
            fee_reserve_last_update,
            settings,
            sqrt_price_x96,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_reserve_last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settings, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x96, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketAccount(pub Market);
impl MarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Market::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_MILL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    28, 200, 141, 206, 141, 183, 203, 16,
];
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
pub struct TokenMillConfig {
    pub admin: Pubkey,
    pub quote_token_mint: Pubkey,
    pub protocol_fee_share: u32,
    pub protocol_fee_reserve: Pubkey,
    pub creator_fee_pool: Pubkey,
    pub fee_recipient_change_cooldown: u32,
    pub default_market_settings: MarketSettings,
}
impl TokenMillConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_recipient_change_cooldown: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let default_market_settings = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSettings>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            admin,
            quote_token_mint,
            protocol_fee_share,
            protocol_fee_reserve,
            creator_fee_pool,
            fee_recipient_change_cooldown,
            default_market_settings,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_recipient_change_cooldown,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.default_market_settings, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenMillConfigAccount(pub TokenMillConfig);
impl TokenMillConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_MILL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenMillConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_MILL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
