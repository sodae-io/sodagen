use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const COLLECT_FEES_EVENT_EVENT_DISCM: [u8; 8] = [226, 2, 53, 67, 132, 158, 125, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectFeesEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub base_token_account: Pubkey,
    pub quote_token_account: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint: Pubkey,
    pub quote_fees_collected: u64,
    pub base_fees_collected: u64,
    pub post_amm_state: FutarchyAmm,
}
impl CollectFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            base_token_account,
            quote_token_account,
            amm_base_vault,
            amm_quote_vault,
            quote_mint,
            base_mint,
            quote_fees_collected,
            base_fees_collected,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesEventEvent(pub CollectFeesEvent);
impl CollectFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_DAO_EVENT_EVENT_DISCM: [u8; 8] = [
    119, 48, 153, 116, 127, 37, 226, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeDaoEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pass_threshold_bps: u16,
    pub seconds_per_proposal: u32,
    pub twap_initial_observation: u128,
    pub twap_max_observation_change_per_update: u128,
    pub twap_start_delay_seconds: u32,
    pub min_quote_futarchic_liquidity: u64,
    pub min_base_futarchic_liquidity: u64,
    pub base_to_stake: u64,
    pub initial_spending_limit: Option<InitialSpendingLimit>,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub team_sponsored_pass_threshold_bps: i16,
    pub team_address: Pubkey,
}
impl InitializeDaoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pass_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_proposal: u32 = crate::borsh_de_or_default(&mut reader)?;
        let twap_initial_observation: u128 = crate::borsh_de_or_default(&mut reader)?;
        let twap_max_observation_change_per_update: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let twap_start_delay_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_futarchic_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_base_futarchic_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_to_stake: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_spending_limit: Option<InitialSpendingLimit> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let squads_multisig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let team_sponsored_pass_threshold_bps: i16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            base_mint,
            quote_mint,
            pass_threshold_bps,
            seconds_per_proposal,
            twap_initial_observation,
            twap_max_observation_change_per_update,
            twap_start_delay_seconds,
            min_quote_futarchic_liquidity,
            min_base_futarchic_liquidity,
            base_to_stake,
            initial_spending_limit,
            squads_multisig,
            squads_multisig_vault,
            team_sponsored_pass_threshold_bps,
            team_address,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pass_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seconds_per_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.twap_initial_observation, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.twap_max_observation_change_per_update,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.twap_start_delay_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_quote_futarchic_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.min_base_futarchic_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.base_to_stake, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_spending_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.team_sponsored_pass_threshold_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.team_address, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeDaoEventEvent(pub InitializeDaoEvent);
impl InitializeDaoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_DAO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeDaoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_DAO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_DAO_EVENT_EVENT_DISCM: [u8; 8] = [12, 58, 244, 224, 171, 25, 33, 56];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDaoEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub pass_threshold_bps: u16,
    pub seconds_per_proposal: u32,
    pub twap_initial_observation: u128,
    pub twap_max_observation_change_per_update: u128,
    pub twap_start_delay_seconds: u32,
    pub min_quote_futarchic_liquidity: u64,
    pub min_base_futarchic_liquidity: u64,
    pub base_to_stake: u64,
    pub team_sponsored_pass_threshold_bps: i16,
    pub team_address: Pubkey,
    pub is_optimistic_governance_enabled: bool,
}
impl UpdateDaoEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pass_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_proposal: u32 = crate::borsh_de_or_default(&mut reader)?;
        let twap_initial_observation: u128 = crate::borsh_de_or_default(&mut reader)?;
        let twap_max_observation_change_per_update: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let twap_start_delay_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_futarchic_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_base_futarchic_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_to_stake: u64 = crate::borsh_de_or_default(&mut reader)?;
        let team_sponsored_pass_threshold_bps: i16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_optimistic_governance_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            pass_threshold_bps,
            seconds_per_proposal,
            twap_initial_observation,
            twap_max_observation_change_per_update,
            twap_start_delay_seconds,
            min_quote_futarchic_liquidity,
            min_base_futarchic_liquidity,
            base_to_stake,
            team_sponsored_pass_threshold_bps,
            team_address,
            is_optimistic_governance_enabled,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pass_threshold_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seconds_per_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.twap_initial_observation, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.twap_max_observation_change_per_update,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.twap_start_delay_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_quote_futarchic_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.min_base_futarchic_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.base_to_stake, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.team_sponsored_pass_threshold_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.team_address, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_optimistic_governance_enabled,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDaoEventEvent(pub UpdateDaoEvent);
