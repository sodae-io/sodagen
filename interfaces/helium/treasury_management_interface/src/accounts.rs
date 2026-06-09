use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ACCOUNT_WINDOWED_CIRCUIT_BREAKER_V0_ACCOUNT_DISCM: [u8; 8] = [
    134, 11, 69, 100, 90, 132, 174, 187,
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
pub struct AccountWindowedCircuitBreakerV0 {
    pub token_account: Pubkey,
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub config: WindowedCircuitBreakerConfigV0,
    pub last_window: WindowV0,
    pub bump_seed: u8,
}
impl AccountWindowedCircuitBreakerV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <WindowedCircuitBreakerConfigV0>::deserialize(&mut reader)?
        };
        let last_window = if reader.is_empty() {
            Default::default()
        } else {
            <WindowV0>::deserialize(&mut reader)?
        };
        let bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_account,
            authority,
            owner,
            config,
            last_window,
            bump_seed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump_seed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AccountWindowedCircuitBreakerV0Account(pub AccountWindowedCircuitBreakerV0);
impl AccountWindowedCircuitBreakerV0Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCOUNT_WINDOWED_CIRCUIT_BREAKER_V0_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AccountWindowedCircuitBreakerV0::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCOUNT_WINDOWED_CIRCUIT_BREAKER_V0_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TREASURY_MANAGEMENT_V0_ACCOUNT_DISCM: [u8; 8] = [
    68, 111, 120, 209, 16, 110, 215, 59,
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
pub struct TreasuryManagementV0 {
    pub treasury_mint: Pubkey,
    pub supply_mint: Pubkey,
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub curve: Curve,
    pub freeze_unix_time: i64,
    pub bump_seed: u8,
}
impl TreasuryManagementV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let treasury_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supply_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let curve = <Curve as borsh::BorshDeserialize>::deserialize_reader(&mut reader)?;
        let freeze_unix_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            treasury_mint,
            supply_mint,
            authority,
            treasury,
            curve,
            freeze_unix_time,
            bump_seed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.treasury_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.freeze_unix_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump_seed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TreasuryManagementV0Account(pub TreasuryManagementV0);
impl TreasuryManagementV0Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TREASURY_MANAGEMENT_V0_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TreasuryManagementV0::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TREASURY_MANAGEMENT_V0_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
