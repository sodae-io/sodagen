use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FEE_VAULT_ACCOUNT_DISCM: [u8; 8] = [192, 178, 69, 232, 58, 149, 157, 132];
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
pub struct FeeVault {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub token_flag: u8,
    pub fee_vault_type: u8,
    pub fee_vault_bump: u8,
    pub padding_0: [u8; 13],
    pub total_share: u32,
    pub padding_1: [u8; 4],
    pub total_funded_fee: u64,
    pub fee_per_share: u128,
    pub base: Pubkey,
    pub padding: [u128; 4],
    pub users: [UserFee; 5],
}
impl FeeVault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 13] = crate::borsh_de_or_default(&mut reader)?;
        let total_share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let total_funded_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_per_share: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 4] = crate::borsh_de_or_default(&mut reader)?;
        let users: [UserFee; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            token_mint,
            token_vault,
            token_flag,
            fee_vault_type,
            fee_vault_bump,
            padding_0,
            total_share,
            padding_1,
            total_funded_fee,
            fee_per_share,
            base,
            padding,
            users,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_funded_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_per_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.users, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeVaultAccount(pub FeeVault);
impl FeeVaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeVault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
