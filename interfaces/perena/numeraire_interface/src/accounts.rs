use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const NUMERAIRE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [230, 62, 124, 43, 102, 101, 88, 63];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct NumeraireConfig {
    pub owner: Pubkey,
    pub status: u32,
    pub rate_mints: [Pubkey; 10],
    pub rate_nums: [u32; 10],
    pub rate_denoms: [u32; 10],
    pub _padding: [u8; 12],
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 1024],
}
impl NumeraireConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let status: u32 = crate::borsh_de_or_default(&mut reader)?;
        let rate_mints: [Pubkey; 10] = crate::borsh_de_or_default(&mut reader)?;
        let rate_nums: [u32; 10] = crate::borsh_de_or_default(&mut reader)?;
        let rate_denoms: [u32; 10] = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 12] = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 1024] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            owner,
            status,
            rate_mints,
            rate_nums,
            rate_denoms,
            _padding,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_mints, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_nums, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_denoms, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NumeraireConfigAccount(pub NumeraireConfig);
impl NumeraireConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NUMERAIRE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(NumeraireConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NUMERAIRE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STABLE_POOL_ACCOUNT_DISCM: [u8; 8] = [239, 91, 93, 162, 171, 14, 42, 66];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct StablePool {
    pub pool_seed: Pubkey,
    pub lp_mint: Pubkey,
    pub whitelisted_adder: Pubkey,
    pub owner: Pubkey,
    pub inv_t: u64,
    pub inv_t_max: u64,
    pub pairs: [VirtualStablePair; 10],
    pub weights: [u32; 10],
    pub total_weight: u64,
    pub status: u32,
    pub fee_num: u32,
    pub fee_denom: u32,
    pub decimals: u8,
    pub num_stables: u8,
    pub _padding: [u8; 2],
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 128],
}
impl StablePool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelisted_adder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let inv_t: u64 = crate::borsh_de_or_default(&mut reader)?;
        let inv_t_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pairs = <[VirtualStablePair; 10] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let weights: [u32; 10] = crate::borsh_de_or_default(&mut reader)?;
        let total_weight: u64 = crate::borsh_de_or_default(&mut reader)?;
        let status: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_denom: u32 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let num_stables: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_seed,
            lp_mint,
            whitelisted_adder,
            owner,
            inv_t,
            inv_t_max,
            pairs,
            weights,
            total_weight,
            status,
            fee_num,
            fee_denom,
            decimals,
            num_stables,
            _padding,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelisted_adder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inv_t, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inv_t_max, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pairs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.weights, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_denom, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_stables, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StablePoolAccount(pub StablePool);
impl StablePoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STABLE_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StablePool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STABLE_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VIRTUAL_STABLE_PAIR_ACCOUNT_DISCM: [u8; 8] = [
    112, 153, 135, 223, 53, 247, 129, 101,
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
pub struct VirtualStablePair {
    pub pair_authority: Pubkey,
    pub x_reserve_amount: u64,
    pub y_reserve: u64,
    pub curve_amp: u128,
    pub curve_a: u128,
    pub curve_b: u128,
    pub inv_l: u128,
    pub owner: Pubkey,
    pub x_mint: Pubkey,
    pub x_vault: Pubkey,
    pub curve_alpha: u64,
    pub curve_beta: u64,
    pub newest_rate_num: u32,
    pub newest_rate_denom: u32,
    pub decimals: u8,
    pub pair_index: u8,
    pub x_is_2022: u8,
    pub _padding: [u8; 5],
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 128],
}
impl VirtualStablePair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let x_reserve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let y_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_amp: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let inv_l: u128 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let x_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let x_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let curve_alpha: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_beta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let newest_rate_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let newest_rate_denom: u32 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pair_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let x_is_2022: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pair_authority,
            x_reserve_amount,
            y_reserve,
            curve_amp,
            curve_a,
            curve_b,
            inv_l,
            owner,
            x_mint,
            x_vault,
            curve_alpha,
            curve_beta,
            newest_rate_num,
            newest_rate_denom,
            decimals,
            pair_index,
            x_is_2022,
            _padding,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_reserve_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_amp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inv_l, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_alpha, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_beta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.newest_rate_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.newest_rate_denom, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pair_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_is_2022, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualStablePairAccount(pub VirtualStablePair);
impl VirtualStablePairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIRTUAL_STABLE_PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VirtualStablePair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIRTUAL_STABLE_PAIR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
