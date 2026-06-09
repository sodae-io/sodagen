use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CLAIM_VESTED_EVENT_EVENT_DISCM: [u8; 8] = [
    21, 194, 114, 87, 120, 211, 226, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimVestedEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub claim_amount: u64,
}
impl ClaimVestedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let claim_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            beneficiary,
            claim_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claim_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimVestedEventEvent(pub ClaimVestedEvent);
impl ClaimVestedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_VESTED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimVestedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_VESTED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_VESTING_EVENT_EVENT_DISCM: [u8; 8] = [
    150, 152, 11, 179, 52, 210, 191, 125,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateVestingEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub share_amount: u64,
}
impl CreateVestingEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            beneficiary,
            share_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.share_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateVestingEventEvent(pub CreateVestingEvent);
impl CreateVestingEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_VESTING_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateVestingEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_VESTING_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    151, 215, 226, 9, 118, 161, 115, 174,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolCreateEvent {
    pub pool_state: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}
impl PoolCreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint_param = if reader.is_empty() {
            Default::default()
        } else {
            <MintParams>::deserialize(&mut reader)?
        };
        let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let vesting_param = if reader.is_empty() {
            Default::default()
        } else {
            <VestingParams>::deserialize(&mut reader)?
        };
        let amm_fee_on: AmmCreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            creator,
            config,
            base_mint_param,
            curve_param,
            vesting_param,
            amm_fee_on,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vesting_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_fee_on, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreateEventEvent(pub PoolCreateEvent);
impl PoolCreateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CREATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolCreateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CREATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADE_EVENT_EVENT_DISCM: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeEvent {
    pub pool_state: Pubkey,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base_before: u64,
    pub real_quote_before: u64,
    pub real_base_after: u64,
    pub real_quote_after: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub protocol_fee: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub share_fee: u64,
    pub trade_direction: TradeDirection,
    pub pool_status: PoolStatus,
    pub exact_in: bool,
}
impl TradeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_base_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_base: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_base_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_quote_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_base_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_quote_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let share_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_direction: TradeDirection = crate::borsh_de_or_default(&mut reader)?;
        let pool_status: PoolStatus = crate::borsh_de_or_default(&mut reader)?;
        let exact_in: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            total_base_sell,
            virtual_base,
            virtual_quote,
            real_base_before,
            real_quote_before,
            real_base_after,
            real_quote_after,
            amount_in,
            amount_out,
            protocol_fee,
            platform_fee,
            creator_fee,
            share_fee,
            trade_direction,
            pool_status,
            exact_in,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_base_sell, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_base_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_quote_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_base_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_quote_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.share_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.exact_in, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeEventEvent(pub TradeEvent);
impl TradeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
