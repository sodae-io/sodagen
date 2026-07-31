use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ASYNC_DEPLOYMENT_TARGET_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    59, 142, 197, 242, 75, 20, 45, 51,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AsyncDeploymentTargetAddedEvent {
    pub pool_id: Pubkey,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
}
impl AsyncDeploymentTargetAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            strategy_type,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncDeploymentTargetAddedEventEvent(pub AsyncDeploymentTargetAddedEvent);
impl AsyncDeploymentTargetAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ASYNC_DEPLOYMENT_TARGET_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AsyncDeploymentTargetAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ASYNC_DEPLOYMENT_TARGET_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ASYNC_DEPLOYMENT_TARGET_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    194, 27, 9, 134, 2, 75, 135, 239,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AsyncDeploymentTargetRemovedEvent {
    pub pool_id: Pubkey,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
}
impl AsyncDeploymentTargetRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            strategy_type,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncDeploymentTargetRemovedEventEvent(pub AsyncDeploymentTargetRemovedEvent);
impl AsyncDeploymentTargetRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ASYNC_DEPLOYMENT_TARGET_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AsyncDeploymentTargetRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ASYNC_DEPLOYMENT_TARGET_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPLOYMENT_TARGET_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    143, 102, 222, 71, 122, 6, 252, 209,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeploymentTargetAddedEvent {
    pub pool_id: Pubkey,
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
}
impl DeploymentTargetAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            strategy_type,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeploymentTargetAddedEventEvent(pub DeploymentTargetAddedEvent);
impl DeploymentTargetAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOYMENT_TARGET_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeploymentTargetAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOYMENT_TARGET_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPLOYMENT_TARGET_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    21, 180, 221, 164, 151, 158, 213, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeploymentTargetRemovedEvent {
    pub pool_id: Pubkey,
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
}
impl DeploymentTargetRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            strategy_type,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeploymentTargetRemovedEventEvent(pub DeploymentTargetRemovedEvent);
impl DeploymentTargetRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOYMENT_TARGET_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeploymentTargetRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOYMENT_TARGET_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPLOYMENT_TARGET_SETUP_EVENT_EVENT_DISCM: [u8; 8] = [
    1, 196, 109, 117, 230, 199, 126, 79,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeploymentTargetSetupEvent {
    pub pool_id: Pubkey,
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
}
impl DeploymentTargetSetupEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            strategy_type,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeploymentTargetSetupEventEvent(pub DeploymentTargetSetupEvent);
impl DeploymentTargetSetupEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPLOYMENT_TARGET_SETUP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeploymentTargetSetupEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPLOYMENT_TARGET_SETUP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FULLY_CANCELED_REDEMPTION_REQUEST_SKIPPED_EVENT_EVENT_DISCM: [u8; 8] = [
    182, 222, 115, 2, 28, 250, 2, 128,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FullyCanceledRedemptionRequestSkippedEvent {
    pub request_id: u128,
    pub lender: Pubkey,
}
impl FullyCanceledRedemptionRequestSkippedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { request_id, lender })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FullyCanceledRedemptionRequestSkippedEventEvent(
    pub FullyCanceledRedemptionRequestSkippedEvent,
);
impl FullyCanceledRedemptionRequestSkippedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULLY_CANCELED_REDEMPTION_REQUEST_SKIPPED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FullyCanceledRedemptionRequestSkippedEvent::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULLY_CANCELED_REDEMPTION_REQUEST_SKIPPED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HUMA_CONFIG_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    160, 214, 104, 167, 105, 118, 72, 49,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HumaConfigChangedEvent {
    pub pool_id: Pubkey,
    pub old_huma_config: Pubkey,
    pub new_huma_config: Pubkey,
}
impl HumaConfigChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_huma_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_huma_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            old_huma_config,
            new_huma_config,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_huma_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_huma_config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HumaConfigChangedEventEvent(pub HumaConfigChangedEvent);
impl HumaConfigChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HUMA_CONFIG_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = HumaConfigChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HUMA_CONFIG_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_FEE_CONFIGS_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    220, 60, 248, 228, 244, 222, 175, 55,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawalFeeConfigsChangedEvent {
    pub pool_id: Pubkey,
    pub fee_configs: Vec<InstantWithdrawalFeeConfigInput>,
}
impl InstantWithdrawalFeeConfigsChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_configs: Vec<InstantWithdrawalFeeConfigInput> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { pool_id, fee_configs })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_configs, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalFeeConfigsChangedEventEvent(
    pub InstantWithdrawalFeeConfigsChangedEvent,
);
impl InstantWithdrawalFeeConfigsChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_FEE_CONFIGS_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawalFeeConfigsChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAWAL_FEE_CONFIGS_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_LENDER_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    247, 184, 251, 18, 253, 27, 127, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawalLenderAddedEvent {
    pub pool_id: Pubkey,
    pub lender: Pubkey,
}
impl InstantWithdrawalLenderAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, lender })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalLenderAddedEventEvent(pub InstantWithdrawalLenderAddedEvent);
impl InstantWithdrawalLenderAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_LENDER_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawalLenderAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAWAL_LENDER_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_LENDER_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    228, 255, 164, 26, 160, 125, 77, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawalLenderRemovedEvent {
    pub pool_id: Pubkey,
    pub lender: Pubkey,
}
impl InstantWithdrawalLenderRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, lender })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalLenderRemovedEventEvent(
    pub InstantWithdrawalLenderRemovedEvent,
);
impl InstantWithdrawalLenderRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_LENDER_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawalLenderRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAWAL_LENDER_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_CLEARED_EVENT_EVENT_DISCM: [u8; 8] = [
    244, 93, 188, 227, 190, 128, 85, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawalLiquiditySourceClearedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub old_liquidity_source: Pubkey,
}
impl InstantWithdrawalLiquiditySourceClearedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_liquidity_source: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            old_liquidity_source,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_liquidity_source, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalLiquiditySourceClearedEventEvent(
    pub InstantWithdrawalLiquiditySourceClearedEvent,
);
impl InstantWithdrawalLiquiditySourceClearedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_CLEARED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawalLiquiditySourceClearedEvent::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(&INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_CLEARED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_SET_EVENT_EVENT_DISCM: [u8; 8] = [
    188, 167, 59, 236, 129, 19, 186, 11,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawalLiquiditySourceSetEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub old_liquidity_source: Option<Pubkey>,
    pub new_liquidity_source: Pubkey,
}
impl InstantWithdrawalLiquiditySourceSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_liquidity_source: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_liquidity_source: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            old_liquidity_source,
            new_liquidity_source,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_liquidity_source, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_liquidity_source, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalLiquiditySourceSetEventEvent(
    pub InstantWithdrawalLiquiditySourceSetEvent,
);
impl InstantWithdrawalLiquiditySourceSetEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_SET_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawalLiquiditySourceSetEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAWAL_LIQUIDITY_SOURCE_SET_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAWAL_RESERVE_LIMIT_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    163, 77, 62, 14, 42, 11, 32, 215,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawalReserveLimitChangedEvent {
    pub pool_id: Pubkey,
    pub old_reserve_limit: u64,
    pub new_reserve_limit: u64,
}
impl InstantWithdrawalReserveLimitChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_reserve_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_reserve_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            old_reserve_limit,
            new_reserve_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_reserve_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reserve_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawalReserveLimitChangedEventEvent(
    pub InstantWithdrawalReserveLimitChangedEvent,
);
impl InstantWithdrawalReserveLimitChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAWAL_RESERVE_LIMIT_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawalReserveLimitChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAWAL_RESERVE_LIMIT_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_CONFIG_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    241, 93, 140, 107, 238, 126, 168, 245,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpConfigChangedEvent {
    pub pool_id: Pubkey,
    pub lp_config: LPConfig,
}
impl LpConfigChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_config = <LPConfig>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, lp_config })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpConfigChangedEventEvent(pub LpConfigChangedEvent);
impl LpConfigChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_CONFIG_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpConfigChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_CONFIG_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDER_ACCOUNTS_CLOSED_EVENT_EVENT_DISCM: [u8; 8] = [
    114, 190, 186, 18, 188, 195, 238, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LenderAccountsClosedEvent {
    pub mode_config: Pubkey,
    pub payer: Pubkey,
    pub lender: Pubkey,
}
impl LenderAccountsClosedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mode_config, payer, lender })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LenderAccountsClosedEventEvent(pub LenderAccountsClosedEvent);
impl LenderAccountsClosedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDER_ACCOUNTS_CLOSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LenderAccountsClosedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDER_ACCOUNTS_CLOSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDER_ACCOUNTS_CREATED_EVENT_V2_EVENT_DISCM: [u8; 8] = [
    39, 223, 41, 109, 60, 101, 244, 228,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LenderAccountsCreatedEventV2 {
    pub mode_config: Pubkey,
    pub payer: Pubkey,
    pub lender: Pubkey,
}
impl LenderAccountsCreatedEventV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mode_config, payer, lender })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LenderAccountsCreatedEventV2Event(pub LenderAccountsCreatedEventV2);
impl LenderAccountsCreatedEventV2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDER_ACCOUNTS_CREATED_EVENT_V2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LenderAccountsCreatedEventV2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDER_ACCOUNTS_CREATED_EVENT_V2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDER_FUND_DISBURSED_EVENT_EVENT_DISCM: [u8; 8] = [
    185, 20, 203, 170, 122, 78, 231, 106,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LenderFundDisbursedEvent {
    pub mode_config: Pubkey,
    pub lender: Pubkey,
    pub amount_disbursed: u64,
}
impl LenderFundDisbursedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_disbursed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            lender,
            amount_disbursed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_disbursed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LenderFundDisbursedEventEvent(pub LenderFundDisbursedEvent);
impl LenderFundDisbursedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDER_FUND_DISBURSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LenderFundDisbursedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDER_FUND_DISBURSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDER_FUND_WITHDRAWN_EVENT_EVENT_DISCM: [u8; 8] = [
    189, 37, 124, 152, 255, 154, 13, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LenderFundWithdrawnEvent {
    pub mode_config: Pubkey,
    pub lender: Pubkey,
    pub shares: u64,
    pub assets: u64,
}
impl LenderFundWithdrawnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            lender,
            shares,
            assets,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LenderFundWithdrawnEventEvent(pub LenderFundWithdrawnEvent);
impl LenderFundWithdrawnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDER_FUND_WITHDRAWN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LenderFundWithdrawnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDER_FUND_WITHDRAWN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDER_FUNDS_INSTANTLY_WITHDRAWN_EVENT_EVENT_DISCM: [u8; 8] = [
    70, 238, 186, 163, 134, 200, 240, 180,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LenderFundsInstantlyWithdrawnEvent {
    pub mode_config: Pubkey,
    pub lender: Pubkey,
    pub shares: u64,
    pub assets: u64,
    pub fee: u64,
    pub strategy_liquidity_withdrawn: u64,
}
impl LenderFundsInstantlyWithdrawnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_liquidity_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            lender,
            shares,
            assets,
            fee,
            strategy_liquidity_withdrawn,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.strategy_liquidity_withdrawn,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LenderFundsInstantlyWithdrawnEventEvent(
    pub LenderFundsInstantlyWithdrawnEvent,
);
impl LenderFundsInstantlyWithdrawnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDER_FUNDS_INSTANTLY_WITHDRAWN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LenderFundsInstantlyWithdrawnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDER_FUNDS_INSTANTLY_WITHDRAWN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUID_ASSETS_DEPLOYED_SET_EVENT_EVENT_DISCM: [u8; 8] = [
    218, 95, 125, 29, 243, 14, 109, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidAssetsDeployedSetEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub liquid_assets_deployed: u64,
}
impl LiquidAssetsDeployedSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquid_assets_deployed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            liquid_assets_deployed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquid_assets_deployed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidAssetsDeployedSetEventEvent(pub LiquidAssetsDeployedSetEvent);
impl LiquidAssetsDeployedSetEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUID_ASSETS_DEPLOYED_SET_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidAssetsDeployedSetEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_ASSETS_DEPLOYED_SET_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_DEPLOYED_ASYNC_EVENT_EVENT_DISCM: [u8; 8] = [
    180, 205, 84, 115, 109, 7, 212, 232,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityDeployedAsyncEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
    pub amount: u64,
}
impl LiquidityDeployedAsyncEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            strategy_type,
            target_key,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDeployedAsyncEventEvent(pub LiquidityDeployedAsyncEvent);
impl LiquidityDeployedAsyncEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_DEPLOYED_ASYNC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityDeployedAsyncEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_DEPLOYED_ASYNC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_DEPLOYED_EVENT_EVENT_DISCM: [u8; 8] = [
    46, 23, 218, 176, 233, 96, 239, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityDeployedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
    pub amount: u64,
}
impl LiquidityDeployedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            strategy_type,
            target_key,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDeployedEventEvent(pub LiquidityDeployedEvent);
impl LiquidityDeployedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_DEPLOYED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityDeployedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_DEPLOYED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_DEPLOYED_MANUALLY_EVENT_EVENT_DISCM: [u8; 8] = [
    42, 190, 47, 33, 92, 171, 96, 19,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityDeployedManuallyEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub target_key: Pubkey,
    pub amount: u64,
}
impl LiquidityDeployedManuallyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            target_key,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDeployedManuallyEventEvent(pub LiquidityDeployedManuallyEvent);
impl LiquidityDeployedManuallyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_DEPLOYED_MANUALLY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityDeployedManuallyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_DEPLOYED_MANUALLY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_DEPOSITED_EVENT_EVENT_DISCM: [u8; 8] = [
    90, 3, 240, 128, 109, 154, 131, 185,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityDepositedEvent {
    pub mode_config: Pubkey,
    pub depositor: Pubkey,
    pub assets: u64,
    pub shares: u64,
    pub commitment: String,
    pub commitment_auto_renewal: bool,
}
impl LiquidityDepositedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let depositor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let commitment: String = crate::borsh_de_or_default(&mut reader)?;
        let commitment_auto_renewal: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            depositor,
            assets,
            shares,
            commitment,
            commitment_auto_renewal,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.commitment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.commitment_auto_renewal, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDepositedEventEvent(pub LiquidityDepositedEvent);
impl LiquidityDepositedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_DEPOSITED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityDepositedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_DEPOSITED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_PAID_BACK_EVENT_EVENT_DISCM: [u8; 8] = [
    50, 105, 163, 223, 164, 6, 91, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityPaidBackEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub amount: u64,
    pub strategy_type: DeploymentStrategyType,
    pub target_key: Pubkey,
}
impl LiquidityPaidBackEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: DeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            amount,
            strategy_type,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityPaidBackEventEvent(pub LiquidityPaidBackEvent);
impl LiquidityPaidBackEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_PAID_BACK_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityPaidBackEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_PAID_BACK_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_PAID_BACK_MANUALLY_EVENT_EVENT_DISCM: [u8; 8] = [
    120, 107, 26, 135, 151, 15, 56, 198,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityPaidBackManuallyEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub amount: u64,
    pub target_key: Pubkey,
}
impl LiquidityPaidBackManuallyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            amount,
            target_key,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityPaidBackManuallyEventEvent(pub LiquidityPaidBackManuallyEvent);
impl LiquidityPaidBackManuallyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_PAID_BACK_MANUALLY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityPaidBackManuallyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_PAID_BACK_MANUALLY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_PAYBACK_COMPLETED_ASYNC_EVENT_EVENT_DISCM: [u8; 8] = [
    50, 150, 252, 234, 89, 51, 153, 196,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityPaybackCompletedAsyncEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
    pub amount: u64,
}
impl LiquidityPaybackCompletedAsyncEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            strategy_type,
            target_key,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityPaybackCompletedAsyncEventEvent(
    pub LiquidityPaybackCompletedAsyncEvent,
);
impl LiquidityPaybackCompletedAsyncEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_PAYBACK_COMPLETED_ASYNC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityPaybackCompletedAsyncEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_PAYBACK_COMPLETED_ASYNC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_PAYBACK_INITIATED_ASYNC_EVENT_EVENT_DISCM: [u8; 8] = [
    240, 26, 247, 116, 21, 215, 151, 108,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityPaybackInitiatedAsyncEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
    pub shares: u64,
}
impl LiquidityPaybackInitiatedAsyncEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            strategy_type,
            target_key,
            shares,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityPaybackInitiatedAsyncEventEvent(
    pub LiquidityPaybackInitiatedAsyncEvent,
);
impl LiquidityPaybackInitiatedAsyncEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_PAYBACK_INITIATED_ASYNC_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityPaybackInitiatedAsyncEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_PAYBACK_INITIATED_ASYNC_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_WITHDRAWN_AFTER_TARGET_CLOSURE_EVENT_EVENT_DISCM: [u8; 8] = [
    67, 98, 31, 56, 243, 232, 139, 236,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityWithdrawnAfterTargetClosureEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
    pub strategy_type: AsyncDeploymentStrategyType,
    pub target_key: Pubkey,
    pub amount: u64,
}
impl LiquidityWithdrawnAfterTargetClosureEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: AsyncDeploymentStrategyType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            wallet,
            strategy_type,
            target_key,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityWithdrawnAfterTargetClosureEventEvent(
    pub LiquidityWithdrawnAfterTargetClosureEvent,
);
impl LiquidityWithdrawnAfterTargetClosureEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_WITHDRAWN_AFTER_TARGET_CLOSURE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityWithdrawnAfterTargetClosureEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_WITHDRAWN_AFTER_TARGET_CLOSURE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOSS_AUTHORITY_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    11, 130, 65, 235, 139, 77, 184, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LossAuthorityChangedEvent {
    pub pool_id: Pubkey,
    pub old_loss_authority: Pubkey,
    pub new_loss_authority: Pubkey,
}
impl LossAuthorityChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            old_loss_authority,
            new_loss_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_loss_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_loss_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LossAuthorityChangedEventEvent(pub LossAuthorityChangedEvent);
impl LossAuthorityChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOSS_AUTHORITY_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LossAuthorityChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOSS_AUTHORITY_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOSS_DECLARED_EVENT_EVENT_DISCM: [u8; 8] = [
    159, 235, 30, 240, 118, 234, 120, 83,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LossDeclaredEvent {
    pub pool_id: Pubkey,
    pub loss_authority: Pubkey,
    pub loss_distributed: Vec<u64>,
}
impl LossDeclaredEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let loss_distributed: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            loss_authority,
            loss_distributed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.loss_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.loss_distributed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LossDeclaredEventEvent(pub LossDeclaredEvent);
impl LossDeclaredEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOSS_DECLARED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LossDeclaredEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOSS_DECLARED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOSS_RECOVERED_EVENT_EVENT_DISCM: [u8; 8] = [
    202, 156, 189, 173, 187, 83, 132, 72,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LossRecoveredEvent {
    pub pool_id: Pubkey,
    pub loss_authority: Pubkey,
    pub amount: u64,
    pub losses_recovered: Vec<u64>,
}
impl LossRecoveredEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let losses_recovered: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            loss_authority,
            amount,
            losses_recovered,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.loss_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.losses_recovered, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LossRecoveredEventEvent(pub LossRecoveredEvent);
impl LossRecoveredEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOSS_RECOVERED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LossRecoveredEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOSS_RECOVERED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_DEPLOYMENT_LIMIT_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    113, 135, 172, 26, 117, 114, 39, 55,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManualDeploymentLimitChangedEvent {
    pub pool_id: Pubkey,
    pub daily_limit: u64,
    pub per_wallet_limit: u64,
}
impl ManualDeploymentLimitChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let daily_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let per_wallet_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            daily_limit,
            per_wallet_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.daily_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.per_wallet_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualDeploymentLimitChangedEventEvent(pub ManualDeploymentLimitChangedEvent);
impl ManualDeploymentLimitChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_DEPLOYMENT_LIMIT_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ManualDeploymentLimitChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_DEPLOYMENT_LIMIT_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_STRATEGY_MANAGER_APPROVED_EVENT_EVENT_DISCM: [u8; 8] = [
    193, 20, 223, 219, 21, 118, 147, 138,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManualStrategyManagerApprovedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
}
impl ManualStrategyManagerApprovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, wallet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualStrategyManagerApprovedEventEvent(
    pub ManualStrategyManagerApprovedEvent,
);
impl ManualStrategyManagerApprovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_STRATEGY_MANAGER_APPROVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ManualStrategyManagerApprovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_STRATEGY_MANAGER_APPROVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_STRATEGY_MANAGER_PROPOSAL_CANCELLED_EVENT_EVENT_DISCM: [u8; 8] = [
    249, 227, 137, 178, 221, 236, 77, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManualStrategyManagerProposalCancelledEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
}
impl ManualStrategyManagerProposalCancelledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, wallet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualStrategyManagerProposalCancelledEventEvent(
    pub ManualStrategyManagerProposalCancelledEvent,
);
impl ManualStrategyManagerProposalCancelledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_STRATEGY_MANAGER_PROPOSAL_CANCELLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ManualStrategyManagerProposalCancelledEvent::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_STRATEGY_MANAGER_PROPOSAL_CANCELLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_STRATEGY_MANAGER_PROPOSED_EVENT_EVENT_DISCM: [u8; 8] = [
    198, 76, 196, 12, 129, 198, 81, 127,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManualStrategyManagerProposedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
}
impl ManualStrategyManagerProposedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, wallet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualStrategyManagerProposedEventEvent(
    pub ManualStrategyManagerProposedEvent,
);
impl ManualStrategyManagerProposedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_STRATEGY_MANAGER_PROPOSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ManualStrategyManagerProposedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_STRATEGY_MANAGER_PROPOSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MANUAL_STRATEGY_MANAGER_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    152, 253, 2, 54, 230, 192, 199, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ManualStrategyManagerRemovedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
}
impl ManualStrategyManagerRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, wallet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ManualStrategyManagerRemovedEventEvent(pub ManualStrategyManagerRemovedEvent);
impl ManualStrategyManagerRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANUAL_STRATEGY_MANAGER_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ManualStrategyManagerRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANUAL_STRATEGY_MANAGER_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_APY_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    32, 227, 98, 156, 108, 3, 184, 44,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeApyChangedEvent {
    pub mode_config: Pubkey,
    pub target_apy_bps: u16,
    pub periodic_apy_bps: f64,
}
impl ModeApyChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let target_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let periodic_apy_bps: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            target_apy_bps,
            periodic_apy_bps,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_apy_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.periodic_apy_bps, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeApyChangedEventEvent(pub ModeApyChangedEvent);
impl ModeApyChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_APY_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeApyChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_APY_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [74, 37, 33, 192, 210, 247, 243, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeAddedEvent {
    pub pool_id: Pubkey,
    pub mode_id: Pubkey,
    pub mode_name: String,
    pub mode_config: Pubkey,
    pub mode_mint: Pubkey,
    pub mode_token: Pubkey,
    pub mode_token_name: String,
    pub mode_token_symbol: String,
    pub mode_token_uri: String,
}
impl ModeAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_name: String = crate::borsh_de_or_default(&mut reader)?;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_token_name: String = crate::borsh_de_or_default(&mut reader)?;
        let mode_token_symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let mode_token_uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            mode_id,
            mode_name,
            mode_config,
            mode_mint,
            mode_token,
            mode_token_name,
            mode_token_symbol,
            mode_token_uri,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_token_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_token_symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_token_uri, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeAddedEventEvent(pub ModeAddedEvent);
impl ModeAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_ASSETS_REFRESHED_EVENT_EVENT_DISCM: [u8; 8] = [
    210, 11, 22, 20, 35, 19, 80, 74,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeAssetsRefreshedEvent {
    pub mode_config: Pubkey,
    pub yield_bps: f64,
    pub old_assets_refreshed_at: u64,
    pub old_assets: u64,
    pub new_assets: u64,
}
impl ModeAssetsRefreshedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let yield_bps: f64 = crate::borsh_de_or_default(&mut reader)?;
        let old_assets_refreshed_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            yield_bps,
            old_assets_refreshed_at,
            old_assets,
            new_assets,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yield_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_assets_refreshed_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_assets, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeAssetsRefreshedEventEvent(pub ModeAssetsRefreshedEvent);
impl ModeAssetsRefreshedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_ASSETS_REFRESHED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeAssetsRefreshedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_ASSETS_REFRESHED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_NAME_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    143, 246, 221, 130, 98, 175, 204, 53,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeNameChangedEvent {
    pub mode_config: Pubkey,
    pub name: String,
}
impl ModeNameChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mode_config, name })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeNameChangedEventEvent(pub ModeNameChangedEvent);
impl ModeNameChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_NAME_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeNameChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_NAME_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    32, 53, 184, 81, 110, 129, 249, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeRemovedEvent {
    pub pool_id: Pubkey,
    pub mode_id: Pubkey,
}
impl ModeRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mode_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, mode_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mode_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeRemovedEventEvent(pub ModeRemovedEvent);
impl ModeRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_SWITCHED_EVENT_EVENT_DISCM: [u8; 8] = [
    180, 176, 130, 85, 150, 49, 56, 187,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeSwitchedEvent {
    pub source_mode_config: Pubkey,
    pub destination_mode_config: Pubkey,
    pub lender: Pubkey,
    pub source_shares_burned: u64,
    pub destination_shares_minted: u64,
    pub assets_switched: u64,
    pub investment_id: String,
}
impl ModeSwitchedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let source_mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let destination_mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let source_shares_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let destination_shares_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        let assets_switched: u64 = crate::borsh_de_or_default(&mut reader)?;
        let investment_id: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            source_mode_config,
            destination_mode_config,
            lender,
            source_shares_burned,
            destination_shares_minted,
            assets_switched,
            investment_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.source_mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.destination_mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.source_shares_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.destination_shares_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets_switched, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.investment_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeSwitchedEventEvent(pub ModeSwitchedEvent);
impl ModeSwitchedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_SWITCHED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeSwitchedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_SWITCHED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MODE_TOKEN_METADATA_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    138, 111, 218, 240, 143, 17, 130, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeTokenMetadataUpdatedEvent {
    pub mode_config: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl ModeTokenMetadataUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            name,
            symbol,
            uri,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.uri, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModeTokenMetadataUpdatedEventEvent(pub ModeTokenMetadataUpdatedEvent);
impl ModeTokenMetadataUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODE_TOKEN_METADATA_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ModeTokenMetadataUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODE_TOKEN_METADATA_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CLOSED_EVENT_EVENT_DISCM: [u8; 8] = [76, 55, 28, 161, 130, 142, 226, 133];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolClosedEvent {
    pub pool_id: Pubkey,
}
impl PoolClosedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolClosedEventEvent(pub PoolClosedEvent);
impl PoolClosedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CLOSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolClosedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CLOSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [25, 94, 75, 47, 112, 99, 53, 63];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolCreatedEvent {
    pub pool_id: Pubkey,
    pub huma_config: Pubkey,
    pub pool_owner: Pubkey,
    pub pool_owner_treasury: Pubkey,
    pub underlying_mint: Pubkey,
    pub pool_underlying_token: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_name: String,
    pub loss_authority: Pubkey,
}
impl PoolCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let huma_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_owner_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let underlying_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_underlying_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_name: String = crate::borsh_de_or_default(&mut reader)?;
        let loss_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            huma_config,
            pool_owner,
            pool_owner_treasury,
            underlying_mint,
            pool_underlying_token,
            pool_authority,
            pool_name,
            loss_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.huma_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_owner_treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.underlying_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_underlying_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.loss_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreatedEventEvent(pub PoolCreatedEvent);
impl PoolCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CREATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_DISABLED_EVENT_EVENT_DISCM: [u8; 8] = [
    253, 229, 56, 71, 40, 225, 125, 122,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolDisabledEvent {
    pub pool_id: Pubkey,
}
impl PoolDisabledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolDisabledEventEvent(pub PoolDisabledEvent);
impl PoolDisabledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_DISABLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolDisabledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_DISABLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ENABLED_EVENT_EVENT_DISCM: [u8; 8] = [169, 245, 50, 35, 124, 58, 231, 48];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolEnabledEvent {
    pub pool_id: Pubkey,
}
impl PoolEnabledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolEnabledEventEvent(pub PoolEnabledEvent);
impl PoolEnabledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ENABLED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolEnabledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ENABLED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ENTERED_PRE_CLOSURE_EVENT_EVENT_DISCM: [u8; 8] = [
    158, 111, 26, 100, 200, 42, 252, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolEnteredPreClosureEvent {
    pub pool_id: Pubkey,
}
impl PoolEnteredPreClosureEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolEnteredPreClosureEventEvent(pub PoolEnteredPreClosureEvent);
impl PoolEnteredPreClosureEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ENTERED_PRE_CLOSURE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolEnteredPreClosureEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ENTERED_PRE_CLOSURE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_NAME_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    197, 42, 231, 168, 23, 125, 82, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolNameChangedEvent {
    pub pool_id: Pubkey,
    pub pool_name: String,
}
impl PoolNameChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, pool_name })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_name, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolNameChangedEventEvent(pub PoolNameChangedEvent);
impl PoolNameChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_NAME_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolNameChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_NAME_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_OPERATOR_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    45, 70, 168, 122, 180, 30, 11, 196,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolOperatorAddedEvent {
    pub pool_id: Pubkey,
    pub operator: Pubkey,
}
impl PoolOperatorAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, operator })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolOperatorAddedEventEvent(pub PoolOperatorAddedEvent);
impl PoolOperatorAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_OPERATOR_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolOperatorAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_OPERATOR_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_OPERATOR_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    171, 56, 197, 75, 3, 90, 107, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolOperatorRemovedEvent {
    pub pool_id: Pubkey,
    pub operator: Pubkey,
}
impl PoolOperatorRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, operator })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolOperatorRemovedEventEvent(pub PoolOperatorRemovedEvent);
impl PoolOperatorRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_OPERATOR_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolOperatorRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_OPERATOR_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_OWNER_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    34, 125, 255, 170, 143, 47, 140, 169,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolOwnerChangedEvent {
    pub pool_id: Pubkey,
    pub owner: Pubkey,
}
impl PoolOwnerChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, owner })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolOwnerChangedEventEvent(pub PoolOwnerChangedEvent);
impl PoolOwnerChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_OWNER_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolOwnerChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_OWNER_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_OWNER_TREASURY_CHANGED_EVENT_EVENT_DISCM: [u8; 8] = [
    140, 110, 16, 105, 86, 252, 169, 49,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolOwnerTreasuryChangedEvent {
    pub pool_id: Pubkey,
    pub old_treasury: Pubkey,
    pub new_treasury: Pubkey,
}
impl PoolOwnerTreasuryChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            old_treasury,
            new_treasury,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_treasury, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolOwnerTreasuryChangedEventEvent(pub PoolOwnerTreasuryChangedEvent);
impl PoolOwnerTreasuryChangedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_OWNER_TREASURY_CHANGED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolOwnerTreasuryChangedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_OWNER_TREASURY_CHANGED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRIVILEGED_INSTANT_WITHDRAWAL_EVENT_EVENT_DISCM: [u8; 8] = [
    22, 138, 222, 135, 112, 247, 135, 142,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivilegedInstantWithdrawalEvent {
    pub mode_config: Pubkey,
    pub lender: Pubkey,
    pub shares: u64,
    pub assets: u64,
    pub strategy_liquidity_withdrawn: u64,
}
impl PrivilegedInstantWithdrawalEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_liquidity_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            lender,
            shares,
            assets,
            strategy_liquidity_withdrawn,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.strategy_liquidity_withdrawn,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PrivilegedInstantWithdrawalEventEvent(pub PrivilegedInstantWithdrawalEvent);
impl PrivilegedInstantWithdrawalEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRIVILEGED_INSTANT_WITHDRAWAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PrivilegedInstantWithdrawalEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRIVILEGED_INSTANT_WITHDRAWAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEMED_FUNDS_PAID_OUT_EVENT_EVENT_DISCM: [u8; 8] = [
    3, 68, 39, 81, 188, 59, 247, 177,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemedFundsPaidOutEvent {
    pub request_id: u128,
    pub lender: Pubkey,
    pub amount: u64,
}
impl RedeemedFundsPaidOutEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { request_id, lender, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemedFundsPaidOutEventEvent(pub RedeemedFundsPaidOutEvent);
impl RedeemedFundsPaidOutEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEMED_FUNDS_PAID_OUT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemedFundsPaidOutEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEMED_FUNDS_PAID_OUT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEEMED_FUNDS_RESERVED_EVENT_EVENT_DISCM: [u8; 8] = [
    141, 45, 128, 85, 166, 158, 191, 48,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemedFundsReservedEvent {
    pub request_id: u128,
    pub lender: Pubkey,
    pub amount: u64,
}
impl RedeemedFundsReservedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { request_id, lender, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemedFundsReservedEventEvent(pub RedeemedFundsReservedEvent);
impl RedeemedFundsReservedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEMED_FUNDS_RESERVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedeemedFundsReservedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEMED_FUNDS_RESERVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_ADDED_EVENT_V2_EVENT_DISCM: [u8; 8] = [
    115, 90, 91, 247, 160, 36, 53, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionRequestAddedEventV2 {
    pub mode_config: Pubkey,
    pub payer: Pubkey,
    pub lender: Pubkey,
    pub request_id: u128,
    pub shares: u64,
}
impl RedemptionRequestAddedEventV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            payer,
            lender,
            request_id,
            shares,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestAddedEventV2Event(pub RedemptionRequestAddedEventV2);
impl RedemptionRequestAddedEventV2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_ADDED_EVENT_V2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionRequestAddedEventV2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_ADDED_EVENT_V2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_CANCELED_EVENT_EVENT_DISCM: [u8; 8] = [
    200, 205, 28, 174, 176, 233, 95, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionRequestCanceledEvent {
    pub mode_config: Pubkey,
    pub lender: Pubkey,
    pub request_id: u128,
    pub shares: u64,
}
impl RedemptionRequestCanceledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mode_config,
            lender,
            request_id,
            shares,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mode_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestCanceledEventEvent(pub RedemptionRequestCanceledEvent);
impl RedemptionRequestCanceledEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_CANCELED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionRequestCanceledEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_CANCELED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_PROCESSED_EVENT_EVENT_DISCM: [u8; 8] = [
    165, 153, 161, 205, 132, 243, 207, 156,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedemptionRequestProcessedEvent {
    pub request_id: u128,
    pub lender: Pubkey,
    pub shares_processed: u64,
    pub amount_processed: u64,
}
impl RedemptionRequestProcessedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares_processed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_processed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            request_id,
            lender,
            shares_processed,
            amount_processed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_processed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_processed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestProcessedEventEvent(pub RedemptionRequestProcessedEvent);
impl RedemptionRequestProcessedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_PROCESSED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RedemptionRequestProcessedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_PROCESSED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STRATEGY_MANAGER_WALLET_ADDED_EVENT_EVENT_DISCM: [u8; 8] = [
    159, 1, 96, 54, 37, 222, 191, 22,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StrategyManagerWalletAddedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
}
impl StrategyManagerWalletAddedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, wallet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyManagerWalletAddedEventEvent(pub StrategyManagerWalletAddedEvent);
impl StrategyManagerWalletAddedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STRATEGY_MANAGER_WALLET_ADDED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = StrategyManagerWalletAddedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STRATEGY_MANAGER_WALLET_ADDED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STRATEGY_MANAGER_WALLET_REMOVED_EVENT_EVENT_DISCM: [u8; 8] = [
    81, 71, 145, 25, 9, 72, 114, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StrategyManagerWalletRemovedEvent {
    pub pool_id: Pubkey,
    pub wallet: Pubkey,
}
impl StrategyManagerWalletRemovedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pool_id, wallet })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyManagerWalletRemovedEventEvent(pub StrategyManagerWalletRemovedEvent);
impl StrategyManagerWalletRemovedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STRATEGY_MANAGER_WALLET_REMOVED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = StrategyManagerWalletRemovedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STRATEGY_MANAGER_WALLET_REMOVED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