impl UpdateDaoEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DAO_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateDaoEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DAO_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    141, 56, 246, 192, 168, 254, 64, 111,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub question: Pubkey,
    pub quote_vault: Pubkey,
    pub base_vault: Pubkey,
    pub proposer: Pubkey,
    pub number: u32,
    pub pda_bump: u8,
    pub duration_in_seconds: u32,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
}
impl InitializeProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let question: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let number: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pda_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let duration_in_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        let squads_proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            dao,
            question,
            quote_vault,
            base_vault,
            proposer,
            number,
            pda_bump,
            duration_in_seconds,
            squads_proposal,
            squads_multisig,
            squads_multisig_vault,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.question, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.duration_in_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig_vault, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeProposalEventEvent(pub InitializeProposalEvent);
impl InitializeProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKE_TO_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    94, 69, 68, 151, 90, 149, 240, 191,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StakeToProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub staker: Pubkey,
    pub amount: u64,
    pub total_staked: u64,
}
impl StakeToProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let staker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_staked: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            staker,
            amount,
            total_staked,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_staked, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeToProposalEventEvent(pub StakeToProposalEvent);
impl StakeToProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_TO_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = StakeToProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_TO_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UNSTAKE_FROM_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    89, 129, 63, 211, 55, 33, 194, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnstakeFromProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub staker: Pubkey,
    pub amount: u64,
    pub total_staked: u64,
}
impl UnstakeFromProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let staker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_staked: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            staker,
            amount,
            total_staked,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_staked, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeFromProposalEventEvent(pub UnstakeFromProposalEvent);
impl UnstakeFromProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNSTAKE_FROM_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UnstakeFromProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_FROM_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LAUNCH_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    177, 55, 191, 93, 203, 124, 184, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LaunchProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub timestamp_enqueued: i64,
    pub total_staked: u64,
    pub post_amm_state: FutarchyAmm,
}
impl LaunchProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp_enqueued: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_staked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            dao,
            timestamp_enqueued,
            total_staked,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp_enqueued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_staked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LaunchProposalEventEvent(pub LaunchProposalEvent);
impl LaunchProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LAUNCH_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LaunchProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LAUNCH_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FINALIZE_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    45, 29, 122, 181, 79, 224, 57, 141,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinalizeProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub pass_market_twap: u128,
    pub fail_market_twap: u128,
    pub threshold: u128,
    pub state: ProposalState,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub post_amm_state: FutarchyAmm,
    pub is_team_sponsored: bool,
}
impl FinalizeProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pass_market_twap: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fail_market_twap: u128 = crate::borsh_de_or_default(&mut reader)?;
        let threshold: u128 = crate::borsh_de_or_default(&mut reader)?;
        let state = <ProposalState as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let squads_proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        let is_team_sponsored: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            dao,
            pass_market_twap,
            fail_market_twap,
            threshold,
            state,
            squads_proposal,
            squads_multisig,
            post_amm_state,
            is_team_sponsored,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pass_market_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fail_market_twap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_team_sponsored, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FinalizeProposalEventEvent(pub FinalizeProposalEvent);
impl FinalizeProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FINALIZE_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FinalizeProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FINALIZE_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SPOT_SWAP_EVENT_EVENT_DISCM: [u8; 8] = [29, 253, 121, 255, 82, 60, 148, 195];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpotSwapEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub user: Pubkey,
    pub swap_type: SwapType,
    pub input_amount: u64,
    pub output_amount: u64,
    pub min_output_amount: u64,
    pub post_amm_state: FutarchyAmm,
}
impl SpotSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            user,
            swap_type,
            input_amount,
            output_amount,
            min_output_amount,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotSwapEventEvent(pub SpotSwapEvent);
impl SpotSwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPOT_SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SpotSwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPOT_SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONDITIONAL_SWAP_EVENT_EVENT_DISCM: [u8; 8] = [
    2, 166, 200, 160, 94, 212, 68, 45,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConditionalSwapEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub proposal: Pubkey,
    pub trader: Pubkey,
    pub market: Market,
    pub swap_type: SwapType,
    pub input_amount: u64,
    pub output_amount: u64,
    pub min_output_amount: u64,
    pub post_amm_state: FutarchyAmm,
}
impl ConditionalSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Market = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            proposal,
            trader,
            market,
            swap_type,
            input_amount,
            output_amount,
            min_output_amount,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trader, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalSwapEventEvent(pub ConditionalSwapEvent);
impl ConditionalSwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONDITIONAL_SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConditionalSwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONDITIONAL_SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROVIDE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    38, 2, 37, 238, 229, 214, 255, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvideLiquidityEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub liquidity_provider: Pubkey,
    pub position_authority: Pubkey,
    pub quote_amount: u64,
    pub base_amount: u64,
    pub liquidity_minted: u128,
    pub min_liquidity: u128,
    pub post_amm_state: FutarchyAmm,
}
impl ProvideLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_provider: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_minted: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            liquidity_provider,
            position_authority,
            quote_amount,
            base_amount,
            liquidity_minted,
            min_liquidity,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_provider, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProvideLiquidityEventEvent(pub ProvideLiquidityEvent);
