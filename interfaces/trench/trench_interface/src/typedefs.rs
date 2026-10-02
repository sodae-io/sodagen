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
pub struct BuyExactOutParams {
    pub tokens_out: u64,
    pub max_sol_in: u64,
}
impl BuyExactOutParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { tokens_out, max_sol_in })
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
pub struct BuyParams {
    pub max_sol_in: u64,
    pub min_tokens_out: u64,
}
impl BuyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { max_sol_in, min_tokens_out })
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
pub struct CreateParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub launch_sol_usd_price: i64,
    pub launch_sol_usd_exponent: i32,
    pub launch_sol_usd_conf: u64,
    pub launch_sol_usd_publish_time: i64,
}
impl CreateParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_exponent: i32 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_conf: u64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_publish_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            symbol,
            uri,
            launch_sol_usd_price,
            launch_sol_usd_exponent,
            launch_sol_usd_conf,
            launch_sol_usd_publish_time,
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
pub struct InitializeConfigParams {
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
}
impl InitializeConfigParams {
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
pub struct InitializeMigrationConfigParams {
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
}
impl InitializeMigrationConfigParams {
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
pub struct InitializeReferralsParams {
    pub user_x_id_hash: [u8; 32],
    pub referrer_x_id_hash: Option<[u8; 32]>,
    pub indirect_x_id_hash: Option<[u8; 32]>,
}
impl InitializeReferralsParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let referrer_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let indirect_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            user_x_id_hash,
            referrer_x_id_hash,
            indirect_x_id_hash,
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
pub struct InitializeTraderReferralParams {
    pub user_x_id_hash: [u8; 32],
    pub referrer_x_id_hash: Option<[u8; 32]>,
    pub indirect_x_id_hash: Option<[u8; 32]>,
}
impl InitializeTraderReferralParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let referrer_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let indirect_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            user_x_id_hash,
            referrer_x_id_hash,
            indirect_x_id_hash,
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
pub struct PayoutCreatorRevenueParams {
    pub expected_settlement_sequence: u64,
    pub expected_recipient_revision: u64,
}
impl PayoutCreatorRevenueParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expected_settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_recipient_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            expected_settlement_sequence,
            expected_recipient_revision,
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
pub struct PayoutProtocolRevenueParams {
    pub expected_settlement_sequence: u64,
}
impl PayoutProtocolRevenueParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expected_settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            expected_settlement_sequence,
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
pub enum ReferralRewardRole {
    #[default]
    Direct,
    PartnerIndirect,
}
impl TryFrom<u8> for ReferralRewardRole {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Direct),
            1u8 => Ok(Self::PartnerIndirect),
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
pub struct RevenueConfigParams {
    pub accounting_authority: Pubkey,
    pub protocol_recipient: Pubkey,
    pub governance_program: Pubkey,
    pub policy_version: u16,
}
impl RevenueConfigParams {
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
pub struct SellParams {
    pub tokens_in: u64,
    pub min_sol_out: u64,
}
impl SellParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tokens_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_sol_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { tokens_in, min_sol_out })
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
pub struct SetPartnerStatusParams {
    pub x_id_hash: [u8; 32],
    pub is_partner: bool,
}
impl SetPartnerStatusParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let is_partner: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { x_id_hash, is_partner })
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
pub struct SettleCreatorRevenueParams {
    pub expected_settlement_sequence: u64,
    pub expected_policy_version: u16,
    pub accounting_cutoff_slot: u64,
    pub cumulative_tier_0_gross_wsol: u64,
    pub cumulative_tier_1_gross_wsol: u64,
    pub cumulative_tier_2_gross_wsol: u64,
    pub cumulative_tier_3_gross_wsol: u64,
}
impl SettleCreatorRevenueParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expected_settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let accounting_cutoff_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_0_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_1_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_2_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_3_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            expected_settlement_sequence,
            expected_policy_version,
            accounting_cutoff_slot,
            cumulative_tier_0_gross_wsol,
            cumulative_tier_1_gross_wsol,
            cumulative_tier_2_gross_wsol,
            cumulative_tier_3_gross_wsol,
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
pub struct UpdateMigrationConfigParams {
    pub executor: Pubkey,
    pub permission: Pubkey,
    pub enabled: bool,
}
impl UpdateMigrationConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let executor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            executor,
            permission,
            enabled,
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
pub struct WhitelistLaunchPairParams {
    pub name: String,
    pub symbol: String,
}
impl WhitelistLaunchPairParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol })
    }
}
