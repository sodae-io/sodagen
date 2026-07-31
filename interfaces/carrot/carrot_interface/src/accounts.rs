use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const VAULT_ACCOUNT_DISCM: [u8; 8] = [211, 8, 232, 43, 2, 152, 117, 119];
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
pub struct Vault {
    pub authority: Pubkey,
    pub shares: Pubkey,
    pub fee: Fee,
    pub paused: bool,
    pub asset_index: u16,
    pub strategy_index: u16,
    pub assets: Vec<Asset>,
    pub strategies: Vec<StrategyRecord>,
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee = if reader.is_empty() {
            Default::default()
        } else {
            <Fee>::deserialize(&mut reader)?
        };
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let asset_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let assets: Vec<Asset> = crate::borsh_de_or_default(&mut reader)?;
        let strategies: Vec<StrategyRecord> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            shares,
            fee,
            paused,
            asset_index,
            strategy_index,
            assets,
            strategies,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategies, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultAccount(pub Vault);
impl VaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Vault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STRATEGY_ACCOUNT_DISCM: [u8; 8] = [174, 110, 39, 119, 82, 106, 169, 102];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Strategy {
    pub metadata: StrategyMetadata,
    pub strategy_type: StrategyType,
}
impl Strategy {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <StrategyMetadata>::deserialize(&mut reader)?
        };
        let strategy_type = <StrategyType as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { metadata, strategy_type })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyAccount(pub Strategy);
impl StrategyAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STRATEGY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Strategy::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STRATEGY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
