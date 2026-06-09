use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const DEPOSIT_LOG_EVENT_DISCM: [u8; 8] = [141, 186, 168, 252, 108, 141, 72, 94];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositLog {
    pub open_orders_account: Pubkey,
    pub signer: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
}
impl DepositLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let open_orders_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            open_orders_account,
            signer,
            base_amount,
            quote_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.open_orders_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositLogEvent(pub DepositLog);
impl DepositLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FILL_LOG_EVENT_DISCM: [u8; 8] = [150, 23, 41, 148, 152, 162, 215, 64];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillLog {
    pub market: Pubkey,
    pub taker_side: u8,
    pub maker_slot: u8,
    pub maker_out: bool,
    pub timestamp: u64,
    pub seq_num: u64,
    pub maker: Pubkey,
    pub maker_client_order_id: u64,
    pub maker_fee: u64,
    pub maker_timestamp: u64,
    pub taker: Pubkey,
    pub taker_client_order_id: u64,
    pub taker_fee_ceil: u64,
    pub price: i64,
    pub quantity: i64,
}
impl FillLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let taker_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let maker_slot: u8 = crate::borsh_de_or_default(&mut reader)?;
        let maker_out: bool = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker_client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let taker_client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee_ceil: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quantity: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            taker_side,
            maker_slot,
            maker_out,
            timestamp,
            seq_num,
            maker,
            maker_client_order_id,
            maker_fee,
            maker_timestamp,
            taker,
            taker_client_order_id,
            taker_fee_ceil,
            price,
            quantity,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seq_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_client_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_client_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_fee_ceil, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quantity, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillLogEvent(pub FillLog);
impl FillLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FillLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_META_DATA_LOG_EVENT_DISCM: [u8; 8] = [
    209, 87, 212, 236, 164, 58, 60, 117,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketMetaDataLog {
    pub market: Pubkey,
    pub name: String,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub base_lot_size: i64,
    pub quote_lot_size: i64,
}
impl MarketMetaDataLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            name,
            base_mint,
            quote_mint,
            base_decimals,
            quote_decimals,
            base_lot_size,
            quote_lot_size,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_lot_size, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketMetaDataLogEvent(pub MarketMetaDataLog);
impl MarketMetaDataLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_META_DATA_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarketMetaDataLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_META_DATA_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOTAL_ORDER_FILL_EVENT_EVENT_DISCM: [u8; 8] = [
    8, 235, 48, 58, 174, 76, 156, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TotalOrderFillEvent {
    pub side: u8,
    pub taker: Pubkey,
    pub total_quantity_paid: u64,
    pub total_quantity_received: u64,
    pub fees: u64,
}
impl TotalOrderFillEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let taker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_quantity_paid: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quantity_received: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            taker,
            total_quantity_paid,
            total_quantity_received,
            fees,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_quantity_paid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_quantity_received, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TotalOrderFillEventEvent(pub TotalOrderFillEvent);
impl TotalOrderFillEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOTAL_ORDER_FILL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TotalOrderFillEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOTAL_ORDER_FILL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_DELEGATE_LOG_EVENT_DISCM: [u8; 8] = [53, 130, 151, 92, 109, 57, 145, 112];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetDelegateLog {
    pub open_orders_account: Pubkey,
    pub delegate: Option<Pubkey>,
}
impl SetDelegateLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let open_orders_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            open_orders_account,
            delegate,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.open_orders_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetDelegateLogEvent(pub SetDelegateLog);
impl SetDelegateLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_DELEGATE_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetDelegateLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_DELEGATE_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_FUNDS_LOG_EVENT_DISCM: [u8; 8] = [10, 50, 240, 117, 237, 67, 230, 233];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleFundsLog {
    pub open_orders_account: Pubkey,
    pub base_native: u64,
    pub quote_native: u64,
    pub referrer_rebate: u64,
    pub referrer: Option<Pubkey>,
}
impl SettleFundsLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let open_orders_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_native: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_native: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_rebate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            open_orders_account,
            base_native,
            quote_native,
            referrer_rebate,
            referrer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.open_orders_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_native, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_native, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_rebate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleFundsLogEvent(pub SettleFundsLog);
impl SettleFundsLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_FUNDS_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettleFundsLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_FUNDS_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWEEP_FEES_LOG_EVENT_DISCM: [u8; 8] = [210, 242, 26, 77, 94, 48, 255, 61];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SweepFeesLog {
    pub market: Pubkey,
    pub amount: u64,
    pub receiver: Pubkey,
}
impl SweepFeesLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market, amount, receiver })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiver, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SweepFeesLogEvent(pub SweepFeesLog);
impl SweepFeesLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWEEP_FEES_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SweepFeesLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWEEP_FEES_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPEN_ORDERS_POSITION_LOG_EVENT_DISCM: [u8; 8] = [
    196, 249, 148, 33, 168, 228, 73, 6,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenOrdersPositionLog {
    pub owner: Pubkey,
    pub open_orders_account_num: u32,
    pub market: Pubkey,
    pub bids_base_lots: i64,
    pub bids_quote_lots: i64,
    pub asks_base_lots: i64,
    pub base_free_native: u64,
    pub quote_free_native: u64,
    pub locked_maker_fees: u64,
    pub referrer_rebates_available: u64,
    pub maker_volume: u128,
    pub taker_volume: u128,
}
impl OpenOrdersPositionLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_orders_account_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bids_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bids_quote_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let asks_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_free_native: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_free_native: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked_maker_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_rebates_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_volume: u128 = crate::borsh_de_or_default(&mut reader)?;
        let taker_volume: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            open_orders_account_num,
            market,
            bids_base_lots,
            bids_quote_lots,
            asks_base_lots,
            base_free_native,
            quote_free_native,
            locked_maker_fees,
            referrer_rebates_available,
            maker_volume,
            taker_volume,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_orders_account_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bids_base_lots, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bids_quote_lots, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asks_base_lots, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_free_native, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_free_native, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_maker_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_rebates_available, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_volume, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenOrdersPositionLogEvent(pub OpenOrdersPositionLog);
impl OpenOrdersPositionLogEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_ORDERS_POSITION_LOG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OpenOrdersPositionLog::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_ORDERS_POSITION_LOG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
