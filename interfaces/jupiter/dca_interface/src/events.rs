use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const COLLECTED_FEE_EVENT_DISCM: [u8; 8] = [42, 136, 216, 116, 181, 209, 109, 181];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectedFee {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
impl CollectedFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dca_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user_key,
            dca_key,
            mint,
            amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dca_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectedFeeEvent(pub CollectedFee);
impl CollectedFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECTED_FEE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectedFee::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECTED_FEE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FILLED_EVENT_DISCM: [u8; 8] = [134, 4, 17, 63, 221, 45, 177, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Filled {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub fee_mint: Pubkey,
    pub fee: u64,
}
impl Filled {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dca_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user_key,
            dca_key,
            input_mint,
            output_mint,
            in_amount,
            out_amount,
            fee_mint,
            fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dca_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FilledEvent(pub Filled);
impl FilledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILLED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Filled::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILLED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPENED_EVENT_DISCM: [u8; 8] = [166, 172, 97, 9, 77, 76, 189, 109];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Opened {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub in_deposited: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub cycle_frequency: i64,
    pub in_amount_per_cycle: u64,
    pub created_at: i64,
}
impl Opened {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dca_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user_key,
            dca_key,
            in_deposited,
            input_mint,
            output_mint,
            cycle_frequency,
            in_amount_per_cycle,
            created_at,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dca_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cycle_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount_per_cycle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenedEvent(pub Opened);
impl OpenedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPENED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Opened::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPENED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLOSED_EVENT_DISCM: [u8; 8] = [50, 31, 87, 155, 135, 220, 195, 239];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Closed {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub in_deposited: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub cycle_frequency: i64,
    pub in_amount_per_cycle: u64,
    pub created_at: i64,
    pub total_in_withdrawn: u64,
    pub total_out_withdrawn: u64,
    pub unfilled_amount: u64,
    pub user_closed: bool,
}
impl Closed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dca_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_in_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_out_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unfilled_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_closed: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user_key,
            dca_key,
            in_deposited,
            input_mint,
            output_mint,
            cycle_frequency,
            in_amount_per_cycle,
            created_at,
            total_in_withdrawn,
            total_out_withdrawn,
            unfilled_amount,
            user_closed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dca_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cycle_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount_per_cycle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_in_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_out_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unfilled_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_closed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClosedEvent(pub Closed);
impl ClosedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Closed::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_EVENT_DISCM: [u8; 8] = [192, 241, 201, 217, 70, 150, 90, 247];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Withdraw {
    pub dca_key: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub user_withdraw: bool,
}
impl Withdraw {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dca_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_withdraw: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            dca_key,
            in_amount,
            out_amount,
            user_withdraw,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dca_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_withdraw, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawEvent(pub Withdraw);
impl WithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Withdraw::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_EVENT_DISCM: [u8; 8] = [62, 205, 242, 175, 244, 169, 136, 52];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Deposit {
    pub dca_key: Pubkey,
    pub amount: u64,
}
impl Deposit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let dca_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { dca_key, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.dca_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositEvent(pub Deposit);
impl DepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Deposit::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
