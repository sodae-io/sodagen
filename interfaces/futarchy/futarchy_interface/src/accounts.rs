use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const AMM_POSITION_ACCOUNT_DISCM: [u8; 8] = [34, 97, 105, 74, 17, 226, 212, 0];
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
pub struct AmmPosition {
    pub dao: Pubkey,
    pub position_authority: Pubkey,
    pub liquidity: u128,
}
impl AmmPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dao,
            position_authority,
            liquidity,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmPositionAccount(pub AmmPosition);
impl AmmPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AmmPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DAO_ACCOUNT_DISCM: [u8; 8] = [163, 9, 47, 31, 52, 85, 197, 49];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Dao {
    pub amm: FutarchyAmm,
    pub nonce: u64,
    pub dao_creator: Pubkey,
    pub pda_bump: u8,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub proposal_count: u32,
    pub pass_threshold_bps: u16,
    pub seconds_per_proposal: u32,
    pub twap_initial_observation: u128,
    pub twap_max_observation_change_per_update: u128,
    pub twap_start_delay_seconds: u32,
    pub min_quote_futarchic_liquidity: u64,
    pub min_base_futarchic_liquidity: u64,
    pub base_to_stake: u64,
    pub seq_num: u64,
    pub initial_spending_limit: Option<InitialSpendingLimit>,
    pub team_sponsored_pass_threshold_bps: i16,
    pub team_address: Pubkey,
    pub optimistic_proposal: Option<OptimisticProposal>,
    pub is_optimistic_governance_enabled: bool,
}
impl Dao {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amm = <FutarchyAmm>::deserialize(&mut reader)?;
        let nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dao_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pda_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposal_count: u32 = crate::borsh_de_or_default(&mut reader)?;
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
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_spending_limit: Option<InitialSpendingLimit> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_sponsored_pass_threshold_bps: i16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let optimistic_proposal: Option<OptimisticProposal> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let is_optimistic_governance_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            amm,
            nonce,
            dao_creator,
            pda_bump,
            squads_multisig,
            squads_multisig_vault,
            base_mint,
            quote_mint,
            proposal_count,
            pass_threshold_bps,
            seconds_per_proposal,
            twap_initial_observation,
            twap_max_observation_change_per_update,
            twap_start_delay_seconds,
            min_quote_futarchic_liquidity,
            min_base_futarchic_liquidity,
            base_to_stake,
            seq_num,
            initial_spending_limit,
            team_sponsored_pass_threshold_bps,
            team_address,
            optimistic_proposal,
            is_optimistic_governance_enabled,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amm, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal_count, &mut writer)?;
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
        borsh::BorshSerialize::serialize(&self.seq_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_spending_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.team_sponsored_pass_threshold_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.team_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.optimistic_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_optimistic_governance_enabled,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DaoAccount(pub Dao);
impl DaoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DAO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Dao::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DAO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OLD_DAO_ACCOUNT_DISCM: [u8; 8] = [81, 235, 225, 156, 146, 147, 224, 129];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OldDao {
    pub amm: FutarchyAmm,
    pub nonce: u64,
    pub dao_creator: Pubkey,
    pub pda_bump: u8,
    pub squads_multisig: Pubkey,
    pub squads_multisig_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub proposal_count: u32,
    pub pass_threshold_bps: u16,
    pub seconds_per_proposal: u32,
    pub twap_initial_observation: u128,
    pub twap_max_observation_change_per_update: u128,
    pub twap_start_delay_seconds: u32,
    pub min_quote_futarchic_liquidity: u64,
    pub min_base_futarchic_liquidity: u64,
    pub base_to_stake: u64,
    pub seq_num: u64,
    pub initial_spending_limit: Option<InitialSpendingLimit>,
    pub team_sponsored_pass_threshold_bps: i16,
    pub team_address: Pubkey,
}
impl OldDao {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amm = <FutarchyAmm>::deserialize(&mut reader)?;
        let nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dao_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pda_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let squads_multisig_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposal_count: u32 = crate::borsh_de_or_default(&mut reader)?;
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
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_spending_limit: Option<InitialSpendingLimit> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_sponsored_pass_threshold_bps: i16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amm,
            nonce,
            dao_creator,
            pda_bump,
            squads_multisig,
            squads_multisig_vault,
            base_mint,
            quote_mint,
            proposal_count,
            pass_threshold_bps,
            seconds_per_proposal,
            twap_initial_observation,
            twap_max_observation_change_per_update,
            twap_start_delay_seconds,
            min_quote_futarchic_liquidity,
            min_base_futarchic_liquidity,
            base_to_stake,
            seq_num,
            initial_spending_limit,
            team_sponsored_pass_threshold_bps,
            team_address,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amm, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_multisig_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposal_count, &mut writer)?;
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
        borsh::BorshSerialize::serialize(&self.seq_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_spending_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.team_sponsored_pass_threshold_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.team_address, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OldDaoAccount(pub OldDao);
impl OldDaoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OLD_DAO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OldDao::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OLD_DAO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROPOSAL_ACCOUNT_DISCM: [u8; 8] = [26, 94, 189, 187, 116, 136, 53, 33];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Proposal {
    pub number: u32,
    pub proposer: Pubkey,
    pub timestamp_enqueued: i64,
    pub state: ProposalState,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub dao: Pubkey,
    pub pda_bump: u8,
    pub question: Pubkey,
    pub duration_in_seconds: u32,
    pub squads_proposal: Pubkey,
    pub pass_base_mint: Pubkey,
    pub pass_quote_mint: Pubkey,
    pub fail_base_mint: Pubkey,
    pub fail_quote_mint: Pubkey,
    pub is_team_sponsored: bool,
}
impl Proposal {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let number: u32 = crate::borsh_de_or_default(&mut reader)?;
        let proposer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let timestamp_enqueued: i64 = crate::borsh_de_or_default(&mut reader)?;
        let state = <ProposalState as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dao: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pda_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let question: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let duration_in_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        let squads_proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pass_base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pass_quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fail_base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fail_quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_team_sponsored: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            number,
            proposer,
            timestamp_enqueued,
            state,
            base_vault,
            quote_vault,
            dao,
            pda_bump,
            question,
            duration_in_seconds,
            squads_proposal,
            pass_base_mint,
            pass_quote_mint,
            fail_base_mint,
            fail_quote_mint,
            is_team_sponsored,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.number, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.timestamp_enqueued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.question, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.duration_in_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.squads_proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pass_base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pass_quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fail_base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fail_quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_team_sponsored, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalAccount(pub Proposal);
impl ProposalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSAL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Proposal::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKE_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [80, 158, 67, 124, 50, 189, 192, 255];
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
pub struct StakeAccount {
    pub proposal: Pubkey,
    pub staker: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
impl StakeAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let staker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            proposal,
            staker,
            amount,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.proposal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeAccountAccount(pub StakeAccount);
impl StakeAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StakeAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
