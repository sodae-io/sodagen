use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CHANGE_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    228, 111, 35, 24, 187, 78, 224, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeAuthorityEvent {
    pub state: Pubkey,
    pub admin_change: Option<PubkeyValueChange>,
    pub validator_manager_change: Option<PubkeyValueChange>,
    pub operational_sol_account_change: Option<PubkeyValueChange>,
    pub treasury_msol_account_change: Option<PubkeyValueChange>,
    pub pause_authority_change: Option<PubkeyValueChange>,
}
impl ChangeAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_change: Option<PubkeyValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let validator_manager_change: Option<PubkeyValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let operational_sol_account_change: Option<PubkeyValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let treasury_msol_account_change: Option<PubkeyValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pause_authority_change: Option<PubkeyValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            state,
            admin_change,
            validator_manager_change,
            operational_sol_account_change,
            treasury_msol_account_change,
            pause_authority_change,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_manager_change, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.operational_sol_account_change,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.treasury_msol_account_change,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pause_authority_change, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeAuthorityEventEvent(pub ChangeAuthorityEvent);
impl ChangeAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ChangeAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_LP_EVENT_EVENT_DISCM: [u8; 8] = [159, 204, 192, 138, 68, 145, 224, 148];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigLpEvent {
    pub state: Pubkey,
    pub min_fee_change: Option<FeeValueChange>,
    pub max_fee_change: Option<FeeValueChange>,
    pub liquidity_target_change: Option<U64ValueChange>,
    pub treasury_cut_change: Option<FeeValueChange>,
}
impl ConfigLpEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let min_fee_change: Option<FeeValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_fee_change: Option<FeeValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidity_target_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let treasury_cut_change: Option<FeeValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            state,
            min_fee_change,
            max_fee_change,
            liquidity_target_change,
            treasury_cut_change,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_fee_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_fee_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_target_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_cut_change, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigLpEventEvent(pub ConfigLpEvent);
