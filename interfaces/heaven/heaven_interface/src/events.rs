use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CREATE_LIQUIDITY_POOL_EVENT_EVENT_DISCM: [u8; 8] = [
    116, 216, 239, 141, 207, 211, 178, 127,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateLiquidityPoolEvent {
    pub liquidity_pool_id: Pubkey,
    pub user: Pubkey,
    pub base_token_input_transfer_fee_amount: u64,
    pub quote_token_input_transfer_fee_amount: u64,
    pub base_token_input_amount: u64,
    pub quote_token_input_amount: u64,
    pub lp_token_output_amount: u64,
    pub locked_lp: bool,
}
impl CreateLiquidityPoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token_input_transfer_fee_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let quote_token_input_transfer_fee_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_token_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_pool_id,
            user,
            base_token_input_transfer_fee_amount,
            quote_token_input_transfer_fee_amount,
            base_token_input_amount,
            quote_token_input_amount,
            lp_token_output_amount,
            locked_lp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.liquidity_pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.base_token_input_transfer_fee_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.quote_token_input_transfer_fee_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.base_token_input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_lp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLiquidityPoolEventEvent(pub CreateLiquidityPoolEvent);
impl CreateLiquidityPoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LIQUIDITY_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateLiquidityPoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LIQUIDITY_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_STANDARD_LIQUIDITY_POOL_EVENT_EVENT_DISCM: [u8; 8] = [
    189, 56, 131, 144, 75, 63, 249, 148,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateStandardLiquidityPoolEvent {
    pub pool_id: Pubkey,
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub config_version: u16,
    pub initial_token_reserve: u64,
    pub initial_virtual_wsol_reserve: u64,
}
impl CreateStandardLiquidityPoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_wsol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            payer,
            creator,
            mint,
            config_version,
            initial_token_reserve,
            initial_virtual_wsol_reserve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_token_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.initial_virtual_wsol_reserve,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateStandardLiquidityPoolEventEvent(pub CreateStandardLiquidityPoolEvent);
impl CreateStandardLiquidityPoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_STANDARD_LIQUIDITY_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateStandardLiquidityPoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_STANDARD_LIQUIDITY_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATING_LIQUIDITY_POOL_EVENT_EVENT_DISCM: [u8; 8] = [
    52, 128, 4, 166, 122, 176, 84, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatingLiquidityPoolEvent {
    pub id: Pubkey,
    pub base: Pubkey,
    pub quote: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
}
impl CreatingLiquidityPoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            id,
            base,
            quote,
            base_amount,
            quote_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatingLiquidityPoolEventEvent(pub CreatingLiquidityPoolEvent);
impl CreatingLiquidityPoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATING_LIQUIDITY_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatingLiquidityPoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATING_LIQUIDITY_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HIGH_REFLECTION_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    198, 217, 229, 205, 94, 214, 14, 177,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HighReflectionFeeEvent {
    pub liquidity_pool_id: Pubkey,
    pub reflection_fee_amount: u64,
    pub total_reflection_fees: u64,
}
impl HighReflectionFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reflection_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_reflection_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_pool_id,
            reflection_fee_amount,
            total_reflection_fees,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.liquidity_pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reflection_fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_reflection_fees, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HighReflectionFeeEventEvent(pub HighReflectionFeeEvent);
impl HighReflectionFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HIGH_REFLECTION_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = HighReflectionFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HIGH_REFLECTION_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MIGRATION_EVENT_EVENT_DISCM: [u8; 8] = [255, 202, 76, 147, 91, 231, 73, 22];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationEvent {
    pub market_cap: f64,
}
impl MigrationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_cap: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market_cap })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market_cap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationEventEvent(pub MigrationEvent);
impl MigrationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADE_EVENT_EVENT_DISCM: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeEvent {
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub total_creator_trading_fees: u64,
    pub total_fee_paid: u64,
    pub price_sol_usd: f64,
    pub base_in: u64,
    pub base_out: u64,
    pub quote_in: u64,
    pub quote_out: u64,
}
impl TradeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_creator_trading_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee_paid: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_sol_usd: f64 = crate::borsh_de_or_default(&mut reader)?;
        let base_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_reserve,
            quote_reserve,
            total_creator_trading_fees,
            total_fee_paid,
            price_sol_usd,
            base_in,
            base_out,
            quote_in,
            quote_out,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_creator_trading_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fee_paid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_sol_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_out, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeEventEvent(pub TradeEvent);
impl TradeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_DEFINED_EVENT_EVENT_DISCM: [u8; 8] = [
    33, 21, 108, 20, 241, 244, 167, 131,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserDefinedEvent {
    pub liquidity_pool_id: Pubkey,
    pub instruction_name: String,
    pub base64_data: String,
}
impl UserDefinedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let instruction_name: String = crate::borsh_de_or_default(&mut reader)?;
        let base64_data: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_pool_id,
            instruction_name,
            base64_data,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.liquidity_pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.instruction_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base64_data, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserDefinedEventEvent(pub UserDefinedEvent);
impl UserDefinedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_DEFINED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserDefinedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_DEFINED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
