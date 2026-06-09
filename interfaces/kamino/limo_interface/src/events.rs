use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ORDER_DISPLAY_EVENT_DISCM: [u8; 8] = [92, 101, 6, 158, 248, 152, 241, 60];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDisplay {
    pub initial_input_amount: u64,
    pub expected_output_amount: u64,
    pub remaining_input_amount: u64,
    pub filled_output_amount: u64,
    pub tip_amount: u64,
    pub number_of_fills: u64,
    pub on_event_output_amount_filled: u64,
    pub on_event_tip_amount: u64,
    pub order_type: u8,
    pub status: u8,
    pub last_updated_timestamp: u64,
}
impl OrderDisplay {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initial_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let filled_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tip_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_fills: u64 = crate::borsh_de_or_default(&mut reader)?;
        let on_event_output_amount_filled: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let on_event_tip_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initial_input_amount,
            expected_output_amount,
            remaining_input_amount,
            filled_output_amount,
            tip_amount,
            number_of_fills,
            on_event_output_amount_filled,
            on_event_tip_amount,
            order_type,
            status,
            last_updated_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.initial_input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expected_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remaining_input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filled_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tip_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_fills, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.on_event_output_amount_filled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.on_event_tip_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderDisplayEvent(pub OrderDisplay);
impl OrderDisplayEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORDER_DISPLAY_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OrderDisplay::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORDER_DISPLAY_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_SWAP_BALANCE_DIFFS_EVENT_DISCM: [u8; 8] = [
    139, 203, 35, 31, 25, 8, 62, 143,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserSwapBalanceDiffs {
    pub user_lamports_before: u64,
    pub input_ta_balance_before: u64,
    pub output_ta_balance_before: u64,
    pub user_lamports_after: u64,
    pub input_ta_balance_after: u64,
    pub output_ta_balance_after: u64,
    pub swap_program: Pubkey,
    pub simulated_swap_amount_out: u64,
    pub simulated_ts: u64,
    pub minimum_amount_out: u64,
    pub swap_amount_in: u64,
    pub simulated_amount_out_next_best: u64,
    pub aggregator: u8,
    pub next_best_aggregator: u8,
}
impl UserSwapBalanceDiffs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_lamports_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_ta_balance_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_ta_balance_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lamports_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_ta_balance_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_ta_balance_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let simulated_swap_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let simulated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let simulated_amount_out_next_best: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let aggregator: u8 = crate::borsh_de_or_default(&mut reader)?;
        let next_best_aggregator: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user_lamports_before,
            input_ta_balance_before,
            output_ta_balance_before,
            user_lamports_after,
            input_ta_balance_after,
            output_ta_balance_after,
            swap_program,
            simulated_swap_amount_out,
            simulated_ts,
            minimum_amount_out,
            swap_amount_in,
            simulated_amount_out_next_best,
            aggregator,
            next_best_aggregator,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_lamports_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_ta_balance_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_ta_balance_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_lamports_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_ta_balance_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_ta_balance_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.simulated_swap_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.simulated_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minimum_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.simulated_amount_out_next_best,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.aggregator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_best_aggregator, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserSwapBalanceDiffsEvent(pub UserSwapBalanceDiffs);
impl UserSwapBalanceDiffsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_SWAP_BALANCE_DIFFS_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserSwapBalanceDiffs::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_SWAP_BALANCE_DIFFS_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
