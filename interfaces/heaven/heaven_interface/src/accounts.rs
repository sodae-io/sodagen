use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LIQUIDITY_POOL_STATE_ACCOUNT_DISCM: [u8; 8] = [
    190, 158, 220, 130, 15, 162, 132, 252,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LiquidityPoolState {
    pub info: LiquidityPoolInfo,
    pub market_cap_based_fees: LiquidityPoolMarketCapBasedFees,
    pub reserve: LiquidityPoolReserve,
    pub lp_token: LiquidityPoolLpTokenInfo,
    pub protocol_trading_fees: u64,
    pub creator_trading_fees: u64,
    pub creator_trading_fees_claimed_by_creator: u64,
    pub creator_trading_fees_claimed_by_others: u64,
    pub liquidity_provider_trading_fees: u64,
    pub creator_trading_fee_protocol_fees: u64,
    pub reflection_trading_fees: u64,
    pub created_at_slot: u64,
    pub trading_volume_usd: f64,
    pub creator_trading_fee_trading_volume_threshold: f64,
    pub creator_trading_fee_trading_volume_threshold_reached_unix_timestamp: u64,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub protocol_config: Pubkey,
    pub key: Pubkey,
    pub token_a: LiquidityPoolTokenInfo,
    pub token_b: LiquidityPoolTokenInfo,
    pub allowlist: LiquidityPoolAllowlist,
    pub feature_flags: LiquidityPoolFeatureFlags,
    pub taxable_side: u8,
    pub taxable_side_type: u8,
    pub creator_trading_fee_distribution: u8,
    pub creator_trading_fee_claim_status: u8,
    pub fee_configuration_mode: u8,
    pub is_migrated: u8,
    pub pad: [u8; 13],
    pub slot_offset_based_fees: LiquidityPoolSlotOffsetBasedFees,
    pub creator_trading_fee_receiver: Pubkey,
}
impl LiquidityPoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let info = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolInfo>::deserialize(&mut reader)?
        };
        let market_cap_based_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolMarketCapBasedFees>::deserialize(&mut reader)?
        };
        let reserve = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolReserve>::deserialize(&mut reader)?
        };
        let lp_token = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolLpTokenInfo>::deserialize(&mut reader)?
        };
        let protocol_trading_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fees_claimed_by_creator: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_trading_fees_claimed_by_others: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidity_provider_trading_fees: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_trading_fee_protocol_fees: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reflection_trading_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trading_volume_usd: f64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fee_trading_volume_threshold: f64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_trading_fee_trading_volume_threshold_reached_unix_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_a_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolTokenInfo>::deserialize(&mut reader)?
        };
        let token_b = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolTokenInfo>::deserialize(&mut reader)?
        };
        let allowlist = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolAllowlist>::deserialize(&mut reader)?
        };
        let feature_flags = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolFeatureFlags>::deserialize(&mut reader)?
        };
        let taxable_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let taxable_side_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fee_distribution: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_trading_fee_claim_status: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_configuration_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_migrated: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad: [u8; 13] = crate::borsh_de_or_default(&mut reader)?;
        let slot_offset_based_fees = <LiquidityPoolSlotOffsetBasedFees>::deserialize(
            &mut reader,
        )?;
        let creator_trading_fee_receiver: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            info,
            market_cap_based_fees,
            reserve,
            lp_token,
            protocol_trading_fees,
            creator_trading_fees,
            creator_trading_fees_claimed_by_creator,
            creator_trading_fees_claimed_by_others,
            liquidity_provider_trading_fees,
            creator_trading_fee_protocol_fees,
            reflection_trading_fees,
            created_at_slot,
            trading_volume_usd,
            creator_trading_fee_trading_volume_threshold,
            creator_trading_fee_trading_volume_threshold_reached_unix_timestamp,
            token_a_vault,
            token_b_vault,
            protocol_config,
            key,
            token_a,
            token_b,
            allowlist,
            feature_flags,
            taxable_side,
            taxable_side_type,
            creator_trading_fee_distribution,
            creator_trading_fee_claim_status,
            fee_configuration_mode,
            is_migrated,
            pad,
            slot_offset_based_fees,
            creator_trading_fee_receiver,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_cap_based_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_trading_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_trading_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fees_claimed_by_creator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fees_claimed_by_others,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.liquidity_provider_trading_fees,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_protocol_fees,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reflection_trading_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trading_volume_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_trading_volume_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_trading_volume_threshold_reached_unix_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_a_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allowlist, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.feature_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taxable_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taxable_side_type, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_distribution,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_claim_status,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.fee_configuration_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_migrated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot_offset_based_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_receiver,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityPoolStateAccount(pub LiquidityPoolState);
