use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const INIT_EVENT_EVENT_DISCM: [u8; 8] = [224, 129, 78, 87, 58, 43, 94, 127];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitEvent {
    pub payer: Pubkey,
    pub decimals: u8,
    pub multiplier: u64,
    pub wrapper_underlying_mint: Pubkey,
    pub wrapper_underlying_tokens: Pubkey,
    pub wrapper_mint: Pubkey,
}
impl InitEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_underlying_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_underlying_tokens: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            payer,
            decimals,
            multiplier,
            wrapper_underlying_mint,
            wrapper_underlying_tokens,
            wrapper_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.multiplier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapper_underlying_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapper_underlying_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapper_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitEventEvent(pub InitEvent);
impl InitEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositEvent {
    pub owner: Pubkey,
    pub underlying_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub deposit_amount: u64,
    pub mint_amount: u64,
}
impl DepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let underlying_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapped_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            underlying_mint,
            wrapped_mint,
            deposit_amount,
            mint_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.underlying_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapped_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositEventEvent(pub DepositEvent);
impl DepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [22, 9, 133, 26, 160, 44, 71, 192];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawEvent {
    pub owner: Pubkey,
    pub underlying_mint: Pubkey,
    pub wrapped_mint: Pubkey,
    pub withdraw_amount: u64,
    pub burn_amount: u64,
    pub dust_amount: u64,
}
impl WithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let underlying_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapped_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dust_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            underlying_mint,
            wrapped_mint,
            withdraw_amount,
            burn_amount,
            dust_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.underlying_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapped_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.burn_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dust_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawEventEvent(pub WithdrawEvent);
impl WithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
