use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
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
pub struct Pool {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub authority_bump: u8,
    pub is_active: bool,
    pub invariant: u64,
    pub swap_fee: u64,
    pub tokens: Vec<PoolToken>,
    pub pending_owner: Option<Pubkey>,
    pub max_supply: u64,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_active: bool = crate::borsh_de_or_default(&mut reader)?;
        let invariant: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens: Vec<PoolToken> = crate::borsh_de_or_default(&mut reader)?;
        let pending_owner: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            vault,
            mint,
            authority_bump,
            is_active,
            invariant,
            swap_fee,
            tokens,
            pending_owner,
            max_supply,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_active, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.invariant, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supply, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
    pub admin: Pubkey,
    pub withdraw_authority: Pubkey,
    pub withdraw_authority_bump: u8,
    pub authority_bump: u8,
    pub is_active: bool,
    pub beneficiary: Pubkey,
    pub beneficiary_fee: u64,
    pub pending_admin: Option<Pubkey>,
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_active: bool = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            withdraw_authority,
            withdraw_authority_bump,
            authority_bump,
            is_active,
            beneficiary,
            beneficiary_fee,
            pending_admin,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_active, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_admin, &mut writer)?;
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