impl ConfigLpEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_LP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigLpEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_LP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_MARINADE_EVENT_EVENT_DISCM: [u8; 8] = [
    159, 164, 245, 114, 94, 253, 3, 9,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigMarinadeEvent {
    pub state: Pubkey,
    pub rewards_fee_change: Option<FeeValueChange>,
    pub slots_for_stake_delta_change: Option<U64ValueChange>,
    pub min_stake_change: Option<U64ValueChange>,
    pub min_deposit_change: Option<U64ValueChange>,
    pub min_withdraw_change: Option<U64ValueChange>,
    pub staking_sol_cap_change: Option<U64ValueChange>,
    pub liquidity_sol_cap_change: Option<U64ValueChange>,
    pub withdraw_stake_account_enabled_change: Option<BoolValueChange>,
    pub delayed_unstake_fee_change: Option<FeeCentsValueChange>,
    pub withdraw_stake_account_fee_change: Option<FeeCentsValueChange>,
    pub max_stake_moved_per_epoch_change: Option<FeeValueChange>,
}
impl ConfigMarinadeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rewards_fee_change: Option<FeeValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let slots_for_stake_delta_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_stake_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_deposit_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_withdraw_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let staking_sol_cap_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidity_sol_cap_change: Option<U64ValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdraw_stake_account_enabled_change: Option<BoolValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let delayed_unstake_fee_change: Option<FeeCentsValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdraw_stake_account_fee_change: Option<FeeCentsValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_stake_moved_per_epoch_change: Option<FeeValueChange> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            state,
            rewards_fee_change,
            slots_for_stake_delta_change,
            min_stake_change,
            min_deposit_change,
            min_withdraw_change,
            staking_sol_cap_change,
            liquidity_sol_cap_change,
            withdraw_stake_account_enabled_change,
            delayed_unstake_fee_change,
            withdraw_stake_account_fee_change,
            max_stake_moved_per_epoch_change,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_fee_change, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.slots_for_stake_delta_change,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_stake_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_deposit_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_withdraw_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staking_sol_cap_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_sol_cap_change, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.withdraw_stake_account_enabled_change,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.delayed_unstake_fee_change, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.withdraw_stake_account_fee_change,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_stake_moved_per_epoch_change,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigMarinadeEventEvent(pub ConfigMarinadeEvent);
impl ConfigMarinadeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_MARINADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigMarinadeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_MARINADE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_EVENT_EVENT_DISCM: [u8; 8] = [
    206, 175, 169, 208, 241, 210, 35, 221,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeEvent {
    pub state: Pubkey,
    pub params: InitializeData,
    pub stake_list: Pubkey,
    pub validator_list: Pubkey,
    pub msol_mint: Pubkey,
    pub operational_sol_account: Pubkey,
    pub lp_mint: Pubkey,
    pub lp_msol_leg: Pubkey,
    pub treasury_msol_account: Pubkey,
}
impl InitializeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeData>::deserialize(&mut reader)?
        };
        let stake_list: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator_list: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let msol_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operational_sol_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_msol_leg: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury_msol_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            params,
            stake_list,
            validator_list,
            msol_mint,
            operational_sol_account,
            lp_mint,
            lp_msol_leg,
            treasury_msol_account,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_list, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_list, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_sol_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_msol_leg, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_msol_account, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeEventEvent(pub InitializeEvent);
impl InitializeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EMERGENCY_PAUSE_EVENT_EVENT_DISCM: [u8; 8] = [
    159, 241, 192, 232, 29, 208, 51, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmergencyPauseEvent {
    pub state: Pubkey,
}
impl EmergencyPauseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { state })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EmergencyPauseEventEvent(pub EmergencyPauseEvent);
impl EmergencyPauseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EMERGENCY_PAUSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EmergencyPauseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EMERGENCY_PAUSE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RESUME_EVENT_EVENT_DISCM: [u8; 8] = [97, 117, 183, 115, 117, 224, 8, 229];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResumeEvent {
    pub state: Pubkey,
}
impl ResumeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { state })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeEventEvent(pub ResumeEvent);
impl ResumeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESUME_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ResumeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESUME_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM: [u8; 8] = [
    70, 191, 242, 164, 56, 156, 130, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReallocValidatorListEvent {
    pub state: Pubkey,
    pub count: u32,
    pub new_capacity: u32,
}
impl ReallocValidatorListEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let count: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_capacity: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { state, count, new_capacity })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_capacity, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReallocValidatorListEventEvent(pub ReallocValidatorListEvent);
impl ReallocValidatorListEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReallocValidatorListEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REALLOC_STAKE_LIST_EVENT_EVENT_DISCM: [u8; 8] = [
    193, 129, 16, 243, 177, 131, 248, 23,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReallocStakeListEvent {
    pub state: Pubkey,
    pub count: u32,
    pub new_capacity: u32,
}
impl ReallocStakeListEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let count: u32 = crate::borsh_de_or_default(&mut reader)?;
        let new_capacity: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { state, count, new_capacity })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_capacity, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReallocStakeListEventEvent(pub ReallocStakeListEvent);
impl ReallocStakeListEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REALLOC_STAKE_LIST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ReallocStakeListEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REALLOC_STAKE_LIST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEACTIVATE_STAKE_EVENT_EVENT_DISCM: [u8; 8] = [
    2, 54, 184, 218, 78, 181, 163, 117,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeactivateStakeEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: Pubkey,
    pub last_update_stake_delegation: u64,
    pub split_stake_account: Option<SplitStakeAccountInfo>,
    pub validator_index: u32,
    pub validator_vote: Pubkey,
    pub total_stake_target: u64,
    pub validator_stake_target: u64,
    pub total_active_balance: u64,
    pub delayed_unstake_cooling_down: u64,
    pub validator_active_balance: u64,
    pub total_unstake_delta: u64,
    pub unstaked_amount: u64,
}
impl DeactivateStakeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update_stake_delegation: u64 = crate::borsh_de_or_default(&mut reader)?;
        let split_stake_account: Option<SplitStakeAccountInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator_vote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_stake_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let validator_stake_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let delayed_unstake_cooling_down: u64 = crate::borsh_de_or_default(&mut reader)?;
        let validator_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_unstake_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unstaked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            stake_index,
            stake_account,
            last_update_stake_delegation,
            split_stake_account,
            validator_index,
            validator_vote,
            total_stake_target,
            validator_stake_target,
            total_active_balance,
            delayed_unstake_cooling_down,
            validator_active_balance,
            total_unstake_delta,
            unstaked_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_update_stake_delegation,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.split_stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_vote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_stake_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_stake_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.delayed_unstake_cooling_down,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.validator_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_unstake_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unstaked_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeactivateStakeEventEvent(pub DeactivateStakeEvent);
