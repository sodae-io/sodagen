use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const NEW_USER_RECORD_EVENT_DISCM: [u8; 8] = [236, 186, 113, 219, 42, 51, 149, 249];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewUserRecord {
    pub ts: i64,
    pub user_authority: Pubkey,
    pub user: Pubkey,
    pub sub_account_id: u16,
    pub name: [u8; 32],
    pub referrer: Pubkey,
}
impl NewUserRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user_authority,
            user,
            sub_account_id,
            name,
            referrer,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sub_account_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewUserRecordEvent(pub NewUserRecord);
impl NewUserRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_USER_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = NewUserRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_USER_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_RECORD_EVENT_DISCM: [u8; 8] = [180, 241, 218, 207, 102, 135, 44, 134];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositRecord {
    pub ts: i64,
    pub user_authority: Pubkey,
    pub user: Pubkey,
    pub direction: DepositDirection,
    pub deposit_record_id: u64,
    pub amount: u64,
    pub market_index: u16,
    pub oracle_price: i64,
    pub market_deposit_balance: u128,
    pub market_withdraw_balance: u128,
    pub market_cumulative_deposit_interest: u128,
    pub market_cumulative_borrow_interest: u128,
    pub total_deposits_after: u64,
    pub total_withdraws_after: u64,
    pub explanation: DepositExplanation,
    pub transfer_user: Option<Pubkey>,
    pub signer: Option<Pubkey>,
    pub user_token_amount_after: i128,
}
impl DepositRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let direction: DepositDirection = crate::borsh_de_or_default(&mut reader)?;
        let deposit_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_deposit_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let market_withdraw_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let market_cumulative_deposit_interest: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let market_cumulative_borrow_interest: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_deposits_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_withdraws_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let explanation: DepositExplanation = crate::borsh_de_or_default(&mut reader)?;
        let transfer_user: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let signer: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let user_token_amount_after: i128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user_authority,
            user,
            direction,
            deposit_record_id,
            amount,
            market_index,
            oracle_price,
            market_deposit_balance,
            market_withdraw_balance,
            market_cumulative_deposit_interest,
            market_cumulative_borrow_interest,
            total_deposits_after,
            total_withdraws_after,
            explanation,
            transfer_user,
            signer,
            user_token_amount_after,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_deposit_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_withdraw_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.market_cumulative_deposit_interest,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.market_cumulative_borrow_interest,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_deposits_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_withdraws_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.explanation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_token_amount_after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositRecordEvent(pub DepositRecord);
