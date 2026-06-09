use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const EVT_CLAIM_POSITION_FEE_EVENT_DISCM: [u8; 8] = [
    198, 182, 183, 52, 97, 12, 49, 56,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimPositionFee {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub fee_a_claimed: u64,
    pub fee_b_claimed: u64,
}
impl EvtClaimPositionFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_a_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            position,
            owner,
            fee_a_claimed,
            fee_b_claimed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a_claimed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_b_claimed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimPositionFeeEvent(pub EvtClaimPositionFee);
impl EvtClaimPositionFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_POSITION_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimPositionFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_POSITION_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLAIM_PROTOCOL_FEE_EVENT_DISCM: [u8; 8] = [
    186, 244, 75, 251, 188, 13, 25, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimProtocolFee {
    pub pool: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
impl EvtClaimProtocolFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            token_a_amount,
            token_b_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimProtocolFeeEvent(pub EvtClaimProtocolFee);
impl EvtClaimProtocolFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_PROTOCOL_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimProtocolFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_PROTOCOL_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLAIM_REWARD_EVENT_DISCM: [u8; 8] = [
    218, 86, 147, 200, 235, 188, 215, 231,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClaimReward {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub mint_reward: Pubkey,
    pub reward_index: u8,
    pub total_reward: u64,
}
impl EvtClaimReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_reward: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            position,
            owner,
            mint_reward,
            reward_index,
            total_reward,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_reward, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimRewardEvent(pub EvtClaimReward);
impl EvtClaimRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLAIM_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClaimReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLAIM_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLOSE_CONFIG_EVENT_DISCM: [u8; 8] = [36, 30, 239, 45, 58, 132, 14, 5];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCloseConfig {
    pub config: Pubkey,
    pub admin: Pubkey,
}
impl EvtCloseConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { config, admin })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCloseConfigEvent(pub EvtCloseConfig);
impl EvtCloseConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLOSE_CONFIG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCloseConfig::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLOSE_CONFIG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CLOSE_POSITION_EVENT_DISCM: [u8; 8] = [
    20, 145, 144, 68, 143, 142, 214, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtClosePosition {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
}
impl EvtClosePosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position,
            position_nft_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClosePositionEvent(pub EvtClosePosition);
impl EvtClosePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CLOSE_POSITION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtClosePosition::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CLOSE_POSITION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_CONFIG_EVENT_DISCM: [u8; 8] = [
    131, 207, 180, 174, 180, 73, 165, 54,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfig {
    pub pool_fees: PoolFeeParameters,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub collect_fee_mode: u8,
    pub index: u64,
    pub config: Pubkey,
}
impl EvtCreateConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeeParameters>::deserialize(&mut reader)?
        };
        let vault_config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            vault_config_key,
            pool_creator_authority,
            activation_type,
            sqrt_min_price,
            sqrt_max_price,
            collect_fee_mode,
            index,
            config,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_min_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_max_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigEvent(pub EvtCreateConfig);
impl EvtCreateConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_CONFIG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateConfig::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_CONFIG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_DYNAMIC_CONFIG_EVENT_DISCM: [u8; 8] = [
    231, 197, 13, 164, 248, 213, 133, 152,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateDynamicConfig {
    pub config: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub index: u64,
}
impl EvtCreateDynamicConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            config,
            pool_creator_authority,
            index,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateDynamicConfigEvent(pub EvtCreateDynamicConfig);
impl EvtCreateDynamicConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_DYNAMIC_CONFIG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateDynamicConfig::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_DYNAMIC_CONFIG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_POSITION_EVENT_DISCM: [u8; 8] = [
    156, 15, 119, 198, 29, 181, 221, 55,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreatePosition {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
}
impl EvtCreatePosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            position,
            position_nft_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreatePositionEvent(pub EvtCreatePosition);
impl EvtCreatePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_POSITION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreatePosition::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_POSITION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_CREATE_TOKEN_BADGE_EVENT_DISCM: [u8; 8] = [
    141, 120, 134, 116, 34, 28, 114, 160,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtCreateTokenBadge {
    pub token_mint: Pubkey,
}
impl EvtCreateTokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_mint })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateTokenBadgeEvent(pub EvtCreateTokenBadge);
impl EvtCreateTokenBadgeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_CREATE_TOKEN_BADGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtCreateTokenBadge::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_CREATE_TOKEN_BADGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_FUND_REWARD_EVENT_DISCM: [u8; 8] = [104, 233, 237, 122, 199, 191, 121, 85];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtFundReward {
    pub pool: Pubkey,
    pub funder: Pubkey,
    pub mint_reward: Pubkey,
    pub reward_index: u8,
    pub amount: u64,
    pub transfer_fee_excluded_amount_in: u64,
    pub reward_duration_end: u64,
    pub pre_reward_rate: u128,
    pub post_reward_rate: u128,
}
impl EvtFundReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_reward: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_excluded_amount_in: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_duration_end: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pre_reward_rate: u128 = crate::borsh_de_or_default(&mut reader)?;
        let post_reward_rate: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            funder,
            mint_reward,
            reward_index,
            amount,
            transfer_fee_excluded_amount_in,
            reward_duration_end,
            pre_reward_rate,
            post_reward_rate,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.transfer_fee_excluded_amount_in,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reward_duration_end, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_reward_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_reward_rate, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtFundRewardEvent(pub EvtFundReward);
impl EvtFundRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_FUND_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtFundReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_FUND_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_INITIALIZE_POOL_EVENT_DISCM: [u8; 8] = [
    228, 50, 246, 85, 203, 66, 134, 37,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtInitializePool {
    pub pool: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub alpha_vault: Pubkey,
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
    pub pool_type: u8,
}
impl EvtInitializePool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let alpha_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeeParameters>::deserialize(&mut reader)?
        };
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            token_a_mint,
            token_b_mint,
            creator,
            payer,
            alpha_vault,
            pool_fees,
            sqrt_min_price,
            sqrt_max_price,
            activation_type,
            collect_fee_mode,
            liquidity,
            sqrt_price,
            activation_point,
            token_a_flag,
            token_b_flag,
            token_a_amount,
            token_b_amount,
            total_amount_a,
            total_amount_b,
            pool_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.alpha_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_min_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_max_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_amount_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializePoolEvent(pub EvtInitializePool);
impl EvtInitializePoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_INITIALIZE_POOL_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtInitializePool::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_INITIALIZE_POOL_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_INITIALIZE_REWARD_EVENT_DISCM: [u8; 8] = [
    129, 91, 188, 3, 246, 52, 185, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtInitializeReward {
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub funder: Pubkey,
    pub creator: Pubkey,
    pub reward_index: u8,
    pub reward_duration: u64,
}
impl EvtInitializeReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            reward_mint,
            funder,
            creator,
            reward_index,
            reward_duration,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_duration, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializeRewardEvent(pub EvtInitializeReward);
impl EvtInitializeRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_INITIALIZE_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtInitializeReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_INITIALIZE_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_LIQUIDITY_CHANGE_EVENT_DISCM: [u8; 8] = [
    197, 171, 78, 127, 224, 211, 87, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtLiquidityChange {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub transfer_fee_included_token_a_amount: u64,
    pub transfer_fee_included_token_b_amount: u64,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
    pub change_type: u8,
}
impl EvtLiquidityChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_included_token_a_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let transfer_fee_included_token_b_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reserve_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_delta: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let change_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            position,
            owner,
            token_a_amount,
            token_b_amount,
            transfer_fee_included_token_a_amount,
            transfer_fee_included_token_b_amount,
            reserve_a_amount,
            reserve_b_amount,
            liquidity_delta,
            token_a_amount_threshold,
            token_b_amount_threshold,
            change_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.transfer_fee_included_token_a_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.transfer_fee_included_token_b_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserve_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.change_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtLiquidityChangeEvent(pub EvtLiquidityChange);
impl EvtLiquidityChangeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_LIQUIDITY_CHANGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtLiquidityChange::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_LIQUIDITY_CHANGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_LOCK_POSITION_EVENT_DISCM: [u8; 8] = [168, 63, 108, 83, 219, 82, 2, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtLockPosition {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub vesting: Pubkey,
    pub cliff_point: u64,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}
impl EvtLockPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vesting: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cliff_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let period_frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_unlock_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_per_period: u128 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            position,
            owner,
            vesting,
            cliff_point,
            period_frequency,
            cliff_unlock_liquidity,
            liquidity_per_period,
            number_of_period,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vesting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cliff_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.period_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cliff_unlock_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_per_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_period, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtLockPositionEvent(pub EvtLockPosition);
impl EvtLockPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_LOCK_POSITION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtLockPosition::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_LOCK_POSITION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_PERMANENT_LOCK_POSITION_EVENT_DISCM: [u8; 8] = [
    145, 143, 162, 218, 218, 80, 67, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtPermanentLockPosition {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub lock_liquidity_amount: u128,
    pub total_permanent_locked_liquidity: u128,
}
impl EvtPermanentLockPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lock_liquidity_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_permanent_locked_liquidity: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool,
            position,
            lock_liquidity_amount,
            total_permanent_locked_liquidity,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lock_liquidity_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_permanent_locked_liquidity,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtPermanentLockPositionEvent(pub EvtPermanentLockPosition);
impl EvtPermanentLockPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_PERMANENT_LOCK_POSITION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtPermanentLockPosition::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_PERMANENT_LOCK_POSITION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SET_POOL_STATUS_EVENT_DISCM: [u8; 8] = [100, 213, 74, 3, 95, 91, 228, 146];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSetPoolStatus {
    pub pool: Pubkey,
    pub status: u8,
}
impl EvtSetPoolStatus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, status })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSetPoolStatusEvent(pub EvtSetPoolStatus);
impl EvtSetPoolStatusEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SET_POOL_STATUS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSetPoolStatus::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SET_POOL_STATUS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SPLIT_POSITION2_EVENT_DISCM: [u8; 8] = [
    165, 32, 203, 174, 72, 100, 233, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSplitPosition2 {
    pub pool: Pubkey,
    pub first_owner: Pubkey,
    pub second_owner: Pubkey,
    pub first_position: Pubkey,
    pub second_position: Pubkey,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo,
    pub first_position_info: SplitPositionInfo,
    pub second_position_info: SplitPositionInfo,
    pub split_position_parameters: SplitPositionParameters2,
}
impl EvtSplitPosition2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let first_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let second_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let first_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let second_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_splits = if reader.is_empty() {
            Default::default()
        } else {
            <SplitAmountInfo>::deserialize(&mut reader)?
        };
        let first_position_info = if reader.is_empty() {
            Default::default()
        } else {
            <SplitPositionInfo>::deserialize(&mut reader)?
        };
        let second_position_info = if reader.is_empty() {
            Default::default()
        } else {
            <SplitPositionInfo>::deserialize(&mut reader)?
        };
        let split_position_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <SplitPositionParameters2>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            pool,
            first_owner,
            second_owner,
            first_position,
            second_position,
            current_sqrt_price,
            amount_splits,
            first_position_info,
            second_position_info,
            split_position_parameters,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_splits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_position_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_position_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.split_position_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSplitPosition2Event(pub EvtSplitPosition2);
impl EvtSplitPosition2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SPLIT_POSITION2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSplitPosition2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SPLIT_POSITION2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SPLIT_POSITION3_EVENT_DISCM: [u8; 8] = [
    232, 117, 190, 218, 85, 162, 207, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSplitPosition3 {
    pub pool: Pubkey,
    pub first_owner: Pubkey,
    pub second_owner: Pubkey,
    pub first_position: Pubkey,
    pub second_position: Pubkey,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo2,
    pub first_position_info: SplitPositionInfo2,
    pub second_position_info: SplitPositionInfo2,
    pub split_position_parameters: SplitPositionParameters3,
}
impl EvtSplitPosition3 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let first_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let second_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let first_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let second_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_splits = if reader.is_empty() {
            Default::default()
        } else {
            <SplitAmountInfo2>::deserialize(&mut reader)?
        };
        let first_position_info = if reader.is_empty() {
            Default::default()
        } else {
            <SplitPositionInfo2>::deserialize(&mut reader)?
        };
        let second_position_info = if reader.is_empty() {
            Default::default()
        } else {
            <SplitPositionInfo2>::deserialize(&mut reader)?
        };
        let split_position_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <SplitPositionParameters3>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            pool,
            first_owner,
            second_owner,
            first_position,
            second_position,
            current_sqrt_price,
            amount_splits,
            first_position_info,
            second_position_info,
            split_position_parameters,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_splits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_position_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_position_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.split_position_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSplitPosition3Event(pub EvtSplitPosition3);
impl EvtSplitPosition3Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SPLIT_POSITION3_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSplitPosition3::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SPLIT_POSITION3_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_SWAP2_EVENT_DISCM: [u8; 8] = [189, 66, 51, 168, 38, 80, 117, 153];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtSwap2 {
    pub pool: Pubkey,
    pub trade_direction: u8,
    pub collect_fee_mode: u8,
    pub has_referral: bool,
    pub params: SwapParameters2,
    pub swap_result: SwapResult2,
    pub included_transfer_fee_amount_in: u64,
    pub included_transfer_fee_amount_out: u64,
    pub excluded_transfer_fee_amount_out: u64,
    pub current_timestamp: u64,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
}
impl EvtSwap2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_direction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_referral: bool = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SwapParameters2>::deserialize(&mut reader)?
        };
        let swap_result = if reader.is_empty() {
            Default::default()
        } else {
            <SwapResult2>::deserialize(&mut reader)?
        };
        let included_transfer_fee_amount_in: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let included_transfer_fee_amount_out: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let excluded_transfer_fee_amount_out: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let current_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            trade_direction,
            collect_fee_mode,
            has_referral,
            params,
            swap_result,
            included_transfer_fee_amount_in,
            included_transfer_fee_amount_out,
            excluded_transfer_fee_amount_out,
            current_timestamp,
            reserve_a_amount,
            reserve_b_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_result, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.included_transfer_fee_amount_in,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.included_transfer_fee_amount_out,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.excluded_transfer_fee_amount_out,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.current_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_b_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwap2Event(pub EvtSwap2);
impl EvtSwap2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_SWAP2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtSwap2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_SWAP2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_UPDATE_POOL_FEES_EVENT_DISCM: [u8; 8] = [
    76, 165, 246, 102, 102, 217, 156, 44,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtUpdatePoolFees {
    pub pool: Pubkey,
    pub operator: Pubkey,
    pub params: UpdatePoolFeesParameters,
}
impl EvtUpdatePoolFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdatePoolFeesParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { pool, operator, params })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdatePoolFeesEvent(pub EvtUpdatePoolFees);
impl EvtUpdatePoolFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_UPDATE_POOL_FEES_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtUpdatePoolFees::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_UPDATE_POOL_FEES_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_UPDATE_REWARD_DURATION_EVENT_DISCM: [u8; 8] = [
    149, 135, 65, 231, 129, 153, 65, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtUpdateRewardDuration {
    pub pool: Pubkey,
    pub reward_index: u8,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}
impl EvtUpdateRewardDuration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let old_reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            reward_index,
            old_reward_duration,
            new_reward_duration,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_reward_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reward_duration, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdateRewardDurationEvent(pub EvtUpdateRewardDuration);
impl EvtUpdateRewardDurationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_UPDATE_REWARD_DURATION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtUpdateRewardDuration::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_UPDATE_REWARD_DURATION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_UPDATE_REWARD_FUNDER_EVENT_DISCM: [u8; 8] = [
    76, 154, 208, 13, 40, 115, 246, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtUpdateRewardFunder {
    pub pool: Pubkey,
    pub reward_index: u8,
    pub old_funder: Pubkey,
    pub new_funder: Pubkey,
}
impl EvtUpdateRewardFunder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let old_funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            reward_index,
            old_funder,
            new_funder,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_funder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_funder, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdateRewardFunderEvent(pub EvtUpdateRewardFunder);
impl EvtUpdateRewardFunderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_UPDATE_REWARD_FUNDER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtUpdateRewardFunder::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_UPDATE_REWARD_FUNDER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVT_WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM: [u8; 8] = [
    248, 215, 184, 78, 31, 180, 179, 168,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvtWithdrawIneligibleReward {
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub amount: u64,
}
impl EvtWithdrawIneligibleReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, reward_mint, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtWithdrawIneligibleRewardEvent(pub EvtWithdrawIneligibleReward);
impl EvtWithdrawIneligibleRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVT_WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EvtWithdrawIneligibleReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVT_WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