impl ProvideLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROVIDE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ProvideLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROVIDE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    214, 6, 161, 45, 191, 142, 124, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawLiquidityEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub liquidity_provider: Pubkey,
    pub liquidity_withdrawn: u128,
    pub min_base_amount: u64,
    pub min_quote_amount: u64,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub post_amm_state: FutarchyAmm,
}
impl WithdrawLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_provider: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_withdrawn: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            liquidity_provider,
            liquidity_withdrawn,
            min_base_amount,
            min_quote_amount,
            base_amount,
            quote_amount,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_provider, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityEventEvent(pub WithdrawLiquidityEvent);
impl WithdrawLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SPONSOR_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    202, 190, 250, 176, 79, 235, 77, 194,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SponsorProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub team_address: Pubkey,
}
impl SponsorProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let team_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            dao,
            team_address,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.team_address, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SponsorProposalEventEvent(pub SponsorProposalEvent);
impl SponsorProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPONSOR_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SponsorProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPONSOR_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    135, 152, 254, 224, 208, 81, 197, 215,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub admin: Pubkey,
}
impl RemoveProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            dao,
            admin,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveProposalEventEvent(pub RemoveProposalEvent);
impl RemoveProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADMIN_CANCEL_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    118, 2, 87, 75, 91, 217, 71, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminCancelProposalEvent {
    pub common: CommonFields,
    pub proposal: Pubkey,
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub post_amm_state: FutarchyAmm,
}
impl AdminCancelProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let post_amm_state = <FutarchyAmm>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            proposal,
            dao,
            admin,
            post_amm_state,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_amm_state, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminCancelProposalEventEvent(pub AdminCancelProposalEvent);
impl AdminCancelProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_CANCEL_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminCancelProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_CANCEL_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_METEORA_DAMM_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    193, 114, 225, 241, 209, 213, 175, 85,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectMeteoraDammFeesEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub pool: Pubkey,
    pub base_token_account: Pubkey,
    pub quote_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint: Pubkey,
    pub quote_fees_collected: u64,
    pub base_fees_collected: u64,
}
impl CollectMeteoraDammFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            pool,
            base_token_account,
            quote_token_account,
            quote_mint,
            base_mint,
            quote_fees_collected,
            base_fees_collected,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fees_collected, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectMeteoraDammFeesEventEvent(pub CollectMeteoraDammFeesEvent);
impl CollectMeteoraDammFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_METEORA_DAMM_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectMeteoraDammFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_METEORA_DAMM_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADMIN_FIX_POSITION_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [
    174, 79, 62, 1, 150, 182, 236, 147,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminFixPositionAuthorityEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub admin: Pubkey,
    pub amm_position: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}
impl AdminFixPositionAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            admin,
            amm_position,
            old_authority,
            new_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminFixPositionAuthorityEventEvent(pub AdminFixPositionAuthorityEvent);
impl AdminFixPositionAuthorityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_FIX_POSITION_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdminFixPositionAuthorityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_FIX_POSITION_AUTHORITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    224, 194, 233, 205, 198, 135, 5, 90,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitiateVaultSpendOptimisticProposalEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub proposer: Pubkey,
    pub squads_proposal: Pubkey,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub amount: u64,
    pub recipient: Pubkey,
    pub dao_quote_vault_account: Pubkey,
    pub recipient_quote_account: Pubkey,
    pub enqueued_timestamp: i64,
}
impl InitiateVaultSpendOptimisticProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao_quote_vault_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_quote_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enqueued_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            proposer,
            squads_proposal,
            squads_multisig,
            squads_multisig_vault,
            amount,
            recipient,
            dao_quote_vault_account,
            recipient_quote_account,
            enqueued_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao_quote_vault_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_quote_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.enqueued_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitiateVaultSpendOptimisticProposalEventEvent(
    pub InitiateVaultSpendOptimisticProposalEvent,
);
impl InitiateVaultSpendOptimisticProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitiateVaultSpendOptimisticProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIATE_VAULT_SPEND_OPTIMISTIC_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FINALIZE_OPTIMISTIC_PROPOSAL_EVENT_EVENT_DISCM: [u8; 8] = [
    64, 171, 35, 255, 228, 201, 18, 97,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinalizeOptimisticProposalEvent {
    pub common: CommonFields,
    pub dao: Pubkey,
    pub squads_proposal: Pubkey,
}
impl FinalizeOptimisticProposalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let common = if reader.is_empty() {
            Default::default()
        } else {
            <CommonFields>::deserialize(&mut reader)?
        };
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            common,
            dao,
            squads_proposal,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.common, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_proposal, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FinalizeOptimisticProposalEventEvent(pub FinalizeOptimisticProposalEvent);
impl FinalizeOptimisticProposalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FINALIZE_OPTIMISTIC_PROPOSAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FinalizeOptimisticProposalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FINALIZE_OPTIMISTIC_PROPOSAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
