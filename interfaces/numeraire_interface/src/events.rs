use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADD_LIQUIDITY_EVENT_DISCM: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidity {
    pub lp_token_mint_amount: u64,
    pub x_reserve_deltas: [u64; 10],
    pub y_reserve_deltas: [u64; 10],
    pub inv_l_deltas: [u64; 10],
    pub min_lp_token_mint_amount: u64,
    pub trader: Pubkey,
    pub pool: Pubkey,
}
impl AddLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_token_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let x_reserve_deltas: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let y_reserve_deltas: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let inv_l_deltas: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let min_lp_token_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_token_mint_amount,
            x_reserve_deltas,
            y_reserve_deltas,
            inv_l_deltas,
            min_lp_token_mint_amount,
            trader,
            pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_token_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_reserve_deltas, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_reserve_deltas, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inv_l_deltas, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_lp_token_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityEvent(pub AddLiquidity);
impl AddLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AddLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const QUOTE_EVENT_DISCM: [u8; 8] = [133, 244, 92, 134, 193, 24, 187, 158];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quote {
    pub amount: u64,
}
impl Quote {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteEvent(pub Quote);
impl QuoteEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Quote::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_LIQUIDITY_EVENT_DISCM: [u8; 8] = [116, 244, 97, 232, 103, 31, 152, 58];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidity {
    pub lp_token_redeem_amount: u64,
    pub x_reserve_deltas: [u64; 10],
    pub y_reserve_deltas: [u64; 10],
    pub inv_l_deltas: [u64; 10],
    pub min_amounts_out: [u64; 10],
    pub trader: Pubkey,
    pub pool: Pubkey,
}
impl RemoveLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_token_redeem_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let x_reserve_deltas: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let y_reserve_deltas: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let inv_l_deltas: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let min_amounts_out: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_token_redeem_amount,
            x_reserve_deltas,
            y_reserve_deltas,
            inv_l_deltas,
            min_amounts_out,
            trader,
            pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_token_redeem_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_reserve_deltas, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_reserve_deltas, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inv_l_deltas, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_amounts_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityEvent(pub RemoveLiquidity);
impl RemoveLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EXACT_IN_EVENT_DISCM: [u8; 8] = [147, 136, 213, 11, 150, 23, 141, 152];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactIn {
    pub amount_in: u64,
    pub amount_out: u64,
    pub min_amount_out: u64,
    pub trader: Pubkey,
    pub in_index: u8,
    pub out_index: u8,
    pub pool: Pubkey,
}
impl SwapExactIn {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            amount_out,
            min_amount_out,
            trader,
            in_index,
            out_index,
            pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactInEvent(pub SwapExactIn);
impl SwapExactInEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_IN_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapExactIn::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_IN_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EXACT_OUT_EVENT_DISCM: [u8; 8] = [71, 66, 127, 123, 231, 29, 227, 92];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOut {
    pub amount_in: u64,
    pub amount_out: u64,
    pub max_amount_in: u64,
    pub trader: Pubkey,
    pub in_index: u8,
    pub out_index: u8,
    pub pool: Pubkey,
}
impl SwapExactOut {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            amount_out,
            max_amount_in,
            trader,
            in_index,
            out_index,
            pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutEvent(pub SwapExactOut);
impl SwapExactOutEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapExactOut::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