impl DeactivateStakeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEACTIVATE_STAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeactivateStakeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEACTIVATE_STAKE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MERGE_STAKES_EVENT_EVENT_DISCM: [u8; 8] = [73, 156, 69, 233, 32, 14, 150, 65];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MergeStakesEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub destination_stake_index: u32,
    pub destination_stake_account: Pubkey,
    pub last_update_destination_stake_delegation: u64,
    pub source_stake_index: u32,
    pub source_stake_account: Pubkey,
    pub last_update_source_stake_delegation: u64,
    pub validator_index: u32,
    pub validator_vote: Pubkey,
    pub extra_delegated: u64,
    pub returned_stake_rent: u64,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub operational_sol_balance: u64,
}
impl MergeStakesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let destination_stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let destination_stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update_destination_stake_delegation: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let source_stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let source_stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update_source_stake_delegation: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator_vote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let extra_delegated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let returned_stake_rent: u64 = crate::borsh_de_or_default(&mut reader)?;
        let validator_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let operational_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            destination_stake_index,
            destination_stake_account,
            last_update_destination_stake_delegation,
            source_stake_index,
            source_stake_account,
            last_update_source_stake_delegation,
            validator_index,
            validator_vote,
            extra_delegated,
            returned_stake_rent,
            validator_active_balance,
            total_active_balance,
            operational_sol_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.destination_stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.destination_stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_update_destination_stake_delegation,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.source_stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source_stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_update_source_stake_delegation,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_vote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.extra_delegated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.returned_stake_rent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_sol_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MergeStakesEventEvent(pub MergeStakesEvent);
impl MergeStakesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERGE_STAKES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MergeStakesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERGE_STAKES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDELEGATE_EVENT_EVENT_DISCM: [u8; 8] = [241, 75, 135, 173, 204, 215, 72, 67];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedelegateEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: Pubkey,
    pub last_update_delegation: u64,
    pub source_validator_index: u32,
    pub source_validator_vote: Pubkey,
    pub source_validator_score: u32,
    pub source_validator_balance: u64,
    pub source_validator_stake_target: u64,
    pub dest_validator_index: u32,
    pub dest_validator_vote: Pubkey,
    pub dest_validator_score: u32,
    pub dest_validator_balance: u64,
    pub dest_validator_stake_target: u64,
    pub redelegate_amount: u64,
    pub split_stake_account: Option<SplitStakeAccountInfo>,
    pub redelegate_stake_index: u32,
    pub redelegate_stake_account: Pubkey,
}
impl RedelegateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update_delegation: u64 = crate::borsh_de_or_default(&mut reader)?;
        let source_validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let source_validator_vote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let source_validator_score: u32 = crate::borsh_de_or_default(&mut reader)?;
        let source_validator_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let source_validator_stake_target: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let dest_validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let dest_validator_vote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dest_validator_score: u32 = crate::borsh_de_or_default(&mut reader)?;
        let dest_validator_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dest_validator_stake_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let redelegate_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let split_stake_account: Option<SplitStakeAccountInfo> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let redelegate_stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let redelegate_stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            stake_index,
            stake_account,
            last_update_delegation,
            source_validator_index,
            source_validator_vote,
            source_validator_score,
            source_validator_balance,
            source_validator_stake_target,
            dest_validator_index,
            dest_validator_vote,
            dest_validator_score,
            dest_validator_balance,
            dest_validator_stake_target,
            redelegate_amount,
            split_stake_account,
            redelegate_stake_index,
            redelegate_stake_account,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_delegation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source_validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source_validator_vote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source_validator_score, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source_validator_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.source_validator_stake_target,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.dest_validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dest_validator_vote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dest_validator_score, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dest_validator_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.dest_validator_stake_target,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.redelegate_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.split_stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redelegate_stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redelegate_stake_account, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedelegateEventEvent(pub RedelegateEvent);
