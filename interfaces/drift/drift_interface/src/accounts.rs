use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const AMM_CACHE_ACCOUNT_DISCM: [u8; 8] = [213, 114, 161, 56, 20, 22, 2, 59];
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
pub struct AmmCache {
    pub bump: u8,
    pub padding: [u8; 3],
    pub cache: Vec<CacheInfo>,
}
impl AmmCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let cache: Vec<CacheInfo> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, padding, cache })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cache, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmCacheAccount(pub AmmCache);
impl AmmCacheAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_CACHE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AmmCache::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_CACHE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPENBOOK_V2_FULFILLMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    3, 43, 58, 106, 131, 132, 199, 171,
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
pub struct OpenbookV2FulfillmentConfig {
    pub pubkey: Pubkey,
    pub openbook_v2_program_id: Pubkey,
    pub openbook_v2_market: Pubkey,
    pub openbook_v2_market_authority: Pubkey,
    pub openbook_v2_event_heap: Pubkey,
    pub openbook_v2_bids: Pubkey,
    pub openbook_v2_asks: Pubkey,
    pub openbook_v2_base_vault: Pubkey,
    pub openbook_v2_quote_vault: Pubkey,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
impl OpenbookV2FulfillmentConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_market_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let openbook_v2_event_heap: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_bids: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_asks: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let openbook_v2_quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fulfillment_type: SpotFulfillmentType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let status: SpotFulfillmentConfigStatus = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            openbook_v2_program_id,
            openbook_v2_market,
            openbook_v2_market_authority,
            openbook_v2_event_heap,
            openbook_v2_bids,
            openbook_v2_asks,
            openbook_v2_base_vault,
            openbook_v2_quote_vault,
            market_index,
            fulfillment_type,
            status,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_market, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.openbook_v2_market_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_event_heap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.openbook_v2_quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fulfillment_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenbookV2FulfillmentConfigAccount(pub OpenbookV2FulfillmentConfig);
impl OpenbookV2FulfillmentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPENBOOK_V2_FULFILLMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OpenbookV2FulfillmentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPENBOOK_V2_FULFILLMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    233, 45, 62, 40, 35, 129, 48, 72,
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
pub struct PhoenixV1FulfillmentConfig {
    pub pubkey: Pubkey,
    pub phoenix_program_id: Pubkey,
    pub phoenix_log_authority: Pubkey,
    pub phoenix_market: Pubkey,
    pub phoenix_base_vault: Pubkey,
    pub phoenix_quote_vault: Pubkey,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
impl PhoenixV1FulfillmentConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let phoenix_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let phoenix_log_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let phoenix_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let phoenix_base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let phoenix_quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fulfillment_type: SpotFulfillmentType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let status: SpotFulfillmentConfigStatus = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            phoenix_program_id,
            phoenix_log_authority,
            phoenix_market,
            phoenix_base_vault,
            phoenix_quote_vault,
            market_index,
            fulfillment_type,
            status,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.phoenix_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.phoenix_log_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.phoenix_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.phoenix_base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.phoenix_quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fulfillment_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PhoenixV1FulfillmentConfigAccount(pub PhoenixV1FulfillmentConfig);
impl PhoenixV1FulfillmentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PhoenixV1FulfillmentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    65, 160, 197, 112, 239, 168, 103, 185,
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
pub struct SerumV3FulfillmentConfig {
    pub pubkey: Pubkey,
    pub serum_program_id: Pubkey,
    pub serum_market: Pubkey,
    pub serum_request_queue: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_base_vault: Pubkey,
    pub serum_quote_vault: Pubkey,
    pub serum_open_orders: Pubkey,
    pub serum_signer_nonce: u64,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
impl SerumV3FulfillmentConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_request_queue: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_event_queue: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_bids: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_asks: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_open_orders: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let serum_signer_nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fulfillment_type: SpotFulfillmentType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let status: SpotFulfillmentConfigStatus = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            serum_program_id,
            serum_market,
            serum_request_queue,
            serum_event_queue,
            serum_bids,
            serum_asks,
            serum_base_vault,
            serum_quote_vault,
            serum_open_orders,
            serum_signer_nonce,
            market_index,
            fulfillment_type,
            status,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_request_queue, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_event_queue, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_open_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.serum_signer_nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fulfillment_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SerumV3FulfillmentConfigAccount(pub SerumV3FulfillmentConfig);
impl SerumV3FulfillmentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SerumV3FulfillmentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HIGH_LEVERAGE_MODE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    3, 196, 90, 189, 193, 64, 228, 234,
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
pub struct HighLeverageModeConfig {
    pub max_users: u32,
    pub current_users: u32,
    pub reduce_only: u8,
    pub padding1: [u8; 3],
    pub current_maintenance_users: u32,
    pub padding2: [u8; 24],
}
impl HighLeverageModeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let current_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reduce_only: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let current_maintenance_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u8; 24] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_users,
            current_users,
            reduce_only,
            padding1,
            current_maintenance_users,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.max_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduce_only, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_maintenance_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HighLeverageModeConfigAccount(pub HighLeverageModeConfig);
impl HighLeverageModeConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HIGH_LEVERAGE_MODE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(HighLeverageModeConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HIGH_LEVERAGE_MODE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const IF_REBALANCE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    214, 84, 40, 251, 107, 144, 173, 239,
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
pub struct IfRebalanceConfig {
    pub pubkey: Pubkey,
    pub total_in_amount: u64,
    pub current_in_amount: u64,
    pub current_out_amount: u64,
    pub current_out_amount_transferred: u64,
    pub current_in_amount_since_last_transfer: u64,
    pub epoch_start_ts: i64,
    pub epoch_in_amount: u64,
    pub epoch_max_in_amount: u64,
    pub epoch_duration: i64,
    pub out_market_index: u16,
    pub in_market_index: u16,
    pub max_slippage_bps: u16,
    pub swap_mode: u8,
    pub status: u8,
    pub padding2: [u8; 32],
}
impl IfRebalanceConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_out_amount_transferred: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let current_in_amount_since_last_transfer: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let epoch_start_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_max_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        let out_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let swap_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            total_in_amount,
            current_in_amount,
            current_out_amount,
            current_out_amount_transferred,
            current_in_amount_since_last_transfer,
            epoch_start_ts,
            epoch_in_amount,
            epoch_max_in_amount,
            epoch_duration,
            out_market_index,
            in_market_index,
            max_slippage_bps,
            swap_mode,
            status,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.current_out_amount_transferred,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.current_in_amount_since_last_transfer,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.epoch_start_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch_max_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_slippage_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfRebalanceConfigAccount(pub IfRebalanceConfig);
impl IfRebalanceConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != IF_REBALANCE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(IfRebalanceConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&IF_REBALANCE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSURANCE_FUND_STAKE_ACCOUNT_DISCM: [u8; 8] = [
    110, 202, 14, 42, 95, 73, 90, 95,
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
pub struct InsuranceFundStake {
    pub authority: Pubkey,
    pub if_shares: u128,
    pub last_withdraw_request_shares: u128,
    pub if_base: u128,
    pub last_valid_ts: i64,
    pub last_withdraw_request_value: u64,
    pub last_withdraw_request_ts: i64,
    pub cost_basis: i64,
    pub market_index: u16,
    pub padding: [u8; 14],
}
impl InsuranceFundStake {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let if_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_withdraw_request_shares: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let if_base: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_withdraw_request_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_withdraw_request_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cost_basis: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 14] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            if_shares,
            last_withdraw_request_shares,
            if_base,
            last_valid_ts,
            last_withdraw_request_value,
            last_withdraw_request_ts,
            cost_basis,
            market_index,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_withdraw_request_shares,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.if_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_valid_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_withdraw_request_value,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_withdraw_request_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cost_basis, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundStakeAccount(pub InsuranceFundStake);
impl InsuranceFundStakeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSURANCE_FUND_STAKE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InsuranceFundStake::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSURANCE_FUND_STAKE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_IF_SHARES_TRANSFER_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    188, 1, 213, 98, 23, 148, 30, 1,
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
pub struct ProtocolIfSharesTransferConfig {
    pub whitelisted_signers: [Pubkey; 4],
    pub max_transfer_per_epoch: u128,
    pub current_epoch_transfer: u128,
    pub next_epoch_ts: i64,
    pub padding: [u128; 8],
}
impl ProtocolIfSharesTransferConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whitelisted_signers: [Pubkey; 4] = crate::borsh_de_or_default(&mut reader)?;
        let max_transfer_per_epoch: u128 = crate::borsh_de_or_default(&mut reader)?;
        let current_epoch_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        let next_epoch_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whitelisted_signers,
            max_transfer_per_epoch,
            current_epoch_transfer,
            next_epoch_ts,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whitelisted_signers, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_transfer_per_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_epoch_transfer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_epoch_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolIfSharesTransferConfigAccount(pub ProtocolIfSharesTransferConfig);
impl ProtocolIfSharesTransferConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_IF_SHARES_TRANSFER_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProtocolIfSharesTransferConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_IF_SHARES_TRANSFER_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_POOL_ACCOUNT_DISCM: [u8; 8] = [228, 152, 141, 224, 161, 170, 11, 89];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LPPool {
    pub pubkey: Pubkey,
    pub mint: Pubkey,
    pub whitelist_mint: Pubkey,
    pub constituent_target_base: Pubkey,
    pub constituent_correlations: Pubkey,
    pub max_aum: u128,
    pub last_aum: u128,
    pub cumulative_quote_sent_to_perp_markets: u128,
    pub cumulative_quote_received_from_perp_markets: u128,
    pub total_mint_redeem_fees_paid: i128,
    pub last_aum_slot: u64,
    pub max_settle_quote_amount: u64,
    pub padding: u64,
    pub mint_redeem_id: u64,
    pub settle_id: u64,
    pub min_mint_fee: i64,
    pub token_supply: u64,
    pub volatility: u64,
    pub constituents: u16,
    pub quote_consituent_index: u16,
    pub bump: u8,
    pub gamma_execution: u8,
    pub xi: u8,
    pub target_oracle_delay_fee_bps_per10_slots: u8,
    pub target_position_delay_fee_bps_per10_slots: u8,
    pub lp_pool_id: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u8; 174],
}
impl LPPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let constituent_target_base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let constituent_correlations: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_aum: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_aum: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_quote_sent_to_perp_markets: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_quote_received_from_perp_markets: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_mint_redeem_fees_paid: i128 = crate::borsh_de_or_default(&mut reader)?;
        let last_aum_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_settle_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_redeem_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let settle_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_mint_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
        let token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volatility: u64 = crate::borsh_de_or_default(&mut reader)?;
        let constituents: u16 = crate::borsh_de_or_default(&mut reader)?;
        let quote_consituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let gamma_execution: u8 = crate::borsh_de_or_default(&mut reader)?;
        let xi: u8 = crate::borsh_de_or_default(&mut reader)?;
        let target_oracle_delay_fee_bps_per10_slots: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_position_delay_fee_bps_per10_slots: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lp_pool_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1 = <[u8; 174] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            mint,
            whitelist_mint,
            constituent_target_base,
            constituent_correlations,
            max_aum,
            last_aum,
            cumulative_quote_sent_to_perp_markets,
            cumulative_quote_received_from_perp_markets,
            total_mint_redeem_fees_paid,
            last_aum_slot,
            max_settle_quote_amount,
            padding,
            mint_redeem_id,
            settle_id,
            min_mint_fee,
            token_supply,
            volatility,
            constituents,
            quote_consituent_index,
            bump,
            gamma_execution,
            xi,
            target_oracle_delay_fee_bps_per10_slots,
            target_position_delay_fee_bps_per10_slots,
            lp_pool_id,
            padding1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelist_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.constituent_target_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.constituent_correlations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_aum, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_aum, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_quote_sent_to_perp_markets,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_quote_received_from_perp_markets,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.total_mint_redeem_fees_paid,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_aum_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_settle_quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_redeem_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settle_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_mint_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.volatility, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.constituents, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_consituent_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.gamma_execution, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.xi, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.target_oracle_delay_fee_bps_per10_slots,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.target_position_delay_fee_bps_per10_slots,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.lp_pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LPPoolAccount(pub LPPool);
impl LPPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LPPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONSTITUENT_ACCOUNT_DISCM: [u8; 8] = [0, 61, 36, 35, 177, 76, 216, 205];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Constituent {
    pub pubkey: Pubkey,
    pub mint: Pubkey,
    pub lp_pool: Pubkey,
    pub vault: Pubkey,
    pub total_swap_fees: i128,
    pub spot_balance: ConstituentSpotBalance,
    pub last_spot_balance_token_amount: i64,
    pub cumulative_spot_interest_accrued_token_amount: i64,
    pub max_weight_deviation: i64,
    pub swap_fee_min: i64,
    pub swap_fee_max: i64,
    pub max_borrow_token_amount: u64,
    pub vault_token_balance: u64,
    pub last_oracle_price: i64,
    pub last_oracle_slot: u64,
    pub oracle_staleness_threshold: u64,
    pub flash_loan_initial_token_amount: u64,
    pub next_swap_id: u64,
    pub derivative_weight: u64,
    pub volatility: u64,
    pub constituent_derivative_depeg_threshold: u64,
    pub constituent_derivative_index: i16,
    pub spot_market_index: u16,
    pub constituent_index: u16,
    pub decimals: u8,
    pub bump: u8,
    pub vault_bump: u8,
    pub gamma_inventory: u8,
    pub gamma_execution: u8,
    pub xi: u8,
    pub status: u8,
    pub paused_operations: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 162],
}
impl Constituent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_swap_fees: i128 = crate::borsh_de_or_default(&mut reader)?;
        let spot_balance = if reader.is_empty() {
            Default::default()
        } else {
            <ConstituentSpotBalance>::deserialize(&mut reader)?
        };
        let last_spot_balance_token_amount: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_spot_interest_accrued_token_amount: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_weight_deviation: i64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_min: i64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_max: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_token_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_staleness_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let flash_loan_initial_token_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let next_swap_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let derivative_weight: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volatility: u64 = crate::borsh_de_or_default(&mut reader)?;
        let constituent_derivative_depeg_threshold: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let constituent_derivative_index: i16 = crate::borsh_de_or_default(&mut reader)?;
        let spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let gamma_inventory: u8 = crate::borsh_de_or_default(&mut reader)?;
        let gamma_execution: u8 = crate::borsh_de_or_default(&mut reader)?;
        let xi: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let paused_operations: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 162] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            mint,
            lp_pool,
            vault,
            total_swap_fees,
            spot_balance,
            last_spot_balance_token_amount,
            cumulative_spot_interest_accrued_token_amount,
            max_weight_deviation,
            swap_fee_min,
            swap_fee_max,
            max_borrow_token_amount,
            vault_token_balance,
            last_oracle_price,
            last_oracle_slot,
            oracle_staleness_threshold,
            flash_loan_initial_token_amount,
            next_swap_id,
            derivative_weight,
            volatility,
            constituent_derivative_depeg_threshold,
            constituent_derivative_index,
            spot_market_index,
            constituent_index,
            decimals,
            bump,
            vault_bump,
            gamma_inventory,
            gamma_execution,
            xi,
            status,
            paused_operations,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_swap_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_spot_balance_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_spot_interest_accrued_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.max_weight_deviation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_min, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_borrow_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_token_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_oracle_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_staleness_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.flash_loan_initial_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.next_swap_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.derivative_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.volatility, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.constituent_derivative_depeg_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.constituent_derivative_index,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.constituent_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.gamma_inventory, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.gamma_execution, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.xi, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused_operations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstituentAccount(pub Constituent);
impl ConstituentAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSTITUENT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Constituent::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSTITUENT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const AMM_CONSTITUENT_MAPPING_ACCOUNT_DISCM: [u8; 8] = [
    254, 89, 5, 173, 66, 54, 214, 247,
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
pub struct AmmConstituentMapping {
    pub lp_pool: Pubkey,
    pub bump: u8,
    pub padding: [u8; 3],
    pub weights: Vec<AmmConstituentDatum>,
}
impl AmmConstituentMapping {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let weights: Vec<AmmConstituentDatum> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_pool,
            bump,
            padding,
            weights,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.weights, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmConstituentMappingAccount(pub AmmConstituentMapping);
impl AmmConstituentMappingAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_CONSTITUENT_MAPPING_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AmmConstituentMapping::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_CONSTITUENT_MAPPING_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONSTITUENT_TARGET_BASE_ACCOUNT_DISCM: [u8; 8] = [
    255, 142, 134, 71, 125, 66, 198, 99,
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
pub struct ConstituentTargetBase {
    pub lp_pool: Pubkey,
    pub bump: u8,
    pub padding: [u8; 3],
    pub targets: Vec<TargetsDatum>,
}
impl ConstituentTargetBase {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let targets: Vec<TargetsDatum> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_pool,
            bump,
            padding,
            targets,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.targets, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstituentTargetBaseAccount(pub ConstituentTargetBase);
impl ConstituentTargetBaseAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSTITUENT_TARGET_BASE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ConstituentTargetBase::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSTITUENT_TARGET_BASE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONSTITUENT_CORRELATIONS_ACCOUNT_DISCM: [u8; 8] = [
    124, 203, 115, 33, 18, 162, 67, 216,
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
pub struct ConstituentCorrelations {
    pub lp_pool: Pubkey,
    pub bump: u8,
    pub padding: [u8; 3],
    pub correlations: Vec<i64>,
}
impl ConstituentCorrelations {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let correlations: Vec<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_pool,
            bump,
            padding,
            correlations,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.correlations, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstituentCorrelationsAccount(pub ConstituentCorrelations);
impl ConstituentCorrelationsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSTITUENT_CORRELATIONS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ConstituentCorrelations::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSTITUENT_CORRELATIONS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRELAUNCH_ORACLE_ACCOUNT_DISCM: [u8; 8] = [92, 14, 139, 234, 72, 244, 68, 26];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PrelaunchOracle {
    pub price: i64,
    pub max_price: i64,
    pub confidence: u64,
    pub last_update_slot: u64,
    pub amm_last_update_slot: u64,
    pub perp_market_index: u16,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 70],
}
impl PrelaunchOracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let confidence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amm_last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 70] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            price,
            max_price,
            confidence,
            last_update_slot,
            amm_last_update_slot,
            perp_market_index,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.confidence, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_last_update_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PrelaunchOracleAccount(pub PrelaunchOracle);
impl PrelaunchOracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRELAUNCH_ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PrelaunchOracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRELAUNCH_ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PERP_MARKET_ACCOUNT_DISCM: [u8; 8] = [10, 223, 12, 44, 107, 245, 55, 247];
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
pub struct PerpMarket {
    pub pubkey: Pubkey,
    pub amm: AMM,
    pub pnl_pool: PoolBalance,
    pub name: [u8; 32],
    pub insurance_claim: InsuranceClaim,
    pub unrealized_pnl_max_imbalance: u64,
    pub expiry_ts: i64,
    pub expiry_price: i64,
    pub next_fill_record_id: u64,
    pub next_funding_rate_record_id: u64,
    pub next_curve_record_id: u64,
    pub imf_factor: u32,
    pub unrealized_pnl_imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
    pub unrealized_pnl_initial_asset_weight: u32,
    pub unrealized_pnl_maintenance_asset_weight: u32,
    pub number_of_users_with_base: u32,
    pub number_of_users: u32,
    pub market_index: u16,
    pub status: MarketStatus,
    pub contract_type: ContractType,
    pub contract_tier: ContractTier,
    pub paused_operations: u8,
    pub quote_spot_market_index: u16,
    pub fee_adjustment: i16,
    pub fuel_boost_position: u8,
    pub fuel_boost_taker: u8,
    pub fuel_boost_maker: u8,
    pub pool_id: u8,
    pub high_leverage_margin_ratio_initial: u16,
    pub high_leverage_margin_ratio_maintenance: u16,
    pub protected_maker_limit_price_divisor: u8,
    pub protected_maker_dynamic_divisor: u8,
    pub lp_fee_transfer_scalar: u8,
    pub lp_status: u8,
    pub lp_paused_operations: u8,
    pub lp_exchange_fee_excluscion_scalar: u8,
    pub last_fill_price: u64,
    pub lp_pool_id: u8,
    pub market_config: u8,
    pub padding: [u8; 22],
}
impl PerpMarket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm = if reader.is_empty() {
            Default::default()
        } else {
            <AMM>::deserialize(&mut reader)?
        };
        let pnl_pool = if reader.is_empty() {
            Default::default()
        } else {
            <PoolBalance>::deserialize(&mut reader)?
        };
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let insurance_claim = if reader.is_empty() {
            Default::default()
        } else {
            <InsuranceClaim>::deserialize(&mut reader)?
        };
        let unrealized_pnl_max_imbalance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expiry_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let expiry_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let next_fill_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_funding_rate_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_curve_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let imf_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let unrealized_pnl_imf_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let if_liquidation_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let margin_ratio_initial: u32 = crate::borsh_de_or_default(&mut reader)?;
        let margin_ratio_maintenance: u32 = crate::borsh_de_or_default(&mut reader)?;
        let unrealized_pnl_initial_asset_weight: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let unrealized_pnl_maintenance_asset_weight: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let number_of_users_with_base: u32 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let status: MarketStatus = crate::borsh_de_or_default(&mut reader)?;
        let contract_type: ContractType = crate::borsh_de_or_default(&mut reader)?;
        let contract_tier: ContractTier = crate::borsh_de_or_default(&mut reader)?;
        let paused_operations: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_adjustment: i16 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_position: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_taker: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_maker: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let high_leverage_margin_ratio_initial: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let high_leverage_margin_ratio_maintenance: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protected_maker_limit_price_divisor: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protected_maker_dynamic_divisor: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lp_fee_transfer_scalar: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_paused_operations: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_exchange_fee_excluscion_scalar: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_fill_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let market_config: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 22] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            amm,
            pnl_pool,
            name,
            insurance_claim,
            unrealized_pnl_max_imbalance,
            expiry_ts,
            expiry_price,
            next_fill_record_id,
            next_funding_rate_record_id,
            next_curve_record_id,
            imf_factor,
            unrealized_pnl_imf_factor,
            liquidator_fee,
            if_liquidation_fee,
            margin_ratio_initial,
            margin_ratio_maintenance,
            unrealized_pnl_initial_asset_weight,
            unrealized_pnl_maintenance_asset_weight,
            number_of_users_with_base,
            number_of_users,
            market_index,
            status,
            contract_type,
            contract_tier,
            paused_operations,
            quote_spot_market_index,
            fee_adjustment,
            fuel_boost_position,
            fuel_boost_taker,
            fuel_boost_maker,
            pool_id,
            high_leverage_margin_ratio_initial,
            high_leverage_margin_ratio_maintenance,
            protected_maker_limit_price_divisor,
            protected_maker_dynamic_divisor,
            lp_fee_transfer_scalar,
            lp_status,
            lp_paused_operations,
            lp_exchange_fee_excluscion_scalar,
            last_fill_price,
            lp_pool_id,
            market_config,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.insurance_claim, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.unrealized_pnl_max_imbalance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.expiry_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expiry_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_fill_record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.next_funding_rate_record_id,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.next_curve_record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.imf_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unrealized_pnl_imf_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_liquidation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_ratio_initial, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_ratio_maintenance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.unrealized_pnl_initial_asset_weight,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.unrealized_pnl_maintenance_asset_weight,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.number_of_users_with_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.contract_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.contract_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused_operations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_adjustment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.high_leverage_margin_ratio_initial,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.high_leverage_margin_ratio_maintenance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.protected_maker_limit_price_divisor,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.protected_maker_dynamic_divisor,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.lp_fee_transfer_scalar, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_paused_operations, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.lp_exchange_fee_excluscion_scalar,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_fill_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PerpMarketAccount(pub PerpMarket);
impl PerpMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERP_MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PerpMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERP_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTECTED_MAKER_MODE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    47, 86, 90, 9, 224, 255, 10, 69,
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
pub struct ProtectedMakerModeConfig {
    pub max_users: u32,
    pub current_users: u32,
    pub reduce_only: u8,
    pub padding: [u8; 31],
}
impl ProtectedMakerModeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let current_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reduce_only: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 31] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_users,
            current_users,
            reduce_only,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.max_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduce_only, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtectedMakerModeConfigAccount(pub ProtectedMakerModeConfig);
impl ProtectedMakerModeConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTECTED_MAKER_MODE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProtectedMakerModeConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTECTED_MAKER_MODE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PYTH_LAZER_ORACLE_ACCOUNT_DISCM: [u8; 8] = [
    159, 7, 161, 249, 34, 81, 121, 133,
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
pub struct PythLazerOracle {
    pub price: i64,
    pub publish_time: u64,
    pub posted_slot: u64,
    pub exponent: i32,
    pub padding: [u8; 4],
    pub conf: u64,
}
impl PythLazerOracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let publish_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let posted_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exponent: i32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let conf: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price,
            publish_time,
            posted_slot,
            exponent,
            padding,
            conf,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.publish_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.posted_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.exponent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.conf, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PythLazerOracleAccount(pub PythLazerOracle);
impl PythLazerOracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PYTH_LAZER_ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PythLazerOracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PYTH_LAZER_ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_SHARE_ACCOUNT_DISCM: [u8; 8] = [55, 40, 228, 7, 139, 52, 180, 110];
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
pub struct RevenueShare {
    pub authority: Pubkey,
    pub total_referrer_rewards: u64,
    pub total_builder_rewards: u64,
    pub padding: [u8; 18],
}
impl RevenueShare {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_referrer_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_builder_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 18] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            total_referrer_rewards,
            total_builder_rewards,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_referrer_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_builder_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevenueShareAccount(pub RevenueShare);
impl RevenueShareAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_SHARE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RevenueShare::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_SHARE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_SHARE_ESCROW_ACCOUNT_DISCM: [u8; 8] = [
    98, 167, 3, 46, 74, 177, 173, 252,
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
pub struct RevenueShareEscrow {
    pub authority: Pubkey,
    pub referrer: Pubkey,
    pub referrer_boost_expire_ts: u32,
    pub referrer_reward_offset: i8,
    pub referee_fee_numerator_offset: i8,
    pub referrer_boost_numerator: i8,
    pub reserved_fixed: [u8; 17],
    pub padding0: u32,
    pub orders: Vec<RevenueShareOrder>,
    pub padding1: u32,
    pub approved_builders: Vec<BuilderInfo>,
}
impl RevenueShareEscrow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer_boost_expire_ts: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_reward_offset: i8 = crate::borsh_de_or_default(&mut reader)?;
        let referee_fee_numerator_offset: i8 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_boost_numerator: i8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_fixed: [u8; 17] = crate::borsh_de_or_default(&mut reader)?;
        let padding0: u32 = crate::borsh_de_or_default(&mut reader)?;
        let orders: Vec<RevenueShareOrder> = crate::borsh_de_or_default(&mut reader)?;
        let padding1: u32 = crate::borsh_de_or_default(&mut reader)?;
        let approved_builders: Vec<BuilderInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            authority,
            referrer,
            referrer_boost_expire_ts,
            referrer_reward_offset,
            referee_fee_numerator_offset,
            referrer_boost_numerator,
            reserved_fixed,
            padding0,
            orders,
            padding1,
            approved_builders,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_boost_expire_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_reward_offset, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.referee_fee_numerator_offset,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.referrer_boost_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_fixed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.approved_builders, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevenueShareEscrowAccount(pub RevenueShareEscrow);
impl RevenueShareEscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_SHARE_ESCROW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RevenueShareEscrow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_SHARE_ESCROW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SIGNED_MSG_USER_ORDERS_ACCOUNT_DISCM: [u8; 8] = [
    70, 6, 50, 248, 222, 1, 143, 49,
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
pub struct SignedMsgUserOrders {
    pub authority_pubkey: Pubkey,
    pub padding: u32,
    pub signed_msg_order_data: Vec<SignedMsgOrderId>,
}
impl SignedMsgUserOrders {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: u32 = crate::borsh_de_or_default(&mut reader)?;
        let signed_msg_order_data: Vec<SignedMsgOrderId> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            authority_pubkey,
            padding,
            signed_msg_order_data,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority_pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signed_msg_order_data, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SignedMsgUserOrdersAccount(pub SignedMsgUserOrders);
impl SignedMsgUserOrdersAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SIGNED_MSG_USER_ORDERS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SignedMsgUserOrders::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SIGNED_MSG_USER_ORDERS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SIGNED_MSG_WS_DELEGATES_ACCOUNT_DISCM: [u8; 8] = [
    190, 115, 111, 44, 216, 252, 108, 85,
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
pub struct SignedMsgWsDelegates {
    pub delegates: Vec<Pubkey>,
}
impl SignedMsgWsDelegates {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let delegates: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { delegates })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.delegates, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SignedMsgWsDelegatesAccount(pub SignedMsgWsDelegates);
impl SignedMsgWsDelegatesAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SIGNED_MSG_WS_DELEGATES_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SignedMsgWsDelegates::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SIGNED_MSG_WS_DELEGATES_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SPOT_MARKET_ACCOUNT_DISCM: [u8; 8] = [100, 177, 8, 107, 168, 65, 65, 39];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SpotMarket {
    pub pubkey: Pubkey,
    pub oracle: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub name: [u8; 32],
    pub historical_oracle_data: HistoricalOracleData,
    pub historical_index_data: HistoricalIndexData,
    pub revenue_pool: PoolBalance,
    pub spot_fee_pool: PoolBalance,
    pub insurance_fund: InsuranceFund,
    pub total_spot_fee: u128,
    pub deposit_balance: u128,
    pub borrow_balance: u128,
    pub cumulative_deposit_interest: u128,
    pub cumulative_borrow_interest: u128,
    pub total_social_loss: u128,
    pub total_quote_social_loss: u128,
    pub withdraw_guard_threshold: u64,
    pub max_token_deposits: u64,
    pub deposit_token_twap: u64,
    pub borrow_token_twap: u64,
    pub utilization_twap: u64,
    pub last_interest_ts: u64,
    pub last_twap_ts: u64,
    pub expiry_ts: i64,
    pub order_step_size: u64,
    pub order_tick_size: u64,
    pub min_order_size: u64,
    pub max_position_size: u64,
    pub next_fill_record_id: u64,
    pub next_deposit_record_id: u64,
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
    pub decimals: u32,
    pub market_index: u16,
    pub orders_enabled: bool,
    pub oracle_source: OracleSource,
    pub status: MarketStatus,
    pub asset_tier: AssetTier,
    pub paused_operations: u8,
    pub if_paused_operations: u8,
    pub fee_adjustment: i16,
    pub max_token_borrows_fraction: u16,
    pub flash_loan_amount: u64,
    pub flash_loan_initial_token_amount: u64,
    pub total_swap_fee: u64,
    pub scale_initial_asset_weight_start: u64,
    pub min_borrow_rate: u8,
    pub fuel_boost_deposits: u8,
    pub fuel_boost_borrows: u8,
    pub fuel_boost_taker: u8,
    pub fuel_boost_maker: u8,
    pub fuel_boost_insurance: u8,
    pub token_program_flag: u8,
    pub pool_id: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 40],
}
impl SpotMarket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let historical_oracle_data = if reader.is_empty() {
            Default::default()
        } else {
            <HistoricalOracleData>::deserialize(&mut reader)?
        };
        let historical_index_data = if reader.is_empty() {
            Default::default()
        } else {
            <HistoricalIndexData>::deserialize(&mut reader)?
        };
        let revenue_pool = if reader.is_empty() {
            Default::default()
        } else {
            <PoolBalance>::deserialize(&mut reader)?
        };
        let spot_fee_pool = if reader.is_empty() {
            Default::default()
        } else {
            <PoolBalance>::deserialize(&mut reader)?
        };
        let insurance_fund = if reader.is_empty() {
            Default::default()
        } else {
            <InsuranceFund>::deserialize(&mut reader)?
        };
        let total_spot_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposit_interest: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_borrow_interest: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_social_loss: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_social_loss: u128 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_guard_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token_deposits: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_token_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_token_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let utilization_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_interest_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_twap_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expiry_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let order_step_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_tick_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_order_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_position_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_fill_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_deposit_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_asset_weight: u32 = crate::borsh_de_or_default(&mut reader)?;
        let maintenance_asset_weight: u32 = crate::borsh_de_or_default(&mut reader)?;
        let initial_liability_weight: u32 = crate::borsh_de_or_default(&mut reader)?;
        let maintenance_liability_weight: u32 = crate::borsh_de_or_default(&mut reader)?;
        let imf_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let if_liquidation_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let optimal_utilization: u32 = crate::borsh_de_or_default(&mut reader)?;
        let optimal_borrow_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u32 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let orders_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let oracle_source: OracleSource = crate::borsh_de_or_default(&mut reader)?;
        let status: MarketStatus = crate::borsh_de_or_default(&mut reader)?;
        let asset_tier: AssetTier = crate::borsh_de_or_default(&mut reader)?;
        let paused_operations: u8 = crate::borsh_de_or_default(&mut reader)?;
        let if_paused_operations: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_adjustment: i16 = crate::borsh_de_or_default(&mut reader)?;
        let max_token_borrows_fraction: u16 = crate::borsh_de_or_default(&mut reader)?;
        let flash_loan_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let flash_loan_initial_token_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scale_initial_asset_weight_start: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_borrow_rate: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_deposits: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_borrows: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_taker: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_maker: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_boost_insurance: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_program_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 40] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            oracle,
            mint,
            vault,
            name,
            historical_oracle_data,
            historical_index_data,
            revenue_pool,
            spot_fee_pool,
            insurance_fund,
            total_spot_fee,
            deposit_balance,
            borrow_balance,
            cumulative_deposit_interest,
            cumulative_borrow_interest,
            total_social_loss,
            total_quote_social_loss,
            withdraw_guard_threshold,
            max_token_deposits,
            deposit_token_twap,
            borrow_token_twap,
            utilization_twap,
            last_interest_ts,
            last_twap_ts,
            expiry_ts,
            order_step_size,
            order_tick_size,
            min_order_size,
            max_position_size,
            next_fill_record_id,
            next_deposit_record_id,
            initial_asset_weight,
            maintenance_asset_weight,
            initial_liability_weight,
            maintenance_liability_weight,
            imf_factor,
            liquidator_fee,
            if_liquidation_fee,
            optimal_utilization,
            optimal_borrow_rate,
            max_borrow_rate,
            decimals,
            market_index,
            orders_enabled,
            oracle_source,
            status,
            asset_tier,
            paused_operations,
            if_paused_operations,
            fee_adjustment,
            max_token_borrows_fraction,
            flash_loan_amount,
            flash_loan_initial_token_amount,
            total_swap_fee,
            scale_initial_asset_weight_start,
            min_borrow_rate,
            fuel_boost_deposits,
            fuel_boost_borrows,
            fuel_boost_taker,
            fuel_boost_maker,
            fuel_boost_insurance,
            token_program_flag,
            pool_id,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.historical_oracle_data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.historical_index_data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_fee_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.insurance_fund, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_spot_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_deposit_interest,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.cumulative_borrow_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_social_loss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_quote_social_loss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_guard_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_token_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_token_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_token_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.utilization_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_interest_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_twap_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expiry_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_step_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_tick_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_order_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_position_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_fill_record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_deposit_record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_asset_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maintenance_asset_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_liability_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.maintenance_liability_weight,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.imf_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_liquidation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.optimal_utilization, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.optimal_borrow_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_borrow_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_source, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused_operations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_paused_operations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_adjustment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_token_borrows_fraction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flash_loan_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.flash_loan_initial_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_swap_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.scale_initial_asset_weight_start,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_borrow_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_boost_insurance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotMarketAccount(pub SpotMarket);
impl SpotMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPOT_MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SpotMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPOT_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
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
pub struct State {
    pub admin: Pubkey,
    pub whitelist_mint: Pubkey,
    pub discount_mint: Pubkey,
    pub signer: Pubkey,
    pub srm_vault: Pubkey,
    pub perp_fee_structure: FeeStructure,
    pub spot_fee_structure: FeeStructure,
    pub oracle_guard_rails: OracleGuardRails,
    pub number_of_authorities: u64,
    pub number_of_sub_accounts: u64,
    pub lp_cooldown_time: u64,
    pub liquidation_margin_buffer_ratio: u32,
    pub settlement_duration: u16,
    pub number_of_markets: u16,
    pub number_of_spot_markets: u16,
    pub signer_nonce: u8,
    pub min_perp_auction_duration: u8,
    pub default_market_order_time_in_force: u8,
    pub default_spot_auction_duration: u8,
    pub exchange_status: u8,
    pub liquidation_duration: u8,
    pub initial_pct_to_liquidate: u16,
    pub max_number_of_sub_accounts: u16,
    pub max_initialize_user_fee: u16,
    pub feature_bit_flags: u8,
    pub lp_pool_feature_bit_flags: u8,
    pub padding: [u8; 8],
}
impl State {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let discount_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let srm_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let perp_fee_structure = if reader.is_empty() {
            Default::default()
        } else {
            <FeeStructure>::deserialize(&mut reader)?
        };
        let spot_fee_structure = if reader.is_empty() {
            Default::default()
        } else {
            <FeeStructure>::deserialize(&mut reader)?
        };
        let oracle_guard_rails = if reader.is_empty() {
            Default::default()
        } else {
            <OracleGuardRails>::deserialize(&mut reader)?
        };
        let number_of_authorities: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_sub_accounts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_cooldown_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_margin_buffer_ratio: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let settlement_duration: u16 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_markets: u16 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_spot_markets: u16 = crate::borsh_de_or_default(&mut reader)?;
        let signer_nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
        let min_perp_auction_duration: u8 = crate::borsh_de_or_default(&mut reader)?;
        let default_market_order_time_in_force: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let default_spot_auction_duration: u8 = crate::borsh_de_or_default(&mut reader)?;
        let exchange_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_duration: u8 = crate::borsh_de_or_default(&mut reader)?;
        let initial_pct_to_liquidate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_number_of_sub_accounts: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_initialize_user_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let feature_bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool_feature_bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            whitelist_mint,
            discount_mint,
            signer,
            srm_vault,
            perp_fee_structure,
            spot_fee_structure,
            oracle_guard_rails,
            number_of_authorities,
            number_of_sub_accounts,
            lp_cooldown_time,
            liquidation_margin_buffer_ratio,
            settlement_duration,
            number_of_markets,
            number_of_spot_markets,
            signer_nonce,
            min_perp_auction_duration,
            default_market_order_time_in_force,
            default_spot_auction_duration,
            exchange_status,
            liquidation_duration,
            initial_pct_to_liquidate,
            max_number_of_sub_accounts,
            max_initialize_user_fee,
            feature_bit_flags,
            lp_pool_feature_bit_flags,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelist_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.discount_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.srm_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_fee_structure, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_fee_structure, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_guard_rails, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_authorities, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_sub_accounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_cooldown_time, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.liquidation_margin_buffer_ratio,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.settlement_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_markets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_spot_markets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer_nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_perp_auction_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.default_market_order_time_in_force,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.default_spot_auction_duration,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.exchange_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_pct_to_liquidate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_number_of_sub_accounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_initialize_user_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.feature_bit_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool_feature_bit_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_ACCOUNT_DISCM: [u8; 8] = [159, 117, 95, 227, 239, 151, 58, 236];
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
pub struct User {
    pub authority: Pubkey,
    pub delegate: Pubkey,
    pub name: [u8; 32],
    pub spot_positions: [SpotPosition; 8],
    pub perp_positions: [PerpPosition; 8],
    pub orders: [Order; 32],
    pub last_add_perp_lp_shares_ts: i64,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub total_social_loss: u64,
    pub settled_perp_pnl: i64,
    pub cumulative_spot_fees: i64,
    pub cumulative_perp_funding: i64,
    pub liquidation_margin_freed: u64,
    pub last_active_slot: u64,
    pub next_order_id: u32,
    pub max_margin_ratio: u32,
    pub next_liquidation_id: u16,
    pub sub_account_id: u16,
    pub status: u8,
    pub is_margin_trading_enabled: bool,
    pub idle: bool,
    pub open_orders: u8,
    pub has_open_order: bool,
    pub open_auctions: u8,
    pub has_open_auction: bool,
    pub margin_mode: MarginMode,
    pub pool_id: u8,
    pub padding1: [u8; 3],
    pub last_fuel_bonus_update_ts: u32,
    pub padding: [u8; 12],
}
impl User {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let spot_positions: [SpotPosition; 8] = crate::borsh_de_or_default(&mut reader)?;
        let perp_positions: [PerpPosition; 8] = crate::borsh_de_or_default(&mut reader)?;
        let orders: [Order; 32] = crate::borsh_de_or_default(&mut reader)?;
        let last_add_perp_lp_shares_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_deposits: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_withdraws: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_social_loss: u64 = crate::borsh_de_or_default(&mut reader)?;
        let settled_perp_pnl: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_spot_fees: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_perp_funding: i64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_margin_freed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_active_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_margin_ratio: u32 = crate::borsh_de_or_default(&mut reader)?;
        let next_liquidation_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_margin_trading_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let idle: bool = crate::borsh_de_or_default(&mut reader)?;
        let open_orders: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_open_order: bool = crate::borsh_de_or_default(&mut reader)?;
        let open_auctions: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_open_auction: bool = crate::borsh_de_or_default(&mut reader)?;
        let margin_mode: MarginMode = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let last_fuel_bonus_update_ts: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            delegate,
            name,
            spot_positions,
            perp_positions,
            orders,
            last_add_perp_lp_shares_ts,
            total_deposits,
            total_withdraws,
            total_social_loss,
            settled_perp_pnl,
            cumulative_spot_fees,
            cumulative_perp_funding,
            liquidation_margin_freed,
            last_active_slot,
            next_order_id,
            max_margin_ratio,
            next_liquidation_id,
            sub_account_id,
            status,
            is_margin_trading_enabled,
            idle,
            open_orders,
            has_open_order,
            open_auctions,
            has_open_auction,
            margin_mode,
            pool_id,
            padding1,
            last_fuel_bonus_update_ts,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_add_perp_lp_shares_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_withdraws, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_social_loss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settled_perp_pnl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cumulative_spot_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cumulative_perp_funding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_margin_freed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_active_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_margin_ratio, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_liquidation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sub_account_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_margin_trading_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.idle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_open_order, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_auctions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_open_auction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_fuel_bonus_update_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserAccount(pub User);
impl UserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(User::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_STATS_ACCOUNT_DISCM: [u8; 8] = [176, 223, 136, 27, 122, 79, 32, 227];
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
pub struct UserStats {
    pub authority: Pubkey,
    pub referrer: Pubkey,
    pub fees: UserFees,
    pub next_epoch_ts: i64,
    pub maker_volume30d: u64,
    pub taker_volume30d: u64,
    pub filler_volume30d: u64,
    pub last_maker_volume30d_ts: i64,
    pub last_taker_volume30d_ts: i64,
    pub last_filler_volume30d_ts: i64,
    pub if_staked_quote_asset_amount: u64,
    pub number_of_sub_accounts: u16,
    pub number_of_sub_accounts_created: u16,
    pub referrer_status: u8,
    pub disable_update_perp_bid_ask_twap: u8,
    pub paused_operations: u8,
    pub fuel_overflow_status: u8,
    pub fuel_insurance: u32,
    pub fuel_deposits: u32,
    pub fuel_borrows: u32,
    pub fuel_positions: u32,
    pub fuel_taker: u32,
    pub fuel_maker: u32,
    pub if_staked_gov_token_amount: u64,
    pub last_fuel_if_bonus_update_ts: u32,
    pub padding: [u8; 12],
}
impl UserStats {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <UserFees>::deserialize(&mut reader)?
        };
        let next_epoch_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_volume30d: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_volume30d: u64 = crate::borsh_de_or_default(&mut reader)?;
        let filler_volume30d: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_maker_volume30d_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_taker_volume30d_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_filler_volume30d_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let if_staked_quote_asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_sub_accounts: u16 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_sub_accounts_created: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let referrer_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let disable_update_perp_bid_ask_twap: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let paused_operations: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_overflow_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_insurance: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_deposits: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_borrows: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_positions: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_taker: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_maker: u32 = crate::borsh_de_or_default(&mut reader)?;
        let if_staked_gov_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_fuel_if_bonus_update_ts: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            referrer,
            fees,
            next_epoch_ts,
            maker_volume30d,
            taker_volume30d,
            filler_volume30d,
            last_maker_volume30d_ts,
            last_taker_volume30d_ts,
            last_filler_volume30d_ts,
            if_staked_quote_asset_amount,
            number_of_sub_accounts,
            number_of_sub_accounts_created,
            referrer_status,
            disable_update_perp_bid_ask_twap,
            paused_operations,
            fuel_overflow_status,
            fuel_insurance,
            fuel_deposits,
            fuel_borrows,
            fuel_positions,
            fuel_taker,
            fuel_maker,
            if_staked_gov_token_amount,
            last_fuel_if_bonus_update_ts,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_epoch_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_volume30d, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_volume30d, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filler_volume30d, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_maker_volume30d_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_taker_volume30d_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_filler_volume30d_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.if_staked_quote_asset_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.number_of_sub_accounts, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.number_of_sub_accounts_created,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.referrer_status, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.disable_update_perp_bid_ask_twap,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.paused_operations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_overflow_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_insurance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_staked_gov_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_fuel_if_bonus_update_ts,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserStatsAccount(pub UserStats);
impl UserStatsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_STATS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserStats::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_STATS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_NAME_ACCOUNT_DISCM: [u8; 8] = [105, 133, 170, 110, 52, 42, 28, 182];
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
pub struct ReferrerName {
    pub authority: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub name: [u8; 32],
}
impl ReferrerName {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_stats: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            user,
            user_stats,
            name,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerNameAccount(pub ReferrerName);
impl ReferrerNameAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_NAME_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ReferrerName::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_NAME_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUEL_OVERFLOW_ACCOUNT_DISCM: [u8; 8] = [182, 64, 231, 177, 226, 142, 69, 58];
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
pub struct FuelOverflow {
    pub authority: Pubkey,
    pub fuel_insurance: u128,
    pub fuel_deposits: u128,
    pub fuel_borrows: u128,
    pub fuel_positions: u128,
    pub fuel_taker: u128,
    pub fuel_maker: u128,
    pub last_fuel_sweep_ts: u32,
    pub last_reset_ts: u32,
    pub padding: [u128; 6],
}
impl FuelOverflow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fuel_insurance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_deposits: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_borrows: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_positions: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_taker: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_maker: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_fuel_sweep_ts: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_reset_ts: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            fuel_insurance,
            fuel_deposits,
            fuel_borrows,
            fuel_positions,
            fuel_taker,
            fuel_maker,
            last_fuel_sweep_ts,
            last_reset_ts,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_insurance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_fuel_sweep_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_reset_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FuelOverflowAccount(pub FuelOverflow);
impl FuelOverflowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUEL_OVERFLOW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FuelOverflow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUEL_OVERFLOW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
