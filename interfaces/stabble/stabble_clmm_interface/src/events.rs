use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const COLLECT_PERSONAL_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    166, 174, 105, 192, 81, 161, 83, 105,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectPersonalFeeEvent {
    pub position_nft_mint: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub amount_0: u64,
    pub amount_1: u64,
}
impl CollectPersonalFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_token_account_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_token_account_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_nft_mint,
            recipient_token_account_0,
            recipient_token_account_1,
            amount_0,
            amount_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_token_account_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_token_account_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectPersonalFeeEventEvent(pub CollectPersonalFeeEvent);
impl CollectPersonalFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PERSONAL_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectPersonalFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PERSONAL_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_PROTOCOL_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    206, 87, 17, 79, 45, 41, 213, 61,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeeEvent {
    pub pool_state: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub amount_0: u64,
    pub amount_1: u64,
}
impl CollectProtocolFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_token_account_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_token_account_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            recipient_token_account_0,
            recipient_token_account_1,
            amount_0,
            amount_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_token_account_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_token_account_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeEventEvent(pub CollectProtocolFeeEvent);
impl CollectProtocolFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectProtocolFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_CHANGE_EVENT_EVENT_DISCM: [u8; 8] = [
    247, 189, 7, 119, 106, 112, 95, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigChangeEvent {
    pub index: u16,
    pub owner: Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub fund_owner: Pubkey,
}
impl ConfigChangeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fee_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fund_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            owner,
            protocol_fee_rate,
            trade_fee_rate,
            tick_spacing,
            fund_fee_rate,
            fund_owner,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigChangeEventEvent(pub ConfigChangeEvent);
impl ConfigChangeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_CHANGE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ConfigChangeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_CHANGE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_PERSONAL_POSITION_EVENT_EVENT_DISCM: [u8; 8] = [
    100, 30, 87, 249, 196, 223, 154, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePersonalPositionEvent {
    pub pool_state: Pubkey,
    pub personal_position: Pubkey,
    pub nft_mint: Pubkey,
    pub minter: Pubkey,
    pub nft_owner: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub deposit_amount_0: u64,
    pub deposit_amount_1: u64,
    pub deposit_amount_0_transfer_fee: u64,
    pub deposit_amount_1_transfer_fee: u64,
}
impl CreatePersonalPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let personal_position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let minter: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let nft_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_amount_0_transfer_fee: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposit_amount_1_transfer_fee: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            personal_position,
            nft_mint,
            minter,
            nft_owner,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            deposit_amount_0,
            deposit_amount_1,
            deposit_amount_0_transfer_fee,
            deposit_amount_1_transfer_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.personal_position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nft_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.deposit_amount_0_transfer_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.deposit_amount_1_transfer_fee,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePersonalPositionEventEvent(pub CreatePersonalPositionEvent);
impl CreatePersonalPositionEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PERSONAL_POSITION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreatePersonalPositionEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PERSONAL_POSITION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DECREASE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    58, 222, 86, 58, 68, 50, 85, 56,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityEvent {
    pub position_nft_mint: Pubkey,
    pub liquidity: u128,
    pub decrease_amount_0: u64,
    pub decrease_amount_1: u64,
    pub fee_amount_0: u64,
    pub fee_amount_1: u64,
    pub reward_amounts: [u64; 3],
    pub transfer_fee_0: u64,
    pub transfer_fee_1: u64,
}
impl DecreaseLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let decrease_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decrease_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_amounts: [u64; 3] = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_nft_mint,
            liquidity,
            decrease_amount_0,
            decrease_amount_1,
            fee_amount_0,
            fee_amount_1,
            reward_amounts,
            transfer_fee_0,
            transfer_fee_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decrease_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decrease_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_amounts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_fee_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_fee_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityEventEvent(pub DecreaseLiquidityEvent);
impl DecreaseLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DecreaseLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    49, 79, 105, 212, 32, 34, 30, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityEvent {
    pub position_nft_mint: Pubkey,
    pub liquidity: u128,
    pub amount_0: u64,
    pub amount_1: u64,
    pub amount_0_transfer_fee: u64,
    pub amount_1_transfer_fee: u64,
}
impl IncreaseLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_0_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_nft_mint,
            liquidity,
            amount_0,
            amount_1,
            amount_0_transfer_fee,
            amount_1_transfer_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1_transfer_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityEventEvent(pub IncreaseLiquidityEvent);
impl IncreaseLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreaseLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_CALCULATE_EVENT_EVENT_DISCM: [u8; 8] = [
    237, 112, 148, 230, 57, 84, 180, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityCalculateEvent {
    pub pool_liquidity: u128,
    pub pool_sqrt_price_x64: u128,
    pub pool_tick: i32,
    pub calc_amount_0: u64,
    pub calc_amount_1: u64,
    pub trade_fee_owed_0: u64,
    pub trade_fee_owed_1: u64,
    pub transfer_fee_0: u64,
    pub transfer_fee_1: u64,
}
impl LiquidityCalculateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let pool_sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let pool_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let calc_amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let calc_amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_owed_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_owed_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_liquidity,
            pool_sqrt_price_x64,
            pool_tick,
            calc_amount_0,
            calc_amount_1,
            trade_fee_owed_0,
            trade_fee_owed_1,
            transfer_fee_0,
            transfer_fee_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_sqrt_price_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_tick, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.calc_amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.calc_amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_owed_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_owed_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_fee_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_fee_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityCalculateEventEvent(pub LiquidityCalculateEvent);
impl LiquidityCalculateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_CALCULATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityCalculateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_CALCULATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_CHANGE_EVENT_EVENT_DISCM: [u8; 8] = [
    126, 240, 175, 206, 158, 88, 153, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityChangeEvent {
    pub pool_state: Pubkey,
    pub tick: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity_before: u128,
    pub liquidity_after: u128,
}
impl LiquidityChangeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_before: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_after: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            tick,
            tick_lower,
            tick_upper,
            liquidity_before,
            liquidity_after,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityChangeEventEvent(pub LiquidityChangeEvent);
impl LiquidityChangeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_CHANGE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityChangeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_CHANGE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [25, 94, 75, 47, 112, 99, 53, 63];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolCreatedEvent {
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub tick_spacing: u16,
    pub pool_state: Pubkey,
    pub sqrt_price_x64: u128,
    pub tick: i32,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub amm_config: Pubkey,
}
impl PoolCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_mint_0,
            token_mint_1,
            tick_spacing,
            pool_state,
            sqrt_price_x64,
            tick,
            token_vault_0,
            token_vault_1,
            amm_config,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreatedEventEvent(pub PoolCreatedEvent);
impl PoolCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CREATED_EVENT_EVENT_DISCM)?;
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
    pub pool_state: Pubkey,
    pub sender: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
    pub amount_0: u64,
    pub transfer_fee_0: u64,
    pub amount_1: u64,
    pub transfer_fee_1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_account_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_account_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let zero_for_one: bool = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            sender,
            token_account_0,
            token_account_1,
            amount_0,
            transfer_fee_0,
            amount_1,
            transfer_fee_1,
            zero_for_one,
            sqrt_price_x64,
            liquidity,
            tick,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_account_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_account_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_fee_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_fee_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.zero_for_one, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x64, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
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
pub const UPDATE_REWARD_INFOS_EVENT_EVENT_DISCM: [u8; 8] = [
    109, 127, 186, 78, 114, 65, 37, 236,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardInfosEvent {
    pub reward_growth_global_x64: [u128; 3],
}
impl UpdateRewardInfosEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reward_growth_global_x64: [u128; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { reward_growth_global_x64 })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.reward_growth_global_x64, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardInfosEventEvent(pub UpdateRewardInfosEvent);
impl UpdateRewardInfosEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_INFOS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateRewardInfosEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_INFOS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
