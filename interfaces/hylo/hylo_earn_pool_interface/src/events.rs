use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ABSORB_LOSS_EVENT_EVENT_DISCM: [u8; 8] = [62, 134, 65, 81, 68, 34, 202, 148];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbsorbLossEvent {
    pub requested_loss: UFixValue64,
    pub amount_stablecoin_burned: UFixValue64,
    pub remaining_pool_balance: UFixValue64,
}
impl AbsorbLossEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let requested_loss = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let amount_stablecoin_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let remaining_pool_balance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            requested_loss,
            amount_stablecoin_burned,
            remaining_pool_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.requested_loss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_stablecoin_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remaining_pool_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AbsorbLossEventEvent(pub AbsorbLossEvent);
impl AbsorbLossEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ABSORB_LOSS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AbsorbLossEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ABSORB_LOSS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAUSE_EVENT_EVENT_DISCM: [u8; 8] = [32, 51, 61, 169, 156, 104, 130, 43];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PauseEvent {}
impl PauseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PauseEventEvent(pub PauseEvent);
impl PauseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PauseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UNPAUSE_EVENT_EVENT_DISCM: [u8; 8] = [134, 156, 8, 215, 185, 128, 192, 217];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnpauseEvent {}
impl UnpauseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseEventEvent(pub UnpauseEvent);
impl UnpauseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UnpauseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_DEPOSIT_LIMIT_EVENT_EVENT_DISCM: [u8; 8] = [
    15, 216, 122, 168, 187, 211, 86, 137,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDepositLimitEvent {
    pub old_deposit_limit: UFixValue64,
    pub new_deposit_limit: UFixValue64,
}
impl UpdateDepositLimitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_deposit_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_deposit_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_deposit_limit,
            new_deposit_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_deposit_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_deposit_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDepositLimitEventEvent(pub UpdateDepositLimitEvent);
impl UpdateDepositLimitEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DEPOSIT_LIMIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateDepositLimitEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DEPOSIT_LIMIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_WITHDRAWAL_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    204, 60, 32, 187, 54, 211, 107, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateWithdrawalFeeEvent {
    pub old_withdrawal_fee: UFixValue64,
    pub new_withdrawal_fee: UFixValue64,
}
impl UpdateWithdrawalFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_withdrawal_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_withdrawal_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_withdrawal_fee,
            new_withdrawal_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_withdrawal_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_withdrawal_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateWithdrawalFeeEventEvent(pub UpdateWithdrawalFeeEvent);
impl UpdateWithdrawalFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_WITHDRAWAL_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateWithdrawalFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WITHDRAWAL_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_WITHDRAWAL_LIMIT_EVENT_EVENT_DISCM: [u8; 8] = [
    100, 152, 116, 186, 126, 43, 176, 213,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateWithdrawalLimitEvent {
    pub old_withdrawal_limit: UFixValue64,
    pub new_withdrawal_limit: UFixValue64,
}
impl UpdateWithdrawalLimitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_withdrawal_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let new_withdrawal_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            old_withdrawal_limit,
            new_withdrawal_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_withdrawal_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_withdrawal_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateWithdrawalLimitEventEvent(pub UpdateWithdrawalLimitEvent);
impl UpdateWithdrawalLimitEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_WITHDRAWAL_LIMIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateWithdrawalLimitEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WITHDRAWAL_LIMIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [
    70, 55, 110, 118, 235, 187, 239, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserDepositEvent {
    pub stablecoin_deposited: UFixValue64,
    pub lp_token_nav: UFixValue64,
    pub lp_token_minted: UFixValue64,
}
impl UserDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stablecoin_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let lp_token_nav = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let lp_token_minted = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            stablecoin_deposited,
            lp_token_nav,
            lp_token_minted,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.stablecoin_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_minted, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserDepositEventEvent(pub UserDepositEvent);
impl UserDepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserDepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [
    133, 189, 140, 255, 132, 211, 143, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserWithdrawEvent {
    pub lp_token_burned: UFixValue64,
    pub stablecoin_withdrawn: UFixValue64,
    pub stablecoin_fees: UFixValue64,
}
impl UserWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_token_burned = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_withdrawn = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            lp_token_burned,
            stablecoin_withdrawn,
            stablecoin_fees,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_token_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_fees, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserWithdrawEventEvent(pub UserWithdrawEvent);
impl UserWithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserWithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
