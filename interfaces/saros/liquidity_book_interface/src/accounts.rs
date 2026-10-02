use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BIN_ARRAY_ACCOUNT_DISCM: [u8; 8] = [92, 142, 92, 220, 5, 148, 70, 181];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BinArray {
    pub pair: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub bins: [Bin; 256],
    pub index: u32,
    pub space: [u8; 12],
}
impl BinArray {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bins = <[Bin; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let space: [u8; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pair, bins, index, space })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bins, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinArrayAccount(pub BinArray);
impl BinArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BinArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BIN_STEP_CONFIG_ACCOUNT_DISCM: [u8; 8] = [44, 12, 82, 45, 127, 124, 191, 199];
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
pub struct BinStepConfig {
    pub bump: u8,
    pub liquidity_book_config: Pubkey,
    pub bin_step: u8,
    pub status: ConfigStatus,
    pub availability: ConfigAvailability,
    pub fee_parameters: StaticFeeParameters,
}
impl BinStepConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_book_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: ConfigStatus = crate::borsh_de_or_default(&mut reader)?;
        let availability: ConfigAvailability = crate::borsh_de_or_default(&mut reader)?;
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            bump,
            liquidity_book_config,
            bin_step,
            status,
            availability,
            fee_parameters,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_book_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.availability, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinStepConfigAccount(pub BinStepConfig);
impl BinStepConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_STEP_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BinStepConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_STEP_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_BOOK_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    173, 36, 130, 129, 45, 178, 44, 86,
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
pub struct LiquidityBookConfig {
    pub preset_authority: Pubkey,
    pub pending_preset_authority: Option<Pubkey>,
}
impl LiquidityBookConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let preset_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_preset_authority: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            preset_authority,
            pending_preset_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.preset_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_preset_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityBookConfigAccount(pub LiquidityBookConfig);
impl LiquidityBookConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_BOOK_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LiquidityBookConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_BOOK_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_ACCOUNT_DISCM: [u8; 8] = [85, 72, 49, 176, 182, 228, 141, 82];
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
pub struct Pair {
    pub bump: [u8; 1],
    pub liquidity_book_config: Pubkey,
    pub bin_step: u8,
    pub bin_step_seed: [u8; 1],
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub static_fee_parameters: StaticFeeParameters,
    pub active_id: u32,
    pub dynamic_fee_parameters: DynamicFeeParameters,
    pub protocol_fees_x: u64,
    pub protocol_fees_y: u64,
    pub hook: Option<Pubkey>,
}
impl Pair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_book_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_seed: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let static_fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        let active_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <DynamicFeeParameters>::deserialize(&mut reader)?
        };
        let protocol_fees_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let hook: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            liquidity_book_config,
            bin_step,
            bin_step_seed,
            token_mint_x,
            token_mint_y,
            static_fee_parameters,
            active_id,
            dynamic_fee_parameters,
            protocol_fees_x,
            protocol_fees_y,
            hook,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_book_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.static_fee_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dynamic_fee_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.hook, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairAccount(pub Pair);
impl PairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_ACCOUNT_DISCM)?;
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
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Position {
    pub pair: Pubkey,
    pub position_mint: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub liquidity_shares: [u128; 64],
    pub lower_bin_id: u32,
    pub upper_bin_id: u32,
    pub space: [u8; 8],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_shares = <[u128; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let lower_bin_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_bin_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let space: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            position_mint,
            liquidity_shares,
            lower_bin_id,
            upper_bin_id,
            space,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
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
pub const QUOTE_ASSET_BADGE_ACCOUNT_DISCM: [u8; 8] = [
    183, 124, 99, 219, 110, 119, 157, 221,
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
pub struct QuoteAssetBadge {
    pub bump: u8,
    pub status: QuoteAssetBadgeStatus,
}
impl QuoteAssetBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: QuoteAssetBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, status })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteAssetBadgeAccount(pub QuoteAssetBadge);
impl QuoteAssetBadgeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_ASSET_BADGE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(QuoteAssetBadge::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_ASSET_BADGE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
