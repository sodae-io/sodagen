use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const EXCHANGE_STATE_ACCOUNT_DISCM: [u8; 8] = [227, 194, 255, 253, 114, 95, 0, 25];
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
pub struct ExchangeState {
    pub admin: Pubkey,
    pub frozen: bool,
    pub router_pubkey: Pubkey,
    pub fee_balance: u64,
    pub bump: u8,
}
impl ExchangeState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let frozen: bool = crate::borsh_de_or_default(&mut reader)?;
        let router_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            frozen,
            router_pubkey,
            fee_balance,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.frozen, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.router_pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExchangeStateAccount(pub ExchangeState);
impl ExchangeStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXCHANGE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ExchangeState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXCHANGE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LP_POSITION_ACCOUNT_DISCM: [u8; 8] = [105, 241, 37, 200, 224, 2, 252, 90];
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
pub struct LpPosition {
    pub owner: Pubkey,
    pub total_deposited: u64,
    pub is_initialized: bool,
    pub bump: u8,
}
impl LpPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            total_deposited,
            is_initialized,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_initialized, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpPositionAccount(pub LpPosition);
impl LpPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LpPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_STATE_ACCOUNT_DISCM: [u8; 8] = [229, 212, 222, 222, 191, 128, 176, 235];
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
pub struct PairState {
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub x_to_y_fee_bps: i16,
    pub y_to_x_fee_bps: i16,
    pub last_updated_slot: u64,
    pub bump: u8,
}
impl PairState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let x_to_y_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        let y_to_x_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint_x,
            mint_y,
            x_to_y_fee_bps,
            y_to_x_fee_bps,
            last_updated_slot,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.x_to_y_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.y_to_x_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairStateAccount(pub PairState);
impl PairStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PairState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_STATE_ACCOUNT_DISCM: [u8; 8] = [218, 112, 6, 149, 55, 186, 168, 163];
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
pub struct TokenState {
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}
impl TokenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            token_program,
            bump,
            vault_bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenStateAccount(pub TokenState);
impl TokenStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHITELIST_ENTRY_ACCOUNT_DISCM: [u8; 8] = [51, 70, 173, 81, 219, 192, 234, 62];
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
pub struct WhitelistEntry {
    pub wallet: Pubkey,
    pub bump: u8,
}
impl WhitelistEntry {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { wallet, bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhitelistEntryAccount(pub WhitelistEntry);
impl WhitelistEntryAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHITELIST_ENTRY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WhitelistEntry::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHITELIST_ENTRY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
