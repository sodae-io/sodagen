use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CLAIM_CREATOR_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    151, 172, 14, 31, 51, 38, 77, 175,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimCreatorFeeEvent {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub creator_fee: u64,
}
impl ClaimCreatorFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            base_mint,
            creator_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeEventEvent(pub ClaimCreatorFeeEvent);
impl ClaimCreatorFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimCreatorFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_PROTOCOL_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    58, 138, 98, 233, 41, 88, 61, 220,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimProtocolFeeEvent {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub protocol_fee: u64,
    pub source: u8,
}
impl ClaimProtocolFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let source: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            base_mint,
            protocol_fee,
            source,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeEventEvent(pub ClaimProtocolFeeEvent);
impl ClaimProtocolFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PROTOCOL_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimProtocolFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PROTOCOL_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_RAYDIUM_CREATOR_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    207, 80, 158, 104, 0, 233, 246, 128,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectRaydiumCreatorFeeEvent {
    pub raydium_pool: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub amount_0: u64,
    pub amount_1: u64,
}
impl CollectRaydiumCreatorFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let raydium_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            raydium_pool,
            token_0_mint,
            token_1_mint,
            amount_0,
            amount_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.raydium_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRaydiumCreatorFeeEventEvent(pub CollectRaydiumCreatorFeeEvent);
impl CollectRaydiumCreatorFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_RAYDIUM_CREATOR_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectRaydiumCreatorFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_RAYDIUM_CREATOR_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_POOL_EVENT_EVENT_DISCM: [u8; 8] = [
    114, 34, 145, 114, 237, 44, 216, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolEvent {
    pub pool: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub virtual_base_reserve: u64,
    pub virtual_quote_reserve: u64,
}
impl InitializePoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            creator,
            base_mint,
            base_reserve,
            quote_reserve,
            virtual_base_reserve,
            virtual_quote_reserve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolEventEvent(pub InitializePoolEvent);
impl InitializePoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializePoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATE_EVENT_EVENT_DISCM: [u8; 8] = [216, 175, 231, 95, 45, 98, 108, 21];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrateEvent {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub raydium_pool: Pubkey,
    pub raydium_amm_config: Pubkey,
    pub creator_fee_on: u8,
    pub raydium_lp_mint: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub migration_payer: Pubkey,
}
impl MigrateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let raydium_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let raydium_amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_on: u8 = crate::borsh_de_or_default(&mut reader)?;
        let raydium_lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            base_mint,
            raydium_pool,
            raydium_amm_config,
            creator_fee_on,
            raydium_lp_mint,
            base_amount,
            quote_amount,
            migration_payer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.raydium_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.raydium_amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_on, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.raydium_lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_payer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateEventEvent(pub MigrateEvent);
impl MigrateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_READY_EVENT_EVENT_DISCM: [u8; 8] = [
    120, 196, 58, 20, 86, 254, 112, 140,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationReadyEvent {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
}
impl MigrationReadyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool, base_mint })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationReadyEventEvent(pub MigrationReadyEvent);
impl MigrationReadyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_READY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrationReadyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_READY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapEvent {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub trader: Pubkey,
    pub trade_direction: u8,
    pub amount_in: u64,
    pub actual_amount_in: u64,
    pub min_amount_out: u64,
    pub actual_amount_out: u64,
    pub creator_fee: u64,
    pub protocol_fee: u64,
    pub referral_fee: u64,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub virtual_base_reserve: u64,
    pub virtual_quote_reserve: u64,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_direction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let actual_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let actual_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            base_mint,
            trader,
            trade_direction,
            amount_in,
            actual_amount_in,
            min_amount_out,
            actual_amount_out,
            creator_fee,
            protocol_fee,
            referral_fee,
            base_reserve,
            quote_reserve,
            virtual_base_reserve,
            virtual_quote_reserve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.actual_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.actual_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);
impl SwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
