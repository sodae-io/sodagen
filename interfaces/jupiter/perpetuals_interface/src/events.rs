use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CREATE_POSITION_REQUEST_EVENT_EVENT_DISCM: [u8; 8] = [
    2, 238, 94, 53, 105, 211, 46, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePositionRequestEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub position_collateral_mint: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_request_key: Pubkey,
    pub position_request_mint: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub price_slippage: Option<u64>,
    pub jupiter_minimum_out: Option<u64>,
    pub pre_swap_amount: Option<u64>,
    pub request_change: u8,
    pub open_time: i64,
    pub referral: Option<Pubkey>,
}
impl CreatePositionRequestEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let jupiter_minimum_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let pre_swap_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let request_change: u8 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_side,
            position_mint,
            position_custody,
            position_collateral_mint,
            position_collateral_custody,
            position_request_key,
            position_request_mint,
            size_usd_delta,
            collateral_delta,
            price_slippage,
            jupiter_minimum_out,
            pre_swap_amount,
            request_change,
            open_time,
            referral,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.jupiter_minimum_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_swap_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePositionRequestEventEvent(pub CreatePositionRequestEvent);
impl CreatePositionRequestEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POSITION_REQUEST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatePositionRequestEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POSITION_REQUEST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_CREATE_TPSL_EVENT_EVENT_DISCM: [u8; 8] = [
    242, 54, 6, 95, 24, 141, 103, 198,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantCreateTpslEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_request_key: Pubkey,
    pub position_request_mint: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub entire_position: bool,
    pub open_time: i64,
}
impl InstantCreateTpslEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let entire_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_side,
            position_mint,
            position_custody,
            position_collateral_custody,
            position_request_key,
            position_request_mint,
            size_usd_delta,
            collateral_delta,
            entire_position,
            open_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.entire_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantCreateTpslEventEvent(pub InstantCreateTpslEvent);
impl InstantCreateTpslEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_CREATE_TPSL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantCreateTpslEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_CREATE_TPSL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_UPDATE_TPSL_EVENT_EVENT_DISCM: [u8; 8] = [
    177, 22, 47, 37, 120, 246, 17, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantUpdateTpslEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_request_key: Pubkey,
    pub position_request_mint: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub entire_position: bool,
    pub update_time: i64,
}
impl InstantUpdateTpslEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let entire_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let update_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_side,
            position_mint,
            position_custody,
            position_collateral_custody,
            position_request_key,
            position_request_mint,
            size_usd_delta,
            collateral_delta,
            entire_position,
            update_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.entire_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.update_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantUpdateTpslEventEvent(pub InstantUpdateTpslEvent);
