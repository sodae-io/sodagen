use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
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
pub struct SplitStakeAccountInfo {
    pub account: Pubkey,
    pub index: u32,
}
impl SplitStakeAccountInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { account, index })
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
pub struct U64ValueChange {
    pub old: u64,
    pub new: u64,
}
impl U64ValueChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old, new })
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
pub struct U32ValueChange {
    pub old: u32,
    pub new: u32,
}
impl U32ValueChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old, new })
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
pub struct FeeValueChange {
    pub old: Fee,
    pub new: Fee,
}
impl FeeValueChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let new = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { old, new })
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
pub struct FeeCentsValueChange {
    pub old: FeeCents,
    pub new: FeeCents,
}
impl FeeCentsValueChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCents>::deserialize(&mut reader)?
        };
        let new = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCents>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { old, new })
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
pub struct PubkeyValueChange {
    pub old: Pubkey,
    pub new: Pubkey,
}
impl PubkeyValueChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old, new })
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
pub struct BoolValueChange {
    pub old: bool,
    pub new: bool,
}
impl BoolValueChange {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old: bool = crate::borsh_de_or_default(&mut reader)?;
        let new: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old, new })
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
pub struct ChangeAuthorityData {
    pub admin: Option<Pubkey>,
    pub validator_manager: Option<Pubkey>,
    pub operational_sol_account: Option<Pubkey>,
    pub treasury_msol_account: Option<Pubkey>,
    pub pause_authority: Option<Pubkey>,
}
impl ChangeAuthorityData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let validator_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let operational_sol_account: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let treasury_msol_account: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pause_authority: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            validator_manager,
            operational_sol_account,
            treasury_msol_account,
            pause_authority,
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
pub struct ConfigLpParams {
    pub min_fee: Option<Fee>,
    pub max_fee: Option<Fee>,
    pub liquidity_target: Option<u64>,
    pub treasury_cut: Option<Fee>,
}
impl ConfigLpParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_fee: Option<Fee> = crate::borsh_de_or_default(&mut reader)?;
        let max_fee: Option<Fee> = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_target: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let treasury_cut: Option<Fee> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            min_fee,
            max_fee,
            liquidity_target,
            treasury_cut,
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
pub struct ConfigMarinadeParams {
    pub rewards_fee: Option<Fee>,
    pub slots_for_stake_delta: Option<u64>,
    pub min_stake: Option<u64>,
    pub min_deposit: Option<u64>,
    pub min_withdraw: Option<u64>,
    pub staking_sol_cap: Option<u64>,
    pub liquidity_sol_cap: Option<u64>,
    pub withdraw_stake_account_enabled: Option<bool>,
    pub delayed_unstake_fee: Option<FeeCents>,
    pub withdraw_stake_account_fee: Option<FeeCents>,
    pub max_stake_moved_per_epoch: Option<Fee>,
}
impl ConfigMarinadeParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let rewards_fee: Option<Fee> = crate::borsh_de_or_default(&mut reader)?;
        let slots_for_stake_delta: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_stake: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let min_deposit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let min_withdraw: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let staking_sol_cap: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_sol_cap: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_stake_account_enabled: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let delayed_unstake_fee: Option<FeeCents> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdraw_stake_account_fee: Option<FeeCents> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_stake_moved_per_epoch: Option<Fee> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            rewards_fee,
            slots_for_stake_delta,
            min_stake,
            min_deposit,
            min_withdraw,
            staking_sol_cap,
            liquidity_sol_cap,
            withdraw_stake_account_enabled,
            delayed_unstake_fee,
            withdraw_stake_account_fee,
            max_stake_moved_per_epoch,
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
pub struct InitializeData {
    pub admin_authority: Pubkey,
    pub validator_manager_authority: Pubkey,
    pub min_stake: u64,
    pub rewards_fee: Fee,
    pub liq_pool: LiqPoolInitializeData,
    pub additional_stake_record_space: u32,
    pub additional_validator_record_space: u32,
    pub slots_for_stake_delta: u64,
    pub pause_authority: Pubkey,
}
impl InitializeData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator_manager_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_stake: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let liq_pool = if reader.is_empty() {
            Default::default()
        } else {
            <LiqPoolInitializeData>::deserialize(&mut reader)?
        };
        let additional_stake_record_space: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_validator_record_space: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let slots_for_stake_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pause_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin_authority,
            validator_manager_authority,
            min_stake,
            rewards_fee,
            liq_pool,
            additional_stake_record_space,
            additional_validator_record_space,
            slots_for_stake_delta,
            pause_authority,
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
pub struct LiqPoolInitializeData {
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub lp_treasury_cut: Fee,
}
impl LiqPoolInitializeData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_liquidity_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let lp_min_fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let lp_treasury_cut = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lp_liquidity_target,
            lp_max_fee,
            lp_min_fee,
            lp_treasury_cut,
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
pub struct Fee {
    pub basis_points: u32,
}
impl Fee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let basis_points: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { basis_points })
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
pub struct FeeCents {
    pub bp_cents: u32,
}
impl FeeCents {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bp_cents: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bp_cents })
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
pub struct LiqPool {
    pub lp_mint: Pubkey,
    pub lp_mint_authority_bump_seed: u8,
    pub sol_leg_bump_seed: u8,
    pub msol_leg_authority_bump_seed: u8,
    pub msol_leg: Pubkey,
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,
    pub lp_supply: u64,
    pub lent_from_sol_leg: u64,
    pub liquidity_sol_cap: u64,
}
impl LiqPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint_authority_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let sol_leg_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let msol_leg_authority_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let msol_leg: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_liquidity_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let lp_min_fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let treasury_cut = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let lp_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lent_from_sol_leg: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_sol_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_mint,
            lp_mint_authority_bump_seed,
            sol_leg_bump_seed,
            msol_leg_authority_bump_seed,
            msol_leg,
            lp_liquidity_target,
            lp_max_fee,
            lp_min_fee,
            treasury_cut,
            lp_supply,
            lent_from_sol_leg,
            liquidity_sol_cap,
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
pub struct List {
    pub account: Pubkey,
    pub item_size: u32,
    pub count: u32,
    pub reserved1: Pubkey,
    pub reserved2: u32,
}
impl List {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let item_size: u32 = crate::borsh_de_or_default(&mut reader)?;
        let count: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reserved1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved2: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            account,
            item_size,
            count,
            reserved1,
            reserved2,
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
pub struct StakeRecord {
    pub stake_account: Pubkey,
    pub last_update_delegated_lamports: u64,
    pub last_update_epoch: u64,
    pub is_emergency_unstaking: u8,
}
impl StakeRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update_delegated_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_update_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_emergency_unstaking: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            stake_account,
            last_update_delegated_lamports,
            last_update_epoch,
            is_emergency_unstaking,
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
pub struct StakeList {}
impl StakeList {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
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
pub struct StakeSystem {
    pub stake_list: List,
    pub delayed_unstake_cooling_down: u64,
    pub stake_deposit_bump_seed: u8,
    pub stake_withdraw_bump_seed: u8,
    pub slots_for_stake_delta: u64,
    pub last_stake_delta_epoch: u64,
    pub min_stake: u64,
    pub extra_stake_delta_runs: u32,
}
impl StakeSystem {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stake_list = if reader.is_empty() {
            Default::default()
        } else {
            <List>::deserialize(&mut reader)?
        };
        let delayed_unstake_cooling_down: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_deposit_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let stake_withdraw_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let slots_for_stake_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_stake_delta_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_stake: u64 = crate::borsh_de_or_default(&mut reader)?;
        let extra_stake_delta_runs: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            stake_list,
            delayed_unstake_cooling_down,
            stake_deposit_bump_seed,
            stake_withdraw_bump_seed,
            slots_for_stake_delta,
            last_stake_delta_epoch,
            min_stake,
            extra_stake_delta_runs,
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
pub struct ValidatorRecord {
    pub validator_account: Pubkey,
    pub active_balance: u64,
    pub score: u32,
    pub last_stake_delta_epoch: u64,
    pub duplication_flag_bump_seed: u8,
}
impl ValidatorRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let validator_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let score: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_stake_delta_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let duplication_flag_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            validator_account,
            active_balance,
            score,
            last_stake_delta_epoch,
            duplication_flag_bump_seed,
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
pub struct ValidatorList {}
impl ValidatorList {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
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
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: Pubkey,
    pub total_validator_score: u32,
    pub total_active_balance: u64,
    pub auto_add_validator_enabled: u8,
}
impl ValidatorSystem {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let validator_list = if reader.is_empty() {
            Default::default()
        } else {
            <List>::deserialize(&mut reader)?
        };
        let manager_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_validator_score: u32 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let auto_add_validator_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            validator_list,
            manager_authority,
            total_validator_score,
            total_active_balance,
            auto_add_validator_enabled,
        })
    }
}
