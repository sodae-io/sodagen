use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FEE_ACCOUNT_DISCM: [u8; 8] = [24, 55, 150, 250, 168, 27, 101, 178];
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
pub struct Fee {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}
impl Fee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let maker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_stable_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            maker_fee,
            maker_stable_fee,
            taker_fee,
            taker_stable_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_stable_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_stable_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeAccount(pub Fee);
impl FeeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Fee::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORDER_ACCOUNT_DISCM: [u8; 8] = [134, 173, 223, 185, 77, 86, 28, 51];
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
pub struct Order {
    pub maker: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub waiting: bool,
    pub ori_making_amount: u64,
    pub ori_taking_amount: u64,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub maker_input_account: Pubkey,
    pub maker_output_account: Pubkey,
    pub reserve: Pubkey,
    pub borrow_making_amount: u64,
    pub expired_at: Option<i64>,
    pub base: Pubkey,
    pub referral: Option<Pubkey>,
}
impl Order {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let waiting: bool = crate::borsh_de_or_default(&mut reader)?;
        let ori_making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ori_taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taking_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_input_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker_output_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let borrow_making_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expired_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            maker,
            input_mint,
            output_mint,
            waiting,
            ori_making_amount,
            ori_taking_amount,
            making_amount,
            taking_amount,
            maker_input_account,
            maker_output_account,
            reserve,
            borrow_making_amount,
            expired_at,
            base,
            referral,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.waiting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ori_making_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ori_taking_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.making_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taking_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_input_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_output_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_making_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expired_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderAccount(pub Order);
impl OrderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Order::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORDER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
