use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const WOO_AMM_POOL_ACCOUNT_DISCM: [u8; 8] = [52, 110, 239, 45, 133, 94, 195, 180];
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
pub struct WooAmmPool {
    pub wooconfig: Pubkey,
    pub wooammpool_bump: [u8; 1],
    pub authority: Pubkey,
    pub wooracle_a: Pubkey,
    pub woopool_a: Pubkey,
    pub feed_account_a: Pubkey,
    pub price_update_a: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub wooracle_b: Pubkey,
    pub woopool_b: Pubkey,
    pub feed_account_b: Pubkey,
    pub price_update_b: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub quote_token_mint: Pubkey,
    pub quote_feed_account: Pubkey,
    pub quote_price_update: Pubkey,
    pub quote_woopool: Pubkey,
    pub quote_vault: Pubkey,
}
impl WooAmmPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let wooconfig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wooammpool_bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wooracle_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let woopool_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let feed_account_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price_update_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wooracle_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let woopool_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let feed_account_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price_update_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_feed_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_price_update: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_woopool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            wooconfig,
            wooammpool_bump,
            authority,
            wooracle_a,
            woopool_a,
            feed_account_a,
            price_update_a,
            token_mint_a,
            token_vault_a,
            wooracle_b,
            woopool_b,
            feed_account_b,
            price_update_b,
            token_mint_b,
            token_vault_b,
            quote_token_mint,
            quote_feed_account,
            quote_price_update,
            quote_woopool,
            quote_vault,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.wooconfig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wooammpool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wooracle_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.woopool_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.feed_account_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_update_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wooracle_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.woopool_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.feed_account_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_update_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_feed_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_price_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_woopool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_vault, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WooAmmPoolAccount(pub WooAmmPool);
impl WooAmmPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WOO_AMM_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WooAmmPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WOO_AMM_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WOO_CONFIG_ACCOUNT_DISCM: [u8; 8] = [232, 1, 149, 51, 160, 111, 135, 176];
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
pub struct WooConfig {
    pub authority: Pubkey,
    pub paused: bool,
    pub woopool_admin_authority: Vec<Pubkey>,
    pub wooracle_admin_authority: Vec<Pubkey>,
    pub fee_authority: Vec<Pubkey>,
    pub guardian_authority: Vec<Pubkey>,
    pub pause_authority: Vec<Pubkey>,
    pub lending_manager_authority: Vec<Pubkey>,
    pub supercharger_vault_whitelist: Vec<Pubkey>,
    pub new_authority: Pubkey,
}
impl WooConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let woopool_admin_authority: Vec<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let wooracle_admin_authority: Vec<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let guardian_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let pause_authority: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let lending_manager_authority: Vec<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let supercharger_vault_whitelist: Vec<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            paused,
            woopool_admin_authority,
            wooracle_admin_authority,
            fee_authority,
            guardian_authority,
            pause_authority,
            lending_manager_authority,
            supercharger_vault_whitelist,
            new_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.woopool_admin_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wooracle_admin_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.guardian_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pause_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_manager_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.supercharger_vault_whitelist,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WooConfigAccount(pub WooConfig);
impl WooConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WOO_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WooConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WOO_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WOO_POOL_ACCOUNT_DISCM: [u8; 8] = [179, 77, 61, 217, 39, 85, 13, 227];
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
pub struct WooPool {
    pub wooconfig: Pubkey,
    pub woopool_bump: [u8; 1],
    pub authority: Pubkey,
    pub wooracle: Pubkey,
    pub fee_rate: u16,
    pub max_gamma: u128,
    pub max_notional_swap: u128,
    pub cap_bal: u128,
    pub min_swap_amount: u128,
    pub unclaimed_fee: u128,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_decimals: u8,
}
impl WooPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let wooconfig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let woopool_bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wooracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_gamma: u128 = crate::borsh_de_or_default(&mut reader)?;
        let max_notional_swap: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cap_bal: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_swap_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unclaimed_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            wooconfig,
            woopool_bump,
            authority,
            wooracle,
            fee_rate,
            max_gamma,
            max_notional_swap,
            cap_bal,
            min_swap_amount,
            unclaimed_fee,
            token_mint,
            token_vault,
            quote_token_mint,
            base_decimals,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.wooconfig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.woopool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wooracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_gamma, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_notional_swap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cap_bal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_swap_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unclaimed_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_decimals, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WooPoolAccount(pub WooPool);
impl WooPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WOO_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WooPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WOO_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WOORACLE_ACCOUNT_DISCM: [u8; 8] = [130, 213, 224, 3, 126, 58, 126, 73];
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
pub struct Wooracle {
    pub wooconfig: Pubkey,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub feed_account: Pubkey,
    pub price_update: Pubkey,
    pub maximum_age: u64,
    pub price_decimals: u8,
    pub quote_decimals: u8,
    pub base_decimals: u8,
    pub updated_at: i64,
    pub stale_duration: i64,
    pub bound: u64,
    pub price: u128,
    pub coeff: u64,
    pub spread: u64,
    pub range_min: u128,
    pub range_max: u128,
    pub quote_token_mint: Pubkey,
    pub quote_feed_account: Pubkey,
    pub quote_price_update: Pubkey,
}
impl Wooracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let wooconfig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let feed_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price_update: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maximum_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let stale_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bound: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let coeff: u64 = crate::borsh_de_or_default(&mut reader)?;
        let spread: u64 = crate::borsh_de_or_default(&mut reader)?;
        let range_min: u128 = crate::borsh_de_or_default(&mut reader)?;
        let range_max: u128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_feed_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_price_update: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            wooconfig,
            authority,
            token_mint,
            feed_account,
            price_update,
            maximum_age,
            price_decimals,
            quote_decimals,
            base_decimals,
            updated_at,
            stale_duration,
            bound,
            price,
            coeff,
            spread,
            range_min,
            range_max,
            quote_token_mint,
            quote_feed_account,
            quote_price_update,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.wooconfig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.feed_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maximum_age, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stale_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bound, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coeff, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spread, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.range_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.range_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_feed_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_price_update, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WooracleAccount(pub Wooracle);
impl WooracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WOORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Wooracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WOORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
