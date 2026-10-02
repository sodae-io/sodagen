use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BONDING_CURVE_ACCOUNT_DISCM: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];
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
pub struct BondingCurve {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub protocol_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub complete: bool,
    pub created_at: i64,
    pub bump: u8,
    pub initial_virtual_sol_lamports: u64,
    pub migration_threshold_lamports: u64,
    pub launch_sol_usd_price: i64,
    pub launch_sol_usd_exponent: i32,
    pub launch_sol_usd_conf: u64,
    pub launch_sol_usd_publish_time: i64,
    pub created_slot: u64,
    pub creator_same_slot_buy_used: bool,
    pub migration_ready_at: i64,
}
impl BondingCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let complete: bool = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_sol_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_threshold_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_exponent: i32 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_conf: u64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_publish_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let created_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_same_slot_buy_used: bool = crate::borsh_de_or_default(&mut reader)?;
        let migration_ready_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            creator,
            virtual_sol_reserves,
            virtual_token_reserves,
            real_sol_reserves,
            real_token_reserves,
            protocol_fee_bps,
            creator_fee_bps,
            complete,
            created_at,
            bump,
            initial_virtual_sol_lamports,
            migration_threshold_lamports,
            launch_sol_usd_price,
            launch_sol_usd_exponent,
            launch_sol_usd_conf,
            launch_sol_usd_publish_time,
            created_slot,
            creator_same_slot_buy_used,
            migration_ready_at,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.complete, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.initial_virtual_sol_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.migration_threshold_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.launch_sol_usd_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.launch_sol_usd_exponent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.launch_sol_usd_conf, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.launch_sol_usd_publish_time,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.created_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_same_slot_buy_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_ready_at, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BondingCurveAccount(pub BondingCurve);
impl BondingCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BONDING_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BondingCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BONDING_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BUYER_LIMIT_ACCOUNT_DISCM: [u8; 8] = [147, 154, 93, 117, 18, 111, 200, 240];
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
pub struct BuyerLimit {
    pub mint: Pubkey,
    pub trader: Pubkey,
    pub net_bought_tokens: u64,
    pub bump: u8,
}
impl BuyerLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let net_bought_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            trader,
            net_bought_tokens,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.net_bought_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyerLimitAccount(pub BuyerLimit);
