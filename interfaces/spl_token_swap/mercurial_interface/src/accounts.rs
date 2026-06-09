use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FEES_ACCOUNT_DISCM: [u8; 8] = [151, 157, 50, 115, 130, 72, 179, 36];
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
pub struct Fees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}
impl Fees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_trade_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_withdraw_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_withdraw_fee_denominator: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let host_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let host_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trade_fee_numerator,
            trade_fee_denominator,
            owner_trade_fee_numerator,
            owner_trade_fee_denominator,
            owner_withdraw_fee_numerator,
            owner_withdraw_fee_denominator,
            host_fee_numerator,
            host_fee_denominator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.trade_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_denominator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner_trade_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.owner_trade_fee_denominator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.owner_withdraw_fee_numerator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.owner_withdraw_fee_denominator,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.host_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_fee_denominator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeesAccount(pub Fees);
impl FeesAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEES_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Fees::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEES_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_CURVE_ACCOUNT_DISCM: [u8; 8] = [140, 189, 243, 0, 127, 104, 149, 41];
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
pub struct SwapCurve {
    pub curve_type: CurveType,
    pub calculator: [u8; 32],
}
impl SwapCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let curve_type: CurveType = crate::borsh_de_or_default(&mut reader)?;
        let calculator: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { curve_type, calculator })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.curve_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.calculator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapCurveAccount(pub SwapCurve);
impl SwapCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_V1_ACCOUNT_DISCM: [u8; 8] = [244, 254, 184, 97, 8, 187, 243, 179];
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
pub struct SwapV1 {
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub pool_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub pool_fee_account: Pubkey,
    pub fees: Fees,
    pub swap_curve: SwapCurve,
}
impl SwapV1 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_fee_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let swap_curve = if reader.is_empty() {
            Default::default()
        } else {
            <SwapCurve>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            is_initialized,
            bump_seed,
            token_program_id,
            token_a,
            token_b,
            pool_mint,
            token_a_mint,
            token_b_mint,
            pool_fee_account,
            fees,
            swap_curve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_initialized, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_fee_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_curve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapV1Account(pub SwapV1);
impl SwapV1Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_V1_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapV1::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_V1_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFSET_CURVE_ACCOUNT_DISCM: [u8; 8] = [66, 153, 235, 11, 88, 204, 171, 242];
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
pub struct OffsetCurve {
    pub token_b_offset: u64,
}
impl OffsetCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_b_offset: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_b_offset })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_b_offset, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCurveAccount(pub OffsetCurve);
impl OffsetCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFSET_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OffsetCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFSET_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONSTANT_PRODUCT_CURVE_ACCOUNT_DISCM: [u8; 8] = [
    146, 194, 5, 249, 89, 100, 210, 56,
];
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
pub struct ConstantProductCurve {}
impl ConstantProductCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantProductCurveAccount(pub ConstantProductCurve);
impl ConstantProductCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSTANT_PRODUCT_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ConstantProductCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSTANT_PRODUCT_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONSTANT_PRICE_CURVE_ACCOUNT_DISCM: [u8; 8] = [
    92, 127, 20, 92, 170, 54, 145, 11,
];
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
pub struct ConstantPriceCurve {
    pub token_b_price: u64,
}
impl ConstantPriceCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_b_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_b_price })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_b_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantPriceCurveAccount(pub ConstantPriceCurve);
impl ConstantPriceCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONSTANT_PRICE_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ConstantPriceCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONSTANT_PRICE_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
