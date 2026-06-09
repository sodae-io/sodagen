use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADD_LIQUIDITY_EVENT_DISCM: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidity {
    pub lb_pair: Pubkey,
    pub from: Pubkey,
    pub position: Pubkey,
    pub amounts: [u64; 2],
    pub active_bin_id: i32,
}
impl AddLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amounts: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let active_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            from,
            position,
            amounts,
            active_bin_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_bin_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityEvent(pub AddLiquidity);
impl AddLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AddLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CANCEL_LIMIT_ORDER_EVT_EVENT_DISCM: [u8; 8] = [
    131, 234, 194, 133, 9, 14, 189, 209,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelLimitOrderEvt {
    pub lb_pair: Pubkey,
    pub from: Pubkey,
    pub limit_order: Pubkey,
    pub amounts: [u64; 2],
    pub active_id: i32,
    pub bins: Vec<i32>,
}
impl CancelLimitOrderEvt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let limit_order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amounts: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bins: Vec<i32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            from,
            limit_order,
            amounts,
            active_id,
            bins,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_order, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bins, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelLimitOrderEvtEvent(pub CancelLimitOrderEvt);
impl CancelLimitOrderEvtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_LIMIT_ORDER_EVT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CancelLimitOrderEvt::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_LIMIT_ORDER_EVT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_FEE_EVENT_DISCM: [u8; 8] = [75, 122, 154, 48, 140, 74, 123, 163];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimFee {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub fee_x: u64,
    pub fee_y: u64,
}
impl ClaimFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            fee_x,
            fee_y,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_y, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeEvent(pub ClaimFee);
impl ClaimFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_FEE2_EVENT_DISCM: [u8; 8] = [232, 171, 242, 97, 58, 77, 35, 45];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimFee2 {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub fee_x: u64,
    pub fee_y: u64,
    pub active_bin_id: i32,
}
impl ClaimFee2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            fee_x,
            fee_y,
            active_bin_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_bin_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFee2Event(pub ClaimFee2);
impl ClaimFee2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimFee2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_REWARD_EVENT_DISCM: [u8; 8] = [148, 116, 134, 204, 22, 171, 85, 95];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimReward {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub reward_index: u64,
    pub total_reward: u64,
}
impl ClaimReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            reward_index,
            total_reward,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_reward, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardEvent(pub ClaimReward);
impl ClaimRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_REWARD2_EVENT_DISCM: [u8; 8] = [27, 143, 244, 33, 80, 43, 110, 146];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimReward2 {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub reward_index: u64,
    pub total_reward: u64,
    pub active_bin_id: i32,
}
impl ClaimReward2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            reward_index,
            total_reward,
            active_bin_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_bin_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimReward2Event(pub ClaimReward2);
impl ClaimReward2Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARD2_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ClaimReward2::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARD2_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLOSE_LIMIT_ORDER_EVT_EVENT_DISCM: [u8; 8] = [
    142, 135, 8, 76, 92, 63, 118, 83,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseLimitOrderEvt {
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub limit_order: Pubkey,
}
impl CloseLimitOrderEvt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let limit_order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            owner,
            limit_order,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_order, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseLimitOrderEvtEvent(pub CloseLimitOrderEvt);
impl CloseLimitOrderEvtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_LIMIT_ORDER_EVT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CloseLimitOrderEvt::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_LIMIT_ORDER_EVT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COMPOSITION_FEE_EVENT_DISCM: [u8; 8] = [128, 151, 123, 106, 17, 102, 113, 142];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompositionFee {
    pub from: Pubkey,
    pub bin_id: i16,
    pub token_x_fee_amount: u64,
    pub token_y_fee_amount: u64,
    pub protocol_token_x_fee_amount: u64,
    pub protocol_token_y_fee_amount: u64,
}
impl CompositionFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_id: i16 = crate::borsh_de_or_default(&mut reader)?;
        let token_x_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_y_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_token_x_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_token_y_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            from,
            bin_id,
            token_x_fee_amount,
            token_y_fee_amount,
            protocol_token_x_fee_amount,
            protocol_token_y_fee_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_x_fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y_fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_token_x_fee_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.protocol_token_y_fee_amount,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompositionFeeEvent(pub CompositionFee);
impl CompositionFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMPOSITION_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CompositionFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMPOSITION_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DECREASE_POSITION_LENGTH_EVENT_DISCM: [u8; 8] = [
    52, 118, 235, 85, 172, 169, 15, 128,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionLength {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub length_to_remove: u16,
    pub side: u8,
}
impl DecreasePositionLength {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let length_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            length_to_remove,
            side,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.length_to_remove, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.side, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreasePositionLengthEvent(pub DecreasePositionLength);
impl DecreasePositionLengthEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_POSITION_LENGTH_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DecreasePositionLength::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_POSITION_LENGTH_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DYNAMIC_FEE_PARAMETER_UPDATE_EVENT_DISCM: [u8; 8] = [
    88, 88, 178, 135, 194, 146, 91, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DynamicFeeParameterUpdate {
    pub lb_pair: Pubkey,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
}
impl DynamicFeeParameterUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filter_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decay_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduction_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.variable_fee_control, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_volatility_accumulator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicFeeParameterUpdateEvent(pub DynamicFeeParameterUpdate);
impl DynamicFeeParameterUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DYNAMIC_FEE_PARAMETER_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DynamicFeeParameterUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DYNAMIC_FEE_PARAMETER_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_PARAMETER_UPDATE_EVENT_DISCM: [u8; 8] = [
    48, 76, 241, 117, 144, 215, 242, 44,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FeeParameterUpdate {
    pub lb_pair: Pubkey,
    pub protocol_share: u16,
    pub base_factor: u16,
    pub base_fee_power_factor: u8,
}
impl FeeParameterUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            protocol_share,
            base_factor,
            base_fee_power_factor,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fee_power_factor, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeParameterUpdateEvent(pub FeeParameterUpdate);
impl FeeParameterUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_PARAMETER_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FeeParameterUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_PARAMETER_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUND_REWARD_EVENT_DISCM: [u8; 8] = [246, 228, 58, 130, 145, 170, 79, 204];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundReward {
    pub lb_pair: Pubkey,
    pub funder: Pubkey,
    pub reward_index: u64,
    pub amount: u64,
}
impl FundReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            funder,
            reward_index,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundRewardEvent(pub FundReward);
impl FundRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FundReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GO_TO_A_BIN_EVENT_DISCM: [u8; 8] = [59, 138, 76, 68, 138, 131, 176, 67];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoToABin {
    pub lb_pair: Pubkey,
    pub from_bin_id: i32,
    pub to_bin_id: i32,
}
impl GoToABin {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let from_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let to_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            from_bin_id,
            to_bin_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.from_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.to_bin_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoToABinEvent(pub GoToABin);
impl GoToABinEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GO_TO_A_BIN_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GoToABin::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GO_TO_A_BIN_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_OBSERVATION_EVENT_DISCM: [u8; 8] = [
    99, 249, 17, 121, 166, 156, 207, 215,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseObservation {
    pub oracle: Pubkey,
    pub new_observation_length: u64,
}
impl IncreaseObservation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_observation_length: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            new_observation_length,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_observation_length, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseObservationEvent(pub IncreaseObservation);
impl IncreaseObservationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_OBSERVATION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreaseObservation::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_OBSERVATION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_POSITION_LENGTH_EVENT_DISCM: [u8; 8] = [
    157, 239, 42, 204, 30, 56, 223, 46,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionLength {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub length_to_add: u16,
    pub side: u8,
}
impl IncreasePositionLength {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let length_to_add: u16 = crate::borsh_de_or_default(&mut reader)?;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            length_to_add,
            side,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.length_to_add, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.side, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreasePositionLengthEvent(pub IncreasePositionLength);
impl IncreasePositionLengthEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_POSITION_LENGTH_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreasePositionLength::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_POSITION_LENGTH_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_REWARD_EVENT_DISCM: [u8; 8] = [211, 153, 88, 62, 149, 60, 177, 70];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeReward {
    pub lb_pair: Pubkey,
    pub reward_mint: Pubkey,
    pub funder: Pubkey,
    pub reward_index: u64,
    pub reward_duration: u64,
}
impl InitializeReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            reward_mint,
            funder,
            reward_index,
            reward_duration,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.funder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_duration, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardEvent(pub InitializeReward);
impl InitializeRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LB_PAIR_CREATE_EVENT_DISCM: [u8; 8] = [185, 74, 252, 125, 27, 215, 188, 111];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LbPairCreate {
    pub lb_pair: Pubkey,
    pub bin_step: u16,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
}
impl LbPairCreate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            bin_step,
            token_x,
            token_y,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LbPairCreateEvent(pub LbPairCreate);
impl LbPairCreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LB_PAIR_CREATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LbPairCreate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LB_PAIR_CREATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PLACE_LIMIT_ORDER_EVT_EVENT_DISCM: [u8; 8] = [
    43, 79, 27, 169, 244, 28, 225, 63,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaceLimitOrderEvt {
    pub lb_pair: Pubkey,
    pub sender: Pubkey,
    pub owner: Pubkey,
    pub limit_order: Pubkey,
    pub active_id: i32,
    pub params: PlaceLimitOrderParams,
}
impl PlaceLimitOrderEvt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let limit_order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <PlaceLimitOrderParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lb_pair,
            sender,
            owner,
            limit_order,
            active_id,
            params,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_order, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderEvtEvent(pub PlaceLimitOrderEvt);
impl PlaceLimitOrderEvtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_LIMIT_ORDER_EVT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PlaceLimitOrderEvt::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_LIMIT_ORDER_EVT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_CLOSE_EVENT_DISCM: [u8; 8] = [255, 196, 16, 107, 28, 202, 53, 128];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionClose {
    pub position: Pubkey,
    pub owner: Pubkey,
}
impl PositionClose {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { position, owner })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionCloseEvent(pub PositionClose);
impl PositionCloseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_CLOSE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PositionClose::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_CLOSE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_CREATE_EVENT_DISCM: [u8; 8] = [144, 142, 252, 84, 157, 53, 37, 121];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionCreate {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
}
impl PositionCreate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lb_pair, position, owner })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionCreateEvent(pub PositionCreate);
impl PositionCreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_CREATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PositionCreate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_CREATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REBALANCING_EVENT_DISCM: [u8; 8] = [0, 109, 117, 179, 61, 91, 199, 200];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rebalancing {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub active_bin_id: i32,
    pub x_withdrawn_amount: u64,
    pub x_added_amount: u64,
    pub y_withdrawn_amount: u64,
    pub y_added_amount: u64,
    pub x_fee_amount: u64,
    pub y_fee_amount: u64,
    pub old_min_id: i32,
    pub old_max_id: i32,
    pub new_min_id: i32,
    pub new_max_id: i32,
    pub rewards: [u64; 2],
}
impl Rebalancing {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let active_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let x_withdrawn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let x_added_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let y_withdrawn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let y_added_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let x_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let y_fee_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_min_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let old_max_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let new_min_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let new_max_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let rewards: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            position,
            owner,
            active_bin_id,
            x_withdrawn_amount,
            x_added_amount,
            y_withdrawn_amount,
            y_added_amount,
            x_fee_amount,
            y_fee_amount,
            old_min_id,
            old_max_id,
            new_min_id,
            new_max_id,
            rewards,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_withdrawn_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_added_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_withdrawn_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_added_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_fee_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_min_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_max_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_min_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_max_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalancingEvent(pub Rebalancing);
impl RebalancingEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCING_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Rebalancing::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCING_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_LIQUIDITY_EVENT_DISCM: [u8; 8] = [116, 244, 97, 232, 103, 31, 152, 58];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidity {
    pub lb_pair: Pubkey,
    pub from: Pubkey,
    pub position: Pubkey,
    pub amounts: [u64; 2],
    pub active_bin_id: i32,
}
impl RemoveLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amounts: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let active_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            from,
            position,
            amounts,
            active_bin_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_bin_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityEvent(pub RemoveLiquidity);
impl RemoveLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveLiquidity::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_POSITION_PERMISSIONLESS_OPERATION_BITS_EVT_EVENT_DISCM: [u8; 8] = [
    195, 229, 147, 245, 29, 125, 48, 168,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPositionPermissionlessOperationBitsEvt {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub old_bits: u8,
    pub new_bits: u8,
}
impl SetPositionPermissionlessOperationBitsEvt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_bits: u8 = crate::borsh_de_or_default(&mut reader)?;
        let new_bits: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position,
            owner,
            old_bits,
            new_bits,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_bits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_bits, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPositionPermissionlessOperationBitsEvtEvent(
    pub SetPositionPermissionlessOperationBitsEvt,
);
impl SetPositionPermissionlessOperationBitsEvtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POSITION_PERMISSIONLESS_OPERATION_BITS_EVT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetPositionPermissionlessOperationBitsEvt::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POSITION_PERMISSIONLESS_OPERATION_BITS_EVT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EVENT_DISCM: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Swap {
    pub lb_pair: Pubkey,
    pub from: Pubkey,
    pub start_bin_id: i32,
    pub end_bin_id: i32,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_for_y: bool,
    pub fee: u64,
    pub protocol_fee: u64,
    pub fee_bps: u128,
    pub host_fee: u64,
}
impl Swap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let end_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_for_y: bool = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u128 = crate::borsh_de_or_default(&mut reader)?;
        let host_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            from,
            start_bin_id,
            end_bin_id,
            amount_in,
            amount_out,
            swap_for_y,
            fee,
            protocol_fee,
            fee_bps,
            host_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_for_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEvent(pub Swap);
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Swap::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP2_EVT_EVENT_DISCM: [u8; 8] = [46, 116, 82, 215, 148, 27, 84, 77];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Swap2Evt {
    pub lb_pair: Pubkey,
    pub from: Pubkey,
    pub start_bin_id: i32,
    pub end_bin_id: i32,
    pub swap_for_y: bool,
    pub fee_bps: u128,
    pub amount_in: u64,
    pub amount_left: u64,
    pub amount_out: u64,
    pub mm_fee: u64,
    pub protocol_fee: u64,
    pub limit_order_fee: u64,
    pub host_fee: u64,
    pub fees_on_input: bool,
    pub fees_on_token_x: bool,
}
impl Swap2Evt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let end_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let swap_for_y: bool = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_left: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mm_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit_order_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let host_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_on_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let fees_on_token_x: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            from,
            start_bin_id,
            end_bin_id,
            swap_for_y,
            fee_bps,
            amount_in,
            amount_left,
            amount_out,
            mm_fee,
            protocol_fee,
            limit_order_fee,
            host_fee,
            fees_on_input,
            fees_on_token_x,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_for_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_left, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mm_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_order_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_on_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_on_token_x, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Swap2EvtEvent(pub Swap2Evt);
impl Swap2EvtEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP2_EVT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Swap2Evt::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP2_EVT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM: [u8; 8] = [
    133, 214, 66, 224, 64, 12, 7, 191,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePositionLockReleasePoint {
    pub position: Pubkey,
    pub current_point: u64,
    pub new_lock_release_point: u64,
    pub old_lock_release_point: u64,
    pub sender: Pubkey,
}
impl UpdatePositionLockReleasePoint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let current_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_lock_release_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_lock_release_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position,
            current_point,
            new_lock_release_point,
            old_lock_release_point,
            sender,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_lock_release_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_lock_release_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePositionLockReleasePointEvent(pub UpdatePositionLockReleasePoint);
impl UpdatePositionLockReleasePointEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdatePositionLockReleasePoint::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_POSITION_OPERATOR_EVENT_DISCM: [u8; 8] = [
    39, 115, 48, 204, 246, 47, 66, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePositionOperator {
    pub position: Pubkey,
    pub old_operator: Pubkey,
    pub new_operator: Pubkey,
}
impl UpdatePositionOperator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position,
            old_operator,
            new_operator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_operator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePositionOperatorEvent(pub UpdatePositionOperator);
impl UpdatePositionOperatorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POSITION_OPERATOR_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdatePositionOperator::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POSITION_OPERATOR_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_REWARD_DURATION_EVENT_DISCM: [u8; 8] = [
    223, 245, 224, 153, 49, 29, 163, 172,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardDuration {
    pub lb_pair: Pubkey,
    pub reward_index: u64,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}
impl UpdateRewardDuration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            reward_index,
            old_reward_duration,
            new_reward_duration,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_reward_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reward_duration, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationEvent(pub UpdateRewardDuration);
impl UpdateRewardDurationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_DURATION_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateRewardDuration::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_DURATION_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_REWARD_FUNDER_EVENT_DISCM: [u8; 8] = [
    224, 178, 174, 74, 252, 165, 85, 180,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardFunder {
    pub lb_pair: Pubkey,
    pub reward_index: u64,
    pub old_funder: Pubkey,
    pub new_funder: Pubkey,
}
impl UpdateRewardFunder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let old_funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            reward_index,
            old_funder,
            new_funder,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_funder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_funder, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderEvent(pub UpdateRewardFunder);
impl UpdateRewardFunderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_FUNDER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateRewardFunder::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_FUNDER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM: [u8; 8] = [
    231, 189, 65, 149, 102, 215, 154, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIneligibleReward {
    pub lb_pair: Pubkey,
    pub reward_mint: Pubkey,
    pub amount: u64,
}
impl WithdrawIneligibleReward {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            reward_mint,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardEvent(pub WithdrawIneligibleReward);
impl WithdrawIneligibleRewardEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawIneligibleReward::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
