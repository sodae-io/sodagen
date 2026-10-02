use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BUY_EVENT_EVENT_DISCM: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyEvent {
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub creator: Pubkey,
    pub creator_vault: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub sol_in: u64,
    pub sol_to_curve: u64,
    pub sol_to_fee: u64,
    pub sol_to_protocol_fee: u64,
    pub sol_to_creator_fee: u64,
    pub sol_to_trader_cashback: u64,
    pub direct_referrer_profile: Option<Pubkey>,
    pub direct_referrer_wallet: Option<Pubkey>,
    pub sol_to_direct_referrer: u64,
    pub partner_indirect_profile: Option<Pubkey>,
    pub partner_indirect_wallet: Option<Pubkey>,
    pub sol_to_partner_indirect: u64,
    pub tokens_out: u64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub timestamp: i64,
}
impl BuyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let buyer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader_cashback_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_curve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_trader_cashback: u64 = crate::borsh_de_or_default(&mut reader)?;
        let direct_referrer_profile: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let direct_referrer_wallet: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let sol_to_direct_referrer: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_indirect_profile: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let partner_indirect_wallet: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let sol_to_partner_indirect: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            buyer,
            mint,
            protocol_fee_recipient,
            creator,
            creator_vault,
            trader_cashback_vault,
            sol_in,
            sol_to_curve,
            sol_to_fee,
            sol_to_protocol_fee,
            sol_to_creator_fee,
            sol_to_trader_cashback,
            direct_referrer_profile,
            direct_referrer_wallet,
            sol_to_direct_referrer,
            partner_indirect_profile,
            partner_indirect_wallet,
            sol_to_partner_indirect,
            tokens_out,
            virtual_sol_reserves,
            virtual_token_reserves,
            real_sol_reserves,
            real_token_reserves,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.buyer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader_cashback_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_trader_cashback, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direct_referrer_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direct_referrer_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_direct_referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_indirect_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_indirect_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_partner_indirect, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyEventEvent(pub BuyEvent);
