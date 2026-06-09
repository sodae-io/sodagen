use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LIQUIDITY_POOL_ACCOUNT_DISCM: [u8; 8] = [66, 38, 17, 64, 188, 80, 68, 129];
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
pub struct LiquidityPool {
    pub creator_wallet: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub company_tax_wallet: Pubkey,
    pub mint_account: Pubkey,
    pub sell_lock_period: i64,
    pub virtual_sol_reserve: u64,
    pub total_sol_volume: u64,
    pub real_token_reserve: u64,
    pub create_time: i64,
    pub ramping_limits: Vec<RampingLimit>,
    pub in_use: bool,
    pub closed: bool,
}
impl LiquidityPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let company_tax_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sell_lock_period: i64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_sol_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let create_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let ramping_limits: Vec<RampingLimit> = crate::borsh_de_or_default(&mut reader)?;
        let in_use: bool = crate::borsh_de_or_default(&mut reader)?;
        let closed: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator_wallet,
            platform_fee_wallet,
            company_tax_wallet,
            mint_account,
            sell_lock_period,
            virtual_sol_reserve,
            total_sol_volume,
            real_token_reserve,
            create_time,
            ramping_limits,
            in_use,
            closed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.creator_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_fee_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.company_tax_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell_lock_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_sol_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ramping_limits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_use, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.closed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityPoolAccount(pub LiquidityPool);
impl LiquidityPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LiquidityPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
