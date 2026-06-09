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
    pub enabled: bool,
    pub owner: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub token_a_reserves: u128,
    pub token_b_reserves: u128,
    pub shift: u128,
    pub royalties: u64,
    pub vertigo_fees: u64,
    pub bump: u8,
    pub fee_params: FeeParams,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_reserves: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_reserves: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shift: u128 = crate::borsh_de_or_default(&mut reader)?;
        let royalties: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vertigo_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_params = if reader.is_empty() {
            Default::default()
        } else {
            <FeeParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            enabled,
            owner,
            mint_a,
            mint_b,
            token_a_reserves,
            token_b_reserves,
            shift,
            royalties,
            vertigo_fees,
            bump,
            fee_params,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.royalties, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vertigo_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_params, &mut writer)?;
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
