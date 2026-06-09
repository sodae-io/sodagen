use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const DELTAFI_USER_ACCOUNT_DISCM: [u8; 8] = [167, 33, 81, 146, 13, 12, 38, 238];
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
pub struct DeltafiUser {
    pub bump: u8,
    pub config_key: Pubkey,
    pub owner: Pubkey,
    pub referrer: Pubkey,
    pub owed_trade_rewards: u64,
    pub claimed_trade_rewards: u64,
    pub owed_referral_rewards: u64,
    pub claimed_referral_rewards: u64,
    pub reserved: [u64; 32],
}
impl DeltafiUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owed_trade_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let claimed_trade_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owed_referral_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let claimed_referral_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            config_key,
            owner,
            referrer,
            owed_trade_rewards,
            claimed_trade_rewards,
            owed_referral_rewards,
            claimed_referral_rewards,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owed_trade_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claimed_trade_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owed_referral_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claimed_referral_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeltafiUserAccount(pub DeltafiUser);
impl DeltafiUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELTAFI_USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DeltafiUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELTAFI_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FARM_USER_ACCOUNT_DISCM: [u8; 8] = [7, 69, 96, 60, 36, 121, 187, 124];
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
pub struct FarmUser {
    pub bump: u8,
    pub config_key: Pubkey,
    pub farm_key: Pubkey,
    pub owner: Pubkey,
    pub base_position: FarmPosition,
    pub quote_position: FarmPosition,
    pub reserved: [u64; 32],
}
impl FarmUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_position = if reader.is_empty() {
            Default::default()
        } else {
            <FarmPosition>::deserialize(&mut reader)?
        };
        let quote_position = if reader.is_empty() {
            Default::default()
        } else {
            <FarmPosition>::deserialize(&mut reader)?
        };
        let reserved: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            config_key,
            farm_key,
            owner,
            base_position,
            quote_position,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FarmUserAccount(pub FarmUser);
impl FarmUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FARM_USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FarmUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FARM_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FARM_INFO_ACCOUNT_DISCM: [u8; 8] = [121, 41, 216, 82, 80, 123, 74, 223];
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
pub struct FarmInfo {
    pub bump: u8,
    pub seed: Pubkey,
    pub config_key: Pubkey,
    pub swap_key: Pubkey,
    pub staked_base_share: u64,
    pub staked_quote_share: u64,
    pub farm_config: FarmConfig,
    pub reserved: [u64; 32],
}
impl FarmInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let staked_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let staked_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let farm_config = if reader.is_empty() {
            Default::default()
        } else {
            <FarmConfig>::deserialize(&mut reader)?
        };
        let reserved: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            seed,
            config_key,
            swap_key,
            staked_base_share,
            staked_quote_share,
            farm_config,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staked_base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staked_quote_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FarmInfoAccount(pub FarmInfo);
impl FarmInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FARM_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FarmInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FARM_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_CONFIG_ACCOUNT_DISCM: [u8; 8] = [119, 255, 200, 88, 252, 82, 128, 24];
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
pub struct MarketConfig {
    pub version: u8,
    pub bump: u8,
    pub seed: Pubkey,
    pub admin_key: Pubkey,
    pub deltafi_mint: Pubkey,
    pub deltafi_token: Pubkey,
    pub pyth_program_id: Pubkey,
    pub serum_program_id: Pubkey,
    pub reserved_u64: [u64; 32],
}
impl MarketConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deltafi_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deltafi_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pyth_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved_u64: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            version,
            bump,
            seed,
            admin_key,
            deltafi_mint,
            deltafi_token,
            pyth_program_id,
            serum_program_id,
            reserved_u64,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deltafi_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deltafi_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pyth_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_u64, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketConfigAccount(pub MarketConfig);
impl MarketConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarketConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_INFO_ACCOUNT_DISCM: [u8; 8] = [204, 115, 6, 6, 209, 226, 41, 242];
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
pub struct SwapInfo {
    pub is_initialized: bool,
    pub bump: u8,
    pub seed: Pubkey,
    pub swap_type: SwapType,
    pub config_key: Pubkey,
    pub mint_base: Pubkey,
    pub mint_quote: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub admin_fee_token_base: Pubkey,
    pub admin_fee_token_quote: Pubkey,
    pub mint_base_decimals: u8,
    pub mint_quote_decimals: u8,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub pool_state: PoolState,
    pub swap_config: SwapConfig,
    pub reserved_u64: [u64; 24],
}
impl SwapInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_quote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_quote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_fee_token_base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_fee_token_quote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_quote_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pyth_price_base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pyth_price_quote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_bids: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_asks: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_state = if reader.is_empty() {
            Default::default()
        } else {
            <PoolState>::deserialize(&mut reader)?
        };
        let swap_config = if reader.is_empty() {
            Default::default()
        } else {
            <SwapConfig>::deserialize(&mut reader)?
        };
        let reserved_u64: [u64; 24] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_initialized,
            bump,
            seed,
            swap_type,
            config_key,
            mint_base,
            mint_quote,
            token_base,
            token_quote,
            admin_fee_token_base,
            admin_fee_token_quote,
            mint_base_decimals,
            mint_quote_decimals,
            pyth_price_base,
            pyth_price_quote,
            serum_market,
            serum_bids,
            serum_asks,
            pool_state,
            swap_config,
            reserved_u64,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_initialized, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_fee_token_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_fee_token_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_base_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_quote_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pyth_price_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pyth_price_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_u64, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapInfoAccount(pub SwapInfo);
impl SwapInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_PROVIDER_ACCOUNT_DISCM: [u8; 8] = [
    219, 241, 238, 133, 56, 225, 229, 191,
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
pub struct LiquidityProvider {
    pub bump: u8,
    pub config_key: Pubkey,
    pub swap_key: Pubkey,
    pub owner: Pubkey,
    pub base_share: u64,
    pub quote_share: u64,
    pub staked_base_share: u64,
    pub staked_quote_share: u64,
    pub deprecated_u64: [u64; 10],
    pub reserved_u64: [u64; 32],
}
impl LiquidityProvider {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let staked_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let staked_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deprecated_u64: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let reserved_u64: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            config_key,
            swap_key,
            owner,
            base_share,
            quote_share,
            staked_base_share,
            staked_quote_share,
            deprecated_u64,
            reserved_u64,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staked_base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staked_quote_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deprecated_u64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_u64, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityProviderAccount(pub LiquidityProvider);
impl LiquidityProviderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_PROVIDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LiquidityProvider::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_PROVIDER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
