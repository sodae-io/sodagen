use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ASYNC_DEPLOYMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    79, 203, 41, 22, 215, 191, 148, 37,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct AsyncDeploymentConfig {
    pub bump: u8,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 160],
}
impl AsyncDeploymentConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            strategy_type,
            target_key,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncDeploymentConfigAccount(pub AsyncDeploymentConfig);
impl AsyncDeploymentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ASYNC_DEPLOYMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AsyncDeploymentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ASYNC_DEPLOYMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ASYNC_DEPLOYMENT_STATE_ACCOUNT_DISCM: [u8; 8] = [
    208, 11, 72, 206, 27, 76, 193, 231,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct AsyncDeploymentState {
    pub bump: u8,
    pub cumulative_amount_deployed: u128,
    pub cumulative_amount_paid_back: u128,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 160],
}
impl AsyncDeploymentState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_amount_deployed: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_amount_paid_back: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            cumulative_amount_deployed,
            cumulative_amount_paid_back,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cumulative_amount_deployed, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_amount_paid_back,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncDeploymentStateAccount(pub AsyncDeploymentState);
impl AsyncDeploymentStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ASYNC_DEPLOYMENT_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AsyncDeploymentState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ASYNC_DEPLOYMENT_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPLOYMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [13, 112, 57, 81, 43, 26, 156, 18];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DeploymentConfig {
    pub bump: u8,
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 160],
}
impl DeploymentConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            strategy_type,
            target_key,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeploymentConfigAccount(pub DeploymentConfig);
impl DeploymentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOYMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DeploymentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOYMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPLOYMENT_STATE_ACCOUNT_DISCM: [u8; 8] = [
    212, 136, 79, 121, 16, 38, 112, 116,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DeploymentState {
    pub bump: u8,
    pub cumulative_amount_deployed: u128,
    pub cumulative_amount_paid_back: u128,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 160],
}
impl DeploymentState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_amount_deployed: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_amount_paid_back: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            cumulative_amount_deployed,
            cumulative_amount_paid_back,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cumulative_amount_deployed, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_amount_paid_back,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeploymentStateAccount(pub DeploymentState);
