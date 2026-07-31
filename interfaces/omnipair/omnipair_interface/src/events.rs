use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADJUST_COLLATERAL_EVENT_EVENT_DISCM: [u8; 8] = [
    99, 246, 67, 126, 44, 252, 193, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdjustCollateralEvent {
    pub amount0: i64,
    pub amount1: i64,
    pub metadata: EventMetadata,
}
impl AdjustCollateralEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: i64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { amount0, amount1, metadata })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdjustCollateralEventEvent(pub AdjustCollateralEvent);
impl AdjustCollateralEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADJUST_COLLATERAL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdjustCollateralEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADJUST_COLLATERAL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADJUST_DEBT_EVENT_EVENT_DISCM: [u8; 8] = [
    153, 8, 169, 116, 207, 116, 155, 128,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdjustDebtEvent {
    pub amount0: i64,
    pub amount1: i64,
    pub metadata: EventMetadata,
}
impl AdjustDebtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: i64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { amount0, amount1, metadata })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdjustDebtEventEvent(pub AdjustDebtEvent);
impl AdjustDebtEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADJUST_DEBT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdjustDebtEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADJUST_DEBT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADJUST_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    229, 162, 211, 39, 159, 251, 24, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdjustLiquidityEvent {
    pub amount0: u64,
    pub amount1: u64,
    pub liquidity: u64,
    pub metadata: EventMetadata,
}
impl AdjustLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            amount0,
            amount1,
            liquidity,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdjustLiquidityEventEvent(pub AdjustLiquidityEvent);
impl AdjustLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADJUST_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AdjustLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADJUST_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BURN_EVENT_EVENT_DISCM: [u8; 8] = [33, 89, 47, 117, 82, 124, 238, 250];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnEvent {
    pub amount0: u64,
    pub amount1: u64,
    pub liquidity: u64,
    pub metadata: EventMetadata,
}
impl BurnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            amount0,
            amount1,
            liquidity,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnEventEvent(pub BurnEvent);
impl BurnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BurnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_PROTOCOL_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    131, 163, 209, 112, 123, 133, 105, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimProtocolFeesEvent {
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub futarchy_treasury_amount0: u64,
    pub futarchy_treasury_amount1: u64,
    pub buybacks_vault_amount0: u64,
    pub buybacks_vault_amount1: u64,
    pub team_treasury_amount0: u64,
    pub team_treasury_amount1: u64,
    pub metadata: EventMetadata,
}
impl ClaimProtocolFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let futarchy_treasury_amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let futarchy_treasury_amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault_amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault_amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury_amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury_amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            token0,
            token1,
            futarchy_treasury_amount0,
            futarchy_treasury_amount1,
            buybacks_vault_amount0,
            buybacks_vault_amount1,
            team_treasury_amount0,
            team_treasury_amount1,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.futarchy_treasury_amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.futarchy_treasury_amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buybacks_vault_amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buybacks_vault_amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.team_treasury_amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.team_treasury_amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeesEventEvent(pub ClaimProtocolFeesEvent);
impl ClaimProtocolFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PROTOCOL_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimProtocolFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PROTOCOL_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FLASHLOAN_EVENT_EVENT_DISCM: [u8; 8] = [34, 49, 239, 242, 228, 45, 20, 97];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashloanEvent {
    pub amount0: u64,
    pub amount1: u64,
    pub fee0: u64,
    pub fee1: u64,
    pub receiver: Pubkey,
    pub metadata: EventMetadata,
}
impl FlashloanEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            amount0,
            amount1,
            fee0,
            fee1,
            receiver,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashloanEventEvent(pub FlashloanEvent);
impl FlashloanEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASHLOAN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FlashloanEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASHLOAN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_EVENT_EVENT_DISCM: [u8; 8] = [197, 144, 146, 149, 66, 164, 95, 16];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintEvent {
    pub amount0: u64,
    pub amount1: u64,
    pub liquidity: u64,
    pub metadata: EventMetadata,
}
impl MintEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            amount0,
            amount1,
            liquidity,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintEventEvent(pub MintEvent);
