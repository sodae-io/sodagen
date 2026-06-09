use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LP_CHANGE_EVENT_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpChangeEvent {
    pub pool_id: Pubkey,
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}
impl LpChangeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let change_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            lp_amount_before,
            token_0_vault_before,
            token_1_vault_before,
            token_0_amount,
            token_1_amount,
            token_0_transfer_fee,
            token_1_transfer_fee,
            change_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_amount_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_vault_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_vault_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_0_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_1_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.change_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpChangeEventEvent(pub LpChangeEvent);
impl LpChangeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LP_CHANGE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LpChangeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LP_CHANGE_EVENT_EVENT_DISCM)?;
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
    pub pool_id: Pubkey,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub trade_fee: u64,
    pub creator_fee: u64,
    pub creator_fee_on_input: bool,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_on_input: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            input_vault_before,
            output_vault_before,
            input_amount,
            output_amount,
            input_transfer_fee,
            output_transfer_fee,
            base_input,
            input_mint,
            output_mint,
            trade_fee,
            creator_fee,
            creator_fee_on_input,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_vault_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_vault_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_on_input, &mut writer)?;
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