impl BuyerLimitAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUYER_LIMIT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BuyerLimit::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUYER_LIMIT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
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
pub struct Config {
    pub admin: Pubkey,
    pub authority: Pubkey,
    pub fee_addresses: [Pubkey; 5],
    pub protocol_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub migration_fee_lamports: u64,
    pub initial_virtual_sol_usd_micros: u64,
    pub initial_virtual_tokens: u64,
    pub token_total_supply: u64,
    pub curve_tokens_for_sale: u64,
    pub migration_target_usd_micros: u64,
    pub max_oracle_age_seconds: u32,
    pub max_oracle_conf_bps: u16,
    pub min_launch_sol_usd_price_micros: u64,
    pub max_launch_sol_usd_price_micros: u64,
    pub bump: u8,
    pub trade_authorities: Vec<Pubkey>,
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_addresses: [Pubkey; 5] = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_sol_usd_micros: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_virtual_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_tokens_for_sale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_target_usd_micros: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_oracle_age_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_oracle_conf_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let min_launch_sol_usd_price_micros: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_launch_sol_usd_price_micros: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let trade_authorities: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            authority,
            fee_addresses,
            protocol_fee_bps,
            creator_fee_bps,
            migration_fee_lamports,
            initial_virtual_sol_usd_micros,
            initial_virtual_tokens,
            token_total_supply,
            curve_tokens_for_sale,
            migration_target_usd_micros,
            max_oracle_age_seconds,
            max_oracle_conf_bps,
            min_launch_sol_usd_price_micros,
            max_launch_sol_usd_price_micros,
            bump,
            trade_authorities,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_addresses, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_fee_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.initial_virtual_sol_usd_micros,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.initial_virtual_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_tokens_for_sale, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.migration_target_usd_micros,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.max_oracle_age_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_oracle_conf_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_launch_sol_usd_price_micros,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_launch_sol_usd_price_micros,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_authorities, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccount(pub Config);
impl ConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Config::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_COOLDOWN_ACCOUNT_DISCM: [u8; 8] = [93, 121, 112, 164, 17, 97, 53, 27];
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
pub struct CreatorCooldown {
    pub creator: Pubkey,
    pub last_created_at: i64,
    pub bump: u8,
}
impl CreatorCooldown {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            last_created_at,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorCooldownAccount(pub CreatorCooldown);
impl CreatorCooldownAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_COOLDOWN_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CreatorCooldown::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_COOLDOWN_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_REVENUE_STATE_ACCOUNT_DISCM: [u8; 8] = [
    14, 136, 90, 233, 137, 210, 163, 86,
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
pub struct CreatorRevenueState {
    pub mint: Pubkey,
    pub migration_state: Pubkey,
    pub governance: Pubkey,
    pub governance_program: Pubkey,
    pub revenue_authority: Pubkey,
    pub creator_recipient: Pubkey,
    pub recipient_revision: u64,
    pub settlement_sequence: u64,
    pub accounting_cutoff_slot: u64,
    pub cumulative_tier_0_gross_wsol: u64,
    pub cumulative_tier_1_gross_wsol: u64,
    pub cumulative_tier_2_gross_wsol: u64,
    pub cumulative_tier_3_gross_wsol: u64,
    pub lifetime_collected_wsol: u64,
    pub lifetime_creator_allocated_wsol: u64,
    pub lifetime_protocol_allocated_wsol: u64,
    pub lifetime_creator_paid_wsol: u64,
    pub lifetime_protocol_paid_wsol: u64,
    pub policy_version: u16,
    pub bump: u8,
}
impl CreatorRevenueState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migration_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let revenue_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        let settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let accounting_cutoff_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_0_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_1_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_2_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_3_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_collected_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_creator_allocated_wsol: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lifetime_protocol_allocated_wsol: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lifetime_creator_paid_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_protocol_paid_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            migration_state,
            governance,
            governance_program,
            revenue_authority,
            creator_recipient,
            recipient_revision,
            settlement_sequence,
            accounting_cutoff_slot,
            cumulative_tier_0_gross_wsol,
            cumulative_tier_1_gross_wsol,
            cumulative_tier_2_gross_wsol,
            cumulative_tier_3_gross_wsol,
            lifetime_collected_wsol,
            lifetime_creator_allocated_wsol,
            lifetime_protocol_allocated_wsol,
            lifetime_creator_paid_wsol,
            lifetime_protocol_paid_wsol,
            policy_version,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.governance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.governance_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_revision, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settlement_sequence, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting_cutoff_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_tier_0_gross_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_tier_1_gross_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_tier_2_gross_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_tier_3_gross_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.lifetime_collected_wsol, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_creator_allocated_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_protocol_allocated_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.lifetime_creator_paid_wsol, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_protocol_paid_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.policy_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorRevenueStateAccount(pub CreatorRevenueState);
impl CreatorRevenueStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_REVENUE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CreatorRevenueState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_REVENUE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_VAULT_ACCOUNT_DISCM: [u8; 8] = [200, 135, 38, 98, 35, 236, 238, 12];
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
pub struct CreatorVault {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub total_earned_lamports: u64,
    pub total_claimed_lamports: u64,
    pub bump: u8,
}
impl CreatorVault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_earned_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            creator,
            total_earned_lamports,
            total_claimed_lamports,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_earned_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorVaultAccount(pub CreatorVault);
impl CreatorVaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CreatorVault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LAUNCH_LIMIT_TRACKER_ACCOUNT_DISCM: [u8; 8] = [
    224, 63, 15, 55, 90, 72, 114, 2,
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
pub struct LaunchLimitTracker {
    pub created_at: [i64; 3],
    pub bump: u8,
}
impl LaunchLimitTracker {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let created_at: [i64; 3] = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { created_at, bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchLimitTrackerAccount(pub LaunchLimitTracker);
impl LaunchLimitTrackerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_LIMIT_TRACKER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LaunchLimitTracker::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_LIMIT_TRACKER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LAUNCH_PAIR_WHITELIST_ACCOUNT_DISCM: [u8; 8] = [
    142, 22, 64, 117, 80, 193, 35, 48,
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
pub struct LaunchPairWhitelist {}
impl LaunchPairWhitelist {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchPairWhitelistAccount(pub LaunchPairWhitelist);
impl LaunchPairWhitelistAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_PAIR_WHITELIST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LaunchPairWhitelist::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_PAIR_WHITELIST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    221, 35, 251, 172, 252, 90, 70, 161,
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
pub struct MigrationConfig {
    pub raydium_cpmm_program: Pubkey,
    pub amm_config: Pubkey,
    pub permission: Pubkey,
    pub wsol_mint: Pubkey,
    pub create_pool_fee_receiver: Pubkey,
    pub executor: Pubkey,
    pub expected_trade_fee_rate: u64,
    pub expected_creator_fee_rate: u64,
    pub expected_protocol_fee_rate: u64,
    pub expected_fund_fee_rate: u64,
    pub expected_create_pool_fee_lamports: u64,
    pub enabled: bool,
    pub version: u16,
    pub bump: u8,
}
impl MigrationConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let raydium_cpmm_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wsol_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let executor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let expected_trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_protocol_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_fund_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_create_pool_fee_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            raydium_cpmm_program,
            amm_config,
            permission,
            wsol_mint,
            create_pool_fee_receiver,
            executor,
            expected_trade_fee_rate,
            expected_creator_fee_rate,
            expected_protocol_fee_rate,
            expected_fund_fee_rate,
            expected_create_pool_fee_lamports,
            enabled,
            version,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.raydium_cpmm_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wsol_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_fee_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expected_trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expected_creator_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expected_protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expected_fund_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.expected_create_pool_fee_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationConfigAccount(pub MigrationConfig);
impl MigrationConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MigrationConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_STATE_ACCOUNT_DISCM: [u8; 8] = [95, 146, 135, 64, 145, 25, 197, 115];
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
pub struct MigrationState {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub amm_config: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub observation: Pubkey,
    pub revenue_authority: Pubkey,
    pub gross_sol_lamports: u64,
    pub create_pool_fee_lamports: u64,
    pub permanent_rent_lamports: u64,
    pub deposited_sol_lamports: u64,
    pub deposited_tokens: u64,
    pub lp_burned: u64,
    pub pool_open_time: u64,
    pub config_version: u16,
    pub migrated_at: i64,
    pub bump: u8,
}
impl MigrationState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bonding_curve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observation: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let revenue_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let gross_sol_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let permanent_rent_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposited_sol_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposited_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let config_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            bonding_curve,
            amm_config,
            pool,
            lp_mint,
            token_0_mint,
            token_1_mint,
            token_0_vault,
            token_1_vault,
            observation,
            revenue_authority,
            gross_sol_lamports,
            create_pool_fee_lamports,
            permanent_rent_lamports,
            deposited_sol_lamports,
            deposited_tokens,
            lp_burned,
            pool_open_time,
            config_version,
            migrated_at,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bonding_curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.gross_sol_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_fee_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permanent_rent_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposited_sol_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposited_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationStateAccount(pub MigrationState);
impl MigrationStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MigrationState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_PROFILE_ACCOUNT_DISCM: [u8; 8] = [
    68, 229, 194, 132, 171, 78, 224, 23,
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
pub struct ReferrerProfile {
    pub x_id_hash: [u8; 32],
    pub payout_wallet: Pubkey,
    pub is_partner: bool,
    pub total_earned_lamports: u64,
    pub bump: u8,
}
impl ReferrerProfile {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let payout_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_partner: bool = crate::borsh_de_or_default(&mut reader)?;
        let total_earned_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            x_id_hash,
            payout_wallet,
            is_partner,
            total_earned_lamports,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payout_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_earned_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerProfileAccount(pub ReferrerProfile);
impl ReferrerProfileAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_PROFILE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ReferrerProfile::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_PROFILE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [7, 27, 216, 11, 136, 50, 125, 57];
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
pub struct RevenueConfig {
    pub accounting_authority: Pubkey,
    pub protocol_recipient: Pubkey,
    pub governance_program: Pubkey,
    pub policy_version: u16,
}
impl RevenueConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accounting_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            accounting_authority,
            protocol_recipient,
            governance_program,
            policy_version,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.accounting_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.governance_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.policy_version, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevenueConfigAccount(pub RevenueConfig);
impl RevenueConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RevenueConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADER_CASHBACK_VAULT_ACCOUNT_DISCM: [u8; 8] = [
    246, 66, 233, 154, 84, 60, 84, 84,
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
pub struct TraderCashbackVault {
    pub trader: Pubkey,
    pub total_earned_lamports: u64,
    pub total_claimed_lamports: u64,
    pub bump: u8,
}
impl TraderCashbackVault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_earned_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader,
            total_earned_lamports,
            total_claimed_lamports,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_earned_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TraderCashbackVaultAccount(pub TraderCashbackVault);
impl TraderCashbackVaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADER_CASHBACK_VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TraderCashbackVault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADER_CASHBACK_VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADER_REFERRAL_ACCOUNT_DISCM: [u8; 8] = [128, 194, 68, 99, 45, 69, 154, 115];
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
pub struct TraderReferral {
    pub trader: Pubkey,
    pub referrer_x_id_hash: Option<[u8; 32]>,
    pub indirect_x_id_hash: Option<[u8; 32]>,
    pub direct_claimable_lamports: u64,
    pub indirect_claimable_lamports: u64,
    pub bump: u8,
}
impl TraderReferral {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let indirect_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let direct_claimable_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let indirect_claimable_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader,
            referrer_x_id_hash,
            indirect_x_id_hash,
            direct_claimable_lamports,
            indirect_claimable_lamports,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.indirect_x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direct_claimable_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.indirect_claimable_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TraderReferralAccount(pub TraderReferral);
impl TraderReferralAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADER_REFERRAL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TraderReferral::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADER_REFERRAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
