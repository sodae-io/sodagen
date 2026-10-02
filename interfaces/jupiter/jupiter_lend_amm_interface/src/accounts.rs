use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const DEX_ACCOUNT_DISCM: [u8; 8] = [236, 30, 181, 80, 209, 217, 25, 163];
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
pub struct Dex {
    pub dex_id: u16,
    pub re_entrancy: u8,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_decimals: u8,
    pub token_1_decimals: u8,
    pub last_to_last_stored_price: u128,
    pub last_stored_price: u128,
    pub center_price: u128,
    pub last_update_timestamp: u64,
    pub last_update_slot: u64,
    pub is_smart_collateral_enabled: u8,
    pub is_smart_debt_enabled: u8,
    pub fee: u32,
    pub revenue_cut: u8,
    pub percent_change_active: u8,
    pub upper_percent: u32,
    pub lower_percent: u32,
    pub threshold_change_active: u8,
    pub upper_shift_threshold_percent: u16,
    pub lower_shift_threshold_percent: u16,
    pub shifting_time: u32,
    pub center_price_address: Pubkey,
    pub max_center_price: u64,
    pub min_center_price: u64,
    pub token_0_max_utilization: u16,
    pub token_1_max_utilization: u16,
    pub is_center_price_shift_active: u8,
    pub swap_and_arbitrage_paused: u8,
    pub total_supply_shares: u64,
    pub max_supply_shares: u64,
    pub total_borrow_shares: u64,
    pub max_borrow_shares: u64,
    pub range_old_upper_shift: u32,
    pub range_old_lower_shift: u32,
    pub range_shift_duration: u32,
    pub range_shift_start_timestamp: u32,
    pub threshold_old_upper_shift: u16,
    pub threshold_old_lower_shift: u16,
    pub threshold_shift_duration: u32,
    pub threshold_shift_start_timestamp: u32,
    pub threshold_shift_old_timestamp: u32,
    pub center_price_shift_start_timestamp: u32,
    pub center_price_shift_percent: u32,
    pub center_price_shift_time: u32,
    pub reserved: [u8; 32],
    pub bump: u8,
}
impl Dex {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let re_entrancy: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let last_to_last_stored_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_stored_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let center_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_smart_collateral_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_smart_debt_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let revenue_cut: u8 = crate::borsh_de_or_default(&mut reader)?;
        let percent_change_active: u8 = crate::borsh_de_or_default(&mut reader)?;
        let upper_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_change_active: u8 = crate::borsh_de_or_default(&mut reader)?;
        let upper_shift_threshold_percent: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lower_shift_threshold_percent: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let shifting_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        let center_price_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_max_utilization: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_max_utilization: u16 = crate::borsh_de_or_default(&mut reader)?;
        let is_center_price_shift_active: u8 = crate::borsh_de_or_default(&mut reader)?;
        let swap_and_arbitrage_paused: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_borrow_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let range_old_upper_shift: u32 = crate::borsh_de_or_default(&mut reader)?;
        let range_old_lower_shift: u32 = crate::borsh_de_or_default(&mut reader)?;
        let range_shift_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let range_shift_start_timestamp: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_old_upper_shift: u16 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_old_lower_shift: u16 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_shift_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_shift_start_timestamp: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let threshold_shift_old_timestamp: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let center_price_shift_start_timestamp: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let center_price_shift_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let center_price_shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            re_entrancy,
            token_0,
            token_1,
            token_0_decimals,
            token_1_decimals,
            last_to_last_stored_price,
            last_stored_price,
            center_price,
            last_update_timestamp,
            last_update_slot,
            is_smart_collateral_enabled,
            is_smart_debt_enabled,
            fee,
            revenue_cut,
            percent_change_active,
            upper_percent,
            lower_percent,
            threshold_change_active,
            upper_shift_threshold_percent,
            lower_shift_threshold_percent,
            shifting_time,
            center_price_address,
            max_center_price,
            min_center_price,
            token_0_max_utilization,
            token_1_max_utilization,
            is_center_price_shift_active,
            swap_and_arbitrage_paused,
            total_supply_shares,
            max_supply_shares,
            total_borrow_shares,
            max_borrow_shares,
            range_old_upper_shift,
            range_old_lower_shift,
            range_shift_duration,
            range_shift_start_timestamp,
            threshold_old_upper_shift,
            threshold_old_lower_shift,
            threshold_shift_duration,
            threshold_shift_start_timestamp,
            threshold_shift_old_timestamp,
            center_price_shift_start_timestamp,
            center_price_shift_percent,
            center_price_shift_time,
            reserved,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.re_entrancy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_to_last_stored_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_stored_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.center_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_smart_collateral_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.is_smart_debt_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_cut, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.percent_change_active, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.threshold_change_active, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.upper_shift_threshold_percent,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lower_shift_threshold_percent,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.shifting_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.center_price_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_center_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_center_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_max_utilization, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_max_utilization, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_center_price_shift_active,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.swap_and_arbitrage_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supply_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_borrow_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_borrow_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.range_old_upper_shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.range_old_lower_shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.range_shift_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.range_shift_start_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.threshold_old_upper_shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.threshold_old_lower_shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.threshold_shift_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.threshold_shift_start_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.threshold_shift_old_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.center_price_shift_start_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.center_price_shift_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.center_price_shift_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DexAccount(pub Dex);
impl DexAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEX_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Dex::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEX_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEX_ADMIN_ACCOUNT_DISCM: [u8; 8] = [82, 155, 122, 221, 230, 96, 118, 155];
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
pub struct DexAdmin {
    pub authority: Pubkey,
    pub liquidity_program: Pubkey,
    pub next_dex_id: u16,
    pub auths: Vec<Pubkey>,
    pub reserved: [u8; 32],
    pub bump: u8,
}
impl DexAdmin {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let next_dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let auths: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            liquidity_program,
            next_dex_id,
            auths,
            reserved,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auths, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DexAdminAccount(pub DexAdmin);
impl DexAdminAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEX_ADMIN_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DexAdmin::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEX_ADMIN_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEX_POSITION_ACCOUNT_DISCM: [u8; 8] = [30, 36, 219, 78, 189, 173, 170, 47];
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
pub struct DexPosition {
    pub dex_id: u16,
    pub protocol: Pubkey,
    pub supply_status: u8,
    pub supply_shares: u64,
    pub withdrawal_limit: u64,
    pub supply_last_update: u64,
    pub supply_expand_pct: u16,
    pub supply_expand_duration: u64,
    pub base_withdrawal_limit: u64,
    pub borrow_status: u8,
    pub borrow_shares: u64,
    pub debt_ceiling: u64,
    pub borrow_last_update: u64,
    pub borrow_expand_pct: u16,
    pub borrow_expand_duration: u32,
    pub base_debt_ceiling: u64,
    pub max_debt_ceiling: u64,
    pub reserved: [u8; 32],
}
impl DexPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dex_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supply_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let supply_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let supply_last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let supply_expand_pct: u16 = crate::borsh_de_or_default(&mut reader)?;
        let supply_expand_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_withdrawal_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_expand_pct: u16 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_expand_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let base_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dex_id,
            protocol,
            supply_status,
            supply_shares,
            withdrawal_limit,
            supply_last_update,
            supply_expand_pct,
            supply_expand_duration,
            base_withdrawal_limit,
            borrow_status,
            borrow_shares,
            debt_ceiling,
            borrow_last_update,
            borrow_expand_pct,
            borrow_expand_duration,
            base_debt_ceiling,
            max_debt_ceiling,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dex_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_expand_pct, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_expand_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_withdrawal_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_expand_pct, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_expand_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DexPositionAccount(pub DexPosition);
impl DexPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEX_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DexPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEX_POSITION_ACCOUNT_DISCM)?;
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
pub const USER_BORROW_POSITION_ACCOUNT_DISCM: [u8; 8] = [
    73, 126, 65, 123, 220, 126, 197, 24,
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
pub struct UserBorrowPosition {
    pub protocol: Pubkey,
    pub mint: Pubkey,
    pub with_interest: u8,
    pub amount: u64,
    pub debt_ceiling: u64,
    pub last_update: u64,
    pub expand_pct: u16,
    pub expand_duration: u32,
    pub base_debt_ceiling: u64,
    pub max_debt_ceiling: u64,
    pub status: u8,
}
impl UserBorrowPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let with_interest: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expand_pct: u16 = crate::borsh_de_or_default(&mut reader)?;
        let expand_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let base_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol,
            mint,
            with_interest,
            amount,
            debt_ceiling,
            last_update,
            expand_pct,
            expand_duration,
            base_debt_ceiling,
            max_debt_ceiling,
            status,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.with_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_pct, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expand_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_debt_ceiling, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserBorrowPositionAccount(pub UserBorrowPosition);
impl UserBorrowPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_BORROW_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserBorrowPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_BORROW_POSITION_ACCOUNT_DISCM)?;
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