impl DeploymentStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOYMENT_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DeploymentState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOYMENT_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_LENDER_ACCOUNT_DISCM: [u8; 8] = [
    12, 99, 0, 7, 73, 193, 37, 217,
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
pub struct InstantWithdrawalLender {
    pub bump: u8,
}
impl InstantWithdrawalLender {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalLenderAccount(pub InstantWithdrawalLender);
impl InstantWithdrawalLenderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_LENDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(InstantWithdrawalLender::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAWAL_LENDER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDER_STATE_ACCOUNT_DISCM: [u8; 8] = [240, 118, 235, 226, 18, 3, 58, 25];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LenderState {
    pub bump: u8,
    pub lender: Pubkey,
    pub redemption_record: LenderRedemptionRecord,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl LenderState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let redemption_record = <LenderRedemptionRecord>::deserialize(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            lender,
            redemption_record,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_record, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LenderStateAccount(pub LenderState);
impl LenderStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LenderState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_STRATEGY_MANAGER_ACCOUNT_DISCM: [u8; 8] = [
    64, 224, 248, 213, 255, 189, 28, 193,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ManualStrategyManager {
    pub bump: u8,
    pub wallet: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl ManualStrategyManager {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { bump, wallet, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualStrategyManagerAccount(pub ManualStrategyManager);
impl ManualStrategyManagerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_STRATEGY_MANAGER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ManualStrategyManager::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_STRATEGY_MANAGER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_STRATEGY_MANAGER_PROPOSAL_ACCOUNT_DISCM: [u8; 8] = [
    1, 133, 162, 3, 159, 227, 96, 239,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ManualStrategyManagerProposal {
    pub bump: u8,
    pub pool_config: Pubkey,
    pub wallet: Pubkey,
    pub proposed_at: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl ManualStrategyManagerProposal {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposed_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            pool_config,
            wallet,
            proposed_at,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposed_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualStrategyManagerProposalAccount(pub ManualStrategyManagerProposal);
impl ManualStrategyManagerProposalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_STRATEGY_MANAGER_PROPOSAL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ManualStrategyManagerProposal::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_STRATEGY_MANAGER_PROPOSAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [249, 180, 144, 225, 126, 159, 202, 209];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ModeConfig {
    pub bump: u8,
    pub mint_bump: u8,
    pub id: Pubkey,
    pub name: String,
    pub target_apy_bps: u16,
    pub periodic_apy_bps: f64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl ModeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let target_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let periodic_apy_bps: f64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            mint_bump,
            id,
            name,
            target_apy_bps,
            periodic_apy_bps,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_apy_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.periodic_apy_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeConfigAccount(pub ModeConfig);
impl ModeConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ModeConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [26, 108, 14, 123, 116, 230, 129, 43];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PoolConfig {
    pub bump: u8,
    pub huma_config: Pubkey,
    pub pool_owner: Pubkey,
    pub pool_owner_treasury: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_authority_bump: u8,
    pub pool_id: Pubkey,
    pub pool_name: String,
    pub lp_config: LPConfig,
    pub instant_withdrawal_config: InstantWithdrawalConfig,
    pub loss_authority: Pubkey,
    pub manual_deployment_daily_limit: u64,
    pub manual_deployment_per_wallet_limit: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 112],
}
impl PoolConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let huma_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_owner_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let underlying_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_name: String = crate::borsh_de_or_default(&mut reader)?;
        let lp_config = <LPConfig>::deserialize(&mut reader)?;
        let instant_withdrawal_config = <InstantWithdrawalConfig>::deserialize(
            &mut reader,
        )?;
        let loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let manual_deployment_daily_limit: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let manual_deployment_per_wallet_limit: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding = <[u8; 112] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            huma_config,
            pool_owner,
            pool_owner_treasury,
            underlying_mint,
            pool_authority_bump,
            pool_id,
            pool_name,
            lp_config,
            instant_withdrawal_config,
            loss_authority,
            manual_deployment_daily_limit,
            manual_deployment_per_wallet_limit,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.huma_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_owner_treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.underlying_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.instant_withdrawal_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.loss_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.manual_deployment_daily_limit,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.manual_deployment_per_wallet_limit,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfigAccount(pub PoolConfig);
impl PoolConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_OPERATOR_ACCOUNT_DISCM: [u8; 8] = [86, 93, 81, 162, 133, 189, 80, 191];
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
pub struct PoolOperator {
    pub bump: u8,
}
impl PoolOperator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolOperatorAccount(pub PoolOperator);
impl PoolOperatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_OPERATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolOperator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_OPERATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_STATE_ACCOUNT_DISCM: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PoolState {
    pub bump: u8,
    pub status: PoolStatus,
    pub disbursement_reserve: u128,
    pub mode_states: Vec<ModeState>,
    pub mode_config_keys: Vec<Pubkey>,
    pub redemption: Redemption,
    pub pool_stats: PoolStats,
    pub liquid_assets_deployed: u64,
    pub manual_daily_amount_deployed: u64,
    pub manual_daily_window_starts_at: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 136],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: PoolStatus = crate::borsh_de_or_default(&mut reader)?;
        let disbursement_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let mode_states: Vec<ModeState> = crate::borsh_de_or_default(&mut reader)?;
        let mode_config_keys: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let redemption = <Redemption>::deserialize(&mut reader)?;
        let pool_stats = <PoolStats>::deserialize(&mut reader)?;
        let liquid_assets_deployed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let manual_daily_amount_deployed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let manual_daily_window_starts_at: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding = <[u8; 136] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            status,
            disbursement_reserve,
            mode_states,
            mode_config_keys,
            redemption,
            pool_stats,
            liquid_assets_deployed,
            manual_daily_amount_deployed,
            manual_daily_window_starts_at,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disbursement_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_states, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_config_keys, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_stats, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquid_assets_deployed, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.manual_daily_amount_deployed,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.manual_daily_window_starts_at,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolStateAccount(pub PoolState);
impl PoolStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_ACCOUNT_DISCM: [u8; 8] = [
    117, 157, 214, 214, 64, 160, 31, 58,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RedemptionRequest {
    pub bump: u8,
    pub id: u128,
    pub lender: Pubkey,
    pub mode_config: Pubkey,
    pub shares_requested: u128,
    pub requested_at: u64,
    pub payer: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 128],
}
impl RedemptionRequest {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares_requested: u128 = crate::borsh_de_or_default(&mut reader)?;
        let requested_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            id,
            lender,
            mode_config,
            shares_requested,
            requested_at,
            payer,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_requested, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.requested_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestAccount(pub RedemptionRequest);
impl RedemptionRequestAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RedemptionRequest::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STRATEGY_MANAGER_WALLET_ACCOUNT_DISCM: [u8; 8] = [
    7, 113, 185, 42, 183, 171, 232, 77,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct StrategyManagerWallet {
    pub bump: u8,
    pub wallet: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl StrategyManagerWallet {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { bump, wallet, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyManagerWalletAccount(pub StrategyManagerWallet);
impl StrategyManagerWalletAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STRATEGY_MANAGER_WALLET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StrategyManagerWallet::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STRATEGY_MANAGER_WALLET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
