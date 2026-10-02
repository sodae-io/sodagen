use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LOG_DEPOSIT_EVENT_DISCM: [u8; 8] = [176, 243, 1, 56, 142, 206, 1, 106];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogDeposit {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub assets: u64,
    pub shares_minted: u64,
}
impl LogDeposit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sender,
            receiver,
            assets,
            shares_minted,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_minted, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogDepositEvent(pub LogDeposit);
impl LogDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_DEPOSIT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogDeposit::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_DEPOSIT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_REBALANCE_EVENT_DISCM: [u8; 8] = [90, 67, 219, 41, 181, 118, 132, 9];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogRebalance {
    pub assets: u64,
}
impl LogRebalance {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { assets })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogRebalanceEvent(pub LogRebalance);
impl LogRebalanceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_REBALANCE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogRebalance::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_REBALANCE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_AUTHORITY_EVENT_DISCM: [u8; 8] = [
    150, 152, 157, 143, 6, 135, 193, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateAuthority {
    pub new_authority: Pubkey,
}
impl LogUpdateAuthority {
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
pub struct LogUpdateAuthorityEvent(pub LogUpdateAuthority);
impl LogUpdateAuthorityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_AUTHORITY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateAuthority::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_AUTHORITY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_AUTHS_EVENT_DISCM: [u8; 8] = [88, 80, 109, 48, 111, 203, 76, 251];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateAuths {
    pub auth_status: Vec<AddressBool>,
}
impl LogUpdateAuths {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let auth_status: Vec<AddressBool> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { auth_status })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.auth_status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateAuthsEvent(pub LogUpdateAuths);
impl LogUpdateAuthsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_AUTHS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateAuths::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_AUTHS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_RATES_EVENT_DISCM: [u8; 8] = [222, 11, 113, 60, 147, 15, 68, 217];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateRates {
    pub token_exchange_price: u64,
    pub liquidity_exchange_price: u64,
}
impl LogUpdateRates {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_exchange_price,
            liquidity_exchange_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_exchange_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateRatesEvent(pub LogUpdateRates);
impl LogUpdateRatesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_RATES_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateRates::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_RATES_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_REBALANCER_EVENT_DISCM: [u8; 8] = [
    66, 79, 144, 204, 26, 217, 153, 225,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateRebalancer {
    pub new_rebalancer: Pubkey,
}
impl LogUpdateRebalancer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_rebalancer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { new_rebalancer })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.new_rebalancer, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateRebalancerEvent(pub LogUpdateRebalancer);
impl LogUpdateRebalancerEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_REBALANCER_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateRebalancer::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_REBALANCER_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_UPDATE_REWARDS_EVENT_DISCM: [u8; 8] = [
    37, 13, 111, 186, 47, 245, 162, 121,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogUpdateRewards {
    pub rewards_rate_model: Pubkey,
}
impl LogUpdateRewards {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let rewards_rate_model: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { rewards_rate_model })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.rewards_rate_model, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogUpdateRewardsEvent(pub LogUpdateRewards);
impl LogUpdateRewardsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_UPDATE_REWARDS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogUpdateRewards::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_UPDATE_REWARDS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOG_WITHDRAW_EVENT_DISCM: [u8; 8] = [49, 9, 176, 179, 222, 190, 6, 117];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogWithdraw {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub owner: Pubkey,
    pub assets: u64,
    pub shares_burned: u64,
}
impl LogWithdraw {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let assets: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sender,
            receiver,
            owner,
            assets,
            shares_burned,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.assets, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shares_burned, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogWithdrawEvent(pub LogWithdraw);
impl LogWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOG_WITHDRAW_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LogWithdraw::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOG_WITHDRAW_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
