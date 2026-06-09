use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BORROW_POSITION_ACCOUNT_DISCM: [u8; 8] = [243, 140, 20, 139, 32, 243, 114, 55];
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
pub struct BorrowPosition {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub borrow_size: u128,
    pub cumulative_compounded_interest_snapshot: u128,
    pub locked_collateral: u64,
    pub bump: u8,
    pub last_borrowed: i64,
}
impl BorrowPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let update_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_size: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_compounded_interest_snapshot: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let locked_collateral: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let last_borrowed: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            custody,
            open_time,
            update_time,
            borrow_size,
            cumulative_compounded_interest_snapshot,
            locked_collateral,
            bump,
            last_borrowed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.update_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_size, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_compounded_interest_snapshot,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.locked_collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_borrowed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowPositionAccount(pub BorrowPosition);
impl BorrowPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BorrowPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CUSTODY_ACCOUNT_DISCM: [u8; 8] = [1, 184, 48, 81, 93, 131, 63, 145];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Custody {
    pub pool: Pubkey,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub decimals: u8,
    pub is_stable: bool,
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub target_ratio_bps: u64,
    pub assets: Assets,
    pub funding_rate_state: FundingRateState,
    pub bump: u8,
    pub token_account_bump: u8,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub max_position_size_usd: u64,
    pub doves_oracle: Pubkey,
    pub jump_rate_state: JumpRateState,
    pub doves_ag_oracle: Pubkey,
    pub price_impact_buffer: PriceImpactBuffer,
    pub borrow_lend_parameters: BorrowLendParams,
    pub borrows_funding_rate_state: FundingRateState,
    pub debt: u128,
    pub borrow_lend_interests_accured: u128,
    pub borrow_limit_in_token_amount: u64,
    pub min_interest_fee_bps: u64,
    pub min_interest_fee_grace_period_seconds: u64,
    pub total_staked_amount_lamports: u64,
    pub max_total_staked_amount_lamports: u64,
    pub external_swap_fee_multiplier_bps: u64,
    pub disable_close_position_request: bool,
    pub withdrawal_limit_token_amount: u64,
    pub withdrawal_token_amount_accumulated: u64,
    pub withdrawal_limit_last_reset_at: i64,
    pub withdrawal_limit_interval_seconds: u64,
}
impl Custody {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_stable: bool = crate::borsh_de_or_default(&mut reader)?;
        let oracle = if reader.is_empty() {
            Default::default()
        } else {
            <OracleParams>::deserialize(&mut reader)?
        };
        let pricing = if reader.is_empty() {
            Default::default()
        } else {
            <PricingParams>::deserialize(&mut reader)?
        };
        let permissions = if reader.is_empty() {
            Default::default()
        } else {
            <Permissions>::deserialize(&mut reader)?
        };
        let target_ratio_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let assets = if reader.is_empty() {
            Default::default()
        } else {
            <Assets>::deserialize(&mut reader)?
        };
        let funding_rate_state = if reader.is_empty() {
            Default::default()
        } else {
            <FundingRateState>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_account_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let increase_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decrease_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let doves_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let jump_rate_state = if reader.is_empty() {
            Default::default()
        } else {
            <JumpRateState>::deserialize(&mut reader)?
        };
        let doves_ag_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_buffer = <PriceImpactBuffer>::deserialize(&mut reader)?;
        let borrow_lend_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowLendParams>::deserialize(&mut reader)?
        };
        let borrows_funding_rate_state = if reader.is_empty() {
            Default::default()
        } else {
            <FundingRateState>::deserialize(&mut reader)?
        };
        let debt: u128 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_lend_interests_accured: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_limit_in_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_interest_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_interest_fee_grace_period_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_staked_amount_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_total_staked_amount_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let external_swap_fee_multiplier_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let disable_close_position_request: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_token_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_token_amount_accumulated: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_last_reset_at: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_interval_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool,
            mint,
            token_account,
            decimals,
            is_stable,
            oracle,
            pricing,
            permissions,
            target_ratio_bps,
            assets,
            funding_rate_state,
            bump,
            token_account_bump,
            increase_position_bps,
            decrease_position_bps,
            max_position_size_usd,
            doves_oracle,
            jump_rate_state,
            doves_ag_oracle,
            price_impact_buffer,
            borrow_lend_parameters,
            borrows_funding_rate_state,
            debt,
            borrow_lend_interests_accured,
            borrow_limit_in_token_amount,
            min_interest_fee_bps,
            min_interest_fee_grace_period_seconds,
            total_staked_amount_lamports,
            max_total_staked_amount_lamports,
            external_swap_fee_multiplier_bps,
            disable_close_position_request,
            withdrawal_limit_token_amount,
            withdrawal_token_amount_accumulated,
            withdrawal_limit_last_reset_at,
            withdrawal_limit_interval_seconds,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_stable, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pricing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permissions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_ratio_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funding_rate_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_account_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.increase_position_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decrease_position_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_position_size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.doves_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.jump_rate_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.doves_ag_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_impact_buffer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_lend_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrows_funding_rate_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.borrow_lend_interests_accured,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.borrow_limit_in_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_interest_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_interest_fee_grace_period_seconds,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.total_staked_amount_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_total_staked_amount_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.external_swap_fee_multiplier_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.disable_close_position_request,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdrawal_limit_token_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdrawal_token_amount_accumulated,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdrawal_limit_last_reset_at,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdrawal_limit_interval_seconds,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CustodyAccount(pub Custody);
impl CustodyAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CUSTODY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Custody::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CUSTODY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PERPETUALS_ACCOUNT_DISCM: [u8; 8] = [28, 167, 98, 191, 104, 82, 108, 196];
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
pub struct Perpetuals {
    pub permissions: Permissions,
    pub pools: Vec<Pubkey>,
    pub admin: Pubkey,
    pub transfer_authority_bump: u8,
    pub perpetuals_bump: u8,
    pub inception_time: i64,
}
impl Perpetuals {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let permissions = if reader.is_empty() {
            Default::default()
        } else {
            <Permissions>::deserialize(&mut reader)?
        };
        let pools: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let transfer_authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let perpetuals_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let inception_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            permissions,
            pools,
            admin,
            transfer_authority_bump,
            perpetuals_bump,
            inception_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.permissions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pools, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.perpetuals_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inception_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PerpetualsAccount(pub Perpetuals);
impl PerpetualsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPETUALS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Perpetuals::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPETUALS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
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
pub struct Pool {
    pub name: String,
    pub custodies: Vec<Pubkey>,
    pub aum_usd: u128,
    pub limit: Limit,
    pub fees: Fees,
    pub pool_apr: PoolApr,
    pub max_request_execution_sec: i64,
    pub bump: u8,
    pub lp_token_bump: u8,
    pub inception_time: i64,
    pub parameter_update_oracle: Secp256k1Pubkey,
    pub aum_usd_updated_at: i64,
    pub max_trigger_price_diff_bps: u64,
    pub disable_close_position_request: bool,
    pub max_lp_token_price_change_bps: u64,
    pub aum_usd_refreshed_at_slot: u64,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let custodies: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let aum_usd: u128 = crate::borsh_de_or_default(&mut reader)?;
        let limit = if reader.is_empty() {
            Default::default()
        } else {
            <Limit>::deserialize(&mut reader)?
        };
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let pool_apr = if reader.is_empty() {
            Default::default()
        } else {
            <PoolApr>::deserialize(&mut reader)?
        };
        let max_request_execution_sec: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let inception_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let parameter_update_oracle = if reader.is_empty() {
            Default::default()
        } else {
            <Secp256k1Pubkey>::deserialize(&mut reader)?
        };
        let aum_usd_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_trigger_price_diff_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let disable_close_position_request: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_lp_token_price_change_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let aum_usd_refreshed_at_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            custodies,
            aum_usd,
            limit,
            fees,
            pool_apr,
            max_request_execution_sec,
            bump,
            lp_token_bump,
            inception_time,
            parameter_update_oracle,
            aum_usd_updated_at,
            max_trigger_price_diff_bps,
            disable_close_position_request,
            max_lp_token_price_change_bps,
            aum_usd_refreshed_at_slot,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custodies, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.aum_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_apr, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_request_execution_sec, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inception_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.parameter_update_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.aum_usd_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_trigger_price_diff_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.disable_close_position_request,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_lp_token_price_change_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.aum_usd_refreshed_at_slot, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_REQUEST_ACCOUNT_DISCM: [u8; 8] = [12, 38, 250, 199, 46, 154, 32, 216];
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
pub struct PositionRequest {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub position: Pubkey,
    pub mint: Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub request_change: RequestChange,
    pub request_type: RequestType,
    pub side: Side,
    pub price_slippage: Option<u64>,
    pub jupiter_minimum_out: Option<u64>,
    pub pre_swap_amount: Option<u64>,
    pub trigger_price: Option<u64>,
    pub trigger_above_threshold: Option<bool>,
    pub entire_position: Option<bool>,
    pub executed: bool,
    pub counter: u64,
    pub bump: u8,
    pub referral: Option<Pubkey>,
}
impl PositionRequest {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let update_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_change: RequestChange = crate::borsh_de_or_default(&mut reader)?;
        let request_type: RequestType = crate::borsh_de_or_default(&mut reader)?;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let jupiter_minimum_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let pre_swap_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_above_threshold: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let entire_position: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let executed: bool = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let referral: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            custody,
            position,
            mint,
            open_time,
            update_time,
            size_usd_delta,
            collateral_delta,
            request_change,
            request_type,
            side,
            price_slippage,
            jupiter_minimum_out,
            pre_swap_amount,
            trigger_price,
            trigger_above_threshold,
            entire_position,
            executed,
            counter,
            bump,
            referral,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.update_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.jupiter_minimum_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_swap_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trigger_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trigger_above_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.entire_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.counter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionRequestAccount(pub PositionRequest);
impl PositionRequestAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_REQUEST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionRequest::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_REQUEST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_ACCOUNT_DISCM: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
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
pub struct Position {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub side: Side,
    pub price: u64,
    pub size_usd: u64,
    pub collateral_usd: u64,
    pub realised_pnl_usd: i64,
    pub cumulative_interest_snapshot: u128,
    pub locked_amount: u64,
    pub bump: u8,
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_custody: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let update_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let realised_pnl_usd: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_interest_snapshot: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let locked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            custody,
            collateral_custody,
            open_time,
            update_time,
            side,
            price,
            size_usd,
            collateral_usd,
            realised_pnl_usd,
            cumulative_interest_snapshot,
            locked_amount,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_custody, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.update_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.side, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.realised_pnl_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_interest_snapshot,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.locked_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionAccount(pub Position);
impl PositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Position::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKE_INFO_ACCOUNT_DISCM: [u8; 8] = [66, 62, 68, 70, 108, 179, 183, 235];
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
pub struct StakeInfo {
    pub pool: Pubkey,
    pub stake_account: Pubkey,
    pub current_staked_amount_lamports: u64,
    pub total_staking_rewards_lamports: u64,
    pub last_updated_at: i64,
    pub deactivating: bool,
    pub stake_account_index: u64,
    pub bump: u8,
}
impl StakeInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_staked_amount_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_staking_rewards_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let deactivating: bool = crate::borsh_de_or_default(&mut reader)?;
        let stake_account_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            stake_account,
            current_staked_amount_lamports,
            total_staking_rewards_lamports,
            last_updated_at,
            deactivating,
            stake_account_index,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.current_staked_amount_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.total_staking_rewards_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deactivating, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeInfoAccount(pub StakeInfo);
impl StakeInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StakeInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_LEDGER_ACCOUNT_DISCM: [u8; 8] = [156, 247, 9, 188, 54, 108, 85, 77];
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
pub struct TokenLedger {
    pub token_account: Pubkey,
    pub amount: u64,
}
impl TokenLedger {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_account, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenLedgerAccount(pub TokenLedger);
impl TokenLedgerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_LEDGER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenLedger::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_LEDGER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
