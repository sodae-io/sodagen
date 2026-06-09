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
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_reserve: u64,
    pub bid_prices: [u64; 11],
    pub ask_prices: [u64; 11],
    pub width_scaled: u64,
    pub total_supply: u64,
    pub fees: MarketFees,
    pub quote_token_decimals: u8,
    pub bump: u8,
    pub _space: [u8; 6],
}
impl Market {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bid_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        let ask_prices: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        let width_scaled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <MarketFees>::deserialize(&mut reader)?
        };
        let quote_token_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _space: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            creator,
            base_token_mint,
            quote_token_mint,
            base_reserve,
            bid_prices,
            ask_prices,
            width_scaled,
            total_supply,
            fees,
            quote_token_decimals,
            bump,
            _space,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bid_prices, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ask_prices, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.width_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._space, &mut writer)?;
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
pub const MARKET_STAKING_ACCOUNT_DISCM: [u8; 8] = [17, 179, 11, 222, 30, 156, 211, 86];
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
pub struct MarketStaking {
    pub market: Pubkey,
    pub amount_staked: u64,
    pub total_amount_vested: u64,
    pub acc_reward_amount_per_share: u128,
}
impl MarketStaking {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_staked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_amount_vested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let acc_reward_amount_per_share: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            amount_staked,
            total_amount_vested,
            acc_reward_amount_per_share,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_staked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_amount_vested, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.acc_reward_amount_per_share,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketStakingAccount(pub MarketStaking);
impl MarketStakingAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_STAKING_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarketStaking::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_STAKING_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const QUOTE_TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [52, 32, 57, 85, 7, 186, 76, 229];
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
pub struct QuoteTokenBadge {
    pub bump: u8,
    pub status: QuoteTokenBadgeStatus,
}
impl QuoteTokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: QuoteTokenBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, status })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteTokenBadgeAccount(pub QuoteTokenBadge);
impl QuoteTokenBadgeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_TOKEN_BADGE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(QuoteTokenBadge::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_TOKEN_BADGE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRAL_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [237, 162, 80, 78, 196, 233, 91, 2];
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
pub struct ReferralAccount {
    pub bump: u8,
    pub config: Pubkey,
    pub referrer: Pubkey,
}
impl ReferralAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, config, referrer })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferralAccountAccount(pub ReferralAccount);
impl ReferralAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRAL_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ReferralAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRAL_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKE_POSITION_ACCOUNT_DISCM: [u8; 8] = [78, 165, 30, 111, 171, 125, 11, 220];
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
pub struct StakePosition {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount_staked: u64,
    pub total_amount_vested: u64,
    pub pending_rewards: u64,
    pub acc_reward_amount_per_share: u128,
}
impl StakePosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_staked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_amount_vested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let acc_reward_amount_per_share: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            user,
            amount_staked,
            total_amount_vested,
            pending_rewards,
            acc_reward_amount_per_share,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_staked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_amount_vested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.acc_reward_amount_per_share,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakePositionAccount(pub StakePosition);
impl StakePositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StakePosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_POSITION_ACCOUNT_DISCM)?;
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
    pub authority: Pubkey,
    pub pending_authority: Option<Pubkey>,
    pub protocol_fee_recipient: Pubkey,
    pub default_protocol_fee_share: u16,
    pub referral_fee_share: u16,
}
impl TokenMillConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let default_protocol_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            pending_authority,
            protocol_fee_recipient,
            default_protocol_fee_share,
            referral_fee_share,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_protocol_fee_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral_fee_share, &mut writer)?;
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
pub const VESTING_PLAN_ACCOUNT_DISCM: [u8; 8] = [220, 100, 188, 22, 177, 159, 229, 3];
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
pub struct VestingPlan {
    pub stake_position: Pubkey,
    pub amount_vested: u64,
    pub amount_released: u64,
    pub start: i64,
    pub cliff_duration: i64,
    pub vesting_duration: i64,
}
impl VestingPlan {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stake_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_vested: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_released: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            stake_position,
            amount_vested,
            amount_released,
            start,
            cliff_duration,
            vesting_duration,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stake_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_vested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_released, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cliff_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vesting_duration, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VestingPlanAccount(pub VestingPlan);
impl VestingPlanAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VESTING_PLAN_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VestingPlan::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VESTING_PLAN_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
