use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
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
pub struct AddLiquidityData {
    pub max_amounts_in: [u64; 10],
    pub min_lp_token_mint_amount: u64,
    pub take_swaps: u8,
    pub swap_paths: [u8; 10],
    pub swap_amounts: [u64; 10],
}
impl AddLiquidityData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_amounts_in: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let min_lp_token_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let take_swaps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let swap_paths: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        let swap_amounts: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_amounts_in,
            min_lp_token_mint_amount,
            take_swaps,
            swap_paths,
            swap_amounts,
        })
    }
}
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
pub struct CreateStablePoolData {
    pub decimals: u8,
    pub fee_num: u32,
    pub fee_denom: u32,
    pub pool_seed: Pubkey,
    pub weights: [u32; 10],
    pub inv_t: u64,
    pub inv_t_max: u64,
}
impl CreateStablePoolData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_denom: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pool_seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let weights: [u32; 10] = crate::borsh_de_or_default(&mut reader)?;
        let inv_t: u64 = crate::borsh_de_or_default(&mut reader)?;
        let inv_t_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            decimals,
            fee_num,
            fee_denom,
            pool_seed,
            weights,
            inv_t,
            inv_t_max,
        })
    }
}
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
pub struct InitVirtualStablePairData {
    pub decimals: u8,
    pub init_amount: u64,
    pub curve_amp: u128,
    pub curve_a: u128,
    pub curve_b: u128,
    pub curve_alpha: u64,
    pub curve_beta: u64,
    pub pair_seed: Pubkey,
}
impl InitVirtualStablePairData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let init_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_amp: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve_alpha: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_beta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pair_seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            decimals,
            init_amount,
            curve_amp,
            curve_a,
            curve_b,
            curve_alpha,
            curve_beta,
            pair_seed,
        })
    }
}
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
pub struct RemoveLiquidityData {
    pub lp_token_redeem_amount: u64,
    pub min_amounts_out: [u64; 10],
    pub out_index: u8,
}
impl RemoveLiquidityData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_token_redeem_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amounts_out: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_token_redeem_amount,
            min_amounts_out,
            out_index,
        })
    }
}
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
pub struct SetFeeData {
    pub fee_num: u32,
    pub fee_denom: u32,
}
impl SetFeeData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_denom: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee_num, fee_denom })
    }
}
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
pub struct SetInvTMaxData {
    pub inv_t_max: u64,
}
impl SetInvTMaxData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inv_t_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inv_t_max })
    }
}
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
pub struct SetMetadataData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl SetMetadataData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol, uri })
    }
}
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
pub struct SetOwnerData {
    pub owner: Pubkey,
}
impl SetOwnerData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owner })
    }
}
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
pub struct SetRateData {
    pub rate_mint: Pubkey,
    pub rate_num: u32,
    pub rate_denom: u32,
}
impl SetRateData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let rate_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rate_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let rate_denom: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            rate_mint,
            rate_num,
            rate_denom,
        })
    }
}
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
pub struct SetStatusData {
    pub status: u32,
}
impl SetStatusData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let status: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { status })
    }
}
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
pub struct SetWhilelistedAddrData {
    pub whitelisted_addr: Pubkey,
}
impl SetWhilelistedAddrData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whitelisted_addr: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { whitelisted_addr })
    }
}
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
pub struct SwapExactInData {
    pub in_index: u8,
    pub out_index: u8,
    pub exact_amount_in: u64,
    pub min_amount_out: u64,
    pub hints: [u64; 10],
    pub path_hints: [u8; 10],
}
impl SwapExactInData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let in_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let exact_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let hints: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let path_hints: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            in_index,
            out_index,
            exact_amount_in,
            min_amount_out,
            hints,
            path_hints,
        })
    }
}
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
pub struct SwapExactInHintlessData {
    pub in_index: u8,
    pub out_index: u8,
    pub exact_amount_in: u64,
    pub min_amount_out: u64,
}
impl SwapExactInHintlessData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let in_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let exact_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            in_index,
            out_index,
            exact_amount_in,
            min_amount_out,
        })
    }
}
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
pub struct SwapExactOutData {
    pub in_index: u8,
    pub out_index: u8,
    pub exact_amount_out: u64,
    pub max_amount_in: u64,
    pub hints: [u64; 10],
    pub path_hints: [u8; 10],
}
impl SwapExactOutData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let in_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let exact_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let hints: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let path_hints: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            in_index,
            out_index,
            exact_amount_out,
            max_amount_in,
            hints,
            path_hints,
        })
    }
}
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
pub struct SwapExactOutHintlessData {
    pub in_index: u8,
    pub out_index: u8,
    pub exact_amount_out: u64,
    pub max_amount_in: u64,
}
impl SwapExactOutHintlessData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let in_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let out_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let exact_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            in_index,
            out_index,
            exact_amount_out,
            max_amount_in,
        })
    }
}
