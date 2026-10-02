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
    pub disable_create_pool: bool,
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub protocol_owner: Pubkey,
    pub fund_owner: Pubkey,
    pub creator_fee_rate: u64,
    pub padding: [u64; 15],
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
        let creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 15] = crate::borsh_de_or_default(&mut reader)?;
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
            creator_fee_rate,
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
        borsh::BorshSerialize::serialize(&self.creator_fee_rate, &mut writer)?;
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
    pub last_update_timestamp: u64,
    pub padding: [u64; 3],
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
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            observation_index,
            pool_id,
            observations,
            last_update_timestamp,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.initialized, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observations, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
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
pub const PERMISSION_ACCOUNT_DISCM: [u8; 8] = [224, 83, 28, 79, 10, 253, 161, 28];
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
pub struct Permission {
    pub authority: Pubkey,
    pub padding: [u64; 30],
}
impl Permission {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 30] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { authority, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionAccount(pub Permission);
impl PermissionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERMISSION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Permission::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERMISSION_ACCOUNT_DISCM)?;
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
    pub lp_mint: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub observation_key: Pubkey,
    pub auth_bump: u8,
    pub status: u8,
    pub lp_mint_decimals: u8,
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,
    pub lp_supply: u64,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub creator_fee_on: u8,
    pub enable_creator_fee: bool,
    pub padding1: [u8; 6],
    pub creator_fees_token_0: u64,
    pub creator_fees_token_1: u64,
    pub padding: [u64; 28],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_1_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let observation_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_0_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_1_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_on: u8 = crate::borsh_de_or_default(&mut reader)?;
        let enable_creator_fee: bool = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let creator_fees_token_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fees_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 28] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amm_config,
            pool_creator,
            token_0_vault,
            token_1_vault,
            lp_mint,
            token_0_mint,
            token_1_mint,
            token_0_program,
            token_1_program,
            observation_key,
            auth_bump,
            status,
            lp_mint_decimals,
            mint_0_decimals,
            mint_1_decimals,
            lp_supply,
            protocol_fees_token_0,
            protocol_fees_token_1,
            fund_fees_token_0,
            fund_fees_token_1,
            open_time,
            recent_epoch,
            creator_fee_on,
            enable_creator_fee,
            padding1,
            creator_fees_token_0,
            creator_fees_token_1,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_0_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_1_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fees_token_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_on, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.enable_creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fees_token_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fees_token_1, &mut writer)?;
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
