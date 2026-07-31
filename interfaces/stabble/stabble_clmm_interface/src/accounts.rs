use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const AMM_CONFIG_ACCOUNT_DISCM: [u8; 8] = [218, 244, 33, 104, 203, 203, 43, 111];
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
pub struct AmmConfig {
    pub bump: u8,
    pub index: u16,
    pub owner: Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub padding_u32: u32,
    pub fund_owner: Pubkey,
    pub padding: [u64; 3],
}
impl AmmConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding_u32: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fund_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            index,
            owner,
            protocol_fee_rate,
            trade_fee_rate,
            tick_spacing,
            fund_fee_rate,
            padding_u32,
            fund_owner,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_u32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmConfigAccount(pub AmmConfig);
impl AmmConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AmmConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OBSERVATION_STATE_ACCOUNT_DISCM: [u8; 8] = [
    122, 174, 197, 53, 129, 9, 165, 132,
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
pub struct ObservationState {
    pub initialized: bool,
    pub recent_epoch: u64,
    pub observation_index: u16,
    pub pool_id: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub observations: [Observation; 100],
    pub padding: [u64; 4],
}
impl ObservationState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let observation_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observations = <[Observation; 100] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            recent_epoch,
            observation_index,
            pool_id,
            observations,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.initialized, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ObservationStateAccount(pub ObservationState);
impl ObservationStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OBSERVATION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ObservationState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OBSERVATION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPERATION_STATE_ACCOUNT_DISCM: [u8; 8] = [19, 236, 58, 237, 81, 222, 183, 252];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OperationState {
    pub bump: u8,
    pub operation_owners: [Pubkey; 10],
    #[serde(with = "crate::big_array_serde")]
    pub whitelist_mints: [Pubkey; 100],
}
impl OperationState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let operation_owners: [Pubkey; 10] = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mints = <[Pubkey; 100] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            operation_owners,
            whitelist_mints,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operation_owners, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelist_mints, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperationStateAccount(pub OperationState);
impl OperationStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPERATION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OperationState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPERATION_STATE_ACCOUNT_DISCM)?;
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
    pub bump: [u8; 1],
    pub nft_mint: Pubkey,
    pub pool_id: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub fee_growth_inside_0_last_x64: u128,
    pub fee_growth_inside_1_last_x64: u128,
    pub token_fees_owed_0: u64,
    pub token_fees_owed_1: u64,
    pub reward_infos: [PositionRewardInfo; 3],
    pub recent_epoch: u64,
    pub padding: [u64; 7],
}
impl PersonalPositionState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside_0_last_x64: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_growth_inside_1_last_x64: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_fees_owed_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_fees_owed_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [PositionRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            nft_mint,
            pool_id,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            fee_growth_inside_0_last_x64,
            fee_growth_inside_1_last_x64,
            token_fees_owed_0,
            token_fees_owed_1,
            reward_infos,
            recent_epoch,
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
            &self.fee_growth_inside_0_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside_1_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
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
    pub bump: [u8; 1],
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_key: Pubkey,
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub padding3: u16,
    pub padding4: u16,
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub swap_in_amount_token_0: u128,
    pub swap_out_amount_token_1: u128,
    pub swap_in_amount_token_1: u128,
    pub swap_out_amount_token_0: u128,
    pub status: u8,
    pub padding: [u8; 7],
    pub reward_infos: [RewardInfo; 3],
    pub tick_array_bitmap: [u64; 16],
    pub total_fees_token_0: u64,
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub padding1: [u64; 24],
    pub padding2: [u64; 32],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observation_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals_1: u8 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_current: i32 = crate::borsh_de_or_default(&mut reader)?;
        let padding3: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding4: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_0_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_1_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_in_amount_token_0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_out_amount_token_1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_in_amount_token_1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_out_amount_token_0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [RewardInfo; 3] = crate::borsh_de_or_default(&mut reader)?;
        let tick_array_bitmap: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_claimed_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fees_claimed_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 24] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            amm_config,
            owner,
            token_mint_0,
            token_mint_1,
            token_vault_0,
            token_vault_1,
            observation_key,
            mint_decimals_0,
            mint_decimals_1,
            tick_spacing,
            liquidity,
            sqrt_price_x64,
            tick_current,
            padding3,
            padding4,
            fee_growth_global_0_x64,
            fee_growth_global_1_x64,
            protocol_fees_token_0,
            protocol_fees_token_1,
            swap_in_amount_token_0,
            swap_out_amount_token_1,
            swap_in_amount_token_1,
            swap_out_amount_token_0,
            status,
            padding,
            reward_infos,
            tick_array_bitmap,
            total_fees_token_0,
            total_fees_claimed_token_0,
            total_fees_token_1,
            total_fees_claimed_token_1,
            fund_fees_token_0,
            fund_fees_token_1,
            open_time,
            recent_epoch,
            padding1,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_current, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_0_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_1_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_in_amount_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_out_amount_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_in_amount_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_out_amount_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_array_bitmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_claimed_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_fees_claimed_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
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
    pub fee_growth_inside_0_last_x64: u128,
    pub fee_growth_inside_1_last_x64: u128,
    pub token_fees_owed_0: u64,
    pub token_fees_owed_1: u64,
    pub reward_growth_inside: [u128; 3],
    pub recent_epoch: u64,
    pub padding: [u64; 7],
}
impl ProtocolPositionState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside_0_last_x64: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_growth_inside_1_last_x64: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_fees_owed_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_fees_owed_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_growth_inside: [u128; 3] = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            pool_id,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            fee_growth_inside_0_last_x64,
            fee_growth_inside_1_last_x64,
            token_fees_owed_0,
            token_fees_owed_1,
            reward_growth_inside,
            recent_epoch,
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
            &self.fee_growth_inside_0_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside_1_last_x64,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_fees_owed_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_growth_inside, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
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
pub const SUPPORT_MINT_ASSOCIATED_ACCOUNT_DISCM: [u8; 8] = [
    134, 40, 183, 79, 12, 112, 162, 53,
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
pub struct SupportMintAssociated {
    pub bump: u8,
    pub mint: Pubkey,
    pub padding: [u64; 8],
}
impl SupportMintAssociated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, mint, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SupportMintAssociatedAccount(pub SupportMintAssociated);
impl SupportMintAssociatedAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SUPPORT_MINT_ASSOCIATED_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SupportMintAssociated::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SUPPORT_MINT_ASSOCIATED_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM: [u8; 8] = [
    60, 150, 36, 219, 97, 128, 139, 153,
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
pub struct TickArrayBitmapExtension {
    pub pool_id: Pubkey,
    pub positive_tick_array_bitmap: [[u64; 8]; 14],
    pub negative_tick_array_bitmap: [[u64; 8]; 14],
}
impl TickArrayBitmapExtension {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let positive_tick_array_bitmap: [[u64; 8]; 14] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let negative_tick_array_bitmap: [[u64; 8]; 14] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            positive_tick_array_bitmap,
            negative_tick_array_bitmap,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.positive_tick_array_bitmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.negative_tick_array_bitmap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayBitmapExtensionAccount(pub TickArrayBitmapExtension);
impl TickArrayBitmapExtensionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickArrayBitmapExtension::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ARRAY_STATE_ACCOUNT_DISCM: [u8; 8] = [
    192, 155, 85, 205, 49, 249, 129, 42,
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
pub struct TickArrayState {
    pub pool_id: Pubkey,
    pub start_tick_index: i32,
    #[serde(with = "crate::big_array_serde")]
    pub ticks: [TickState; 60],
    pub initialized_tick_count: u8,
    pub recent_epoch: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 107],
}
impl TickArrayState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks = <[TickState; 60] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let initialized_tick_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 107] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            start_tick_index,
            ticks,
            initialized_tick_count,
            recent_epoch,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initialized_tick_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayStateAccount(pub TickArrayState);
impl TickArrayStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickArrayState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