impl InstantUpdateTpslEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_UPDATE_TPSL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantUpdateTpslEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_UPDATE_TPSL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLOSE_POSITION_REQUEST_EVENT_EVENT_DISCM: [u8; 8] = [
    21, 34, 92, 158, 224, 29, 180, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClosePositionRequestEvent {
    pub entire_position: Option<bool>,
    pub executed: bool,
    pub request_change: u8,
    pub request_type: u8,
    pub side: u8,
    pub position_request_key: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub open_time: i64,
}
impl ClosePositionRequestEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let entire_position: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let executed: bool = crate::borsh_de_or_default(&mut reader)?;
        let request_change: u8 = crate::borsh_de_or_default(&mut reader)?;
        let request_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            entire_position,
            executed,
            request_change,
            request_type,
            side,
            position_request_key,
            owner,
            mint,
            amount,
            open_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.entire_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionRequestEventEvent(pub ClosePositionRequestEvent);
impl ClosePositionRequestEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_REQUEST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClosePositionRequestEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_REQUEST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    245, 113, 85, 52, 214, 187, 153, 132,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionEvent {
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_size_usd: u64,
    pub position_mint: Pubkey,
    pub position_request_key: Pubkey,
    pub position_request_mint: Pubkey,
    pub position_request_change: u8,
    pub position_request_type: u8,
    pub position_request_collateral_delta: u64,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub price: u64,
    pub price_slippage: Option<u64>,
    pub fee_token: u64,
    pub fee_usd: u64,
    pub open_time: i64,
    pub referral: Option<Pubkey>,
    pub position_fee_usd: u64,
    pub funding_fee_usd: u64,
    pub price_impact_fee_usd: u64,
}
impl IncreasePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_change: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_request_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_request_collateral_delta: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let fee_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let position_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_key,
            position_side,
            position_custody,
            position_collateral_custody,
            position_size_usd,
            position_mint,
            position_request_key,
            position_request_mint,
            position_request_change,
            position_request_type,
            position_request_collateral_delta,
            owner,
            pool,
            size_usd_delta,
            collateral_usd_delta,
            collateral_token_delta,
            price,
            price_slippage,
            fee_token,
            fee_usd,
            open_time,
            referral,
            position_fee_usd,
            funding_fee_usd,
            price_impact_fee_usd,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_type, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_request_collateral_delta,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_token_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_impact_fee_usd, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionEventEvent(pub IncreasePositionEvent);
impl IncreasePositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreasePositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_POSITION_PRE_SWAP_EVENT_EVENT_DISCM: [u8; 8] = [
    237, 107, 9, 139, 22, 75, 4, 213,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionPreSwapEvent {
    pub position_request_key: Pubkey,
    pub transfer_amount: u64,
    pub collateral_custody_pre_swap_amount: u64,
}
impl IncreasePositionPreSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_custody_pre_swap_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            position_request_key,
            transfer_amount,
            collateral_custody_pre_swap_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collateral_custody_pre_swap_amount,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionPreSwapEventEvent(pub IncreasePositionPreSwapEvent);
impl IncreasePositionPreSwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_PRE_SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreasePositionPreSwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_PRE_SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DECREASE_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    64, 156, 43, 74, 109, 131, 16, 127,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionEvent {
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_size_usd: u64,
    pub position_mint: Pubkey,
    pub position_request_key: Pubkey,
    pub position_request_mint: Pubkey,
    pub position_request_change: u8,
    pub position_request_type: u8,
    pub has_profit: bool,
    pub pnl_delta: u64,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub size_usd_delta: u64,
    pub transfer_amount_usd: u64,
    pub transfer_token: Option<u64>,
    pub price: u64,
    pub price_slippage: Option<u64>,
    pub fee_usd: u64,
    pub open_time: i64,
    pub referral: Option<Pubkey>,
    pub position_fee_usd: u64,
    pub funding_fee_usd: u64,
    pub price_impact_fee_usd: u64,
    pub original_position_collateral_usd: u64,
    pub position_collateral_usd: u64,
    pub position_open_time: i64,
    pub position_price: u64,
}
impl DecreasePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_change: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_request_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_profit: bool = crate::borsh_de_or_default(&mut reader)?;
        let pnl_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_token: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let position_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let original_position_collateral_usd: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_collateral_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let position_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_key,
            position_side,
            position_custody,
            position_collateral_custody,
            position_size_usd,
            position_mint,
            position_request_key,
            position_request_mint,
            position_request_change,
            position_request_type,
            has_profit,
            pnl_delta,
            owner,
            pool,
            size_usd_delta,
            transfer_amount_usd,
            transfer_token,
            price,
            price_slippage,
            fee_usd,
            open_time,
            referral,
            position_fee_usd,
            funding_fee_usd,
            price_impact_fee_usd,
            original_position_collateral_usd,
            position_collateral_usd,
            position_open_time,
            position_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_profit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_impact_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.original_position_collateral_usd,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_collateral_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionEventEvent(pub DecreasePositionEvent);
impl DecreasePositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DecreasePositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DECREASE_POSITION_POST_SWAP_EVENT_EVENT_DISCM: [u8; 8] = [
    23, 210, 16, 233, 98, 245, 89, 82,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionPostSwapEvent {
    pub position_request_key: Pubkey,
    pub swap_amount: u64,
    pub jupiter_minimum_out: Option<u64>,
}
impl DecreasePositionPostSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let jupiter_minimum_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_request_key,
            swap_amount,
            jupiter_minimum_out,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.jupiter_minimum_out, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionPostSwapEventEvent(pub DecreasePositionPostSwapEvent);
impl DecreasePositionPostSwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_POST_SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DecreasePositionPostSwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_POST_SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDATE_FULL_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    128, 101, 71, 168, 128, 72, 86, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateFullPositionEvent {
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_collateral_mint: Pubkey,
    pub position_mint: Pubkey,
    pub position_size_usd: u64,
    pub has_profit: bool,
    pub pnl_delta: u64,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub transfer_amount_usd: u64,
    pub transfer_token: u64,
    pub price: u64,
    pub fee_usd: u64,
    pub liquidation_fee_usd: u64,
    pub open_time: i64,
    pub position_fee_usd: u64,
    pub funding_fee_usd: u64,
    pub price_impact_fee_usd: u64,
    pub original_position_collateral_usd: u64,
    pub position_collateral_usd: u64,
    pub position_open_time: i64,
    pub position_price: u64,
}
impl LiquidateFullPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let has_profit: bool = crate::borsh_de_or_default(&mut reader)?;
        let pnl_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let transfer_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let position_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let original_position_collateral_usd: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_collateral_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let position_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_key,
            position_side,
            position_custody,
            position_collateral_custody,
            position_collateral_mint,
            position_mint,
            position_size_usd,
            has_profit,
            pnl_delta,
            owner,
            pool,
            transfer_amount_usd,
            transfer_token,
            price,
            fee_usd,
            liquidation_fee_usd,
            open_time,
            position_fee_usd,
            funding_fee_usd,
            price_impact_fee_usd,
            original_position_collateral_usd,
            position_collateral_usd,
            position_open_time,
            position_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_profit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_impact_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.original_position_collateral_usd,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_collateral_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateFullPositionEventEvent(pub LiquidateFullPositionEvent);
impl LiquidateFullPositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_FULL_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidateFullPositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_FULL_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_SWAP_EVENT_EVENT_DISCM: [u8; 8] = [40, 107, 212, 26, 223, 136, 39, 220];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolSwapEvent {
    pub receiving_custody_key: Pubkey,
    pub dispensing_custody_key: Pubkey,
    pub pool_key: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_usd_amount: u64,
    pub amount_out_after_fees: u64,
    pub fee_bps: u64,
    pub owner_key: Pubkey,
    pub receiving_account_key: Pubkey,
}
impl PoolSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let receiving_custody_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dispensing_custody_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_usd_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_after_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiving_account_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            receiving_custody_key,
            dispensing_custody_key,
            pool_key,
            amount_in,
            amount_out,
            swap_usd_amount,
            amount_out_after_fees,
            fee_bps,
            owner_key,
            receiving_account_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.receiving_custody_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dispensing_custody_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_usd_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out_after_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiving_account_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolSwapEventEvent(pub PoolSwapEvent);
impl PoolSwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolSwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_SWAP_EXACT_OUT_EVENT_EVENT_DISCM: [u8; 8] = [
    121, 118, 11, 11, 198, 66, 142, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolSwapExactOutEvent {
    pub receiving_custody_key: Pubkey,
    pub dispensing_custody_key: Pubkey,
    pub pool_key: Pubkey,
    pub amount_in: u64,
    pub amount_in_after_fees: u64,
    pub amount_out: u64,
    pub swap_usd_amount: u64,
    pub fee_bps: u64,
    pub owner_key: Pubkey,
    pub receiving_account_key: Pubkey,
}
impl PoolSwapExactOutEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let receiving_custody_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dispensing_custody_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_after_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_usd_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiving_account_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            receiving_custody_key,
            dispensing_custody_key,
            pool_key,
            amount_in,
            amount_in_after_fees,
            amount_out,
            swap_usd_amount,
            fee_bps,
            owner_key,
            receiving_account_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.receiving_custody_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dispensing_custody_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in_after_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_usd_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiving_account_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolSwapExactOutEventEvent(pub PoolSwapExactOutEvent);
impl PoolSwapExactOutEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_SWAP_EXACT_OUT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolSwapExactOutEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_SWAP_EXACT_OUT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADD_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    27, 178, 153, 186, 47, 196, 140, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityEvent {
    pub custody_key: Pubkey,
    pub pool_key: Pubkey,
    pub token_amount_in: u64,
    pub pre_pool_amount_usd: u128,
    pub token_amount_usd: u64,
    pub fee_bps: u64,
    pub token_amount_after_fee: u64,
    pub mint_amount_usd: u64,
    pub lp_amount: u64,
    pub post_pool_amount_usd: u128,
}
impl AddLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let custody_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pre_pool_amount_usd: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_amount_after_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_pool_amount_usd: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            custody_key,
            pool_key,
            token_amount_in,
            pre_pool_amount_usd,
            token_amount_usd,
            fee_bps,
            token_amount_after_fee,
            mint_amount_usd,
            lp_amount,
            post_pool_amount_usd,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.custody_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_pool_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_amount_after_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_pool_amount_usd, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityEventEvent(pub AddLiquidityEvent);
impl AddLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AddLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    141, 199, 182, 123, 159, 94, 215, 102,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityEvent {
    pub custody_key: Pubkey,
    pub pool_key: Pubkey,
    pub lp_amount_in: u64,
    pub remove_amount_usd: u64,
    pub fee_bps: u64,
    pub remove_token_amount: u64,
    pub token_amount_after_fee: u64,
    pub post_pool_amount_usd: u128,
}
impl RemoveLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let custody_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remove_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remove_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_amount_after_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_pool_amount_usd: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            custody_key,
            pool_key,
            lp_amount_in,
            remove_amount_usd,
            fee_bps,
            remove_token_amount,
            token_amount_after_fee,
            post_pool_amount_usd,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.custody_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remove_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remove_token_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_amount_after_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_pool_amount_usd, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityEventEvent(pub RemoveLiquidityEvent);
impl RemoveLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_CREATE_LIMIT_ORDER_EVENT_EVENT_DISCM: [u8; 8] = [
    10, 163, 85, 115, 129, 224, 80, 192,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantCreateLimitOrderEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub position_collateral_mint: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_request_key: Pubkey,
    pub position_request_mint: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub open_time: i64,
}
impl InstantCreateLimitOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_request_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_request_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_side,
            position_mint,
            position_custody,
            position_collateral_mint,
            position_collateral_custody,
            position_request_key,
            position_request_mint,
            size_usd_delta,
            collateral_delta,
            open_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_collateral_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_request_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantCreateLimitOrderEventEvent(pub InstantCreateLimitOrderEvent);
impl InstantCreateLimitOrderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_CREATE_LIMIT_ORDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantCreateLimitOrderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_CREATE_LIMIT_ORDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_INCREASE_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    205, 236, 57, 4, 209, 106, 87, 69,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantIncreasePositionEvent {
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_size_usd: u64,
    pub position_mint: Pubkey,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub price: u64,
    pub price_slippage: u64,
    pub fee_token: u64,
    pub fee_usd: u64,
    pub open_time: i64,
    pub referral: Option<Pubkey>,
    pub position_fee_usd: u64,
    pub funding_fee_usd: u64,
    pub price_impact_fee_usd: u64,
}
impl InstantIncreasePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let position_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_key,
            position_side,
            position_custody,
            position_collateral_custody,
            position_size_usd,
            position_mint,
            owner,
            pool,
            size_usd_delta,
            collateral_usd_delta,
            collateral_token_delta,
            price,
            price_slippage,
            fee_token,
            fee_usd,
            open_time,
            referral,
            position_fee_usd,
            funding_fee_usd,
            price_impact_fee_usd,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_token_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_impact_fee_usd, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantIncreasePositionEventEvent(pub InstantIncreasePositionEvent);
impl InstantIncreasePositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_INCREASE_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantIncreasePositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_INCREASE_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_DECREASE_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    171, 173, 106, 25, 239, 190, 58, 59,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantDecreasePositionEvent {
    pub position_key: Pubkey,
    pub position_side: u8,
    pub position_custody: Pubkey,
    pub position_collateral_custody: Pubkey,
    pub position_size_usd: u64,
    pub position_mint: Pubkey,
    pub desired_mint: Pubkey,
    pub has_profit: bool,
    pub pnl_delta: u64,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub size_usd_delta: u64,
    pub transfer_amount_usd: u64,
    pub transfer_token: u64,
    pub price: u64,
    pub price_slippage: u64,
    pub fee_usd: u64,
    pub open_time: i64,
    pub referral: Option<Pubkey>,
    pub position_fee_usd: u64,
    pub funding_fee_usd: u64,
    pub original_position_collateral_usd: u64,
    pub position_collateral_usd: u64,
    pub price_impact_fee_usd: u64,
    pub position_open_time: i64,
    pub position_price: u64,
    pub position_request: Option<Pubkey>,
}
impl InstantDecreasePositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_collateral_custody: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let desired_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let has_profit: bool = crate::borsh_de_or_default(&mut reader)?;
        let pnl_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let position_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let original_position_collateral_usd: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let position_collateral_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let position_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_request: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_key,
            position_side,
            position_custody,
            position_collateral_custody,
            position_size_usd,
            position_mint,
            desired_mint,
            has_profit,
            pnl_delta,
            owner,
            pool,
            size_usd_delta,
            transfer_amount_usd,
            transfer_token,
            price,
            price_slippage,
            fee_usd,
            open_time,
            referral,
            position_fee_usd,
            funding_fee_usd,
            original_position_collateral_usd,
            position_collateral_usd,
            price_impact_fee_usd,
            position_open_time,
            position_price,
            position_request,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.position_collateral_custody,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.desired_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_profit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.original_position_collateral_usd,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.position_collateral_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_impact_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_request, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantDecreasePositionEventEvent(pub InstantDecreasePositionEvent);
impl InstantDecreasePositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_DECREASE_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantDecreasePositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_DECREASE_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_COLLATERAL_EVENT_EVENT_DISCM: [u8; 8] = [
    169, 14, 102, 148, 155, 137, 18, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositCollateralEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub deposit_amount: u64,
    pub user_token_account: Pubkey,
    pub time: i64,
}
impl DepositCollateralEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_mint,
            position_custody,
            deposit_amount,
            user_token_account,
            time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositCollateralEventEvent(pub DepositCollateralEvent);
impl DepositCollateralEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_COLLATERAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositCollateralEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_COLLATERAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_COLLATERAL_EVENT_EVENT_DISCM: [u8; 8] = [
    145, 38, 46, 87, 190, 149, 253, 191,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawCollateralEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub withdraw_amount: u64,
    pub user_token_account: Pubkey,
    pub custody: Pubkey,
    pub previous_collateral_amount: u64,
    pub collateral_amount: u64,
    pub collateral_amount_usd: u64,
    pub margin_usd: u64,
    pub time: i64,
}
impl WithdrawCollateralEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let previous_collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let margin_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_mint,
            position_custody,
            withdraw_amount,
            user_token_account,
            custody,
            previous_collateral_amount,
            collateral_amount,
            collateral_amount_usd,
            margin_usd,
            time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.previous_collateral_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawCollateralEventEvent(pub WithdrawCollateralEvent);
impl WithdrawCollateralEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_COLLATERAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawCollateralEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_COLLATERAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BORROW_FROM_CUSTODY_EVENT_EVENT_DISCM: [u8; 8] = [
    23, 121, 131, 68, 168, 70, 14, 76,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowFromCustodyEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub size_custody_token: u64,
    pub collateral_amount: u64,
    pub collateral_amount_usd: u64,
    pub margin_usd: u64,
    pub update_time: i64,
}
impl BorrowFromCustodyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_custody_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let margin_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let update_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_mint,
            position_custody,
            size_custody_token,
            collateral_amount,
            collateral_amount_usd,
            margin_usd,
            update_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_custody_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_amount_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.update_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowFromCustodyEventEvent(pub BorrowFromCustodyEvent);
impl BorrowFromCustodyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_FROM_CUSTODY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowFromCustodyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_FROM_CUSTODY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REPAY_TO_CUSTODY_EVENT_EVENT_DISCM: [u8; 8] = [
    232, 54, 116, 175, 186, 24, 249, 221,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RepayToCustodyEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_key: Pubkey,
    pub position_mint: Pubkey,
    pub position_custody: Pubkey,
    pub size_custody_token: u64,
    pub update_time: i64,
    pub interest: u128,
}
impl RepayToCustodyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let size_custody_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let update_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let interest: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            position_key,
            position_mint,
            position_custody,
            size_custody_token,
            update_time,
            interest,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_custody_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.update_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.interest, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayToCustodyEventEvent(pub RepayToCustodyEvent);
impl RepayToCustodyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_TO_CUSTODY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RepayToCustodyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_TO_CUSTODY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDATE_BORROW_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    11, 128, 252, 59, 49, 192, 56, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidateBorrowPositionEvent {
    pub position_key: Pubkey,
    pub position_custody: Pubkey,
    pub position_size_usd: u64,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub collateral_locked_in_usd: u64,
    pub collateral_locked_in_lp: u64,
    pub remaining_collateral_in_lp: u64,
    pub custody_token_price: u64,
    pub total_borrows_in_usd: u64,
    pub liquidation_fee_usd: u64,
    pub liquidation_time: i64,
}
impl LiquidateBorrowPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_locked_in_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_locked_in_lp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_collateral_in_lp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let custody_token_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_borrows_in_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_key,
            position_custody,
            position_size_usd,
            owner,
            pool,
            collateral_locked_in_usd,
            collateral_locked_in_lp,
            remaining_collateral_in_lp,
            custody_token_price,
            total_borrows_in_usd,
            liquidation_fee_usd,
            liquidation_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_locked_in_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_locked_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remaining_collateral_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody_token_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_borrows_in_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_fee_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateBorrowPositionEventEvent(pub LiquidateBorrowPositionEvent);
impl LiquidateBorrowPositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_BORROW_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidateBorrowPositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_BORROW_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEM_STAKE_EVENT_EVENT_DISCM: [u8; 8] = [
    0, 241, 84, 141, 139, 170, 218, 110,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStakeEvent {
    pub custody: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
    pub stake_rewards: u64,
    pub custody_total_staked_amount: u64,
    pub current_staked_amount: u64,
    pub total_staking_rewards: u64,
    pub redeem_time: i64,
}
impl RedeemStakeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_info: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let custody_total_staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_staking_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let redeem_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            custody,
            stake_account,
            stake_info,
            stake_rewards,
            custody_total_staked_amount,
            current_staked_amount,
            total_staking_rewards,
            redeem_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.custody_total_staked_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.current_staked_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_staking_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStakeEventEvent(pub RedeemStakeEvent);
impl RedeemStakeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemStakeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STAKE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_STAKE_EVENT_EVENT_DISCM: [u8; 8] = [
    47, 85, 239, 214, 207, 29, 151, 88,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawStakeEvent {
    pub custody: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
    pub stake_rewards: u64,
    pub custody_total_staked_amount: u64,
    pub total_staking_rewards: u64,
    pub withdraw_amount: u64,
    pub withdraw_time: i64,
}
impl WithdrawStakeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_info: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let custody_total_staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_staking_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            custody,
            stake_account,
            stake_info,
            stake_rewards,
            custody_total_staked_amount,
            total_staking_rewards,
            withdraw_amount,
            withdraw_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.custody_total_staked_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_staking_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStakeEventEvent(pub WithdrawStakeEvent);
impl WithdrawStakeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawStakeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STAKE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DELEGATE_STAKE_EVENT_EVENT_DISCM: [u8; 8] = [
    85, 135, 75, 222, 168, 133, 159, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DelegateStakeEvent {
    pub custody: Pubkey,
    pub stake_account: Pubkey,
    pub stake_info: Pubkey,
    pub custody_total_staked_amount: u64,
    pub stake_amount: u64,
    pub validator_vote_account: Pubkey,
    pub delegate_time: i64,
}
impl DelegateStakeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_info: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody_total_staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let validator_vote_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            custody,
            stake_account,
            stake_info,
            custody_total_staked_amount,
            stake_amount,
            validator_vote_account,
            delegate_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_info, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.custody_total_staked_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.stake_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_vote_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DelegateStakeEventEvent(pub DelegateStakeEvent);
impl DelegateStakeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELEGATE_STAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DelegateStakeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELEGATE_STAKE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    236, 118, 138, 90, 139, 173, 177, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFeesEvent {
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_mint: Pubkey,
    pub receiving_token_account: Pubkey,
    pub total_trade_swap_fees: u64,
    pub pool_trade_swap_fees: u64,
    pub protocol_trade_swap_fees: u64,
    pub total_borrow_lending_fees: u64,
    pub pool_borrow_lending_fees: u64,
    pub protocol_borrow_lending_fees: u64,
    pub pool_total_fees_usd: u64,
    pub apr_bps_before: u64,
    pub apr_bps_after: u64,
    pub apr_bps_updated_at: i64,
    pub pool_realized_fee_usd_before: u64,
    pub pool_realized_fee_usd_after: u64,
    pub curtime: i64,
}
impl WithdrawFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiving_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_trade_swap_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_trade_swap_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_trade_swap_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_borrow_lending_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_borrow_lending_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_borrow_lending_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_total_fees_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr_bps_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr_bps_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr_bps_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_realized_fee_usd_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_realized_fee_usd_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curtime: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            custody,
            custody_mint,
            receiving_token_account,
            total_trade_swap_fees,
            pool_trade_swap_fees,
            protocol_trade_swap_fees,
            total_borrow_lending_fees,
            pool_borrow_lending_fees,
            protocol_borrow_lending_fees,
            pool_total_fees_usd,
            apr_bps_before,
            apr_bps_after,
            apr_bps_updated_at,
            pool_realized_fee_usd_before,
            pool_realized_fee_usd_after,
            curtime,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiving_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_trade_swap_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_trade_swap_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_trade_swap_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_borrow_lending_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_borrow_lending_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_borrow_lending_fees,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pool_total_fees_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apr_bps_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apr_bps_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.apr_bps_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.pool_realized_fee_usd_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.pool_realized_fee_usd_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.curtime, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeesEventEvent(pub WithdrawFeesEvent);
impl WithdrawFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