impl LiquidityPoolStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_POOL_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LiquidityPoolState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_POOL_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MSOL_TICKET_SOL_SPENT_ACCOUNT_DISCM: [u8; 8] = [
    66, 196, 62, 134, 124, 149, 250, 66,
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
pub struct MsolTicketSolSpent {
    pub cost_basis: u64,
    pub msol_unstaked: u64,
}
impl MsolTicketSolSpent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cost_basis: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_unstaked: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { cost_basis, msol_unstaked })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.cost_basis, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_unstaked, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MsolTicketSolSpentAccount(pub MsolTicketSolSpent);
impl MsolTicketSolSpentAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MSOL_TICKET_SOL_SPENT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MsolTicketSolSpent::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MSOL_TICKET_SOL_SPENT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_ADMIN_STATE_ACCOUNT_DISCM: [u8; 8] = [
    24, 124, 174, 225, 232, 30, 115, 192,
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
pub struct ProtocolAdminState {
    pub current_protocol_admin: Pubkey,
}
impl ProtocolAdminState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let current_protocol_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { current_protocol_admin })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.current_protocol_admin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolAdminStateAccount(pub ProtocolAdminState);
impl ProtocolAdminStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_ADMIN_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProtocolAdminState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_ADMIN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    207, 91, 250, 28, 152, 179, 215, 209,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ProtocolConfig {
    pub create_pool_fee: u64,
    pub initial_token_b_amount: f64,
    pub initial_token_a_amount: u64,
    pub unstaked_wsol_reserve: u64,
    pub total_sol_spent: u64,
    pub total_msol_received: u64,
    pub total_realized_profit: u64,
    pub pool_count: u64,
    pub max_supply_per_wallet: u64,
    pub creator_trading_fee_trading_volume_threshold: f64,
    pub market_cap_based_fees: LiquidityPoolMarketCapBasedFees,
    pub buffer_bps: u16,
    pub auto_staking_threshold_bps: u16,
    pub version: u16,
    pub protocol_config_state_bump: u8,
    pub allow_create_pool: u8,
    pub supported_pool_type: u8,
    pub default_leader_slot_window: u8,
    pub auto_staking_enabled: u8,
    pub leader_slot_window: u8,
    pub sandwich_resistence_enabled: u8,
    pub token_a_decimals: u8,
    pub migration_market_cap_threshold: u16,
    pub pad: [u8; 8],
    pub max_creator_trading_fee: u32,
    pub slot_offset_based_fees: LiquidityPoolSlotOffsetBasedFees,
}
impl ProtocolConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let create_pool_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_b_amount: f64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unstaked_wsol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_sol_spent: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_msol_received: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_realized_profit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply_per_wallet: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fee_trading_volume_threshold: f64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let market_cap_based_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolMarketCapBasedFees>::deserialize(&mut reader)?
        };
        let buffer_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let auto_staking_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_config_state_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let allow_create_pool: u8 = crate::borsh_de_or_default(&mut reader)?;
        let supported_pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let default_leader_slot_window: u8 = crate::borsh_de_or_default(&mut reader)?;
        let auto_staking_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let leader_slot_window: u8 = crate::borsh_de_or_default(&mut reader)?;
        let sandwich_resistence_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_market_cap_threshold: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pad: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let max_creator_trading_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let slot_offset_based_fees = <LiquidityPoolSlotOffsetBasedFees>::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            create_pool_fee,
            initial_token_b_amount,
            initial_token_a_amount,
            unstaked_wsol_reserve,
            total_sol_spent,
            total_msol_received,
            total_realized_profit,
            pool_count,
            max_supply_per_wallet,
            creator_trading_fee_trading_volume_threshold,
            market_cap_based_fees,
            buffer_bps,
            auto_staking_threshold_bps,
            version,
            protocol_config_state_bump,
            allow_create_pool,
            supported_pool_type,
            default_leader_slot_window,
            auto_staking_enabled,
            leader_slot_window,
            sandwich_resistence_enabled,
            token_a_decimals,
            migration_market_cap_threshold,
            pad,
            max_creator_trading_fee,
            slot_offset_based_fees,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.create_pool_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unstaked_wsol_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_sol_spent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_msol_received, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_realized_profit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supply_per_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_trading_volume_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.market_cap_based_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buffer_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auto_staking_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_config_state_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_create_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supported_pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_leader_slot_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auto_staking_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.leader_slot_window, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.sandwich_resistence_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_a_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.migration_market_cap_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pad, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_creator_trading_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot_offset_based_fees, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolConfigAccount(pub ProtocolConfig);
impl ProtocolConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProtocolConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_OWNER_STATE_ACCOUNT_DISCM: [u8; 8] = [
    208, 64, 209, 204, 113, 226, 22, 98,
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
pub struct ProtocolOwnerState {
    pub current_protocol_owner: Pubkey,
}
impl ProtocolOwnerState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let current_protocol_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { current_protocol_owner })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.current_protocol_owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolOwnerStateAccount(pub ProtocolOwnerState);
impl ProtocolOwnerStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_OWNER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProtocolOwnerState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_OWNER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_LP_POSITION_ACCOUNT_DISCM: [u8; 8] = [
    115, 204, 229, 204, 54, 180, 29, 195,
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
pub struct UserLpPosition {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub lp_token_balance: u64,
    pub reward_debt: u64,
    pub pending_fees: u64,
    pub bump: u8,
}
impl UserLpPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_debt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            user,
            lp_token_balance,
            reward_debt,
            pending_fees,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_debt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserLpPositionAccount(pub UserLpPosition);
impl UserLpPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_LP_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserLpPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_LP_POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