impl BuyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BuyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COMPLETE_EVENT_EVENT_DISCM: [u8; 8] = [95, 114, 97, 156, 212, 46, 152, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteEvent {
    pub mint: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub migration_threshold_lamports: u64,
    pub timestamp: i64,
}
impl CompleteEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_threshold_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            virtual_sol_reserves,
            virtual_token_reserves,
            real_sol_reserves,
            real_token_reserves,
            migration_threshold_lamports,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.migration_threshold_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompleteEventEvent(pub CompleteEvent);
impl CompleteEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMPLETE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CompleteEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMPLETE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_EVENT_EVENT_DISCM: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateEvent {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_token_reserves: u64,
    pub protocol_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub creator_vault: Pubkey,
    pub created_at: i64,
    pub initial_virtual_sol_lamports: u64,
    pub migration_threshold_lamports: u64,
    pub launch_sol_usd_price: i64,
    pub launch_sol_usd_exponent: i32,
    pub launch_sol_usd_conf: u64,
    pub launch_sol_usd_publish_time: i64,
}
impl CreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_sol_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_threshold_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_exponent: i32 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_conf: u64 = crate::borsh_de_or_default(&mut reader)?;
        let launch_sol_usd_publish_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            creator,
            name,
            symbol,
            uri,
            virtual_sol_reserves,
            virtual_token_reserves,
            real_token_reserves,
            protocol_fee_bps,
            creator_fee_bps,
            creator_vault,
            created_at,
            initial_virtual_sol_lamports,
            migration_threshold_lamports,
            launch_sol_usd_price,
            launch_sol_usd_exponent,
            launch_sol_usd_conf,
            launch_sol_usd_publish_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
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
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateEventEvent(pub CreateEvent);
impl CreateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_REVENUE_PAID_EVENT_EVENT_DISCM: [u8; 8] = [
    2, 206, 74, 114, 155, 59, 128, 150,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatorRevenuePaidEvent {
    pub mint: Pubkey,
    pub recipient: Pubkey,
    pub recipient_revision: u64,
    pub amount_lamports: u64,
    pub through_settlement_sequence: u64,
    pub lifetime_creator_allocated_wsol: u64,
    pub lifetime_creator_paid_wsol: u64,
    pub timestamp: i64,
}
impl CreatorRevenuePaidEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let through_settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_creator_allocated_wsol: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lifetime_creator_paid_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            recipient,
            recipient_revision,
            amount_lamports,
            through_settlement_sequence,
            lifetime_creator_allocated_wsol,
            lifetime_creator_paid_wsol,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_revision, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.through_settlement_sequence,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_creator_allocated_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.lifetime_creator_paid_wsol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorRevenuePaidEventEvent(pub CreatorRevenuePaidEvent);
impl CreatorRevenuePaidEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_REVENUE_PAID_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatorRevenuePaidEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_REVENUE_PAID_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_REVENUE_RECIPIENT_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    87, 112, 71, 61, 82, 191, 212, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatorRevenueRecipientUpdatedEvent {
    pub mint: Pubkey,
    pub governance: Pubkey,
    pub old_recipient: Pubkey,
    pub new_recipient: Pubkey,
    pub old_revision: u64,
    pub new_revision: u64,
    pub timestamp: i64,
}
impl CreatorRevenueRecipientUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            governance,
            old_recipient,
            new_recipient,
            old_revision,
            new_revision,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.governance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_revision, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_revision, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorRevenueRecipientUpdatedEventEvent(
    pub CreatorRevenueRecipientUpdatedEvent,
);
impl CreatorRevenueRecipientUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_REVENUE_RECIPIENT_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatorRevenueRecipientUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_REVENUE_RECIPIENT_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_REVENUE_STATE_INITIALIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    135, 86, 31, 178, 220, 69, 255, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatorRevenueStateInitializedEvent {
    pub mint: Pubkey,
    pub migration_state: Pubkey,
    pub governance: Pubkey,
    pub governance_program: Pubkey,
    pub revenue_authority: Pubkey,
    pub creator_recipient: Pubkey,
    pub recipient_revision: u64,
    pub policy_version: u16,
}
impl CreatorRevenueStateInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migration_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let revenue_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        let policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            migration_state,
            governance,
            governance_program,
            revenue_authority,
            creator_recipient,
            recipient_revision,
            policy_version,
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
        borsh::BorshSerialize::serialize(&self.policy_version, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorRevenueStateInitializedEventEvent(
    pub CreatorRevenueStateInitializedEvent,
);
impl CreatorRevenueStateInitializedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_REVENUE_STATE_INITIALIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatorRevenueStateInitializedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_REVENUE_STATE_INITIALIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATOR_REWARDS_CLAIMED_EVENT_EVENT_DISCM: [u8; 8] = [
    37, 196, 71, 244, 120, 224, 26, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatorRewardsClaimedEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub creator_vault: Pubkey,
    pub amount: u64,
    pub total_claimed_lamports: u64,
    pub timestamp: i64,
}
impl CreatorRewardsClaimedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            mint,
            creator_vault,
            amount,
            total_claimed_lamports,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatorRewardsClaimedEventEvent(pub CreatorRewardsClaimedEvent);
impl CreatorRewardsClaimedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_REWARDS_CLAIMED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatorRewardsClaimedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_REWARDS_CLAIMED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const IDENTITY_BYPASS_PAID_EVENT_EVENT_DISCM: [u8; 8] = [
    1, 111, 230, 198, 50, 18, 112, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IdentityBypassPaidEvent {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub name_bypass_fee_lamports: u64,
    pub symbol_bypass_fee_lamports: u64,
    pub total_bypass_fee_lamports: u64,
    pub timestamp: i64,
}
impl IdentityBypassPaidEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name_bypass_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let symbol_bypass_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_bypass_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            creator,
            protocol_fee_recipient,
            name_bypass_fee_lamports,
            symbol_bypass_fee_lamports,
            total_bypass_fee_lamports,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name_bypass_fee_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol_bypass_fee_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_bypass_fee_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IdentityBypassPaidEventEvent(pub IdentityBypassPaidEvent);
impl IdentityBypassPaidEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != IDENTITY_BYPASS_PAID_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IdentityBypassPaidEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&IDENTITY_BYPASS_PAID_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    115, 64, 125, 137, 211, 17, 190, 43,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeConfigEvent {
    pub admin: Pubkey,
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub migration_target_usd_micros: u64,
    pub initial_virtual_sol_usd_micros: u64,
    pub initial_virtual_tokens: u64,
    pub token_total_supply: u64,
    pub curve_tokens_for_sale: u64,
    pub max_oracle_age_seconds: u32,
    pub max_oracle_conf_bps: u16,
    pub min_launch_sol_usd_price_micros: u64,
    pub max_launch_sol_usd_price_micros: u64,
}
impl InitializeConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let migration_target_usd_micros: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_sol_usd_micros: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_virtual_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_tokens_for_sale: u64 = crate::borsh_de_or_default(&mut reader)?;
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
            protocol_fee_bps,
            creator_fee_bps,
            migration_target_usd_micros,
            initial_virtual_sol_usd_micros,
            initial_virtual_tokens,
            token_total_supply,
            curve_tokens_for_sale,
            max_oracle_age_seconds,
            max_oracle_conf_bps,
            min_launch_sol_usd_price_micros,
            max_launch_sol_usd_price_micros,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.migration_target_usd_micros,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.initial_virtual_sol_usd_micros,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.initial_virtual_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_tokens_for_sale, &mut writer)?;
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
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigEventEvent(pub InitializeConfigEvent);
impl InitializeConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LAUNCH_PAIR_UNWHITELISTED_EVENT_EVENT_DISCM: [u8; 8] = [
    15, 152, 92, 100, 174, 245, 171, 112,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LaunchPairUnwhitelistedEvent {
    pub admin: Pubkey,
    pub name: String,
    pub symbol: String,
}
impl LaunchPairUnwhitelistedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, name, symbol })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchPairUnwhitelistedEventEvent(pub LaunchPairUnwhitelistedEvent);
impl LaunchPairUnwhitelistedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_PAIR_UNWHITELISTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LaunchPairUnwhitelistedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_PAIR_UNWHITELISTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LAUNCH_PAIR_WHITELISTED_EVENT_EVENT_DISCM: [u8; 8] = [
    14, 118, 198, 37, 104, 61, 72, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LaunchPairWhitelistedEvent {
    pub admin: Pubkey,
    pub name: String,
    pub symbol: String,
}
impl LaunchPairWhitelistedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, name, symbol })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchPairWhitelistedEventEvent(pub LaunchPairWhitelistedEvent);
impl LaunchPairWhitelistedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_PAIR_WHITELISTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LaunchPairWhitelistedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_PAIR_WHITELISTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_COMPLETED_EVENT_EVENT_DISCM: [u8; 8] = [
    188, 136, 172, 224, 135, 77, 206, 225,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationCompletedEvent {
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
    pub timestamp: i64,
}
impl MigrationCompletedEvent {
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
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
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
            timestamp,
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
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationCompletedEventEvent(pub MigrationCompletedEvent);
impl MigrationCompletedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_COMPLETED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrationCompletedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_COMPLETED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_CONFIG_INITIALIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    80, 138, 145, 57, 44, 119, 75, 183,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationConfigInitializedEvent {
    pub raydium_cpmm_program: Pubkey,
    pub amm_config: Pubkey,
    pub permission: Pubkey,
    pub executor: Pubkey,
    pub enabled: bool,
    pub version: u16,
}
impl MigrationConfigInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let raydium_cpmm_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let executor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            raydium_cpmm_program,
            amm_config,
            permission,
            executor,
            enabled,
            version,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.raydium_cpmm_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationConfigInitializedEventEvent(pub MigrationConfigInitializedEvent);
impl MigrationConfigInitializedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_CONFIG_INITIALIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrationConfigInitializedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_CONFIG_INITIALIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_CONFIG_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    212, 80, 181, 33, 103, 125, 107, 141,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationConfigUpdatedEvent {
    pub executor: Pubkey,
    pub permission: Pubkey,
    pub enabled: bool,
    pub version: u16,
}
impl MigrationConfigUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let executor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            executor,
            permission,
            enabled,
            version,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.executor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationConfigUpdatedEventEvent(pub MigrationConfigUpdatedEvent);
impl MigrationConfigUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_CONFIG_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrationConfigUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_CONFIG_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PARTNER_STATUS_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    93, 228, 179, 243, 33, 227, 12, 41,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerStatusUpdatedEvent {
    pub admin: Pubkey,
    pub referrer_profile: Pubkey,
    pub x_id_hash: [u8; 32],
    pub is_partner: bool,
}
impl PartnerStatusUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer_profile: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let is_partner: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            referrer_profile,
            x_id_hash,
            is_partner,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_partner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartnerStatusUpdatedEventEvent(pub PartnerStatusUpdatedEvent);
impl PartnerStatusUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTNER_STATUS_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PartnerStatusUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTNER_STATUS_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_REVENUE_PAID_EVENT_EVENT_DISCM: [u8; 8] = [
    243, 211, 169, 119, 199, 62, 150, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProtocolRevenuePaidEvent {
    pub mint: Pubkey,
    pub recipient: Pubkey,
    pub policy_version: u16,
    pub amount_lamports: u64,
    pub through_settlement_sequence: u64,
    pub lifetime_protocol_allocated_wsol: u64,
    pub lifetime_protocol_paid_wsol: u64,
    pub timestamp: i64,
}
impl ProtocolRevenuePaidEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let through_settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_protocol_allocated_wsol: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lifetime_protocol_paid_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            recipient,
            policy_version,
            amount_lamports,
            through_settlement_sequence,
            lifetime_protocol_allocated_wsol,
            lifetime_protocol_paid_wsol,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.policy_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.through_settlement_sequence,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_protocol_allocated_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_protocol_paid_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolRevenuePaidEventEvent(pub ProtocolRevenuePaidEvent);
impl ProtocolRevenuePaidEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_REVENUE_PAID_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ProtocolRevenuePaidEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_REVENUE_PAID_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRAL_REWARDS_CLAIMED_EVENT_EVENT_DISCM: [u8; 8] = [
    227, 143, 230, 124, 105, 220, 133, 118,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferralRewardsClaimedEvent {
    pub payout_wallet: Pubkey,
    pub referrer_profile: Pubkey,
    pub trader: Pubkey,
    pub trader_referral: Pubkey,
    pub role: ReferralRewardRole,
    pub amount: u64,
    pub total_earned_lamports: u64,
    pub timestamp: i64,
}
impl ReferralRewardsClaimedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payout_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer_profile: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader_referral: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let role: ReferralRewardRole = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_earned_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            payout_wallet,
            referrer_profile,
            trader,
            trader_referral,
            role,
            amount,
            total_earned_lamports,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payout_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_earned_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferralRewardsClaimedEventEvent(pub ReferralRewardsClaimedEvent);
impl ReferralRewardsClaimedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRAL_REWARDS_CLAIMED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReferralRewardsClaimedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRAL_REWARDS_CLAIMED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRALS_INITIALIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    244, 91, 129, 204, 249, 225, 56, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferralsInitializedEvent {
    pub user: Pubkey,
    pub trader_referral: Pubkey,
    pub referrer_profile: Pubkey,
    pub user_x_id_hash: [u8; 32],
    pub referrer_x_id_hash: Option<[u8; 32]>,
    pub indirect_x_id_hash: Option<[u8; 32]>,
}
impl ReferralsInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader_referral: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer_profile: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let referrer_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let indirect_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            user,
            trader_referral,
            referrer_profile,
            user_x_id_hash,
            referrer_x_id_hash,
            indirect_x_id_hash,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.indirect_x_id_hash, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferralsInitializedEventEvent(pub ReferralsInitializedEvent);
impl ReferralsInitializedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRALS_INITIALIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReferralsInitializedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRALS_INITIALIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_CONFIG_INITIALIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    186, 216, 82, 76, 71, 81, 148, 31,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevenueConfigInitializedEvent {
    pub accounting_authority: Pubkey,
    pub protocol_recipient: Pubkey,
    pub governance_program: Pubkey,
    pub policy_version: u16,
}
impl RevenueConfigInitializedEvent {
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
pub struct RevenueConfigInitializedEventEvent(pub RevenueConfigInitializedEvent);
impl RevenueConfigInitializedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_CONFIG_INITIALIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RevenueConfigInitializedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_CONFIG_INITIALIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_CONFIG_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    233, 253, 37, 170, 243, 105, 70, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevenueConfigUpdatedEvent {
    pub old_accounting_authority: Pubkey,
    pub new_accounting_authority: Pubkey,
    pub old_protocol_recipient: Pubkey,
    pub new_protocol_recipient: Pubkey,
    pub governance_program: Pubkey,
    pub old_policy_version: u16,
    pub new_policy_version: u16,
}
impl RevenueConfigUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_accounting_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_accounting_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_protocol_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_protocol_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let governance_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_accounting_authority,
            new_accounting_authority,
            old_protocol_recipient,
            new_protocol_recipient,
            governance_program,
            old_policy_version,
            new_policy_version,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_accounting_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_accounting_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_protocol_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_protocol_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.governance_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_policy_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_policy_version, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevenueConfigUpdatedEventEvent(pub RevenueConfigUpdatedEvent);
impl RevenueConfigUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_CONFIG_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RevenueConfigUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_CONFIG_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_SETTLED_EVENT_EVENT_DISCM: [u8; 8] = [
    59, 53, 197, 71, 142, 121, 24, 164,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevenueSettledEvent {
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub settlement_sequence: u64,
    pub accounting_cutoff_slot: u64,
    pub policy_version: u16,
    pub cumulative_tier_0_gross_wsol: u64,
    pub cumulative_tier_1_gross_wsol: u64,
    pub cumulative_tier_2_gross_wsol: u64,
    pub cumulative_tier_3_gross_wsol: u64,
    pub lifetime_creator_allocated_wsol: u64,
    pub lifetime_protocol_allocated_wsol: u64,
    pub collected_wsol: u64,
    pub lifetime_collected_wsol: u64,
    pub unallocated_wsol: u64,
    pub timestamp: i64,
}
impl RevenueSettledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let settlement_sequence: u64 = crate::borsh_de_or_default(&mut reader)?;
        let accounting_cutoff_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let policy_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_0_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_1_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_2_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_tier_3_gross_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_creator_allocated_wsol: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lifetime_protocol_allocated_wsol: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let collected_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lifetime_collected_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unallocated_wsol: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            pool,
            settlement_sequence,
            accounting_cutoff_slot,
            policy_version,
            cumulative_tier_0_gross_wsol,
            cumulative_tier_1_gross_wsol,
            cumulative_tier_2_gross_wsol,
            cumulative_tier_3_gross_wsol,
            lifetime_creator_allocated_wsol,
            lifetime_protocol_allocated_wsol,
            collected_wsol,
            lifetime_collected_wsol,
            unallocated_wsol,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settlement_sequence, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting_cutoff_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.policy_version, &mut writer)?;
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
        borsh::BorshSerialize::serialize(
            &self.lifetime_creator_allocated_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lifetime_protocol_allocated_wsol,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.collected_wsol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lifetime_collected_wsol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unallocated_wsol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevenueSettledEventEvent(pub RevenueSettledEvent);
impl RevenueSettledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_SETTLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RevenueSettledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_SETTLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SELL_EVENT_EVENT_DISCM: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellEvent {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub creator: Pubkey,
    pub creator_vault: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub tokens_in: u64,
    pub sol_gross: u64,
    pub sol_to_user: u64,
    pub sol_to_fee: u64,
    pub sol_to_protocol_fee: u64,
    pub sol_to_creator_fee: u64,
    pub sol_to_trader_cashback: u64,
    pub direct_referrer_profile: Option<Pubkey>,
    pub direct_referrer_wallet: Option<Pubkey>,
    pub sol_to_direct_referrer: u64,
    pub partner_indirect_profile: Option<Pubkey>,
    pub partner_indirect_wallet: Option<Pubkey>,
    pub sol_to_partner_indirect: u64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub timestamp: i64,
}
impl SellEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let seller: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader_cashback_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tokens_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_gross: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_user: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_to_trader_cashback: u64 = crate::borsh_de_or_default(&mut reader)?;
        let direct_referrer_profile: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let direct_referrer_wallet: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let sol_to_direct_referrer: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_indirect_profile: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let partner_indirect_wallet: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let sol_to_partner_indirect: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            seller,
            mint,
            protocol_fee_recipient,
            creator,
            creator_vault,
            trader_cashback_vault,
            tokens_in,
            sol_gross,
            sol_to_user,
            sol_to_fee,
            sol_to_protocol_fee,
            sol_to_creator_fee,
            sol_to_trader_cashback,
            direct_referrer_profile,
            direct_referrer_wallet,
            sol_to_direct_referrer,
            partner_indirect_profile,
            partner_indirect_wallet,
            sol_to_partner_indirect,
            virtual_sol_reserves,
            virtual_token_reserves,
            real_sol_reserves,
            real_token_reserves,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.seller, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader_cashback_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_gross, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_trader_cashback, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direct_referrer_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direct_referrer_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_direct_referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_indirect_profile, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_indirect_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_to_partner_indirect, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellEventEvent(pub SellEvent);
impl SellEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SellEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADE_AUTHORITY_UNWHITELISTED_EVENT_EVENT_DISCM: [u8; 8] = [
    139, 191, 194, 1, 244, 80, 3, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeAuthorityUnwhitelistedEvent {
    pub admin: Pubkey,
    pub authority: Pubkey,
}
impl TradeAuthorityUnwhitelistedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeAuthorityUnwhitelistedEventEvent(pub TradeAuthorityUnwhitelistedEvent);
impl TradeAuthorityUnwhitelistedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_AUTHORITY_UNWHITELISTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeAuthorityUnwhitelistedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_AUTHORITY_UNWHITELISTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADE_AUTHORITY_WHITELIST_MIGRATED_EVENT_EVENT_DISCM: [u8; 8] = [
    9, 100, 2, 27, 13, 238, 222, 0,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeAuthorityWhitelistMigratedEvent {
    pub admin: Pubkey,
    pub initial_authority: Pubkey,
}
impl TradeAuthorityWhitelistMigratedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let initial_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, initial_authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeAuthorityWhitelistMigratedEventEvent(
    pub TradeAuthorityWhitelistMigratedEvent,
);
impl TradeAuthorityWhitelistMigratedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_AUTHORITY_WHITELIST_MIGRATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeAuthorityWhitelistMigratedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_AUTHORITY_WHITELIST_MIGRATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADE_AUTHORITY_WHITELISTED_EVENT_EVENT_DISCM: [u8; 8] = [
    92, 249, 194, 254, 246, 24, 181, 108,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeAuthorityWhitelistedEvent {
    pub admin: Pubkey,
    pub authority: Pubkey,
}
impl TradeAuthorityWhitelistedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { admin, authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeAuthorityWhitelistedEventEvent(pub TradeAuthorityWhitelistedEvent);
impl TradeAuthorityWhitelistedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_AUTHORITY_WHITELISTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeAuthorityWhitelistedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_AUTHORITY_WHITELISTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADER_CASHBACK_CLAIMED_EVENT_EVENT_DISCM: [u8; 8] = [
    15, 62, 104, 149, 119, 124, 217, 168,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraderCashbackClaimedEvent {
    pub trader: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub amount: u64,
    pub total_claimed_lamports: u64,
    pub timestamp: i64,
}
impl TraderCashbackClaimedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader_cashback_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader,
            trader_cashback_vault,
            amount,
            total_claimed_lamports,
            timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader_cashback_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TraderCashbackClaimedEventEvent(pub TraderCashbackClaimedEvent);
impl TraderCashbackClaimedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADER_CASHBACK_CLAIMED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TraderCashbackClaimedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADER_CASHBACK_CLAIMED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADER_REFERRAL_INITIALIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    193, 10, 72, 253, 131, 36, 197, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraderReferralInitializedEvent {
    pub user: Pubkey,
    pub trader_referral: Pubkey,
    pub user_x_id_hash: [u8; 32],
    pub referrer_x_id_hash: Option<[u8; 32]>,
    pub indirect_x_id_hash: Option<[u8; 32]>,
}
impl TraderReferralInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader_referral: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_x_id_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let referrer_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let indirect_x_id_hash: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            user,
            trader_referral,
            user_x_id_hash,
            referrer_x_id_hash,
            indirect_x_id_hash,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader_referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_x_id_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.indirect_x_id_hash, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TraderReferralInitializedEventEvent(pub TraderReferralInitializedEvent);
impl TraderReferralInitializedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADER_REFERRAL_INITIALIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TraderReferralInitializedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADER_REFERRAL_INITIALIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
