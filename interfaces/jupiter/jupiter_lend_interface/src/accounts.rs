use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LENDING_ACCOUNT_DISCM: [u8; 8] = [135, 199, 82, 16, 249, 131, 182, 241];
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
pub struct Lending {
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub lending_id: u16,
    pub decimals: u8,
    pub rewards_rate_model: Pubkey,
    pub liquidity_exchange_price: u64,
    pub token_exchange_price: u64,
    pub last_update_timestamp: u64,
    pub token_reserves_liquidity: Pubkey,
    pub supply_position_on_liquidity: Pubkey,
    pub bump: u8,
}
impl Lending {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let f_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lending_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_rate_model: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_reserves_liquidity: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supply_position_on_liquidity: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            f_token_mint,
            lending_id,
            decimals,
            rewards_rate_model,
            liquidity_exchange_price,
            token_exchange_price,
            last_update_timestamp,
            token_reserves_liquidity,
            supply_position_on_liquidity,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.f_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_rate_model, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_reserves_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.supply_position_on_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccount(pub Lending);
impl LendingAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Lending::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_ADMIN_ACCOUNT_DISCM: [u8; 8] = [42, 8, 33, 220, 163, 40, 210, 5];
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
pub struct LendingAdmin {
    pub authority: Pubkey,
    pub liquidity_program: Pubkey,
    pub rebalancer: Pubkey,
    pub next_lending_id: u16,
    pub auths: Vec<Pubkey>,
    pub bump: u8,
}
impl LendingAdmin {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rebalancer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let next_lending_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let auths: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            liquidity_program,
            rebalancer,
            next_lending_id,
            auths,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalancer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_lending_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auths, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAdminAccount(pub LendingAdmin);
impl LendingAdminAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ADMIN_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LendingAdmin::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ADMIN_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_REWARDS_RATE_MODEL_ACCOUNT_DISCM: [u8; 8] = [
    166, 72, 71, 131, 172, 74, 166, 181,
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
pub struct LendingRewardsRateModel {
    pub mint: Pubkey,
    pub start_tvl: u64,
    pub duration: u64,
    pub start_time: u64,
    pub yearly_reward: u64,
    pub next_duration: u64,
    pub next_reward_amount: u64,
    pub bump: u8,
}
impl LendingRewardsRateModel {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yearly_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_reward_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            start_tvl,
            duration,
            start_time,
            yearly_reward,
            next_duration,
            next_reward_amount,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_tvl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yearly_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_reward_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingRewardsRateModelAccount(pub LendingRewardsRateModel);
impl LendingRewardsRateModelAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_REWARDS_RATE_MODEL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LendingRewardsRateModel::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_REWARDS_RATE_MODEL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_RESERVE_ACCOUNT_DISCM: [u8; 8] = [21, 18, 59, 135, 120, 20, 31, 12];
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
pub struct TokenReserve {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub borrow_rate: u16,
    pub fee_on_interest: u16,
    pub last_utilization: u16,
    pub last_update_timestamp: u64,
    pub supply_exchange_price: u64,
    pub borrow_exchange_price: u64,
    pub max_utilization: u16,
    pub total_supply_with_interest: u64,
    pub total_supply_interest_free: u64,
    pub total_borrow_with_interest: u64,
    pub total_borrow_interest_free: u64,
    pub total_claim_amount: u64,
    pub interacting_protocol: Pubkey,
    pub interacting_timestamp: u64,
    pub interacting_balance: u64,
}
impl TokenReserve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let borrow_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_on_interest: u16 = crate::borsh_de_or_default(&mut reader)?;
        let last_utilization: u16 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let supply_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_utilization: u16 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply_with_interest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply_interest_free: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_borrow_with_interest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_borrow_interest_free: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claim_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let interacting_protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let interacting_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let interacting_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            borrow_rate,
            fee_on_interest,
            last_utilization,
            last_update_timestamp,
            supply_exchange_price,
            borrow_exchange_price,
            max_utilization,
            total_supply_with_interest,
            total_supply_interest_free,
            total_borrow_with_interest,
            total_borrow_interest_free,
            total_claim_amount,
            interacting_protocol,
            interacting_timestamp,
            interacting_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_on_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_utilization, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_utilization, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply_with_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply_interest_free, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_borrow_with_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_borrow_interest_free, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claim_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.interacting_protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.interacting_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.interacting_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenReserveAccount(pub TokenReserve);
impl TokenReserveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_RESERVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenReserve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_RESERVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_SUPPLY_POSITION_ACCOUNT_DISCM: [u8; 8] = [
    202, 219, 136, 118, 61, 177, 21, 146,
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
pub struct UserSupplyPosition {
    pub protocol: Pubkey,
    pub mint: Pubkey,
    pub with_interest: u8,
    pub amount: u64,
    pub withdrawal_limit: u128,
    pub last_update: u64,
    pub expand_pct: u16,
    pub expand_duration: u64,
    pub base_withdrawal_limit: u64,
    pub status: u8,
}
impl UserSupplyPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let with_interest: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expand_pct: u16 = crate::borsh_de_or_default(&mut reader)?;
        let expand_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_withdrawal_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol,
            mint,
            with_interest,
            amount,
            withdrawal_limit,
            last_update,
            expand_pct,
            expand_duration,
            base_withdrawal_limit,
            status,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.with_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_pct, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_withdrawal_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserSupplyPositionAccount(pub UserSupplyPosition);
impl UserSupplyPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_SUPPLY_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserSupplyPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_SUPPLY_POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
