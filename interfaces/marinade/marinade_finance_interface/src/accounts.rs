use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const TICKET_ACCOUNT_DATA_ACCOUNT_DISCM: [u8; 8] = [133, 77, 18, 98, 211, 1, 231, 3];
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
pub struct TicketAccountData {
    pub state_address: Pubkey,
    pub beneficiary: Pubkey,
    pub lamports_amount: u64,
    pub created_epoch: u64,
}
impl TicketAccountData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lamports_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state_address,
            beneficiary,
            lamports_amount,
            created_epoch,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lamports_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_epoch, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TicketAccountDataAccount(pub TicketAccountData);
impl TicketAccountDataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICKET_ACCOUNT_DATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TicketAccountData::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICKET_ACCOUNT_DATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
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
pub struct State {
    pub msol_mint: Pubkey,
    pub admin_authority: Pubkey,
    pub operational_sol_account: Pubkey,
    pub treasury_msol_account: Pubkey,
    pub reserve_bump_seed: u8,
    pub msol_mint_authority_bump_seed: u8,
    pub rent_exempt_for_token_acc: u64,
    pub reward_fee: Fee,
    pub stake_system: StakeSystem,
    pub validator_system: ValidatorSystem,
    pub liq_pool: LiqPool,
    pub available_reserve_balance: u64,
    pub msol_supply: u64,
    pub msol_price: u64,
    pub circulating_ticket_count: u64,
    pub circulating_ticket_balance: u64,
    pub lent_from_reserve: u64,
    pub min_deposit: u64,
    pub min_withdraw: u64,
    pub staking_sol_cap: u64,
    pub emergency_cooling_down: u64,
    pub pause_authority: Pubkey,
    pub paused: bool,
    pub delayed_unstake_fee: FeeCents,
    pub withdraw_stake_account_fee: FeeCents,
    pub withdraw_stake_account_enabled: bool,
    pub last_stake_move_epoch: u64,
    pub stake_moved: u64,
    pub max_stake_moved_per_epoch: Fee,
}
impl State {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let msol_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operational_sol_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury_msol_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let msol_mint_authority_bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let rent_exempt_for_token_acc: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let stake_system = if reader.is_empty() {
            Default::default()
        } else {
            <StakeSystem>::deserialize(&mut reader)?
        };
        let validator_system = if reader.is_empty() {
            Default::default()
        } else {
            <ValidatorSystem>::deserialize(&mut reader)?
        };
        let liq_pool = if reader.is_empty() {
            Default::default()
        } else {
            <LiqPool>::deserialize(&mut reader)?
        };
        let available_reserve_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let circulating_ticket_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let circulating_ticket_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lent_from_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_withdraw: u64 = crate::borsh_de_or_default(&mut reader)?;
        let staking_sol_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emergency_cooling_down: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pause_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let delayed_unstake_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCents>::deserialize(&mut reader)?
        };
        let withdraw_stake_account_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCents>::deserialize(&mut reader)?
        };
        let withdraw_stake_account_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_stake_move_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_moved: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_stake_moved_per_epoch = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            msol_mint,
            admin_authority,
            operational_sol_account,
            treasury_msol_account,
            reserve_bump_seed,
            msol_mint_authority_bump_seed,
            rent_exempt_for_token_acc,
            reward_fee,
            stake_system,
            validator_system,
            liq_pool,
            available_reserve_balance,
            msol_supply,
            msol_price,
            circulating_ticket_count,
            circulating_ticket_balance,
            lent_from_reserve,
            min_deposit,
            min_withdraw,
            staking_sol_cap,
            emergency_cooling_down,
            pause_authority,
            paused,
            delayed_unstake_fee,
            withdraw_stake_account_fee,
            withdraw_stake_account_enabled,
            last_stake_move_epoch,
            stake_moved,
            max_stake_moved_per_epoch,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.msol_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_sol_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_msol_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_bump_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.msol_mint_authority_bump_seed,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.rent_exempt_for_token_acc, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_system, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_system, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liq_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.available_reserve_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_ticket_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_ticket_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lent_from_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_withdraw, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staking_sol_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emergency_cooling_down, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pause_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delayed_unstake_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_stake_account_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.withdraw_stake_account_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_stake_move_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_moved, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_stake_moved_per_epoch, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
