use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LOG_ARBITRAGE_EVENT_DISCM: [u8; 8] = [105, 165, 52, 9, 218, 211, 46, 13];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogArbitrage {
    pub dex_id: u16,
    pub routing: i128,
    pub amt_out: u64,
}
impl LogArbitrage {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let routing: i128 = crate::borsh_de_or_default(&mut reader)?;
        let amt_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id, routing, amt_out })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.routing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amt_out, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogArbitrageEvent(pub LogArbitrage);
impl LogArbitrageEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_ARBITRAGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogArbitrage::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_ARBITRAGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_BORROW_DEBT_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    70, 124, 172, 119, 252, 91, 62, 4,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogBorrowDebtLiquidity {
    pub dex_id: u16,
    pub amount_0: u64,
    pub amount_1: u64,
    pub shares: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogBorrowDebtLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            amount_0,
            amount_1,
            shares,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogBorrowDebtLiquidityEvent(pub LogBorrowDebtLiquidity);
impl LogBorrowDebtLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_BORROW_DEBT_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogBorrowDebtLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_BORROW_DEBT_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_BORROW_PERFECT_DEBT_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    164, 250, 16, 192, 152, 3, 238, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogBorrowPerfectDebtLiquidity {
    pub dex_id: u16,
    pub shares: u64,
    pub token_0_amt: u64,
    pub token_1_amt: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogBorrowPerfectDebtLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            shares,
            token_0_amt,
            token_1_amt,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogBorrowPerfectDebtLiquidityEvent(pub LogBorrowPerfectDebtLiquidity);
impl LogBorrowPerfectDebtLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_BORROW_PERFECT_DEBT_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogBorrowPerfectDebtLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_BORROW_PERFECT_DEBT_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_DEPOSIT_COL_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    162, 105, 100, 76, 89, 95, 69, 189,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogDepositColLiquidity {
    pub dex_id: u16,
    pub amount_0: u64,
    pub amount_1: u64,
    pub shares: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogDepositColLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            amount_0,
            amount_1,
            shares,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogDepositColLiquidityEvent(pub LogDepositColLiquidity);
impl LogDepositColLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_DEPOSIT_COL_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogDepositColLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_DEPOSIT_COL_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_DEPOSIT_PERFECT_COL_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    33, 91, 169, 231, 163, 83, 37, 254,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogDepositPerfectColLiquidity {
    pub dex_id: u16,
    pub shares: u64,
    pub token_0_amt: u64,
    pub token_1_amt: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogDepositPerfectColLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            shares,
            token_0_amt,
            token_1_amt,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogDepositPerfectColLiquidityEvent(pub LogDepositPerfectColLiquidity);
impl LogDepositPerfectColLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_DEPOSIT_PERFECT_COL_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogDepositPerfectColLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_DEPOSIT_PERFECT_COL_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_INIT_DEX_EVENT_DISCM: [u8; 8] = [170, 65, 241, 125, 34, 194, 79, 132];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogInitDex {
    pub dex_id: u16,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub smart_col: bool,
    pub smart_debt: bool,
    pub fee: u32,
    pub revenue_cut: u32,
    pub center_price: u64,
}
impl LogInitDex {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let smart_col: bool = crate::borsh_de_or_default(&mut reader)?;
        let smart_debt: bool = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let revenue_cut: u32 = crate::borsh_de_or_default(&mut reader)?;
        let center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            token_0,
            token_1,
            smart_col,
            smart_debt,
            fee,
            revenue_cut,
            center_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.smart_col, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.smart_debt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_cut, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.center_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogInitDexEvent(pub LogInitDex);
impl LogInitDexEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_INIT_DEX_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogInitDex::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_INIT_DEX_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_INIT_DEX_PRICE_PARAMS_EVENT_DISCM: [u8; 8] = [
    154, 98, 101, 72, 8, 203, 37, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogInitDexPriceParams {
    pub dex_id: u16,
    pub upper_percent: u32,
    pub lower_percent: u32,
    pub upper_shift_threshold: u32,
    pub lower_shift_threshold: u32,
    pub threshold_shift_time: u32,
    pub max_center_price: u64,
    pub min_center_price: u64,
}
impl LogInitDexPriceParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let upper_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_shift_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_shift_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            upper_percent,
            lower_percent,
            upper_shift_threshold,
            lower_shift_threshold,
            threshold_shift_time,
            max_center_price,
            min_center_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_shift_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_shift_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.threshold_shift_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_center_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_center_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogInitDexPriceParamsEvent(pub LogInitDexPriceParams);
impl LogInitDexPriceParamsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_INIT_DEX_PRICE_PARAMS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogInitDexPriceParams::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_INIT_DEX_PRICE_PARAMS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_PAUSE_DEX_EVENT_DISCM: [u8; 8] = [107, 202, 204, 255, 100, 73, 92, 117];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogPauseDex {
    pub dex_id: u16,
}
impl LogPauseDex {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogPauseDexEvent(pub LogPauseDex);
impl LogPauseDexEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_PAUSE_DEX_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogPauseDex::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_PAUSE_DEX_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_PAUSE_SWAP_AND_ARBITRAGE_EVENT_DISCM: [u8; 8] = [
    103, 184, 228, 52, 0, 229, 4, 154,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogPauseSwapAndArbitrage {
    pub dex_id: u16,
}
impl LogPauseSwapAndArbitrage {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogPauseSwapAndArbitrageEvent(pub LogPauseSwapAndArbitrage);
impl LogPauseSwapAndArbitrageEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_PAUSE_SWAP_AND_ARBITRAGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogPauseSwapAndArbitrage::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_PAUSE_SWAP_AND_ARBITRAGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_PAUSE_USER_EVENT_DISCM: [u8; 8] = [100, 17, 114, 224, 180, 30, 52, 170];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogPauseUser {
    pub dex_id: u16,
    pub protocol: Pubkey,
    pub pause_supply: bool,
    pub pause_borrow: bool,
}
impl LogPauseUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pause_supply: bool = crate::borsh_de_or_default(&mut reader)?;
        let pause_borrow: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            protocol,
            pause_supply,
            pause_borrow,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pause_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pause_borrow, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogPauseUserEvent(pub LogPauseUser);
impl LogPauseUserEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_PAUSE_USER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogPauseUser::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_PAUSE_USER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_PAYBACK_DEBT_IN_ONE_TOKEN_EVENT_DISCM: [u8; 8] = [
    123, 31, 108, 14, 12, 201, 20, 83,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogPaybackDebtInOneToken {
    pub dex_id: u16,
    pub shares: u64,
    pub token_0_amt: u64,
    pub token_1_amt: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogPaybackDebtInOneToken {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            shares,
            token_0_amt,
            token_1_amt,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogPaybackDebtInOneTokenEvent(pub LogPaybackDebtInOneToken);
impl LogPaybackDebtInOneTokenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_PAYBACK_DEBT_IN_ONE_TOKEN_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogPaybackDebtInOneToken::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_PAYBACK_DEBT_IN_ONE_TOKEN_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_PAYBACK_DEBT_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    230, 190, 245, 114, 113, 92, 173, 27,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogPaybackDebtLiquidity {
    pub dex_id: u16,
    pub amount_0: u64,
    pub amount_1: u64,
    pub shares: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogPaybackDebtLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            amount_0,
            amount_1,
            shares,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogPaybackDebtLiquidityEvent(pub LogPaybackDebtLiquidity);
impl LogPaybackDebtLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_PAYBACK_DEBT_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogPaybackDebtLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_PAYBACK_DEBT_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_PAYBACK_PERFECT_DEBT_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    75, 233, 232, 38, 153, 31, 239, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogPaybackPerfectDebtLiquidity {
    pub dex_id: u16,
    pub shares: u64,
    pub token_0_amt: u64,
    pub token_1_amt: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogPaybackPerfectDebtLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            shares,
            token_0_amt,
            token_1_amt,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogPaybackPerfectDebtLiquidityEvent(pub LogPaybackPerfectDebtLiquidity);
impl LogPaybackPerfectDebtLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_PAYBACK_PERFECT_DEBT_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogPaybackPerfectDebtLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_PAYBACK_PERFECT_DEBT_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_SWAP_EVENT_DISCM: [u8; 8] = [202, 242, 228, 28, 37, 194, 52, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogSwap {
    pub dex_id: u16,
    pub swap_0_to_1: bool,
    pub amount_in: u64,
    pub amount_out: u64,
    pub to: Pubkey,
}
impl LogSwap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let swap_0_to_1: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let to: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            swap_0_to_1,
            amount_in,
            amount_out,
            to,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_0_to_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.to, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogSwapEvent(pub LogSwap);
impl LogSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_SWAP_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogSwap::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_SWAP_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_TURN_ON_SMART_COL_EVENT_DISCM: [u8; 8] = [
    108, 254, 255, 147, 80, 55, 98, 86,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogTurnOnSmartCol {
    pub dex_id: u16,
    pub token_0_amt: u64,
}
impl LogTurnOnSmartCol {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id, token_0_amt })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogTurnOnSmartColEvent(pub LogTurnOnSmartCol);
impl LogTurnOnSmartColEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_TURN_ON_SMART_COL_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogTurnOnSmartCol::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_TURN_ON_SMART_COL_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_TURN_ON_SMART_DEBT_EVENT_DISCM: [u8; 8] = [
    23, 36, 134, 104, 91, 138, 126, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogTurnOnSmartDebt {
    pub dex_id: u16,
    pub token_0_amt: u64,
}
impl LogTurnOnSmartDebt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id, token_0_amt })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogTurnOnSmartDebtEvent(pub LogTurnOnSmartDebt);
impl LogTurnOnSmartDebtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_TURN_ON_SMART_DEBT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogTurnOnSmartDebt::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_TURN_ON_SMART_DEBT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UNPAUSE_DEX_EVENT_DISCM: [u8; 8] = [6, 190, 255, 207, 165, 71, 170, 212];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUnpauseDex {
    pub dex_id: u16,
}
impl LogUnpauseDex {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUnpauseDexEvent(pub LogUnpauseDex);
impl LogUnpauseDexEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UNPAUSE_DEX_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUnpauseDex::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UNPAUSE_DEX_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UNPAUSE_SWAP_AND_ARBITRAGE_EVENT_DISCM: [u8; 8] = [
    229, 120, 123, 188, 146, 222, 159, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUnpauseSwapAndArbitrage {
    pub dex_id: u16,
}
impl LogUnpauseSwapAndArbitrage {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUnpauseSwapAndArbitrageEvent(pub LogUnpauseSwapAndArbitrage);
impl LogUnpauseSwapAndArbitrageEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UNPAUSE_SWAP_AND_ARBITRAGE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUnpauseSwapAndArbitrage::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UNPAUSE_SWAP_AND_ARBITRAGE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UNPAUSE_USER_EVENT_DISCM: [u8; 8] = [170, 91, 132, 96, 179, 77, 168, 26];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUnpauseUser {
    pub dex_id: u16,
    pub protocol: Pubkey,
    pub unpause_supply: bool,
    pub unpause_borrow: bool,
}
impl LogUnpauseUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let unpause_supply: bool = crate::borsh_de_or_default(&mut reader)?;
        let unpause_borrow: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            protocol,
            unpause_supply,
            unpause_borrow,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unpause_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unpause_borrow, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUnpauseUserEvent(pub LogUnpauseUser);
impl LogUnpauseUserEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UNPAUSE_USER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUnpauseUser::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UNPAUSE_USER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_AUTHORITY_EVENT_DISCM: [u8; 8] = [
    150, 152, 157, 143, 6, 135, 193, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateAuthority {
    pub new_authority: Pubkey,
}
impl LogUpdateAuthority {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { new_authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateAuthorityEvent(pub LogUpdateAuthority);
impl LogUpdateAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_AUTHORITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateAuthority::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_AUTHORITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_AUTHS_EVENT_DISCM: [u8; 8] = [88, 80, 109, 48, 111, 203, 76, 251];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateAuths {
    pub auth_status: Vec<AddressBool>,
}
impl LogUpdateAuths {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let auth_status: Vec<AddressBool> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { auth_status })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.auth_status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateAuthsEvent(pub LogUpdateAuths);
impl LogUpdateAuthsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_AUTHS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateAuths::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_AUTHS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_CENTER_PRICE_ADDRESS_EVENT_DISCM: [u8; 8] = [
    96, 24, 207, 2, 38, 203, 43, 145,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateCenterPriceAddress {
    pub dex_id: u16,
    pub center_price_address: Pubkey,
    pub percent: u32,
    pub time: u32,
}
impl LogUpdateCenterPriceAddress {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let center_price_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let time: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            center_price_address,
            percent,
            time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.center_price_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateCenterPriceAddressEvent(pub LogUpdateCenterPriceAddress);
impl LogUpdateCenterPriceAddressEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_CENTER_PRICE_ADDRESS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateCenterPriceAddress::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_CENTER_PRICE_ADDRESS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_CENTER_PRICE_LIMITS_EVENT_DISCM: [u8; 8] = [
    111, 228, 61, 31, 234, 76, 5, 17,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateCenterPriceLimits {
    pub dex_id: u16,
    pub max_center_price: u64,
    pub min_center_price: u64,
}
impl LogUpdateCenterPriceLimits {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            max_center_price,
            min_center_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_center_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_center_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateCenterPriceLimitsEvent(pub LogUpdateCenterPriceLimits);
impl LogUpdateCenterPriceLimitsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_CENTER_PRICE_LIMITS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateCenterPriceLimits::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_CENTER_PRICE_LIMITS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_FEE_AND_REVENUE_CUT_EVENT_DISCM: [u8; 8] = [
    187, 9, 16, 229, 8, 231, 7, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateFeeAndRevenueCut {
    pub dex_id: u16,
    pub fee: u32,
    pub revenue_cut: u32,
}
impl LogUpdateFeeAndRevenueCut {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let revenue_cut: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id, fee, revenue_cut })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_cut, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateFeeAndRevenueCutEvent(pub LogUpdateFeeAndRevenueCut);
impl LogUpdateFeeAndRevenueCutEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_FEE_AND_REVENUE_CUT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateFeeAndRevenueCut::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_FEE_AND_REVENUE_CUT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_MAX_BORROW_SHARES_EVENT_DISCM: [u8; 8] = [
    22, 124, 242, 112, 255, 39, 100, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateMaxBorrowShares {
    pub dex_id: u16,
    pub max_borrow_shares: u64,
}
impl LogUpdateMaxBorrowShares {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id, max_borrow_shares })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_borrow_shares, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateMaxBorrowSharesEvent(pub LogUpdateMaxBorrowShares);
impl LogUpdateMaxBorrowSharesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_MAX_BORROW_SHARES_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateMaxBorrowShares::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_MAX_BORROW_SHARES_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_MAX_SUPPLY_SHARES_EVENT_DISCM: [u8; 8] = [
    209, 150, 112, 193, 243, 63, 233, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateMaxSupplyShares {
    pub dex_id: u16,
    pub max_supply_shares: u64,
}
impl LogUpdateMaxSupplyShares {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dex_id, max_supply_shares })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supply_shares, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateMaxSupplySharesEvent(pub LogUpdateMaxSupplyShares);
impl LogUpdateMaxSupplySharesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_MAX_SUPPLY_SHARES_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateMaxSupplyShares::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_MAX_SUPPLY_SHARES_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_RANGE_PERCENTS_EVENT_DISCM: [u8; 8] = [
    149, 47, 161, 6, 129, 240, 48, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateRangePercents {
    pub dex_id: u16,
    pub upper_percent: u32,
    pub lower_percent: u32,
    pub shift_time: u32,
}
impl LogUpdateRangePercents {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let upper_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            upper_percent,
            lower_percent,
            shift_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shift_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateRangePercentsEvent(pub LogUpdateRangePercents);
impl LogUpdateRangePercentsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_RANGE_PERCENTS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateRangePercents::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_RANGE_PERCENTS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_THRESHOLD_PERCENT_EVENT_DISCM: [u8; 8] = [
    44, 46, 70, 115, 25, 38, 92, 99,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateThresholdPercent {
    pub dex_id: u16,
    pub upper_threshold_percent: u32,
    pub lower_threshold_percent: u32,
    pub threshold_shift_time: u32,
    pub shift_time: u32,
}
impl LogUpdateThresholdPercent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let upper_threshold_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_threshold_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        let shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            upper_threshold_percent,
            lower_threshold_percent,
            threshold_shift_time,
            shift_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_threshold_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_threshold_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.threshold_shift_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shift_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateThresholdPercentEvent(pub LogUpdateThresholdPercent);
impl LogUpdateThresholdPercentEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_THRESHOLD_PERCENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateThresholdPercent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_THRESHOLD_PERCENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_USER_BORROW_CONFIG_EVENT_DISCM: [u8; 8] = [
    70, 142, 184, 48, 44, 158, 166, 3,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateUserBorrowConfig {
    pub dex_id: u16,
    pub protocol: Pubkey,
    pub expand_percent: u16,
    pub expand_duration: u32,
    pub base_debt_ceiling: u64,
    pub max_debt_ceiling: u64,
}
impl LogUpdateUserBorrowConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let expand_percent: u16 = crate::borsh_de_or_default(&mut reader)?;
        let expand_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let base_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            protocol,
            expand_percent,
            expand_duration,
            base_debt_ceiling,
            max_debt_ceiling,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_debt_ceiling, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateUserBorrowConfigEvent(pub LogUpdateUserBorrowConfig);
impl LogUpdateUserBorrowConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_USER_BORROW_CONFIG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateUserBorrowConfig::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_USER_BORROW_CONFIG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_USER_SUPPLY_CONFIG_EVENT_DISCM: [u8; 8] = [
    86, 139, 35, 235, 30, 42, 192, 245,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateUserSupplyConfig {
    pub dex_id: u16,
    pub protocol: Pubkey,
    pub expand_percent: u16,
    pub expand_duration: u64,
    pub base_withdrawal_limit: u64,
}
impl LogUpdateUserSupplyConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let expand_percent: u16 = crate::borsh_de_or_default(&mut reader)?;
        let expand_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_withdrawal_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            protocol,
            expand_percent,
            expand_duration,
            base_withdrawal_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_withdrawal_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateUserSupplyConfigEvent(pub LogUpdateUserSupplyConfig);
impl LogUpdateUserSupplyConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_USER_SUPPLY_CONFIG_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateUserSupplyConfig::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_USER_SUPPLY_CONFIG_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_USER_WITHDRAWAL_LIMIT_EVENT_DISCM: [u8; 8] = [
    114, 131, 152, 189, 120, 253, 88, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateUserWithdrawalLimit {
    pub dex_id: u16,
    pub protocol: Pubkey,
    pub new_limit: u64,
}
impl LogUpdateUserWithdrawalLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            protocol,
            new_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateUserWithdrawalLimitEvent(pub LogUpdateUserWithdrawalLimit);
impl LogUpdateUserWithdrawalLimitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_USER_WITHDRAWAL_LIMIT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateUserWithdrawalLimit::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_USER_WITHDRAWAL_LIMIT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_UTILIZATION_LIMIT_EVENT_DISCM: [u8; 8] = [
    153, 239, 227, 172, 250, 247, 155, 69,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateUtilizationLimit {
    pub dex_id: u16,
    pub token_0_utilization_limit: u16,
    pub token_1_utilization_limit: u16,
}
impl LogUpdateUtilizationLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_utilization_limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_utilization_limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            token_0_utilization_limit,
            token_1_utilization_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_utilization_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_utilization_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateUtilizationLimitEvent(pub LogUpdateUtilizationLimit);
impl LogUpdateUtilizationLimitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_UTILIZATION_LIMIT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateUtilizationLimit::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_UTILIZATION_LIMIT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_WITHDRAW_COL_IN_ONE_TOKEN_EVENT_DISCM: [u8; 8] = [
    86, 6, 224, 183, 211, 209, 199, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogWithdrawColInOneToken {
    pub dex_id: u16,
    pub shares: u64,
    pub token_0_amt: u64,
    pub token_1_amt: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogWithdrawColInOneToken {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            shares,
            token_0_amt,
            token_1_amt,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogWithdrawColInOneTokenEvent(pub LogWithdrawColInOneToken);
impl LogWithdrawColInOneTokenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_WITHDRAW_COL_IN_ONE_TOKEN_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogWithdrawColInOneToken::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_WITHDRAW_COL_IN_ONE_TOKEN_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_WITHDRAW_COL_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    29, 6, 184, 148, 156, 65, 164, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogWithdrawColLiquidity {
    pub dex_id: u16,
    pub amount_0: u64,
    pub amount_1: u64,
    pub shares: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogWithdrawColLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            amount_0,
            amount_1,
            shares,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogWithdrawColLiquidityEvent(pub LogWithdrawColLiquidity);
impl LogWithdrawColLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_WITHDRAW_COL_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogWithdrawColLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_WITHDRAW_COL_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_WITHDRAW_PERFECT_COL_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    199, 40, 242, 90, 55, 165, 41, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogWithdrawPerfectColLiquidity {
    pub dex_id: u16,
    pub shares: u64,
    pub token_0_amt: u64,
    pub token_1_amt: u64,
    pub user: Pubkey,
    pub protocol: Pubkey,
}
impl LogWithdrawPerfectColLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            shares,
            token_0_amt,
            token_1_amt,
            user,
            protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogWithdrawPerfectColLiquidityEvent(pub LogWithdrawPerfectColLiquidity);
impl LogWithdrawPerfectColLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_WITHDRAW_PERFECT_COL_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogWithdrawPerfectColLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_WITHDRAW_PERFECT_COL_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
