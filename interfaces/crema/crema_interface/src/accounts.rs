use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CLMM_CONFIG_ACCOUNT_DISCM: [u8; 8] = [40, 174, 244, 248, 111, 209, 177, 215];
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
pub struct ClmmConfig {
    pub protocol_authority: Pubkey,
    pub protocol_fee_claim_authority: Pubkey,
    pub protocol_fee_rate: u16,
    pub pending_authority: Pubkey,
    pub create_pool_authority: Pubkey,
}
impl ClmmConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_claim_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pending_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol_authority,
            protocol_fee_claim_authority,
            protocol_fee_rate,
            pending_authority,
            create_pool_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.protocol_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_fee_claim_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClmmConfigAccount(pub ClmmConfig);
impl ClmmConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLMM_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ClmmConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLMM_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLMMPOOL_METADATA_ACCOUNT_DISCM: [u8; 8] = [
    36, 153, 240, 153, 178, 88, 255, 38,
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
pub struct ClmmpoolMetadata {
    pub clmmpool: Pubkey,
    pub position_nums: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl ClmmpoolMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let clmmpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nums: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            clmmpool,
            position_nums,
            name,
            symbol,
            uri,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.clmmpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nums, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.uri, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClmmpoolMetadataAccount(pub ClmmpoolMetadata);
impl ClmmpoolMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLMMPOOL_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ClmmpoolMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLMMPOOL_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLMMPOOL_ACCOUNT_DISCM: [u8; 8] = [170, 160, 33, 122, 149, 217, 183, 244];
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
pub struct Clmmpool {
    pub clmm_config: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_spacing: u16,
    pub tick_spacing_seed: [u8; 2],
    pub fee_rate: u16,
    pub liquidity: u128,
    pub current_sqrt_price: u128,
    pub current_tick_index: i32,
    pub fee_growth_global_a: u128,
    pub fee_growth_global_b: u128,
    pub fee_protocol_token_a: u64,
    pub fee_protocol_token_b: u64,
    pub bump: [u8; 1],
    pub rewarder_infos: [Rewarder; 3],
    pub rewarder_last_updated_time: u64,
    pub is_pause: bool,
}
impl Clmmpool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let clmm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing_seed: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let current_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let current_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_protocol_token_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_protocol_token_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let rewarder_infos: [Rewarder; 3] = crate::borsh_de_or_default(&mut reader)?;
        let rewarder_last_updated_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_pause: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            clmm_config,
            token_a,
            token_b,
            token_a_vault,
            token_b_vault,
            tick_spacing,
            tick_spacing_seed,
            fee_rate,
            liquidity,
            current_sqrt_price,
            current_tick_index,
            fee_growth_global_a,
            fee_growth_global_b,
            fee_protocol_token_a,
            fee_protocol_token_b,
            bump,
            rewarder_infos,
            rewarder_last_updated_time,
            is_pause,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.clmm_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_protocol_token_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_protocol_token_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewarder_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewarder_last_updated_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_pause, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClmmpoolAccount(pub Clmmpool);
impl ClmmpoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLMMPOOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Clmmpool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLMMPOOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_TIER_ACCOUNT_DISCM: [u8; 8] = [56, 75, 159, 76, 142, 68, 190, 105];
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
pub struct FeeTier {
    pub fee_rate: u16,
    pub tick_spacing: u16,
    pub bump: u8,
}
impl FeeTier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_rate,
            tick_spacing,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeTierAccount(pub FeeTier);
impl FeeTierAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_TIER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeTier::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_TIER_ACCOUNT_DISCM)?;
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
    pub partner_fee_claim_authority: Pubkey,
    pub pending_authority: Pubkey,
    pub base: Pubkey,
    pub fee_rate: u16,
    pub bump: [u8; 1],
    pub start_time: u64,
    pub end_time: u64,
    pub name: String,
}
impl Partner {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let partner_fee_claim_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pending_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            partner_fee_claim_authority,
            pending_authority,
            base,
            fee_rate,
            bump,
            start_time,
            end_time,
            name,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(
            &self.partner_fee_claim_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pending_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
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
    pub clmmpool: Pubkey,
    pub position_nft_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_inside_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_inside_b: u128,
    pub fee_owed_b: u64,
    pub rewarder_infos: [PositionReward; 3],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let clmmpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewarder_infos: [PositionReward; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            clmmpool,
            position_nft_mint,
            liquidity,
            tick_lower_index,
            tick_upper_index,
            fee_growth_inside_a,
            fee_owed_a,
            fee_growth_inside_b,
            fee_owed_b,
            rewarder_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.clmmpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_inside_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_inside_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewarder_infos, &mut writer)?;
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
pub const TICK_ARRAY_ACCOUNT_DISCM: [u8; 8] = [69, 97, 189, 190, 110, 7, 66, 187];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TickArray {
    pub array_index: u16,
    pub tick_spacing: u16,
    pub clmmpool: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub ticks: [Tick; 64],
}
impl TickArray {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let array_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let clmmpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ticks = <[Tick; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            array_index,
            tick_spacing,
            clmmpool,
            ticks,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.array_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.clmmpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticks, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayAccount(pub TickArray);
impl TickArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ARRAY_MAP_ACCOUNT_DISCM: [u8; 8] = [108, 203, 48, 165, 116, 213, 96, 221];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TickArrayMap {
    #[serde(with = "crate::big_array_serde")]
    pub bitmap: [u8; 868],
}
impl TickArrayMap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bitmap = <[u8; 868] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { bitmap })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bitmap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayMapAccount(pub TickArrayMap);
impl TickArrayMapAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_MAP_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickArrayMap::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_MAP_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