impl MintEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [118, 0, 50, 196, 55, 255, 121, 43];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PairCreatedEvent {
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub lp_mint: Pubkey,
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    pub rate_model: Pubkey,
    pub swap_fee_bps: u16,
    pub half_life: u64,
    pub fixed_cf_bps: Option<u16>,
    pub target_util_start_bps: u64,
    pub target_util_end_bps: u64,
    pub rate_half_life_ms: u64,
    pub min_rate_bps: u64,
    pub max_rate_bps: u64,
    pub params_hash: [u8; 32],
    pub version: u8,
    pub metadata: EventMetadata,
}
impl PairCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token0_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token1_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let rate_model: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let half_life: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_cf_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let target_util_start_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_util_end_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rate_half_life_ms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let params_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            token0,
            token1,
            lp_mint,
            token0_decimals,
            token1_decimals,
            rate_model,
            swap_fee_bps,
            half_life,
            fixed_cf_bps,
            target_util_start_bps,
            target_util_end_bps,
            rate_half_life_ms,
            min_rate_bps,
            max_rate_bps,
            params_hash,
            version,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token0_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_model, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.half_life, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fixed_cf_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_util_start_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_util_end_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_half_life_ms, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_rate_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_rate_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairCreatedEventEvent(pub PairCreatedEvent);
impl PairCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PairCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_CREATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapEvent {
    pub reserve0: u64,
    pub reserve1: u64,
    pub is_token0_in: bool,
    pub amount_in: u64,
    pub amount_out: u64,
    pub amount_in_after_fee: u64,
    pub lp_fee: u64,
    pub protocol_fee: u64,
    pub metadata: EventMetadata,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reserve0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_token0_in: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_after_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            reserve0,
            reserve1,
            is_token0_in,
            amount_in,
            amount_out,
            amount_in_after_fee,
            lp_fee,
            protocol_fee,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.reserve0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_token0_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in_after_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);
impl SwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_PAIR_EVENT_EVENT_DISCM: [u8; 8] = [44, 6, 60, 245, 142, 38, 166, 247];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePairEvent {
    pub price0_ema: u64,
    pub price1_ema: u64,
    pub rate0: u64,
    pub rate1: u64,
    pub accrued_interest0: u128,
    pub accrued_interest1: u128,
    pub lp_interest0: u64,
    pub lp_interest1: u64,
    pub protocol_interest0: u64,
    pub protocol_interest1: u64,
    pub cash_reserve0: u64,
    pub cash_reserve1: u64,
    pub reserve0_after_interest: u64,
    pub reserve1_after_interest: u64,
    pub metadata: EventMetadata,
}
impl UpdatePairEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price0_ema: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price1_ema: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rate0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rate1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let accrued_interest0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let accrued_interest1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lp_interest0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_interest1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_interest0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_interest1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cash_reserve0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cash_reserve1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve0_after_interest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve1_after_interest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            price0_ema,
            price1_ema,
            rate0,
            rate1,
            accrued_interest0,
            accrued_interest1,
            lp_interest0,
            lp_interest1,
            protocol_interest0,
            protocol_interest1,
            cash_reserve0,
            cash_reserve1,
            reserve0_after_interest,
            reserve1_after_interest,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.price0_ema, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price1_ema, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accrued_interest0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accrued_interest1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_interest0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_interest1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_interest0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_interest1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cash_reserve0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cash_reserve1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve0_after_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve1_after_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePairEventEvent(pub UpdatePairEvent);
