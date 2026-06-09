use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const WHIRLPOOL_ACCOUNT_DISCM: [u8; 8] = [63, 149, 209, 12, 225, 128, 99, 9];
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
pub struct Whirlpool {
    pub whirlpools_config: Pubkey,
    pub whirlpool_bump: [u8; 1],
    pub tick_spacing: u16,
    pub tick_spacing_seed: [u8; 2],
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,
    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub fee_growth_global_a: u128,
    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub fee_growth_global_b: u128,
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [WhirlpoolRewardInfo; 3],
}
impl Whirlpool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whirlpool_bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing_seed: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_current_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_last_updated_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_infos: [WhirlpoolRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpools_config,
            whirlpool_bump,
            tick_spacing,
            tick_spacing_seed,
            fee_rate,
            protocol_fee_rate,
            liquidity,
            sqrt_price,
            tick_current_index,
            protocol_fee_owed_a,
            protocol_fee_owed_b,
            token_mint_a,
            token_vault_a,
            fee_growth_global_a,
            token_mint_b,
            token_vault_b,
            fee_growth_global_b,
            reward_last_updated_timestamp,
            reward_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whirlpool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_current_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_b, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.reward_last_updated_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolAccount(pub Whirlpool);
impl WhirlpoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Whirlpool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_ACCOUNT_DISCM: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
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
pub struct Position {
    pub whirlpool: Pubkey,
    pub position_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,
    pub reward_infos: [PositionRewardInfo; 3],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_checkpoint_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_checkpoint_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [PositionRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            position_mint,
            liquidity,
            tick_lower_index,
            tick_upper_index,
            fee_growth_checkpoint_a,
            fee_owed_a,
            fee_growth_checkpoint_b,
            fee_owed_b,
            reward_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_checkpoint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_checkpoint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionAccount(pub Position);
impl PositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Position::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_ACCOUNT_DISCM)?;
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
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PoolState {
    pub bump: u8,
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub token_mint0: Pubkey,
    pub token_mint1: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub observation_key: Pubkey,
    pub mint_decimals0: u8,
    pub mint_decimals1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub observation_index: u16,
    pub observation_update_duration: u16,
    pub fee_growth_global0_x64: u128,
    pub fee_growth_global1_x64: u128,
    pub protocol_fees_token0: u64,
    pub protocol_fees_token1: u64,
    pub swap_in_amount_token0: u128,
    pub swap_out_amount_token1: u128,
    pub swap_in_amount_token1: u128,
    pub swap_out_amount_token0: u128,
    pub status: u8,
    pub padding: [u8; 7],
    pub reward_infos: [RewardInfo; 3],
    pub tick_array_bitmap: [u64; 16],
    pub total_fees_token0: u64,
    pub total_fees_claimed_token0: u64,
    pub total_fees_token1: u64,
    pub total_fees_claimed_token1: u64,
    pub fund_fees_token0: u64,
    pub fund_fees_token1: u64,
    pub open_time: u64,
    pub padding1: [u64; 25],
    pub padding2: [u64; 32],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observation_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals1: u8 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_current: i32 = crate::borsh_de_or_default(&mut reader)?;
        let observation_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let observation_update_duration: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global0_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global1_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_in_amount_token0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_out_amount_token1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_in_amount_token1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_out_amount_token0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [RewardInfo; 3] = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_bitmap: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_claimed_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_claimed_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 25] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            amm_config,
            owner,
            token_mint0,
            token_mint1,
            token_vault0,
            token_vault1,
            observation_key,
            mint_decimals0,
            mint_decimals1,
            tick_spacing,
            liquidity,
            sqrt_price_x64,
            tick_current,
            observation_index,
            observation_update_duration,
            fee_growth_global0_x64,
            fee_growth_global1_x64,
            protocol_fees_token0,
            protocol_fees_token1,
            swap_in_amount_token0,
            swap_out_amount_token1,
            swap_in_amount_token1,
            swap_out_amount_token0,
            status,
            padding,
            reward_infos,
            tick_array_bitmap,
            total_fees_token0,
            total_fees_claimed_token0,
            total_fees_token1,
            total_fees_claimed_token1,
            fund_fees_token0,
            fund_fees_token1,
            open_time,
            padding1,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_current, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_index, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.observation_update_duration,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global0_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global1_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_in_amount_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_out_amount_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_in_amount_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_out_amount_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_array_bitmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_claimed_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_claimed_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
pub const PERSONAL_POSITION_STATE_ACCOUNT_DISCM: [u8; 8] = [
    70, 111, 150, 126, 230, 15, 25, 117,
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
pub struct PersonalPositionState {
    pub bump: u8,
    pub nft_mint: Pubkey,
    pub pool_id: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub fee_growth_inside0_last_x64: u128,
    pub fee_growth_inside1_last_x64: u128,
    pub token_fees_owed0: u64,
    pub token_fees_owed1: u64,
    pub reward_infos: [PositionRewardInfo; 3],
    pub padding: [u64; 8],
}
impl PersonalPositionState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside0_last_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside1_last_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_fees_owed0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_fees_owed1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [PositionRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            nft_mint,
            pool_id,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            fee_growth_inside0_last_x64,
            fee_growth_inside1_last_x64,
            token_fees_owed0,
            token_fees_owed1,
            reward_infos,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside0_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside1_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PersonalPositionStateAccount(pub PersonalPositionState);
impl PersonalPositionStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERSONAL_POSITION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PersonalPositionState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERSONAL_POSITION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_POSITION_STATE_ACCOUNT_DISCM: [u8; 8] = [
    100, 226, 145, 99, 146, 218, 160, 106,
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
pub struct ProtocolPositionState {
    pub bump: u8,
    pub pool_id: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub fee_growth_inside0_last_x64: u128,
    pub fee_growth_inside1_last_x64: u128,
    pub token_fees_owed0: u64,
    pub token_fees_owed1: u64,
    pub reward_growth_inside: [u128; 3],
    pub padding: [u64; 8],
}
impl ProtocolPositionState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside0_last_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside1_last_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_fees_owed0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_fees_owed1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_growth_inside: [u128; 3] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            pool_id,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            fee_growth_inside0_last_x64,
            fee_growth_inside1_last_x64,
            token_fees_owed0,
            token_fees_owed1,
            reward_growth_inside,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside0_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside1_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_growth_inside, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolPositionStateAccount(pub ProtocolPositionState);
impl ProtocolPositionStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_POSITION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProtocolPositionState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_POSITION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHIRLPOOL_STRATEGY_ACCOUNT_DISCM: [u8; 8] = [
    190, 178, 231, 184, 49, 186, 103, 13,
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
pub struct WhirlpoolStrategy {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub base_vault_authority: Pubkey,
    pub base_vault_authority_bump: u64,
    pub pool: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_metadata: Pubkey,
    pub position_token_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub deprecated0: [Pubkey; 2],
    pub deprecated1: [u64; 2],
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_mint_decimals: u64,
    pub token_b_mint_decimals: u64,
    pub token_a_amounts: u64,
    pub token_b_amounts: u64,
    pub token_a_collateral_id: u64,
    pub token_b_collateral_id: u64,
    pub deprecated2: Pubkey,
    pub deprecated3: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_decimals: u64,
    pub shares_mint_authority: Pubkey,
    pub shares_mint_authority_bump: u64,
    pub shares_issued: u64,
    pub status: u64,
    pub reward0_amount: u64,
    pub reward0_vault: Pubkey,
    pub reward0_collateral_id: u64,
    pub reward0_decimals: u64,
    pub reward1_amount: u64,
    pub reward1_vault: Pubkey,
    pub reward1_collateral_id: u64,
    pub reward1_decimals: u64,
    pub reward2_amount: u64,
    pub reward2_vault: Pubkey,
    pub reward2_collateral_id: u64,
    pub reward2_decimals: u64,
    pub deposit_cap_usd: u64,
    pub fees_a_cumulative: u64,
    pub fees_b_cumulative: u64,
    pub reward0_amount_cumulative: u64,
    pub reward1_amount_cumulative: u64,
    pub reward2_amount_cumulative: u64,
    pub deposit_cap_usd_per_ixn: u64,
    pub withdrawal_cap_a: WithdrawalCaps,
    pub withdrawal_cap_b: WithdrawalCaps,
    pub max_price_deviation_bps: u64,
    pub swap_vault_max_slippage_bps: u32,
    pub swap_vault_max_slippage_from_reference_bps: u32,
    pub strategy_type: u64,
    pub padding0: u64,
    pub withdraw_fee: u64,
    pub fees_fee: u64,
    pub reward0_fee: u64,
    pub reward1_fee: u64,
    pub reward2_fee: u64,
    pub position_timestamp: u64,
    pub kamino_rewards: [KaminoRewardInfo; 3],
    pub strategy_dex: u64,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub allow_deposit_without_invest: u64,
    pub raydium_pool_config_or_base_vault_authority: Pubkey,
    pub deposit_blocked: u8,
    pub creation_status: u8,
    pub invest_blocked: u8,
    pub share_calculation_method: u8,
    pub withdraw_blocked: u8,
    pub reserved_flag2: u8,
    pub local_admin_blocked: u8,
    pub flash_vault_swap_allowed: u8,
    pub reference_swap_price_a: Price,
    pub reference_swap_price_b: Price,
    pub is_community: u8,
    pub rebalance_type: u8,
    pub flash_swap_in_progress: u8,
    pub padding1: [u8; 5],
    pub rebalance_raw: RebalanceRaw,
    pub padding2: [u8; 7],
    pub token_a_fees_from_rewards_cumulative: u64,
    pub token_b_fees_from_rewards_cumulative: u64,
    pub strategy_lookup_table: Pubkey,
    pub last_swap_uneven_step_timestamp: u64,
    pub farm: Pubkey,
    pub rebalances_cap: WithdrawalCaps,
    pub padding3_non_zeroed: [u64; 4],
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub pending_admin: Pubkey,
    pub max_deviation_from_ref_price_on_invest_bps: u32,
    pub padding3: u32,
    pub last_invest_slot: u64,
    pub padding4: u64,
    pub padding5: [u128; 12],
    pub padding6: [u128; 32],
    pub padding7: [u128; 32],
    pub padding8: [u128; 32],
}
impl WhirlpoolStrategy {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault_authority_bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_token_vault_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_token_vault_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_lower: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_upper: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_metadata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deprecated0: [Pubkey; 2] = crate::borsh_de_or_default(&mut reader)?;
        let deprecated1: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let token_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_mint_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amounts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amounts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deprecated2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deprecated3: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares_mint_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares_mint_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares_mint_authority_bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares_issued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let status: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward0_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward0_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward0_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward1_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward1_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward1_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward2_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward2_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward2_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward2_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_cap_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_a_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_b_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward0_amount_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward1_amount_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward2_amount_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_cap_usd_per_ixn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_cap_a = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalCaps>::deserialize(&mut reader)?
        };
        let withdrawal_cap_b = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalCaps>::deserialize(&mut reader)?
        };
        let max_price_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_vault_max_slippage_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let swap_vault_max_slippage_from_reference_bps: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_type: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward0_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward1_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward2_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let position_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let kamino_rewards: [KaminoRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_dex: u64 = crate::borsh_de_or_default(&mut reader)?;
        let raydium_protocol_position_or_base_vault_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let allow_deposit_without_invest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let raydium_pool_config_or_base_vault_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposit_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creation_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let invest_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let share_calculation_method: u8 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_flag2: u8 = crate::borsh_de_or_default(&mut reader)?;
        let local_admin_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let flash_vault_swap_allowed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reference_swap_price_a = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        let reference_swap_price_b = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        let is_community: u8 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let flash_swap_in_progress: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_raw = <RebalanceRaw>::deserialize(&mut reader)?;
        let padding2: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let token_a_fees_from_rewards_cumulative: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_b_fees_from_rewards_cumulative: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_lookup_table: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_swap_uneven_step_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let farm: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rebalances_cap = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalCaps>::deserialize(&mut reader)?
        };
        let padding3_non_zeroed: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let token_a_token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_deviation_from_ref_price_on_invest_bps: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding3: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_invest_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding5: [u128; 12] = crate::borsh_de_or_default(&mut reader)?;
        let padding6: [u128; 32] = crate::borsh_de_or_default(&mut reader)?;
        let padding7: [u128; 32] = crate::borsh_de_or_default(&mut reader)?;
        let padding8: [u128; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin_authority,
            global_config,
            base_vault_authority,
            base_vault_authority_bump,
            pool,
            pool_token_vault_a,
            pool_token_vault_b,
            tick_array_lower,
            tick_array_upper,
            position,
            position_mint,
            position_metadata,
            position_token_account,
            token_a_vault,
            token_b_vault,
            deprecated0,
            deprecated1,
            token_a_mint,
            token_b_mint,
            token_a_mint_decimals,
            token_b_mint_decimals,
            token_a_amounts,
            token_b_amounts,
            token_a_collateral_id,
            token_b_collateral_id,
            deprecated2,
            deprecated3,
            shares_mint,
            shares_mint_decimals,
            shares_mint_authority,
            shares_mint_authority_bump,
            shares_issued,
            status,
            reward0_amount,
            reward0_vault,
            reward0_collateral_id,
            reward0_decimals,
            reward1_amount,
            reward1_vault,
            reward1_collateral_id,
            reward1_decimals,
            reward2_amount,
            reward2_vault,
            reward2_collateral_id,
            reward2_decimals,
            deposit_cap_usd,
            fees_a_cumulative,
            fees_b_cumulative,
            reward0_amount_cumulative,
            reward1_amount_cumulative,
            reward2_amount_cumulative,
            deposit_cap_usd_per_ixn,
            withdrawal_cap_a,
            withdrawal_cap_b,
            max_price_deviation_bps,
            swap_vault_max_slippage_bps,
            swap_vault_max_slippage_from_reference_bps,
            strategy_type,
            padding0,
            withdraw_fee,
            fees_fee,
            reward0_fee,
            reward1_fee,
            reward2_fee,
            position_timestamp,
            kamino_rewards,
            strategy_dex,
            raydium_protocol_position_or_base_vault_authority,
            allow_deposit_without_invest,
            raydium_pool_config_or_base_vault_authority,
            deposit_blocked,
            creation_status,
            invest_blocked,
            share_calculation_method,
            withdraw_blocked,
            reserved_flag2,
            local_admin_blocked,
            flash_vault_swap_allowed,
            reference_swap_price_a,
            reference_swap_price_b,
            is_community,
            rebalance_type,
            flash_swap_in_progress,
            padding1,
            rebalance_raw,
            padding2,
            token_a_fees_from_rewards_cumulative,
            token_b_fees_from_rewards_cumulative,
            strategy_lookup_table,
            last_swap_uneven_step_timestamp,
            farm,
            rebalances_cap,
            padding3_non_zeroed,
            token_a_token_program,
            token_b_token_program,
            pending_admin,
            max_deviation_from_ref_price_on_invest_bps,
            padding3,
            last_invest_slot,
            padding4,
            padding5,
            padding6,
            padding7,
            padding8,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_token_vault_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_token_vault_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_array_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_array_upper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deprecated0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deprecated1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deprecated2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deprecated3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_mint_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_mint_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_issued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward0_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward0_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward0_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward0_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward1_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward1_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward1_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward1_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward2_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward2_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward2_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward2_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_cap_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_a_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_b_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward0_amount_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward1_amount_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward2_amount_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_cap_usd_per_ixn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_cap_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_cap_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_price_deviation_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.swap_vault_max_slippage_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.swap_vault_max_slippage_from_reference_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward0_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward1_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward2_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.kamino_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_dex, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.raydium_protocol_position_or_base_vault_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.allow_deposit_without_invest,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.raydium_pool_config_or_base_vault_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.deposit_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creation_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.invest_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.share_calculation_method, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_flag2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.local_admin_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flash_vault_swap_allowed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reference_swap_price_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reference_swap_price_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_community, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flash_swap_in_progress, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalance_raw, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.token_a_fees_from_rewards_cumulative,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.token_b_fees_from_rewards_cumulative,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.strategy_lookup_table, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.last_swap_uneven_step_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.farm, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rebalances_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3_non_zeroed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_token_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_token_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.max_deviation_from_ref_price_on_invest_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_invest_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding6, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding7, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding8, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolStrategyAccount(pub WhirlpoolStrategy);
impl WhirlpoolStrategyAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOL_STRATEGY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WhirlpoolStrategy::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOL_STRATEGY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GLOBAL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalConfig {
    pub emergency_mode: u64,
    pub block_deposit: u64,
    pub block_invest: u64,
    pub block_withdraw: u64,
    pub block_collect_fees: u64,
    pub block_collect_rewards: u64,
    pub block_swap_rewards: u64,
    pub block_swap_uneven_vaults: u32,
    pub block_emergency_swap: u32,
    pub min_withdrawal_fee_bps: u64,
    pub scope_program_id: Pubkey,
    pub deprecated: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding0_non_zeroed: [u64; 256],
    pub actions_authority: Pubkey,
    pub admin_authority: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub treasury_fee_vaults: [Pubkey; 256],
    pub token_infos: Pubkey,
    pub block_local_admin: u64,
    pub min_performance_fee_bps: u64,
    pub min_swap_uneven_slippage_tolerance_bps: u64,
    pub min_reference_price_slippage_tolerance_bps: u64,
    pub actions_after_rebalance_delay_seconds: u64,
    pub treasury_fee_vault_receiver: Pubkey,
    pub scope_price_ids: [Pubkey; 16],
    pub max_deviation_from_ref_price_on_invest_bps: u32,
    pub padding1: u32,
    pub invest_cooldown_slots: u64,
    pub min_invest_trigger_value_usd: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u64; 1968],
}
impl GlobalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let emergency_mode: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_invest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_withdraw: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_collect_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_collect_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_swap_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let block_swap_uneven_vaults: u32 = crate::borsh_de_or_default(&mut reader)?;
        let block_emergency_swap: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_withdrawal_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scope_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deprecated: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding0_non_zeroed = <[u64; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let actions_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury_fee_vaults = <[Pubkey; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let token_infos: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let block_local_admin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_performance_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_swap_uneven_slippage_tolerance_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_reference_price_slippage_tolerance_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let actions_after_rebalance_delay_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let treasury_fee_vault_receiver: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let scope_price_ids: [Pubkey; 16] = crate::borsh_de_or_default(&mut reader)?;
        let max_deviation_from_ref_price_on_invest_bps: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding1: u32 = crate::borsh_de_or_default(&mut reader)?;
        let invest_cooldown_slots: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_invest_trigger_value_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u64; 1968] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            emergency_mode,
            block_deposit,
            block_invest,
            block_withdraw,
            block_collect_fees,
            block_collect_rewards,
            block_swap_rewards,
            block_swap_uneven_vaults,
            block_emergency_swap,
            min_withdrawal_fee_bps,
            scope_program_id,
            deprecated,
            padding0_non_zeroed,
            actions_authority,
            admin_authority,
            treasury_fee_vaults,
            token_infos,
            block_local_admin,
            min_performance_fee_bps,
            min_swap_uneven_slippage_tolerance_bps,
            min_reference_price_slippage_tolerance_bps,
            actions_after_rebalance_delay_seconds,
            treasury_fee_vault_receiver,
            scope_price_ids,
            max_deviation_from_ref_price_on_invest_bps,
            padding1,
            invest_cooldown_slots,
            min_invest_trigger_value_usd,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.emergency_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_invest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_withdraw, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_collect_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_collect_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_swap_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_swap_uneven_vaults, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_emergency_swap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_withdrawal_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.scope_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deprecated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0_non_zeroed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.actions_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_fee_vaults, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_local_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_performance_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_swap_uneven_slippage_tolerance_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.min_reference_price_slippage_tolerance_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.actions_after_rebalance_delay_seconds,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.treasury_fee_vault_receiver,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.scope_price_ids, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.max_deviation_from_ref_price_on_invest_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.invest_cooldown_slots, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_invest_trigger_value_usd,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigAccount(pub GlobalConfig);
impl GlobalConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLATERAL_INFOS_ACCOUNT_DISCM: [u8; 8] = [127, 210, 52, 226, 74, 169, 111, 9];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CollateralInfos {
    #[serde(with = "crate::big_array_serde")]
    pub infos: [CollateralInfo; 303],
}
impl CollateralInfos {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let infos = <[CollateralInfo; 303] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { infos })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollateralInfosAccount(pub CollateralInfos);
impl CollateralInfosAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLATERAL_INFOS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CollateralInfos::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLATERAL_INFOS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SCOPE_CHAIN_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [
    180, 51, 138, 247, 240, 173, 119, 79,
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
pub struct ScopeChainAccount {
    #[serde(with = "crate::big_array_serde")]
    pub chain_array: [[u16; 4]; 512],
}
impl ScopeChainAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let chain_array = <[[u16; 4]; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { chain_array })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.chain_array, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ScopeChainAccountAccount(pub ScopeChainAccount);
impl ScopeChainAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SCOPE_CHAIN_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ScopeChainAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SCOPE_CHAIN_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TERMS_SIGNATURE_ACCOUNT_DISCM: [u8; 8] = [197, 173, 136, 91, 182, 49, 113, 19];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TermsSignature {
    #[serde(with = "crate::big_array_serde")]
    pub signature: [u8; 64],
}
impl TermsSignature {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signature = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { signature })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.signature, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TermsSignatureAccount(pub TermsSignature);
impl TermsSignatureAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TERMS_SIGNATURE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TermsSignature::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TERMS_SIGNATURE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
