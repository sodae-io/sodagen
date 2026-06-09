use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ORDER_ACCOUNT_DISCM: [u8; 8] = [134, 173, 223, 185, 77, 86, 28, 51];
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
pub struct Order {
    pub global_config: Pubkey,
    pub maker: Pubkey,
    pub input_mint: Pubkey,
    pub input_mint_program_id: Pubkey,
    pub output_mint: Pubkey,
    pub output_mint_program_id: Pubkey,
    pub initial_input_amount: u64,
    pub expected_output_amount: u64,
    pub remaining_input_amount: u64,
    pub filled_output_amount: u64,
    pub tip_amount: u64,
    pub number_of_fills: u64,
    pub order_type: u8,
    pub status: u8,
    pub in_vault_bump: u8,
    pub flash_ix_lock: u8,
    pub permissionless: u8,
    pub padding0: [u8; 3],
    pub last_updated_timestamp: u64,
    pub flash_start_taker_output_balance: u64,
    pub counterparty: Pubkey,
    pub padding: [u64; 15],
}
impl Order {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_mint_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint_program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let initial_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let filled_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tip_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_fills: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let in_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let flash_ix_lock: u8 = crate::borsh_de_or_default(&mut reader)?;
        let permissionless: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let flash_start_taker_output_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let counterparty: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 15] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            global_config,
            maker,
            input_mint,
            input_mint_program_id,
            output_mint,
            output_mint_program_id,
            initial_input_amount,
            expected_output_amount,
            remaining_input_amount,
            filled_output_amount,
            tip_amount,
            number_of_fills,
            order_type,
            status,
            in_vault_bump,
            flash_ix_lock,
            permissionless,
            padding0,
            last_updated_timestamp,
            flash_start_taker_output_balance,
            counterparty,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.global_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint_program_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expected_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remaining_input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filled_output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tip_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_fills, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_vault_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flash_ix_lock, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permissionless, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.flash_start_taker_output_balance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.counterparty, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderAccount(pub Order);
impl OrderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Order::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORDER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_SWAP_BALANCES_STATE_ACCOUNT_DISCM: [u8; 8] = [
    140, 228, 152, 62, 231, 27, 245, 198,
];
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
pub struct UserSwapBalancesState {
    pub user_lamports: u64,
    pub input_ta_balance: u64,
    pub output_ta_balance: u64,
}
impl UserSwapBalancesState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_ta_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_ta_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user_lamports,
            input_ta_balance,
            output_ta_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_lamports, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_ta_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_ta_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserSwapBalancesStateAccount(pub UserSwapBalancesState);
impl UserSwapBalancesStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_SWAP_BALANCES_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserSwapBalancesState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_SWAP_BALANCES_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GLOBAL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalConfig {
    pub emergency_mode: u8,
    pub flash_take_order_blocked: u8,
    pub new_orders_blocked: u8,
    pub orders_taking_blocked: u8,
    pub host_fee_bps: u16,
    pub padding0: [u8; 2],
    pub order_close_delay_seconds: u64,
    pub padding1: [u64; 9],
    pub pda_authority_previous_lamports_balance: u64,
    pub total_tip_amount: u64,
    pub host_tip_amount: u64,
    pub pda_authority: Pubkey,
    pub pda_authority_bump: u64,
    pub admin_authority: Pubkey,
    pub admin_authority_cached: Pubkey,
    pub txn_fee_cost: u64,
    pub ata_creation_cost: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding2: [u64; 241],
}
impl GlobalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let emergency_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let flash_take_order_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let new_orders_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let orders_taking_blocked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let host_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let order_close_delay_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 9] = crate::borsh_de_or_default(&mut reader)?;
        let pda_authority_previous_lamports_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_tip_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let host_tip_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pda_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pda_authority_bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let admin_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_authority_cached: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let txn_fee_cost: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ata_creation_cost: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding2 = <[u64; 241] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            emergency_mode,
            flash_take_order_blocked,
            new_orders_blocked,
            orders_taking_blocked,
            host_fee_bps,
            padding0,
            order_close_delay_seconds,
            padding1,
            pda_authority_previous_lamports_balance,
            total_tip_amount,
            host_tip_amount,
            pda_authority,
            pda_authority_bump,
            admin_authority,
            admin_authority_cached,
            txn_fee_cost,
            ata_creation_cost,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.emergency_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flash_take_order_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_orders_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders_taking_blocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_close_delay_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.pda_authority_previous_lamports_balance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_tip_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.host_tip_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pda_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_authority_cached, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.txn_fee_cost, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ata_creation_cost, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigAccount(pub GlobalConfig);
impl GlobalConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
