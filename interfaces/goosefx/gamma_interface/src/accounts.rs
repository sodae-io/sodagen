use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ACTOR_ACCOUNT_DISCM: [u8; 8] = [46, 77, 47, 204, 204, 54, 34, 88];
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
pub struct Actor {
    pub authority: Pubkey,
    pub is_admin: bool,
}
impl Actor {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_admin: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { authority, is_admin })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_admin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ActorAccount(pub Actor);
impl ActorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACTOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Actor::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACTOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
    pub disable_create_pool: bool,
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub protocol_owner: Pubkey,
    pub fund_owner: Pubkey,
    pub referral_project: Pubkey,
    pub max_open_time: u64,
    pub secondary_admin: Pubkey,
    pub padding: [u64; 7],
}
impl AmmConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let disable_create_pool: bool = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fund_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referral_project: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let secondary_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            disable_create_pool,
            index,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
            create_pool_fee,
            protocol_owner,
            fund_owner,
            referral_project,
            max_open_time,
            secondary_admin,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_create_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral_project, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.secondary_admin, &mut writer)?;
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
        let observation_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observations = <[Observation; 100] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            observation_index,
            pool_id,
            observations,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.initialized, &mut writer)?;
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
pub const PARTNER_ACCOUNT_DISCM: [u8; 8] = [122, 43, 246, 239, 141, 56, 243, 182];
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
pub struct Partner {
    pub name: String,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_token_account: Pubkey,
    pub token_1_token_account: Pubkey,
}
impl Partner {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            authority,
            pool_state,
            token_0_token_account,
            token_1_token_account,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_token_account, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartnerAccount(pub Partner);
impl PartnerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTNER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Partner::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTNER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_PARTNER_INFOS_ACCOUNT_DISCM: [u8; 8] = [
    130, 98, 226, 231, 158, 49, 44, 231,
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
pub struct PoolPartnerInfos {
    pub last_observed_fee_amount_token_0: u64,
    pub last_observed_fee_amount_token_1: u64,
    pub infos: [PartnerInfo; 5],
}
impl PoolPartnerInfos {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_observed_fee_amount_token_0: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_observed_fee_amount_token_1: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let infos: [PartnerInfo; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_observed_fee_amount_token_0,
            last_observed_fee_amount_token_1,
            infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(
            &self.last_observed_fee_amount_token_0,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.last_observed_fee_amount_token_1,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolPartnerInfosAccount(pub PoolPartnerInfos);
impl PoolPartnerInfosAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_PARTNER_INFOS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolPartnerInfos::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_PARTNER_INFOS_ACCOUNT_DISCM)?;
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
    pub amm_config: Pubkey,
    pub pool_creator: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub h1: u128,
    pub h2: u64,
    pub h3: u32,
    pub h4: u32,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub observation_key: Pubkey,
    pub auth_bump: u8,
    pub status: u8,
    pub padding2: u8,
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,
    pub lp_supply: u64,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub cumulative_trade_fees_token_0: u128,
    pub cumulative_trade_fees_token_1: u128,
    pub cumulative_volume_token_0: u128,
    pub cumulative_volume_token_1: u128,
    pub latest_dynamic_fee_rate: u64,
    pub max_trade_fee_rate: u64,
    pub volatility_factor: u64,
    pub token_0_vault_amount: u64,
    pub token_1_vault_amount: u64,
    pub max_shared_token0: u64,
    pub max_shared_token1: u64,
    pub h16: u32,
    pub h15: u32,
    pub h14: u32,
    pub h13: u32,
    pub h12: u32,
    pub h10: u32,
    pub padding3: [u8; 8],
    pub token_0_amount_in_kamino: u64,
    pub token_1_amount_in_kamino: u64,
    pub withdrawn_kamino_profit_token_0: u64,
    pub withdrawn_kamino_profit_token_1: u64,
    pub partner_share_rate: u64,
    pub partner_protocol_fees_token_0: u64,
    pub partner_protocol_fees_token_1: u64,
    pub min_swap_at_spot_price: u64,
    pub max_swap_at_spot_price: u64,
    pub h20: u128,
    pub padding: [u64; 1],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let h1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let h2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let h3: u32 = crate::borsh_de_or_default(&mut reader)?;
        let h4: u32 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observation_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_0_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_1_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_trade_fees_token_0: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_trade_fees_token_1: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_volume_token_0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_volume_token_1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let latest_dynamic_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_vault_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_vault_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_shared_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_shared_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let h16: u32 = crate::borsh_de_or_default(&mut reader)?;
        let h15: u32 = crate::borsh_de_or_default(&mut reader)?;
        let h14: u32 = crate::borsh_de_or_default(&mut reader)?;
        let h13: u32 = crate::borsh_de_or_default(&mut reader)?;
        let h12: u32 = crate::borsh_de_or_default(&mut reader)?;
        let h10: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding3: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amount_in_kamino: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amount_in_kamino: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawn_kamino_profit_token_0: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawn_kamino_profit_token_1: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let partner_share_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_protocol_fees_token_0: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let partner_protocol_fees_token_1: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_swap_at_spot_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_swap_at_spot_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let h20: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 1] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amm_config,
            pool_creator,
            token_0_vault,
            token_1_vault,
            h1,
            h2,
            h3,
            h4,
            token_0_mint,
            token_1_mint,
            token_0_program,
            token_1_program,
            observation_key,
            auth_bump,
            status,
            padding2,
            mint_0_decimals,
            mint_1_decimals,
            lp_supply,
            protocol_fees_token_0,
            protocol_fees_token_1,
            fund_fees_token_0,
            fund_fees_token_1,
            open_time,
            recent_epoch,
            cumulative_trade_fees_token_0,
            cumulative_trade_fees_token_1,
            cumulative_volume_token_0,
            cumulative_volume_token_1,
            latest_dynamic_fee_rate,
            max_trade_fee_rate,
            volatility_factor,
            token_0_vault_amount,
            token_1_vault_amount,
            max_shared_token0,
            max_shared_token1,
            h16,
            h15,
            h14,
            h13,
            h12,
            h10,
            padding3,
            token_0_amount_in_kamino,
            token_1_amount_in_kamino,
            withdrawn_kamino_profit_token_0,
            withdrawn_kamino_profit_token_1,
            partner_share_rate,
            partner_protocol_fees_token_0,
            partner_protocol_fees_token_1,
            min_swap_at_spot_price,
            max_swap_at_spot_price,
            h20,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_0_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_1_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_trade_fees_token_0,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_trade_fees_token_1,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.cumulative_volume_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cumulative_volume_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.latest_dynamic_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.volatility_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_vault_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_vault_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_shared_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_shared_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h16, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h15, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h14, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h13, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h12, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h10, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amount_in_kamino, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amount_in_kamino, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.withdrawn_kamino_profit_token_0,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdrawn_kamino_profit_token_1,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.partner_share_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.partner_protocol_fees_token_0,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.partner_protocol_fees_token_1,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_swap_at_spot_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_swap_at_spot_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.h20, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
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
pub const REWARD_INFO_ACCOUNT_DISCM: [u8; 8] = [39, 7, 129, 22, 241, 96, 83, 133];
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
pub struct RewardInfo {
    pub pool: Pubkey,
    pub start_at: u64,
    pub end_rewards_at: u64,
    pub mint: Pubkey,
    pub total_to_disburse: u64,
    pub rewarded_by: Pubkey,
    pub amount_disbursed: u64,
}
impl RewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_rewards_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_to_disburse: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewarded_by: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_disbursed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            start_at,
            end_rewards_at,
            mint,
            total_to_disburse,
            rewarded_by,
            amount_disbursed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_rewards_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_to_disburse, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewarded_by, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_disbursed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RewardInfoAccount(pub RewardInfo);
impl RewardInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REWARD_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RewardInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REWARD_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_POOL_LIQUIDITY_ACCOUNT_DISCM: [u8; 8] = [0, 141, 89, 29, 236, 6, 14, 15];
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
pub struct UserPoolLiquidity {
    pub user: Pubkey,
    pub pool_state: Pubkey,
    pub token_0_deposited: u128,
    pub token_1_deposited: u128,
    pub token_0_withdrawn: u128,
    pub token_1_withdrawn: u128,
    pub lp_tokens_owned: u128,
    pub p1: u64,
    pub p2: u8,
    pub first_investment_at: u64,
    pub partner: Option<Pubkey>,
    pub padding: [u8; 15],
}
impl UserPoolLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_deposited: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_deposited: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_withdrawn: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_withdrawn: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lp_tokens_owned: u128 = crate::borsh_de_or_default(&mut reader)?;
        let p1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let p2: u8 = crate::borsh_de_or_default(&mut reader)?;
        let first_investment_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 15] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            pool_state,
            token_0_deposited,
            token_1_deposited,
            token_0_withdrawn,
            token_1_withdrawn,
            lp_tokens_owned,
            p1,
            p2,
            first_investment_at,
            partner,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_tokens_owned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.p1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.p2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.first_investment_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserPoolLiquidityAccount(pub UserPoolLiquidity);
impl UserPoolLiquidityAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_POOL_LIQUIDITY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserPoolLiquidity::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_POOL_LIQUIDITY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_REWARD_INFO_ACCOUNT_DISCM: [u8; 8] = [
    110, 57, 251, 139, 250, 236, 213, 178,
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
pub struct UserRewardInfo {
    pub user: Pubkey,
    pub reward_info: Pubkey,
    pub pool_state: Pubkey,
    pub total_claimed: u64,
    pub total_rewards: u64,
    pub rewards_last_calculated_at: u64,
}
impl UserRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_info: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_last_calculated_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            reward_info,
            pool_state,
            total_claimed,
            total_rewards,
            rewards_last_calculated_at,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_last_calculated_at, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserRewardInfoAccount(pub UserRewardInfo);
impl UserRewardInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_REWARD_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserRewardInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_REWARD_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
