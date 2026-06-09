use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BIN_ARRAY_INITIALIZATION_EVENT_EVENT_DISCM: [u8; 8] = [
    237, 158, 3, 184, 253, 238, 102, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BinArrayInitializationEvent {
    pub pair: Pubkey,
    pub index: u32,
}
impl BinArrayInitializationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { pair, index })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinArrayInitializationEventEvent(pub BinArrayInitializationEvent);
impl BinArrayInitializationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_ARRAY_INITIALIZATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BinArrayInitializationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_ARRAY_INITIALIZATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BIN_STEP_CONFIG_INITIALIZATION_EVENT_EVENT_DISCM: [u8; 8] = [
    2, 138, 209, 132, 61, 232, 124, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BinStepConfigInitializationEvent {
    pub liquidity_book_config: Pubkey,
    pub bin_step_config: Pubkey,
    pub bin_step: u8,
    pub fee_parameters: StaticFeeParameters,
}
impl BinStepConfigInitializationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_book_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            liquidity_book_config,
            bin_step_config,
            bin_step,
            fee_parameters,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.liquidity_book_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinStepConfigInitializationEventEvent(pub BinStepConfigInitializationEvent);
impl BinStepConfigInitializationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_STEP_CONFIG_INITIALIZATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BinStepConfigInitializationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_STEP_CONFIG_INITIALIZATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BIN_STEP_CONFIG_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    241, 69, 172, 53, 135, 27, 238, 248,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BinStepConfigUpdateEvent {
    pub bin_step_config: Pubkey,
    pub status: ConfigStatus,
    pub availability: ConfigAvailability,
    pub fee_parameters: StaticFeeParameters,
}
impl BinStepConfigUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_step_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let status: ConfigStatus = crate::borsh_de_or_default(&mut reader)?;
        let availability: ConfigAvailability = crate::borsh_de_or_default(&mut reader)?;
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            bin_step_config,
            status,
            availability,
            fee_parameters,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bin_step_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.availability, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinStepConfigUpdateEventEvent(pub BinStepConfigUpdateEvent);
impl BinStepConfigUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_STEP_CONFIG_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BinStepConfigUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_STEP_CONFIG_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BIN_SWAP_EVENT_EVENT_DISCM: [u8; 8] = [55, 42, 192, 194, 230, 243, 9, 72];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BinSwapEvent {
    pub pair: Pubkey,
    pub swap_for_y: bool,
    pub protocol_fee: u64,
    pub bin_id: u32,
    pub amount_in: u64,
    pub amount_out: u64,
    pub volatility_accumulator: u32,
    pub fee: u64,
}
impl BinSwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_for_y: bool = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bin_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            swap_for_y,
            protocol_fee,
            bin_id,
            amount_in,
            amount_out,
            volatility_accumulator,
            fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_for_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.volatility_accumulator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinSwapEventEvent(pub BinSwapEvent);
impl BinSwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BinSwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COMPOSITION_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    83, 234, 249, 47, 88, 125, 2, 86,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompositionFeesEvent {
    pub pair: Pubkey,
    pub active_id: u32,
    pub composition_fees_x: u64,
    pub composition_fees_y: u64,
    pub protocol_fees_x: u64,
    pub protocol_fees_y: u64,
}
impl CompositionFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let active_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let composition_fees_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let composition_fees_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            active_id,
            composition_fees_x,
            composition_fees_y,
            protocol_fees_x,
            protocol_fees_y,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.composition_fees_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.composition_fees_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_y, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompositionFeesEventEvent(pub CompositionFeesEvent);
impl CompositionFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMPOSITION_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CompositionFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMPOSITION_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_BOOK_CONFIG_INITIALIZATION_EVENT_EVENT_DISCM: [u8; 8] = [
    90, 99, 66, 116, 24, 72, 145, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityBookConfigInitializationEvent {
    pub config: Pubkey,
    pub preset_authority: Pubkey,
}
impl LiquidityBookConfigInitializationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let preset_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { config, preset_authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.preset_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityBookConfigInitializationEventEvent(
    pub LiquidityBookConfigInitializationEvent,
);
impl LiquidityBookConfigInitializationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_BOOK_CONFIG_INITIALIZATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityBookConfigInitializationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_BOOK_CONFIG_INITIALIZATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_BOOK_CONFIG_TRANSFER_OWNERSHIP_EVENT_EVENT_DISCM: [u8; 8] = [
    181, 131, 103, 224, 188, 170, 226, 65,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityBookConfigTransferOwnershipEvent {
    pub new_authority: Pubkey,
}
impl LiquidityBookConfigTransferOwnershipEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { new_authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityBookConfigTransferOwnershipEventEvent(
    pub LiquidityBookConfigTransferOwnershipEvent,
);
impl LiquidityBookConfigTransferOwnershipEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_BOOK_CONFIG_TRANSFER_OWNERSHIP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityBookConfigTransferOwnershipEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_BOOK_CONFIG_TRANSFER_OWNERSHIP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_BOOK_CONFIG_TRANSFER_OWNERSHIP_INIT_EVENT_EVENT_DISCM: [u8; 8] = [
    69, 165, 109, 99, 223, 38, 229, 100,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityBookConfigTransferOwnershipInitEvent {
    pub new_pending_authority: Option<Pubkey>,
}
impl LiquidityBookConfigTransferOwnershipInitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_pending_authority: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { new_pending_authority })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_pending_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityBookConfigTransferOwnershipInitEventEvent(
    pub LiquidityBookConfigTransferOwnershipInitEvent,
);
impl LiquidityBookConfigTransferOwnershipInitEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_BOOK_CONFIG_TRANSFER_OWNERSHIP_INIT_EVENT_EVENT_DISCM
        {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityBookConfigTransferOwnershipInitEvent::deserialize(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &LIQUIDITY_BOOK_CONFIG_TRANSFER_OWNERSHIP_INIT_EVENT_EVENT_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_INITIALIZATION_EVENT_EVENT_DISCM: [u8; 8] = [
    132, 133, 209, 222, 229, 215, 206, 245,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PairInitializationEvent {
    pub pair: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub bin_step_config: Pubkey,
    pub active_id: u32,
}
impl PairInitializationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let active_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            token_mint_x,
            token_mint_y,
            bin_step_config,
            active_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairInitializationEventEvent(pub PairInitializationEvent);
impl PairInitializationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_INITIALIZATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PairInitializationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_INITIALIZATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_STATIC_FEE_PARAMETERS_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    57, 109, 202, 252, 154, 9, 121, 131,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PairStaticFeeParametersUpdateEvent {
    pub pair: Pubkey,
    pub fee_parameters: StaticFeeParameters,
}
impl PairStaticFeeParametersUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticFeeParameters>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { pair, fee_parameters })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_parameters, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairStaticFeeParametersUpdateEventEvent(
    pub PairStaticFeeParametersUpdateEvent,
);
impl PairStaticFeeParametersUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_STATIC_FEE_PARAMETERS_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PairStaticFeeParametersUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_STATIC_FEE_PARAMETERS_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_CREATION_EVENT_EVENT_DISCM: [u8; 8] = [
    97, 21, 205, 201, 62, 41, 111, 164,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionCreationEvent {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub lower_bin_id: u32,
    pub upper_bin_id: u32,
}
impl PositionCreationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lower_bin_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_bin_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            position,
            position_mint,
            lower_bin_id,
            upper_bin_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_bin_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionCreationEventEvent(pub PositionCreationEvent);
impl PositionCreationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_CREATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PositionCreationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_CREATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_DECREASE_EVENT_EVENT_DISCM: [u8; 8] = [
    200, 116, 151, 126, 182, 237, 245, 254,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionDecreaseEvent {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub bin_ids: Vec<u32>,
    pub amounts_x: Vec<u64>,
    pub amounts_y: Vec<u64>,
    pub liquidity_burned: Vec<u128>,
}
impl PositionDecreaseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_ids: Vec<u32> = crate::borsh_de_or_default(&mut reader)?;
        let amounts_x: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        let amounts_y: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_burned: Vec<u128> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            position,
            bin_ids,
            amounts_x,
            amounts_y,
            liquidity_burned,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_ids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_burned, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionDecreaseEventEvent(pub PositionDecreaseEvent);
impl PositionDecreaseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_DECREASE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PositionDecreaseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_DECREASE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_INCREASE_EVENT_EVENT_DISCM: [u8; 8] = [
    247, 40, 58, 113, 28, 175, 60, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionIncreaseEvent {
    pub pair: Pubkey,
    pub position: Pubkey,
    pub bin_ids: Vec<u32>,
    pub amounts_x: Vec<u64>,
    pub amounts_y: Vec<u64>,
    pub liquidity_minted: Vec<u128>,
}
impl PositionIncreaseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_ids: Vec<u32> = crate::borsh_de_or_default(&mut reader)?;
        let amounts_x: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        let amounts_y: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_minted: Vec<u128> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            position,
            bin_ids,
            amounts_x,
            amounts_y,
            liquidity_minted,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_ids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_minted, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionIncreaseEventEvent(pub PositionIncreaseEvent);
impl PositionIncreaseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_INCREASE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PositionIncreaseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_INCREASE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_FEES_COLLECTION_EVENT_EVENT_DISCM: [u8; 8] = [
    196, 36, 190, 66, 172, 52, 142, 15,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProtocolFeesCollectionEvent {
    pub pair: Pubkey,
    pub protocol_fees_x: u64,
    pub protocol_fees_y: u64,
}
impl ProtocolFeesCollectionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            protocol_fees_x,
            protocol_fees_y,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_y, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolFeesCollectionEventEvent(pub ProtocolFeesCollectionEvent);
impl ProtocolFeesCollectionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_FEES_COLLECTION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ProtocolFeesCollectionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_FEES_COLLECTION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const QUOTE_ASSET_BADGE_INITIALIZATION_EVENT_EVENT_DISCM: [u8; 8] = [
    202, 110, 93, 186, 165, 96, 200, 27,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteAssetBadgeInitializationEvent {
    pub liquidity_book_config: Pubkey,
    pub quote_asset_badge: Pubkey,
    pub token_mint: Pubkey,
}
impl QuoteAssetBadgeInitializationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_book_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_badge: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_book_config,
            quote_asset_badge,
            token_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.liquidity_book_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_asset_badge, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteAssetBadgeInitializationEventEvent(
    pub QuoteAssetBadgeInitializationEvent,
);
impl QuoteAssetBadgeInitializationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_ASSET_BADGE_INITIALIZATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = QuoteAssetBadgeInitializationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_ASSET_BADGE_INITIALIZATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const QUOTE_ASSET_BADGE_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    102, 149, 171, 236, 123, 73, 205, 194,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteAssetBadgeUpdateEvent {
    pub quote_asset_badge: Pubkey,
    pub status: QuoteAssetBadgeStatus,
}
impl QuoteAssetBadgeUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_asset_badge: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let status: QuoteAssetBadgeStatus = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { quote_asset_badge, status })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.quote_asset_badge, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteAssetBadgeUpdateEventEvent(pub QuoteAssetBadgeUpdateEvent);
impl QuoteAssetBadgeUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUOTE_ASSET_BADGE_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = QuoteAssetBadgeUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUOTE_ASSET_BADGE_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