impl RedelegateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDELEGATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedelegateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDELEGATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKE_RESERVE_EVENT_EVENT_DISCM: [u8; 8] = [
    112, 117, 149, 185, 77, 119, 190, 106,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StakeReserveEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: Pubkey,
    pub validator_index: u32,
    pub validator_vote: Pubkey,
    pub total_stake_target: u64,
    pub validator_stake_target: u64,
    pub reserve_balance: u64,
    pub total_active_balance: u64,
    pub validator_active_balance: u64,
    pub total_stake_delta: u64,
    pub amount: u64,
}
impl StakeReserveEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator_vote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_stake_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let validator_stake_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let validator_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_stake_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            stake_index,
            stake_account,
            validator_index,
            validator_vote,
            total_stake_target,
            validator_stake_target,
            reserve_balance,
            total_active_balance,
            validator_active_balance,
            total_stake_delta,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_vote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_stake_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_stake_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_stake_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeReserveEventEvent(pub StakeReserveEvent);
impl StakeReserveEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_RESERVE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = StakeReserveEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_RESERVE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_ACTIVE_EVENT_EVENT_DISCM: [u8; 8] = [
    251, 18, 128, 75, 208, 80, 174, 140,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateActiveEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: Pubkey,
    pub validator_index: u32,
    pub validator_vote: Pubkey,
    pub delegation_change: U64ValueChange,
    pub delegation_growth_msol_fees: Option<u64>,
    pub extra_lamports: u64,
    pub extra_msol_fees: Option<u64>,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub msol_price_change: U64ValueChange,
    pub reward_fee_used: Fee,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl UpdateActiveEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator_vote: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegation_change = if reader.is_empty() {
            Default::default()
        } else {
            <U64ValueChange>::deserialize(&mut reader)?
        };
        let delegation_growth_msol_fees: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let extra_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let extra_msol_fees: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let validator_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_price_change = if reader.is_empty() {
            Default::default()
        } else {
            <U64ValueChange>::deserialize(&mut reader)?
        };
        let reward_fee_used = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            stake_index,
            stake_account,
            validator_index,
            validator_vote,
            delegation_change,
            delegation_growth_msol_fees,
            extra_lamports,
            extra_msol_fees,
            validator_active_balance,
            total_active_balance,
            msol_price_change,
            reward_fee_used,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_vote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegation_change, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.delegation_growth_msol_fees,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.extra_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.extra_msol_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_price_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_fee_used, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateActiveEventEvent(pub UpdateActiveEvent);
impl UpdateActiveEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ACTIVE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateActiveEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ACTIVE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_DEACTIVATED_EVENT_EVENT_DISCM: [u8; 8] = [
    252, 159, 177, 147, 182, 113, 186, 94,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDeactivatedEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: Pubkey,
    pub balance_without_rent_exempt: u64,
    pub last_update_delegated_lamports: u64,
    pub msol_fees: Option<u64>,
    pub msol_price_change: U64ValueChange,
    pub reward_fee_used: Fee,
    pub operational_sol_balance: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl UpdateDeactivatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let stake_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let balance_without_rent_exempt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_delegated_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_fees: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let msol_price_change = if reader.is_empty() {
            Default::default()
        } else {
            <U64ValueChange>::deserialize(&mut reader)?
        };
        let reward_fee_used = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let operational_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            stake_index,
            stake_account,
            balance_without_rent_exempt,
            last_update_delegated_lamports,
            msol_fees,
            msol_price_change,
            reward_fee_used,
            operational_sol_balance,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_account, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.balance_without_rent_exempt,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.last_update_delegated_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_price_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_fee_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_sol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDeactivatedEventEvent(pub UpdateDeactivatedEvent);
impl UpdateDeactivatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DEACTIVATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateDeactivatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DEACTIVATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_EVENT_EVENT_DISCM: [u8; 8] = [93, 15, 70, 170, 48, 140, 212, 219];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub ticket: Pubkey,
    pub beneficiary: Pubkey,
    pub circulating_ticket_balance: u64,
    pub circulating_ticket_count: u64,
    pub reserve_balance: u64,
    pub user_balance: u64,
    pub amount: u64,
}
impl ClaimEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ticket: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let circulating_ticket_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let circulating_ticket_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            ticket,
            beneficiary,
            circulating_ticket_balance,
            circulating_ticket_count,
            reserve_balance,
            user_balance,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticket, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_ticket_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_ticket_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimEventEvent(pub ClaimEvent);
impl ClaimEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORDER_UNSTAKE_EVENT_EVENT_DISCM: [u8; 8] = [
    228, 63, 155, 249, 132, 160, 135, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderUnstakeEvent {
    pub state: Pubkey,
    pub ticket_epoch: u64,
    pub ticket: Pubkey,
    pub beneficiary: Pubkey,
    pub circulating_ticket_balance: u64,
    pub circulating_ticket_count: u64,
    pub user_msol_balance: u64,
    pub burned_msol_amount: u64,
    pub sol_amount: u64,
    pub fee_bp_cents: u32,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl OrderUnstakeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ticket_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ticket: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let circulating_ticket_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let circulating_ticket_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burned_msol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bp_cents: u32 = crate::borsh_de_or_default(&mut reader)?;
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            ticket_epoch,
            ticket,
            beneficiary,
            circulating_ticket_balance,
            circulating_ticket_count,
            user_msol_balance,
            burned_msol_amount,
            sol_amount,
            fee_bp_cents,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticket_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticket, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_ticket_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_ticket_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.burned_msol_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bp_cents, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderUnstakeEventEvent(pub OrderUnstakeEvent);
impl OrderUnstakeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORDER_UNSTAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OrderUnstakeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORDER_UNSTAKE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADD_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    27, 178, 153, 186, 47, 196, 140, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityEvent {
    pub state: Pubkey,
    pub sol_owner: Pubkey,
    pub user_sol_balance: u64,
    pub user_lp_balance: u64,
    pub sol_leg_balance: u64,
    pub lp_supply: u64,
    pub sol_added_amount: u64,
    pub lp_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl AddLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sol_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_leg_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_added_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            sol_owner,
            user_sol_balance,
            user_lp_balance,
            sol_leg_balance,
            lp_supply,
            sol_added_amount,
            lp_minted,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_sol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_lp_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_leg_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_added_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityEventEvent(pub AddLiquidityEvent);
impl AddLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AddLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUID_UNSTAKE_EVENT_EVENT_DISCM: [u8; 8] = [173, 5, 147, 15, 5, 14, 194, 116];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidUnstakeEvent {
    pub state: Pubkey,
    pub msol_owner: Pubkey,
    pub liq_pool_sol_balance: u64,
    pub liq_pool_msol_balance: u64,
    pub treasury_msol_balance: Option<u64>,
    pub user_msol_balance: u64,
    pub user_sol_balance: u64,
    pub msol_amount: u64,
    pub msol_fee: u64,
    pub treasury_msol_cut: u64,
    pub sol_amount: u64,
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,
}
impl LiquidUnstakeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let msol_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liq_pool_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liq_pool_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_msol_balance: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let user_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_msol_cut: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
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
        *__buf = reader;
        Ok(Self {
            state,
            msol_owner,
            liq_pool_sol_balance,
            liq_pool_msol_balance,
            treasury_msol_balance,
            user_msol_balance,
            user_sol_balance,
            msol_amount,
            msol_fee,
            treasury_msol_cut,
            sol_amount,
            lp_liquidity_target,
            lp_max_fee,
            lp_min_fee,
            treasury_cut,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liq_pool_sol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liq_pool_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_sol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_msol_cut, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_liquidity_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_max_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_min_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_cut, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeEventEvent(pub LiquidUnstakeEvent);
impl LiquidUnstakeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_UNSTAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidUnstakeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    141, 199, 182, 123, 159, 94, 215, 102,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityEvent {
    pub state: Pubkey,
    pub sol_leg_balance: u64,
    pub msol_leg_balance: u64,
    pub user_lp_balance: u64,
    pub user_sol_balance: u64,
    pub user_msol_balance: u64,
    pub lp_mint_supply: u64,
    pub lp_burned: u64,
    pub sol_out_amount: u64,
    pub msol_out_amount: u64,
}
impl RemoveLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sol_leg_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_leg_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            sol_leg_balance,
            msol_leg_balance,
            user_lp_balance,
            user_sol_balance,
            user_msol_balance,
            lp_mint_supply,
            lp_burned,
            sol_out_amount,
            msol_out_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_leg_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_leg_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_lp_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_sol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_out_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityEventEvent(pub RemoveLiquidityEvent);
impl RemoveLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADD_VALIDATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    190, 231, 170, 244, 14, 227, 129, 66,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddValidatorEvent {
    pub state: Pubkey,
    pub validator: Pubkey,
    pub index: u32,
    pub score: u32,
}
impl AddValidatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let score: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            validator,
            index,
            score,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.score, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddValidatorEventEvent(pub AddValidatorEvent);
impl AddValidatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_VALIDATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AddValidatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_VALIDATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_VALIDATOR_EVENT_EVENT_DISCM: [u8; 8] = [
    67, 164, 190, 192, 156, 156, 168, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveValidatorEvent {
    pub state: Pubkey,
    pub validator: Pubkey,
    pub index: u32,
    pub operational_sol_balance: u64,
}
impl RemoveValidatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let operational_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            validator,
            index,
            operational_sol_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_sol_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveValidatorEventEvent(pub RemoveValidatorEvent);
impl RemoveValidatorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_VALIDATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveValidatorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_VALIDATOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM: [u8; 8] = [
    58, 53, 237, 178, 238, 153, 85, 156,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetValidatorScoreEvent {
    pub state: Pubkey,
    pub validator: Pubkey,
    pub index: u32,
    pub score_change: U32ValueChange,
}
impl SetValidatorScoreEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let score_change = if reader.is_empty() {
            Default::default()
        } else {
            <U32ValueChange>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            state,
            validator,
            index,
            score_change,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.score_change, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetValidatorScoreEventEvent(pub SetValidatorScoreEvent);
impl SetValidatorScoreEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetValidatorScoreEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [
    231, 203, 118, 96, 75, 116, 70, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositStakeAccountEvent {
    pub state: Pubkey,
    pub stake: Pubkey,
    pub delegated: u64,
    pub withdrawer: Pubkey,
    pub stake_index: u32,
    pub validator: Pubkey,
    pub validator_index: u32,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub user_msol_balance: u64,
    pub msol_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl DepositStakeAccountEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            stake,
            delegated,
            withdrawer,
            stake_index,
            validator,
            validator_index,
            validator_active_balance,
            total_active_balance,
            user_msol_balance,
            msol_minted,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_active_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStakeAccountEventEvent(pub DepositStakeAccountEvent);
impl DepositStakeAccountEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositStakeAccountEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositEvent {
    pub state: Pubkey,
    pub sol_owner: Pubkey,
    pub user_sol_balance: u64,
    pub user_msol_balance: u64,
    pub sol_leg_balance: u64,
    pub msol_leg_balance: u64,
    pub reserve_balance: u64,
    pub sol_swapped: u64,
    pub msol_swapped: u64,
    pub sol_deposited: u64,
    pub msol_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl DepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sol_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_sol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_leg_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_leg_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_swapped: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_swapped: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            sol_owner,
            user_sol_balance,
            user_msol_balance,
            sol_leg_balance,
            msol_leg_balance,
            reserve_balance,
            sol_swapped,
            msol_swapped,
            sol_deposited,
            msol_minted,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_sol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_leg_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_leg_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_swapped, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_swapped, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositEventEvent(pub DepositEvent);
impl DepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [
    131, 238, 39, 48, 30, 27, 165, 28,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawStakeAccountEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake: Pubkey,
    pub last_update_stake_delegation: u64,
    pub stake_index: u32,
    pub validator: Pubkey,
    pub validator_index: u32,
    pub user_msol_balance: u64,
    pub user_msol_auth: Pubkey,
    pub msol_burned: u64,
    pub msol_fees: u64,
    pub split_stake: Pubkey,
    pub beneficiary: Pubkey,
    pub split_lamports: u64,
    pub fee_bp_cents: u32,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
impl WithdrawStakeAccountEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update_stake_delegation: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let validator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let validator_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_msol_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_msol_auth: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let msol_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let msol_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let split_stake: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let split_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bp_cents: u32 = crate::borsh_de_or_default(&mut reader)?;
        let total_virtual_staked_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let msol_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            epoch,
            stake,
            last_update_stake_delegation,
            stake_index,
            validator,
            validator_index,
            user_msol_balance,
            user_msol_auth,
            msol_burned,
            msol_fees,
            split_stake,
            beneficiary,
            split_lamports,
            fee_bp_cents,
            total_virtual_staked_lamports,
            msol_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_update_stake_delegation,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.stake_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.validator_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_msol_auth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.msol_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.split_stake, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.split_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bp_cents, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_virtual_staked_lamports,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.msol_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStakeAccountEventEvent(pub WithdrawStakeAccountEvent);
impl WithdrawStakeAccountEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawStakeAccountEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
