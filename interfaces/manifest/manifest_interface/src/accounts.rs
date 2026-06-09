use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
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
pub struct CreateMarketLog {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
}
impl CreateMarketLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            creator,
            base_mint,
            quote_mint,
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
pub struct ClaimSeatLog {
    pub market: Pubkey,
    pub trader: Pubkey,
}
impl ClaimSeatLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market, trader })
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
pub struct DepositLog {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub mint: Pubkey,
    pub amount_atoms: u64,
}
impl DepositLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            trader,
            mint,
            amount_atoms,
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
pub struct WithdrawLog {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub mint: Pubkey,
    pub amount_atoms: u64,
}
impl WithdrawLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            trader,
            mint,
            amount_atoms,
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
pub struct FillLog {
    pub market: Pubkey,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub price: QuoteAtomsPerBaseAtom,
    pub base_atoms: BaseAtoms,
    pub quote_atoms: QuoteAtoms,
    pub maker_sequence_number: u64,
    pub taker_sequence_number: u64,
    pub taker_is_buy: bool,
    pub is_maker_global: bool,
    pub padding: [u8; 14],
}
impl FillLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let taker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <QuoteAtomsPerBaseAtom>::deserialize(&mut reader)?
        };
        let base_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <BaseAtoms>::deserialize(&mut reader)?
        };
        let quote_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <QuoteAtoms>::deserialize(&mut reader)?
        };
        let maker_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_is_buy: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_maker_global: bool = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 14] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            maker,
            taker,
            base_mint,
            quote_mint,
            price,
            base_atoms,
            quote_atoms,
            maker_sequence_number,
            taker_sequence_number,
            taker_is_buy,
            is_maker_global,
            padding,
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
pub struct PlaceOrderLog {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub price: QuoteAtomsPerBaseAtom,
    pub base_atoms: BaseAtoms,
    pub order_sequence_number: u64,
    pub order_index: u32,
    pub last_valid_slot: u32,
    pub order_type: OrderType,
    pub is_bid: bool,
    pub padding: [u8; 6],
}
impl PlaceOrderLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <QuoteAtomsPerBaseAtom>::deserialize(&mut reader)?
        };
        let base_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <BaseAtoms>::deserialize(&mut reader)?
        };
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_slot: u32 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let is_bid: bool = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            trader,
            price,
            base_atoms,
            order_sequence_number,
            order_index,
            last_valid_slot,
            order_type,
            is_bid,
            padding,
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
pub struct CancelOrderLog {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub order_sequence_number: u64,
}
impl CancelOrderLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            trader,
            order_sequence_number,
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
pub struct GlobalCreateLog {
    pub global: Pubkey,
    pub creator: Pubkey,
}
impl GlobalCreateLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { global, creator })
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
pub struct GlobalAddTraderLog {
    pub global: Pubkey,
    pub trader: Pubkey,
}
impl GlobalAddTraderLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { global, trader })
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
pub struct GlobalClaimSeatLog {
    pub global: Pubkey,
    pub market: Pubkey,
    pub trader: Pubkey,
}
impl GlobalClaimSeatLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { global, market, trader })
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
pub struct GlobalDepositLog {
    pub global: Pubkey,
    pub trader: Pubkey,
    pub global_atoms: GlobalAtoms,
}
impl GlobalDepositLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalAtoms>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            global,
            trader,
            global_atoms,
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
pub struct GlobalWithdrawLog {
    pub global: Pubkey,
    pub trader: Pubkey,
    pub global_atoms: GlobalAtoms,
}
impl GlobalWithdrawLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalAtoms>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            global,
            trader,
            global_atoms,
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
pub struct GlobalEvictLog {
    pub evictor: Pubkey,
    pub evictee: Pubkey,
    pub evictor_atoms: GlobalAtoms,
    pub evictee_atoms: GlobalAtoms,
}
impl GlobalEvictLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let evictor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let evictee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let evictor_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalAtoms>::deserialize(&mut reader)?
        };
        let evictee_atoms = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalAtoms>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            evictor,
            evictee,
            evictor_atoms,
            evictee_atoms,
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
pub struct GlobalCleanupLog {
    pub cleaner: Pubkey,
    pub maker: Pubkey,
    pub amount_desired: GlobalAtoms,
    pub amount_deposited: GlobalAtoms,
}
impl GlobalCleanupLog {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cleaner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_desired = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalAtoms>::deserialize(&mut reader)?
        };
        let amount_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <GlobalAtoms>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            cleaner,
            maker,
            amount_desired,
            amount_deposited,
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
pub struct QuoteAtoms {
    pub inner: u64,
}
impl QuoteAtoms {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
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
pub struct BaseAtoms {
    pub inner: u64,
}
impl BaseAtoms {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
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
pub struct GlobalAtoms {
    pub inner: u64,
}
impl GlobalAtoms {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
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
pub struct QuoteAtomsPerBaseAtom {
    pub inner: u128,
}
impl QuoteAtomsPerBaseAtom {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
    }
}