impl UpdatePairEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PAIR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdatePairEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PAIR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_LIQUIDITY_POSITION_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    255, 227, 32, 107, 211, 246, 39, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserLiquidityPositionUpdatedEvent {
    pub token0_amount: u64,
    pub token1_amount: u64,
    pub lp_amount: u64,
    pub cash_reserve0: u64,
    pub cash_reserve1: u64,
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub metadata: EventMetadata,
}
impl UserLiquidityPositionUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cash_reserve0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cash_reserve1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token0_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token1_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            token0_amount,
            token1_amount,
            lp_amount,
            cash_reserve0,
            cash_reserve1,
            token0_mint,
            token1_mint,
            lp_mint,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token0_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cash_reserve0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cash_reserve1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token0_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserLiquidityPositionUpdatedEventEvent(pub UserLiquidityPositionUpdatedEvent);
impl UserLiquidityPositionUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_LIQUIDITY_POSITION_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserLiquidityPositionUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_LIQUIDITY_POSITION_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_POSITION_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [
    240, 132, 92, 227, 209, 72, 178, 169,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserPositionCreatedEvent {
    pub position: Pubkey,
    pub metadata: EventMetadata,
}
impl UserPositionCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { position, metadata })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserPositionCreatedEventEvent(pub UserPositionCreatedEvent);
impl UserPositionCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_POSITION_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserPositionCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_POSITION_CREATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_POSITION_LIQUIDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    220, 137, 217, 3, 242, 190, 238, 216,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserPositionLiquidatedEvent {
    pub position: Pubkey,
    pub liquidator: Pubkey,
    pub collateral0_liquidated: u64,
    pub collateral1_liquidated: u64,
    pub debt0_liquidated: u64,
    pub debt1_liquidated: u64,
    pub collateral_price: u64,
    pub shortfall: u128,
    pub liquidation_bonus_applied: u64,
    pub k0: u128,
    pub k1: u128,
    pub metadata: EventMetadata,
}
impl UserPositionLiquidatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral0_liquidated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral1_liquidated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt0_liquidated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt1_liquidated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shortfall: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_bonus_applied: u64 = crate::borsh_de_or_default(&mut reader)?;
        let k0: u128 = crate::borsh_de_or_default(&mut reader)?;
        let k1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            position,
            liquidator,
            collateral0_liquidated,
            collateral1_liquidated,
            debt0_liquidated,
            debt1_liquidated,
            collateral_price,
            shortfall,
            liquidation_bonus_applied,
            k0,
            k1,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral0_liquidated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral1_liquidated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt0_liquidated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt1_liquidated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shortfall, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_bonus_applied, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.k0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.k1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserPositionLiquidatedEventEvent(pub UserPositionLiquidatedEvent);
impl UserPositionLiquidatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_POSITION_LIQUIDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserPositionLiquidatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_POSITION_LIQUIDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_POSITION_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    83, 168, 197, 88, 89, 42, 58, 102,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserPositionUpdatedEvent {
    pub position: Pubkey,
    pub collateral0: u64,
    pub collateral1: u64,
    pub debt0_shares: u128,
    pub debt1_shares: u128,
    pub collateral0_max_cf_bps: u16,
    pub collateral1_max_cf_bps: u16,
    pub collateral0_liquidation_cf_bps: u16,
    pub collateral1_liquidation_cf_bps: u16,
    pub metadata: EventMetadata,
}
impl UserPositionUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt0_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let debt1_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let collateral0_max_cf_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let collateral1_max_cf_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let collateral0_liquidation_cf_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let collateral1_liquidation_cf_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <EventMetadata>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            position,
            collateral0,
            collateral1,
            debt0_shares,
            debt1_shares,
            collateral0_max_cf_bps,
            collateral1_max_cf_bps,
            collateral0_liquidation_cf_bps,
            collateral1_liquidation_cf_bps,
            metadata,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt0_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt1_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral0_max_cf_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral1_max_cf_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collateral0_liquidation_cf_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.collateral1_liquidation_cf_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.metadata, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserPositionUpdatedEventEvent(pub UserPositionUpdatedEvent);
impl UserPositionUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_POSITION_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserPositionUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_POSITION_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
