use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const PAIR_STATE_ACCOUNT_DISCM: [u8; 8] = [229, 212, 222, 222, 191, 128, 176, 235];
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
pub struct PairState {
    pub enabled: bool,
    pub graduated: bool,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub token_a_reserves: u128,
    pub token_b_reserves: u128,
    pub shift: u128,
    pub curve: CurveType,
    pub fee_beneficiary_count: u8,
    pub fee_beneficiaries: [FeeBeneficiary; 5],
    pub amm_pool: Pubkey,
    pub bump: u8,
}
impl PairState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let graduated: bool = crate::borsh_de_or_default(&mut reader)?;
        let mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_reserves: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_reserves: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shift: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve: CurveType = crate::borsh_de_or_default(&mut reader)?;
        let fee_beneficiary_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_beneficiaries: [FeeBeneficiary; 5] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amm_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            enabled,
            graduated,
            mint_a,
            mint_b,
            token_a_reserves,
            token_b_reserves,
            shift,
            curve,
            fee_beneficiary_count,
            fee_beneficiaries,
            amm_pool,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.graduated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_beneficiary_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_beneficiaries, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairStateAccount(pub PairState);
impl PairStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PairState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PLATFORM_CONFIG_ACCOUNT_DISCM: [u8; 8] = [160, 78, 128, 0, 248, 83, 230, 160];
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
pub struct PlatformConfig {
    pub authority: Pubkey,
    pub fee_beneficiary: Pubkey,
    pub base_token: Pubkey,
    pub platform_fee_bps: u16,
    pub graduation_threshold: u64,
    pub bump: u8,
}
impl PlatformConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let graduation_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            fee_beneficiary,
            base_token,
            platform_fee_bps,
            graduation_threshold,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.graduation_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlatformConfigAccount(pub PlatformConfig);
impl PlatformConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLATFORM_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PlatformConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLATFORM_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