impl DepositRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SPOT_INTEREST_RECORD_EVENT_DISCM: [u8; 8] = [
    183, 186, 203, 186, 225, 187, 95, 130,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpotInterestRecord {
    pub ts: i64,
    pub market_index: u16,
    pub deposit_balance: u128,
    pub cumulative_deposit_interest: u128,
    pub borrow_balance: u128,
    pub cumulative_borrow_interest: u128,
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
}
impl SpotInterestRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposit_interest: u128 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_borrow_interest: u128 = crate::borsh_de_or_default(&mut reader)?;
        let optimal_utilization: u32 = crate::borsh_de_or_default(&mut reader)?;
        let optimal_borrow_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            market_index,
            deposit_balance,
            cumulative_deposit_interest,
            borrow_balance,
            cumulative_borrow_interest,
            optimal_utilization,
            optimal_borrow_rate,
            max_borrow_rate,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_deposit_interest,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.borrow_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cumulative_borrow_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.optimal_utilization, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.optimal_borrow_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_borrow_rate, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotInterestRecordEvent(pub SpotInterestRecord);
impl SpotInterestRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPOT_INTEREST_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SpotInterestRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPOT_INTEREST_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUNDING_PAYMENT_RECORD_EVENT_DISCM: [u8; 8] = [
    8, 59, 96, 20, 137, 201, 56, 95,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundingPaymentRecord {
    pub ts: i64,
    pub user_authority: Pubkey,
    pub user: Pubkey,
    pub market_index: u16,
    pub funding_payment: i64,
    pub base_asset_amount: i64,
    pub user_last_cumulative_funding: i64,
    pub amm_cumulative_funding_long: i128,
    pub amm_cumulative_funding_short: i128,
}
impl FundingPaymentRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let funding_payment: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_last_cumulative_funding: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amm_cumulative_funding_long: i128 = crate::borsh_de_or_default(&mut reader)?;
        let amm_cumulative_funding_short: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            ts,
            user_authority,
            user,
            market_index,
            funding_payment,
            base_asset_amount,
            user_last_cumulative_funding,
            amm_cumulative_funding_long,
            amm_cumulative_funding_short,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_payment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_last_cumulative_funding,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.amm_cumulative_funding_long,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.amm_cumulative_funding_short,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundingPaymentRecordEvent(pub FundingPaymentRecord);
impl FundingPaymentRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUNDING_PAYMENT_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FundingPaymentRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUNDING_PAYMENT_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUNDING_RATE_RECORD_EVENT_DISCM: [u8; 8] = [68, 3, 255, 26, 133, 91, 147, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundingRateRecord {
    pub ts: i64,
    pub record_id: u64,
    pub market_index: u16,
    pub funding_rate: i64,
    pub funding_rate_long: i128,
    pub funding_rate_short: i128,
    pub cumulative_funding_rate_long: i128,
    pub cumulative_funding_rate_short: i128,
    pub oracle_price_twap: i64,
    pub mark_price_twap: u64,
    pub period_revenue: i64,
    pub base_asset_amount_with_amm: i128,
    pub base_asset_amount_with_unsettled_lp: i128,
}
impl FundingRateRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let funding_rate: i64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_rate_long: i128 = crate::borsh_de_or_default(&mut reader)?;
        let funding_rate_short: i128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_funding_rate_long: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_funding_rate_short: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_price_twap: i64 = crate::borsh_de_or_default(&mut reader)?;
        let mark_price_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let period_revenue: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_with_amm: i128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_with_unsettled_lp: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            ts,
            record_id,
            market_index,
            funding_rate,
            funding_rate_long,
            funding_rate_short,
            cumulative_funding_rate_long,
            cumulative_funding_rate_short,
            oracle_price_twap,
            mark_price_twap,
            period_revenue,
            base_asset_amount_with_amm,
            base_asset_amount_with_unsettled_lp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_rate_long, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_rate_short, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_funding_rate_long,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_funding_rate_short,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.oracle_price_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mark_price_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.period_revenue, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount_with_amm, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.base_asset_amount_with_unsettled_lp,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundingRateRecordEvent(pub FundingRateRecord);
impl FundingRateRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUNDING_RATE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FundingRateRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUNDING_RATE_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CURVE_RECORD_EVENT_DISCM: [u8; 8] = [101, 238, 40, 228, 70, 46, 61, 117];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurveRecord {
    pub ts: i64,
    pub record_id: u64,
    pub peg_multiplier_before: u128,
    pub base_asset_reserve_before: u128,
    pub quote_asset_reserve_before: u128,
    pub sqrt_k_before: u128,
    pub peg_multiplier_after: u128,
    pub base_asset_reserve_after: u128,
    pub quote_asset_reserve_after: u128,
    pub sqrt_k_after: u128,
    pub base_asset_amount_long: u128,
    pub base_asset_amount_short: u128,
    pub base_asset_amount_with_amm: i128,
    pub total_fee: i128,
    pub total_fee_minus_distributions: i128,
    pub adjustment_cost: i128,
    pub oracle_price: i64,
    pub fill_record: u128,
    pub number_of_users: u32,
    pub market_index: u16,
}
impl CurveRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let peg_multiplier_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_reserve_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_reserve_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_k_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let peg_multiplier_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_reserve_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_reserve_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_k_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_long: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_short: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_with_amm: i128 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee: i128 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee_minus_distributions: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let adjustment_cost: i128 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let fill_record: u128 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_users: u32 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            record_id,
            peg_multiplier_before,
            base_asset_reserve_before,
            quote_asset_reserve_before,
            sqrt_k_before,
            peg_multiplier_after,
            base_asset_reserve_after,
            quote_asset_reserve_after,
            sqrt_k_after,
            base_asset_amount_long,
            base_asset_amount_short,
            base_asset_amount_with_amm,
            total_fee,
            total_fee_minus_distributions,
            adjustment_cost,
            oracle_price,
            fill_record,
            number_of_users,
            market_index,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.peg_multiplier_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_reserve_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_asset_reserve_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_k_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.peg_multiplier_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_reserve_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_asset_reserve_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_k_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount_long, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount_short, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount_with_amm, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_fee_minus_distributions,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.adjustment_cost, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fill_record, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CurveRecordEvent(pub CurveRecord);
impl CurveRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CURVE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CurveRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CURVE_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SIGNED_MSG_ORDER_RECORD_EVENT_DISCM: [u8; 8] = [
    211, 197, 25, 18, 142, 86, 113, 27,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignedMsgOrderRecord {
    pub user: Pubkey,
    pub hash: String,
    pub matching_order_params: OrderParams,
    pub user_order_id: u32,
    pub signed_msg_order_max_slot: u64,
    pub signed_msg_order_uuid: [u8; 8],
    pub ts: i64,
}
impl SignedMsgOrderRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let hash: String = crate::borsh_de_or_default(&mut reader)?;
        let matching_order_params = if reader.is_empty() {
            Default::default()
        } else {
            <OrderParams>::deserialize(&mut reader)?
        };
        let user_order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let signed_msg_order_max_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let signed_msg_order_uuid: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            hash,
            matching_order_params,
            user_order_id,
            signed_msg_order_max_slot,
            signed_msg_order_uuid,
            ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.matching_order_params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signed_msg_order_max_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.signed_msg_order_uuid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SignedMsgOrderRecordEvent(pub SignedMsgOrderRecord);
impl SignedMsgOrderRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SIGNED_MSG_ORDER_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SignedMsgOrderRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SIGNED_MSG_ORDER_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORDER_RECORD_EVENT_DISCM: [u8; 8] = [104, 19, 64, 56, 89, 21, 2, 90];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderRecord {
    pub ts: i64,
    pub user: Pubkey,
    pub order: Order,
}
impl OrderRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let order = if reader.is_empty() {
            Default::default()
        } else {
            <Order>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { ts, user, order })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderRecordEvent(pub OrderRecord);
impl OrderRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORDER_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OrderRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORDER_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORDER_ACTION_RECORD_EVENT_DISCM: [u8; 8] = [224, 52, 67, 71, 194, 237, 109, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderActionRecord {
    pub ts: i64,
    pub action: OrderAction,
    pub action_explanation: OrderActionExplanation,
    pub market_index: u16,
    pub market_type: MarketType,
    pub filler: Option<Pubkey>,
    pub filler_reward: Option<u64>,
    pub fill_record_id: Option<u64>,
    pub base_asset_amount_filled: Option<u64>,
    pub quote_asset_amount_filled: Option<u64>,
    pub taker_fee: Option<u64>,
    pub maker_fee: Option<i64>,
    pub referrer_reward: Option<u32>,
    pub quote_asset_amount_surplus: Option<i64>,
    pub spot_fulfillment_method_fee: Option<u64>,
    pub taker: Option<Pubkey>,
    pub taker_order_id: Option<u32>,
    pub taker_order_direction: Option<PositionDirection>,
    pub taker_order_base_asset_amount: Option<u64>,
    pub taker_order_cumulative_base_asset_amount_filled: Option<u64>,
    pub taker_order_cumulative_quote_asset_amount_filled: Option<u64>,
    pub maker: Option<Pubkey>,
    pub maker_order_id: Option<u32>,
    pub maker_order_direction: Option<PositionDirection>,
    pub maker_order_base_asset_amount: Option<u64>,
    pub maker_order_cumulative_base_asset_amount_filled: Option<u64>,
    pub maker_order_cumulative_quote_asset_amount_filled: Option<u64>,
    pub oracle_price: i64,
    pub bit_flags: u8,
    pub taker_existing_quote_entry_amount: Option<u64>,
    pub taker_existing_base_asset_amount: Option<u64>,
    pub maker_existing_quote_entry_amount: Option<u64>,
    pub maker_existing_base_asset_amount: Option<u64>,
    pub trigger_price: Option<u64>,
    pub builder_idx: Option<u8>,
    pub builder_fee: Option<u64>,
}
impl OrderActionRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let action: OrderAction = crate::borsh_de_or_default(&mut reader)?;
        let action_explanation: OrderActionExplanation = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let filler: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let filler_reward: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let fill_record_id: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_filled: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let quote_asset_amount_filled: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let taker_fee: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let maker_fee: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let referrer_reward: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount_surplus: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let spot_fulfillment_method_fee: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let taker: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let taker_order_id: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let taker_order_direction: Option<PositionDirection> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let taker_order_base_asset_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let taker_order_cumulative_base_asset_amount_filled: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let taker_order_cumulative_quote_asset_amount_filled: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let maker: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let maker_order_id: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let maker_order_direction: Option<PositionDirection> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let maker_order_base_asset_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let maker_order_cumulative_base_asset_amount_filled: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let maker_order_cumulative_quote_asset_amount_filled: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let taker_existing_quote_entry_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let taker_existing_base_asset_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let maker_existing_quote_entry_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let maker_existing_base_asset_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let trigger_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let builder_idx: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let builder_fee: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            action,
            action_explanation,
            market_index,
            market_type,
            filler,
            filler_reward,
            fill_record_id,
            base_asset_amount_filled,
            quote_asset_amount_filled,
            taker_fee,
            maker_fee,
            referrer_reward,
            quote_asset_amount_surplus,
            spot_fulfillment_method_fee,
            taker,
            taker_order_id,
            taker_order_direction,
            taker_order_base_asset_amount,
            taker_order_cumulative_base_asset_amount_filled,
            taker_order_cumulative_quote_asset_amount_filled,
            maker,
            maker_order_id,
            maker_order_direction,
            maker_order_base_asset_amount,
            maker_order_cumulative_base_asset_amount_filled,
            maker_order_cumulative_quote_asset_amount_filled,
            oracle_price,
            bit_flags,
            taker_existing_quote_entry_amount,
            taker_existing_base_asset_amount,
            maker_existing_quote_entry_amount,
            maker_existing_base_asset_amount,
            trigger_price,
            builder_idx,
            builder_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.action, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.action_explanation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filler, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filler_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fill_record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount_filled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_asset_amount_filled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_asset_amount_surplus, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.spot_fulfillment_method_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_order_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.taker_order_base_asset_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.taker_order_cumulative_base_asset_amount_filled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.taker_order_cumulative_quote_asset_amount_filled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_order_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.maker_order_base_asset_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.maker_order_cumulative_base_asset_amount_filled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.maker_order_cumulative_quote_asset_amount_filled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bit_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.taker_existing_quote_entry_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.taker_existing_base_asset_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.maker_existing_quote_entry_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.maker_existing_base_asset_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.trigger_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.builder_idx, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.builder_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderActionRecordEvent(pub OrderActionRecord);
impl OrderActionRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORDER_ACTION_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OrderActionRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORDER_ACTION_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_RECORD_EVENT_DISCM: [u8; 8] = [101, 22, 54, 38, 178, 13, 142, 111];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpRecord {
    pub ts: i64,
    pub user: Pubkey,
    pub action: LPAction,
    pub n_shares: u64,
    pub market_index: u16,
    pub delta_base_asset_amount: i64,
    pub delta_quote_asset_amount: i64,
    pub pnl: i64,
}
impl LpRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let action: LPAction = crate::borsh_de_or_default(&mut reader)?;
        let n_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let delta_base_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let delta_quote_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let pnl: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user,
            action,
            n_shares,
            market_index,
            delta_base_asset_amount,
            delta_quote_asset_amount,
            pnl,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.action, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.n_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delta_base_asset_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delta_quote_asset_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpRecordEvent(pub LpRecord);
impl LpRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDATION_RECORD_EVENT_DISCM: [u8; 8] = [127, 17, 0, 108, 182, 13, 231, 53];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidationRecord {
    pub ts: i64,
    pub liquidation_type: LiquidationType,
    pub user: Pubkey,
    pub liquidator: Pubkey,
    pub margin_requirement: u128,
    pub total_collateral: i128,
    pub margin_freed: u64,
    pub liquidation_id: u16,
    pub bankrupt: bool,
    pub canceled_order_ids: Vec<u32>,
    pub liquidate_perp: LiquidatePerpRecord,
    pub liquidate_spot: LiquidateSpotRecord,
    pub liquidate_borrow_for_perp_pnl: LiquidateBorrowForPerpPnlRecord,
    pub liquidate_perp_pnl_for_deposit: LiquidatePerpPnlForDepositRecord,
    pub perp_bankruptcy: PerpBankruptcyRecord,
    pub spot_bankruptcy: SpotBankruptcyRecord,
    pub bit_flags: u8,
}
impl LiquidationRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_type: LiquidationType = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let margin_requirement: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_collateral: i128 = crate::borsh_de_or_default(&mut reader)?;
        let margin_freed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bankrupt: bool = crate::borsh_de_or_default(&mut reader)?;
        let canceled_order_ids: Vec<u32> = crate::borsh_de_or_default(&mut reader)?;
        let liquidate_perp = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidatePerpRecord>::deserialize(&mut reader)?
        };
        let liquidate_spot = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidateSpotRecord>::deserialize(&mut reader)?
        };
        let liquidate_borrow_for_perp_pnl = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidateBorrowForPerpPnlRecord>::deserialize(&mut reader)?
        };
        let liquidate_perp_pnl_for_deposit = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidatePerpPnlForDepositRecord>::deserialize(&mut reader)?
        };
        let perp_bankruptcy = if reader.is_empty() {
            Default::default()
        } else {
            <PerpBankruptcyRecord>::deserialize(&mut reader)?
        };
        let spot_bankruptcy = if reader.is_empty() {
            Default::default()
        } else {
            <SpotBankruptcyRecord>::deserialize(&mut reader)?
        };
        let bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            liquidation_type,
            user,
            liquidator,
            margin_requirement,
            total_collateral,
            margin_freed,
            liquidation_id,
            bankrupt,
            canceled_order_ids,
            liquidate_perp,
            liquidate_spot,
            liquidate_borrow_for_perp_pnl,
            liquidate_perp_pnl_for_deposit,
            perp_bankruptcy,
            spot_bankruptcy,
            bit_flags,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_requirement, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.margin_freed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bankrupt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.canceled_order_ids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidate_perp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidate_spot, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.liquidate_borrow_for_perp_pnl,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.liquidate_perp_pnl_for_deposit,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.perp_bankruptcy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_bankruptcy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bit_flags, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidationRecordEvent(pub LiquidationRecord);
impl LiquidationRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATION_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidationRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATION_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SETTLE_PNL_RECORD_EVENT_DISCM: [u8; 8] = [57, 68, 105, 26, 119, 198, 213, 89];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlePnlRecord {
    pub ts: i64,
    pub user: Pubkey,
    pub market_index: u16,
    pub pnl: i128,
    pub base_asset_amount: i64,
    pub quote_asset_amount_after: i64,
    pub quote_entry_amount: i64,
    pub settle_price: i64,
    pub explanation: SettlePnlExplanation,
}
impl SettlePnlRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pnl: i128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount_after: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_entry_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let settle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let explanation: SettlePnlExplanation = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user,
            market_index,
            pnl,
            base_asset_amount,
            quote_asset_amount_after,
            quote_entry_amount,
            settle_price,
            explanation,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pnl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_asset_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_asset_amount_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_entry_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.explanation, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettlePnlRecordEvent(pub SettlePnlRecord);
impl SettlePnlRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_PNL_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SettlePnlRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_PNL_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSURANCE_FUND_RECORD_EVENT_DISCM: [u8; 8] = [
    56, 222, 215, 235, 78, 197, 99, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsuranceFundRecord {
    pub ts: i64,
    pub spot_market_index: u16,
    pub perp_market_index: u16,
    pub user_if_factor: u32,
    pub total_if_factor: u32,
    pub vault_amount_before: u64,
    pub insurance_vault_amount_before: u64,
    pub total_if_shares_before: u128,
    pub total_if_shares_after: u128,
    pub amount: i64,
}
impl InsuranceFundRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let user_if_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let total_if_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let vault_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let insurance_vault_amount_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_if_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_if_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            spot_market_index,
            perp_market_index,
            user_if_factor,
            total_if_factor,
            vault_amount_before,
            insurance_vault_amount_before,
            total_if_shares_before,
            total_if_shares_after,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_if_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_if_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_amount_before, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.insurance_vault_amount_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_if_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_if_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundRecordEvent(pub InsuranceFundRecord);
impl InsuranceFundRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSURANCE_FUND_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InsuranceFundRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSURANCE_FUND_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM: [u8; 8] = [
    68, 66, 156, 7, 216, 148, 250, 114,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsuranceFundStakeRecord {
    pub ts: i64,
    pub user_authority: Pubkey,
    pub action: StakeAction,
    pub amount: u64,
    pub market_index: u16,
    pub insurance_vault_amount_before: u64,
    pub if_shares_before: u128,
    pub user_if_shares_before: u128,
    pub total_if_shares_before: u128,
    pub if_shares_after: u128,
    pub user_if_shares_after: u128,
    pub total_if_shares_after: u128,
}
impl InsuranceFundStakeRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let action: StakeAction = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let insurance_vault_amount_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let if_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let user_if_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_if_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let if_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let user_if_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_if_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user_authority,
            action,
            amount,
            market_index,
            insurance_vault_amount_before,
            if_shares_before,
            user_if_shares_before,
            total_if_shares_before,
            if_shares_after,
            user_if_shares_after,
            total_if_shares_after,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.action, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.insurance_vault_amount_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.if_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_if_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_if_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_if_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_if_shares_after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundStakeRecordEvent(pub InsuranceFundStakeRecord);
impl InsuranceFundStakeRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InsuranceFundStakeRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSURANCE_FUND_SWAP_RECORD_EVENT_DISCM: [u8; 8] = [
    85, 190, 99, 203, 237, 33, 227, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsuranceFundSwapRecord {
    pub rebalance_config: Pubkey,
    pub in_if_total_shares_before: u128,
    pub out_if_total_shares_before: u128,
    pub in_if_user_shares_before: u128,
    pub out_if_user_shares_before: u128,
    pub in_if_total_shares_after: u128,
    pub out_if_total_shares_after: u128,
    pub in_if_user_shares_after: u128,
    pub out_if_user_shares_after: u128,
    pub ts: i64,
    pub in_amount: u64,
    pub out_amount: u64,
    pub out_oracle_price: u64,
    pub out_oracle_price_twap: i64,
    pub in_vault_amount_before: u64,
    pub out_vault_amount_before: u64,
    pub in_fund_vault_amount_after: u64,
    pub out_fund_vault_amount_after: u64,
    pub in_market_index: u16,
    pub out_market_index: u16,
}
impl InsuranceFundSwapRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let rebalance_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_if_total_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let out_if_total_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let in_if_user_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let out_if_user_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let in_if_total_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let out_if_total_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let in_if_user_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let out_if_user_shares_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_oracle_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_oracle_price_twap: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_vault_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_vault_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_fund_vault_amount_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_fund_vault_amount_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let out_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            rebalance_config,
            in_if_total_shares_before,
            out_if_total_shares_before,
            in_if_user_shares_before,
            out_if_user_shares_before,
            in_if_total_shares_after,
            out_if_total_shares_after,
            in_if_user_shares_after,
            out_if_user_shares_after,
            ts,
            in_amount,
            out_amount,
            out_oracle_price,
            out_oracle_price_twap,
            in_vault_amount_before,
            out_vault_amount_before,
            in_fund_vault_amount_after,
            out_fund_vault_amount_after,
            in_market_index,
            out_market_index,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.rebalance_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_if_total_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_if_total_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_if_user_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_if_user_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_if_total_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_if_total_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_if_user_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_if_user_shares_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_oracle_price_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_vault_amount_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_vault_amount_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_fund_vault_amount_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.out_fund_vault_amount_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.in_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_market_index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundSwapRecordEvent(pub InsuranceFundSwapRecord);
impl InsuranceFundSwapRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSURANCE_FUND_SWAP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InsuranceFundSwapRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSURANCE_FUND_SWAP_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRANSFER_PROTOCOL_IF_SHARES_TO_REVENUE_POOL_RECORD_EVENT_DISCM: [u8; 8] = [
    209, 118, 142, 167, 130, 46, 164, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferProtocolIfSharesToRevenuePoolRecord {
    pub ts: i64,
    pub market_index: u16,
    pub amount: u64,
    pub shares: u128,
    pub if_vault_amount_before: u64,
    pub protocol_shares_before: u128,
    pub transfer_amount: u64,
}
impl TransferProtocolIfSharesToRevenuePoolRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let if_vault_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_shares_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            market_index,
            amount,
            shares,
            if_vault_amount_before,
            protocol_shares_before,
            transfer_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.if_vault_amount_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_shares_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferProtocolIfSharesToRevenuePoolRecordEvent(
    pub TransferProtocolIfSharesToRevenuePoolRecord,
);
impl TransferProtocolIfSharesToRevenuePoolRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_PROTOCOL_IF_SHARES_TO_REVENUE_POOL_RECORD_EVENT_DISCM
        {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TransferProtocolIfSharesToRevenuePoolRecord::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(&TRANSFER_PROTOCOL_IF_SHARES_TO_REVENUE_POOL_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_RECORD_EVENT_DISCM: [u8; 8] = [162, 187, 123, 194, 138, 56, 250, 241];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapRecord {
    pub ts: i64,
    pub user: Pubkey,
    pub amount_out: u64,
    pub amount_in: u64,
    pub out_market_index: u16,
    pub in_market_index: u16,
    pub out_oracle_price: i64,
    pub in_oracle_price: i64,
    pub fee: u64,
}
impl SwapRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let out_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user,
            amount_out,
            amount_in,
            out_market_index,
            in_market_index,
            out_oracle_price,
            in_oracle_price,
            fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapRecordEvent(pub SwapRecord);
impl SwapRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SPOT_MARKET_VAULT_DEPOSIT_RECORD_EVENT_DISCM: [u8; 8] = [
    178, 217, 23, 188, 127, 190, 32, 73,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpotMarketVaultDepositRecord {
    pub ts: i64,
    pub market_index: u16,
    pub deposit_balance: u128,
    pub cumulative_deposit_interest_before: u128,
    pub cumulative_deposit_interest_after: u128,
    pub deposit_token_amount_before: u64,
    pub amount: u64,
}
impl SpotMarketVaultDepositRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposit_interest_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_deposit_interest_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposit_token_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            market_index,
            deposit_balance,
            cumulative_deposit_interest_before,
            cumulative_deposit_interest_after,
            deposit_token_amount_before,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_deposit_interest_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_deposit_interest_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.deposit_token_amount_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotMarketVaultDepositRecordEvent(pub SpotMarketVaultDepositRecord);
impl SpotMarketVaultDepositRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPOT_MARKET_VAULT_DEPOSIT_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SpotMarketVaultDepositRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPOT_MARKET_VAULT_DEPOSIT_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DELETE_USER_RECORD_EVENT_DISCM: [u8; 8] = [71, 111, 190, 118, 7, 3, 132, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteUserRecord {
    pub ts: i64,
    pub user_authority: Pubkey,
    pub user: Pubkey,
    pub sub_account_id: u16,
    pub keeper: Option<Pubkey>,
}
impl DeleteUserRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let keeper: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            user_authority,
            user,
            sub_account_id,
            keeper,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sub_account_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.keeper, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteUserRecordEvent(pub DeleteUserRecord);
impl DeleteUserRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_USER_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeleteUserRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_USER_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUEL_SWEEP_RECORD_EVENT_DISCM: [u8; 8] = [41, 84, 37, 246, 132, 240, 131, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FuelSweepRecord {
    pub ts: i64,
    pub authority: Pubkey,
    pub user_stats_fuel_insurance: u32,
    pub user_stats_fuel_deposits: u32,
    pub user_stats_fuel_borrows: u32,
    pub user_stats_fuel_positions: u32,
    pub user_stats_fuel_taker: u32,
    pub user_stats_fuel_maker: u32,
    pub fuel_overflow_fuel_insurance: u128,
    pub fuel_overflow_fuel_deposits: u128,
    pub fuel_overflow_fuel_borrows: u128,
    pub fuel_overflow_fuel_positions: u128,
    pub fuel_overflow_fuel_taker: u128,
    pub fuel_overflow_fuel_maker: u128,
}
impl FuelSweepRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_stats_fuel_insurance: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_stats_fuel_deposits: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_stats_fuel_borrows: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_stats_fuel_positions: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_stats_fuel_taker: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_stats_fuel_maker: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_overflow_fuel_insurance: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fuel_overflow_fuel_deposits: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_overflow_fuel_borrows: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_overflow_fuel_positions: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fuel_overflow_fuel_taker: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_overflow_fuel_maker: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            authority,
            user_stats_fuel_insurance,
            user_stats_fuel_deposits,
            user_stats_fuel_borrows,
            user_stats_fuel_positions,
            user_stats_fuel_taker,
            user_stats_fuel_maker,
            fuel_overflow_fuel_insurance,
            fuel_overflow_fuel_deposits,
            fuel_overflow_fuel_borrows,
            fuel_overflow_fuel_positions,
            fuel_overflow_fuel_taker,
            fuel_overflow_fuel_maker,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats_fuel_insurance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats_fuel_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats_fuel_borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats_fuel_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats_fuel_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_stats_fuel_maker, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fuel_overflow_fuel_insurance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fuel_overflow_fuel_deposits,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.fuel_overflow_fuel_borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fuel_overflow_fuel_positions,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.fuel_overflow_fuel_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_overflow_fuel_maker, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FuelSweepRecordEvent(pub FuelSweepRecord);
impl FuelSweepRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUEL_SWEEP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FuelSweepRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUEL_SWEEP_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUEL_SEASON_RECORD_EVENT_DISCM: [u8; 8] = [19, 137, 119, 33, 224, 249, 6, 87];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FuelSeasonRecord {
    pub ts: i64,
    pub authority: Pubkey,
    pub fuel_insurance: u128,
    pub fuel_deposits: u128,
    pub fuel_borrows: u128,
    pub fuel_positions: u128,
    pub fuel_taker: u128,
    pub fuel_maker: u128,
    pub fuel_total: u128,
}
impl FuelSeasonRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fuel_insurance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_deposits: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_borrows: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_positions: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_taker: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_maker: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fuel_total: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            authority,
            fuel_insurance,
            fuel_deposits,
            fuel_borrows,
            fuel_positions,
            fuel_taker,
            fuel_maker,
            fuel_total,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_insurance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fuel_total, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FuelSeasonRecordEvent(pub FuelSeasonRecord);
impl FuelSeasonRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUEL_SEASON_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FuelSeasonRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUEL_SEASON_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REVENUE_SHARE_SETTLE_RECORD_EVENT_DISCM: [u8; 8] = [
    61, 162, 89, 10, 24, 20, 59, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevenueShareSettleRecord {
    pub ts: i64,
    pub builder: Option<Pubkey>,
    pub referrer: Option<Pubkey>,
    pub fee_settled: u64,
    pub market_index: u16,
    pub market_type: MarketType,
    pub builder_sub_account_id: u16,
    pub builder_total_referrer_rewards: u64,
    pub builder_total_builder_rewards: u64,
}
impl RevenueShareSettleRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let builder: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let fee_settled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let builder_sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let builder_total_referrer_rewards: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let builder_total_builder_rewards: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            ts,
            builder,
            referrer,
            fee_settled,
            market_index,
            market_type,
            builder_sub_account_id,
            builder_total_referrer_rewards,
            builder_total_builder_rewards,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.builder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_settled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.builder_sub_account_id, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.builder_total_referrer_rewards,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.builder_total_builder_rewards,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevenueShareSettleRecordEvent(pub RevenueShareSettleRecord);
impl RevenueShareSettleRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVENUE_SHARE_SETTLE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RevenueShareSettleRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVENUE_SHARE_SETTLE_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_SETTLE_RECORD_EVENT_DISCM: [u8; 8] = [208, 191, 131, 110, 173, 48, 7, 2];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpSettleRecord {
    pub record_id: u64,
    pub last_ts: i64,
    pub last_slot: u64,
    pub ts: i64,
    pub slot: u64,
    pub perp_market_index: u16,
    pub settle_to_lp_amount: i64,
    pub perp_amm_pnl_delta: i64,
    pub perp_amm_ex_fee_delta: i64,
    pub lp_aum: u128,
    pub lp_price: u128,
    pub lp_pool: Pubkey,
}
impl LpSettleRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let settle_to_lp_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let perp_amm_pnl_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        let perp_amm_ex_fee_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_aum: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lp_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            record_id,
            last_ts,
            last_slot,
            ts,
            slot,
            perp_market_index,
            settle_to_lp_amount,
            perp_amm_pnl_delta,
            perp_amm_ex_fee_delta,
            lp_aum,
            lp_price,
            lp_pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.record_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settle_to_lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_amm_pnl_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perp_amm_ex_fee_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_aum, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpSettleRecordEvent(pub LpSettleRecord);
impl LpSettleRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_SETTLE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpSettleRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_SETTLE_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_SWAP_RECORD_EVENT_DISCM: [u8; 8] = [159, 62, 130, 196, 96, 79, 176, 254];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpSwapRecord {
    pub ts: i64,
    pub slot: u64,
    pub authority: Pubkey,
    pub out_amount: u128,
    pub in_amount: u128,
    pub out_fee: i128,
    pub in_fee: i128,
    pub out_spot_market_index: u16,
    pub in_spot_market_index: u16,
    pub out_constituent_index: u16,
    pub in_constituent_index: u16,
    pub out_oracle_price: i64,
    pub in_oracle_price: i64,
    pub last_aum: u128,
    pub last_aum_slot: u64,
    pub in_market_current_weight: i64,
    pub out_market_current_weight: i64,
    pub in_market_target_weight: i64,
    pub out_market_target_weight: i64,
    pub in_swap_id: u64,
    pub out_swap_id: u64,
    pub lp_pool: Pubkey,
}
impl LpSwapRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let out_fee: i128 = crate::borsh_de_or_default(&mut reader)?;
        let in_fee: i128 = crate::borsh_de_or_default(&mut reader)?;
        let out_spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let in_spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let out_constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let in_constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let out_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_aum: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_aum_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_current_weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        let out_market_current_weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_target_weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        let out_market_target_weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_swap_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_swap_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            slot,
            authority,
            out_amount,
            in_amount,
            out_fee,
            in_fee,
            out_spot_market_index,
            in_spot_market_index,
            out_constituent_index,
            in_constituent_index,
            out_oracle_price,
            in_oracle_price,
            last_aum,
            last_aum_slot,
            in_market_current_weight,
            out_market_current_weight,
            in_market_target_weight,
            out_market_target_weight,
            in_swap_id,
            out_swap_id,
            lp_pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_constituent_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_constituent_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_aum, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_aum_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_market_current_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_market_current_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_market_target_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_market_target_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_swap_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_swap_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpSwapRecordEvent(pub LpSwapRecord);
impl LpSwapRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_SWAP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpSwapRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_SWAP_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_MINT_REDEEM_RECORD_EVENT_DISCM: [u8; 8] = [53, 178, 142, 73, 78, 91, 91, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpMintRedeemRecord {
    pub ts: i64,
    pub slot: u64,
    pub authority: Pubkey,
    pub description: u8,
    pub amount: u128,
    pub fee: i128,
    pub spot_market_index: u16,
    pub constituent_index: u16,
    pub oracle_price: i64,
    pub mint: Pubkey,
    pub lp_amount: u64,
    pub lp_fee: i64,
    pub lp_price: u128,
    pub mint_redeem_id: u64,
    pub last_aum: u128,
    pub last_aum_slot: u64,
    pub in_market_current_weight: i64,
    pub in_market_target_weight: i64,
    pub lp_pool: Pubkey,
}
impl LpMintRedeemRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let description: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee: i128 = crate::borsh_de_or_default(&mut reader)?;
        let spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let mint_redeem_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_aum: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_aum_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_current_weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_target_weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            slot,
            authority,
            description,
            amount,
            fee,
            spot_market_index,
            constituent_index,
            oracle_price,
            mint,
            lp_amount,
            lp_fee,
            lp_price,
            mint_redeem_id,
            last_aum,
            last_aum_slot,
            in_market_current_weight,
            in_market_target_weight,
            lp_pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.description, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.constituent_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_redeem_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_aum, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_aum_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_market_current_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_market_target_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpMintRedeemRecordEvent(pub LpMintRedeemRecord);
impl LpMintRedeemRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_MINT_REDEEM_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpMintRedeemRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_MINT_REDEEM_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_BORROW_LEND_DEPOSIT_RECORD_EVENT_DISCM: [u8; 8] = [
    242, 181, 11, 56, 243, 61, 79, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpBorrowLendDepositRecord {
    pub ts: i64,
    pub slot: u64,
    pub spot_market_index: u16,
    pub constituent_index: u16,
    pub direction: DepositDirection,
    pub token_balance: i64,
    pub last_token_balance: i64,
    pub interest_accrued_token_amount: i64,
    pub amount_deposit_withdraw: u64,
    pub lp_pool: Pubkey,
}
impl LpBorrowLendDepositRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let spot_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let direction: DepositDirection = crate::borsh_de_or_default(&mut reader)?;
        let token_balance: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_token_balance: i64 = crate::borsh_de_or_default(&mut reader)?;
        let interest_accrued_token_amount: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount_deposit_withdraw: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts,
            slot,
            spot_market_index,
            constituent_index,
            direction,
            token_balance,
            last_token_balance,
            interest_accrued_token_amount,
            amount_deposit_withdraw,
            lp_pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.constituent_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_token_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.interest_accrued_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.amount_deposit_withdraw, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpBorrowLendDepositRecordEvent(pub LpBorrowLendDepositRecord);
impl LpBorrowLendDepositRecordEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_BORROW_LEND_DEPOSIT_RECORD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpBorrowLendDepositRecord::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_BORROW_LEND_DEPOSIT_RECORD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
