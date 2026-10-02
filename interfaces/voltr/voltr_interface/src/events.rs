use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ACCEPT_PROTOCOL_ADMIN_EVENT_EVENT_DISCM: [u8; 8] = [
    196, 93, 173, 253, 175, 140, 121, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptProtocolAdminEvent {
    pub protocol: Pubkey,
    pub admin_before: Pubkey,
    pub admin_after: Pubkey,
    pub accepted_ts: u64,
}
impl AcceptProtocolAdminEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_before: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_after: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let accepted_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol,
            admin_before,
            admin_after,
            accepted_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accepted_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptProtocolAdminEventEvent(pub AcceptProtocolAdminEvent);
impl AcceptProtocolAdminEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_PROTOCOL_ADMIN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AcceptProtocolAdminEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_PROTOCOL_ADMIN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ACCEPT_VAULT_ADMIN_EVENT_EVENT_DISCM: [u8; 8] = [
    140, 255, 29, 68, 16, 194, 251, 35,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptVaultAdminEvent {
    pub vault: Pubkey,
    pub admin_before: Pubkey,
    pub admin_after: Pubkey,
    pub accepted_ts: u64,
}
impl AcceptVaultAdminEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_before: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_after: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let accepted_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            admin_before,
            admin_after,
            accepted_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_after, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accepted_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptVaultAdminEventEvent(pub AcceptVaultAdminEvent);
impl AcceptVaultAdminEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_VAULT_ADMIN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AcceptVaultAdminEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_VAULT_ADMIN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ADD_ADAPTOR_EVENT_EVENT_DISCM: [u8; 8] = [
    24, 181, 201, 148, 240, 183, 235, 12,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddAdaptorEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub adaptor_program: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub adaptor_last_updated_ts: u64,
    pub added_ts: u64,
}
impl AddAdaptorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_add_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_last_updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let added_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            adaptor_program,
            adaptor_add_receipt,
            adaptor_last_updated_ts,
            added_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_add_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_last_updated_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.added_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddAdaptorEventEvent(pub AddAdaptorEvent);
impl AddAdaptorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_ADAPTOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = AddAdaptorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_ADAPTOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CALIBRATE_HIGH_WATER_MARK_EVENT_EVENT_DISCM: [u8; 8] = [
    140, 76, 195, 144, 180, 178, 0, 155,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CalibrateHighWaterMarkEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub vault_asset_total_value: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub amount_lp_burned: u64,
    pub calibrated_ts: u64,
}
impl CalibrateHighWaterMarkEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount_lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let calibrated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            vault_asset_total_value,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            amount_lp_burned,
            calibrated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_total_value, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.amount_lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.calibrated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CalibrateHighWaterMarkEventEvent(pub CalibrateHighWaterMarkEvent);
impl CalibrateHighWaterMarkEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CALIBRATE_HIGH_WATER_MARK_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CalibrateHighWaterMarkEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CALIBRATE_HIGH_WATER_MARK_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CALIBRATE_HIGH_WATER_MARK_UNSAFE_EVENT_EVENT_DISCM: [u8; 8] = [
    205, 76, 169, 255, 37, 228, 65, 130,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CalibrateHighWaterMarkUnsafeEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub vault_asset_total_value: u64,
    pub vault_lp_supply_incl_fees: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub calibrated_ts: u64,
}
impl CalibrateHighWaterMarkUnsafeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_supply_incl_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let calibrated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            vault_asset_total_value,
            vault_lp_supply_incl_fees,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            calibrated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_total_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_lp_supply_incl_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.calibrated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CalibrateHighWaterMarkUnsafeEventEvent(pub CalibrateHighWaterMarkUnsafeEvent);
impl CalibrateHighWaterMarkUnsafeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CALIBRATE_HIGH_WATER_MARK_UNSAFE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CalibrateHighWaterMarkUnsafeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CALIBRATE_HIGH_WATER_MARK_UNSAFE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CANCEL_REQUEST_WITHDRAW_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [
    46, 165, 24, 114, 1, 80, 205, 136,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelRequestWithdrawVaultEvent {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub request_withdraw_vault_receipt: Pubkey,
    pub amount_lp_refunded: u64,
    pub amount_lp_burned: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub cancelled_ts: u64,
}
impl CancelRequestWithdrawVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let request_withdraw_vault_receipt: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount_lp_refunded: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cancelled_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            user,
            request_withdraw_vault_receipt,
            amount_lp_refunded,
            amount_lp_burned,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            cancelled_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.request_withdraw_vault_receipt,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.amount_lp_refunded, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.cancelled_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelRequestWithdrawVaultEventEvent(pub CancelRequestWithdrawVaultEvent);
impl CancelRequestWithdrawVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_REQUEST_WITHDRAW_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CancelRequestWithdrawVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_REQUEST_WITHDRAW_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLOSE_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    213, 95, 219, 161, 17, 208, 93, 255,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloseStrategyEvent {
    pub manager: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub closed_ts: u64,
}
impl CloseStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let closed_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            manager,
            vault,
            strategy,
            strategy_init_receipt,
            closed_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.closed_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseStrategyEventEvent(pub CloseStrategyEvent);
impl CloseStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CloseStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    202, 201, 118, 49, 29, 180, 116, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositStrategyEvent {
    pub manager: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_amount_asset_deposited: u64,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub vault_asset_idle_ata_amount_before: u64,
    pub vault_asset_idle_ata_amount_after: u64,
    pub strategy_position_value_before: u64,
    pub strategy_position_value_after: u64,
    pub deposited_ts: u64,
}
impl DepositStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_amount_asset_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_idle_ata_amount_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_idle_ata_amount_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposited_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            manager,
            vault,
            strategy,
            strategy_init_receipt,
            adaptor_program,
            vault_asset_mint,
            vault_amount_asset_deposited,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            vault_asset_idle_ata_amount_before,
            vault_asset_idle_ata_amount_after,
            strategy_position_value_before,
            strategy_position_value_after,
            deposited_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_amount_asset_deposited,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_idle_ata_amount_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_idle_ata_amount_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.deposited_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStrategyEventEvent(pub DepositStrategyEvent);
impl DepositStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEPOSIT_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [11, 15, 7, 92, 150, 100, 165, 232];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositVaultEvent {
    pub user: Pubkey,
    pub user_amount_asset_deposited: u64,
    pub user_amount_lp_minted: u64,
    pub vault: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_lp_total_accumulated_fees_before: u64,
    pub vault_lp_total_accumulated_fees_after: u64,
    pub vault_lp_dead_weight_before: u64,
    pub vault_lp_dead_weight_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub deposited_ts: u64,
}
impl DepositVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_asset_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_lp_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_dead_weight_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_dead_weight_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposited_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            user_amount_asset_deposited,
            user_amount_lp_minted,
            vault,
            vault_asset_mint,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_lp_total_accumulated_fees_before,
            vault_lp_total_accumulated_fees_after,
            vault_lp_dead_weight_before,
            vault_lp_dead_weight_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            deposited_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_amount_asset_deposited,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_amount_lp_minted, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_dead_weight_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_lp_dead_weight_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.deposited_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositVaultEventEvent(pub DepositVaultEvent);
impl DepositVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DepositVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DIRECT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    113, 202, 151, 124, 137, 255, 153, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DirectWithdrawStrategyEvent {
    pub user: Pubkey,
    pub user_amount_asset_withdrawn: u64,
    pub user_amount_lp_burned: u64,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_total_value_unlocked_before: u64,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_lp_total_accumulated_fees_before: u64,
    pub vault_lp_total_accumulated_fees_after: u64,
    pub vault_lp_dead_weight_before: u64,
    pub vault_lp_dead_weight_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub strategy_position_value_before: u64,
    pub strategy_position_value_after: u64,
    pub withdrawn_ts: u64,
}
impl DirectWithdrawStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_asset_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let direct_withdraw_init_receipt: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_unlocked_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_dead_weight_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_dead_weight_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawn_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            user_amount_asset_withdrawn,
            user_amount_lp_burned,
            vault,
            strategy,
            strategy_init_receipt,
            direct_withdraw_init_receipt,
            adaptor_program,
            vault_asset_mint,
            vault_asset_total_value_unlocked_before,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_lp_total_accumulated_fees_before,
            vault_lp_total_accumulated_fees_after,
            vault_lp_dead_weight_before,
            vault_lp_dead_weight_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            strategy_position_value_before,
            strategy_position_value_after,
            withdrawn_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_amount_asset_withdrawn,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_amount_lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.direct_withdraw_init_receipt,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_unlocked_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_dead_weight_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_lp_dead_weight_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawn_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DirectWithdrawStrategyEventEvent(pub DirectWithdrawStrategyEvent);
impl DirectWithdrawStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DIRECT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DirectWithdrawStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DIRECT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HARVEST_FEE_EVENT_EVENT_DISCM: [u8; 8] = [69, 48, 192, 23, 232, 22, 23, 30];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HarvestFeeEvent {
    pub harvester: Pubkey,
    pub vault: Pubkey,
    pub protocol: Pubkey,
    pub admin: Pubkey,
    pub manager: Pubkey,
    pub amount_lp_admin_fees: u64,
    pub amount_lp_manager_fees: u64,
    pub amount_lp_protocol_fees: u64,
}
impl HarvestFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let harvester: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_lp_admin_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_lp_manager_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_lp_protocol_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            harvester,
            vault,
            protocol,
            admin,
            manager,
            amount_lp_admin_fees,
            amount_lp_manager_fees,
            amount_lp_protocol_fees,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.harvester, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lp_admin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lp_manager_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lp_protocol_fees, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestFeeEventEvent(pub HarvestFeeEvent);
impl HarvestFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = HarvestFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INIT_PROTOCOL_EVENT_EVENT_DISCM: [u8; 8] = [
    13, 81, 183, 132, 88, 43, 202, 213,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitProtocolEvent {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub operational_state: u16,
    pub initialized_ts: u64,
}
impl InitProtocolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: u16 = crate::borsh_de_or_default(&mut reader)?;
        let initialized_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            treasury,
            operational_state,
            initialized_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initialized_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitProtocolEventEvent(pub InitProtocolEvent);
impl InitProtocolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_PROTOCOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitProtocolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_PROTOCOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_DIRECT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    169, 22, 57, 8, 15, 73, 255, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeDirectWithdrawStrategyEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub instruction_discriminator: Vec<u8>,
    pub additional_args: Vec<u8>,
    pub allow_user_args: bool,
    pub initialized_ts: u64,
}
impl InitializeDirectWithdrawStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let instruction_discriminator: Vec<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_args: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        let allow_user_args: bool = crate::borsh_de_or_default(&mut reader)?;
        let initialized_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            strategy,
            strategy_init_receipt,
            adaptor_program,
            instruction_discriminator,
            additional_args,
            allow_user_args,
            initialized_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.instruction_discriminator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.additional_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_user_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initialized_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeDirectWithdrawStrategyEventEvent(
    pub InitializeDirectWithdrawStrategyEvent,
);
impl InitializeDirectWithdrawStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_DIRECT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeDirectWithdrawStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_DIRECT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    30, 233, 211, 249, 83, 188, 234, 152,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeStrategyEvent {
    pub manager: Pubkey,
    pub vault: Pubkey,
    pub vault_strategy_auth: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub initialized_ts: u64,
    pub is_initialized: bool,
}
impl InitializeStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_strategy_auth: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let initialized_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            manager,
            vault,
            vault_strategy_auth,
            strategy,
            strategy_init_receipt,
            adaptor_program,
            initialized_ts,
            is_initialized,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_strategy_auth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initialized_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_initialized, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeStrategyEventEvent(pub InitializeStrategyEvent);
impl InitializeStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [
    179, 75, 50, 161, 191, 28, 245, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeVaultEvent {
    pub payer: Pubkey,
    pub vault: Pubkey,
    pub vault_name: String,
    pub vault_description: String,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_idle_ata: Pubkey,
    pub vault_lp_mint: Pubkey,
    pub vault_manager: Pubkey,
    pub vault_admin: Pubkey,
    pub vault_config_max_cap: u64,
    pub vault_config_start_at_ts: u64,
    pub vault_config_locked_profit_degradation_duration: u64,
    pub vault_config_withdrawal_waiting_period: u64,
    pub vault_config_manager_performance_fee: u16,
    pub vault_config_admin_performance_fee: u16,
    pub vault_config_manager_management_fee: u16,
    pub vault_config_admin_management_fee: u16,
    pub vault_config_redemption_fee: u16,
    pub vault_config_issuance_fee: u16,
    pub initialized_ts: u64,
}
impl InitializeVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_name: String = crate::borsh_de_or_default(&mut reader)?;
        let vault_description: String = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_idle_ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_start_at_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_locked_profit_degradation_duration: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_withdrawal_waiting_period: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_manager_performance_fee: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_admin_performance_fee: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_manager_management_fee: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_admin_management_fee: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_redemption_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_issuance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let initialized_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            payer,
            vault,
            vault_name,
            vault_description,
            vault_asset_mint,
            vault_asset_idle_ata,
            vault_lp_mint,
            vault_manager,
            vault_admin,
            vault_config_max_cap,
            vault_config_start_at_ts,
            vault_config_locked_profit_degradation_duration,
            vault_config_withdrawal_waiting_period,
            vault_config_manager_performance_fee,
            vault_config_admin_performance_fee,
            vault_config_manager_management_fee,
            vault_config_admin_management_fee,
            vault_config_redemption_fee,
            vault_config_issuance_fee,
            initialized_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_description, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_idle_ata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_config_max_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_config_start_at_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_locked_profit_degradation_duration,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_withdrawal_waiting_period,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_manager_performance_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_admin_performance_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_manager_management_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_admin_management_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_redemption_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_config_issuance_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initialized_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultEventEvent(pub InitializeVaultEvent);
impl InitializeVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InitializeVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    221, 23, 12, 215, 100, 42, 97, 149,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawStrategyEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
    pub user_amount_asset_withdrawn: u64,
    pub user_amount_lp_burned: u64,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub direct_withdraw_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_total_value_unlocked_before: u64,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_lp_total_accumulated_fees_before: u64,
    pub vault_lp_total_accumulated_fees_after: u64,
    pub vault_lp_dead_weight_before: u64,
    pub vault_lp_dead_weight_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub strategy_position_value_before: u64,
    pub strategy_position_value_after: u64,
    pub withdrawn_ts: u64,
}
impl InstantWithdrawStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_asset_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let direct_withdraw_init_receipt: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_unlocked_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_dead_weight_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_dead_weight_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawn_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            amount,
            is_amount_in_lp,
            is_withdraw_all,
            user_amount_asset_withdrawn,
            user_amount_lp_burned,
            vault,
            strategy,
            strategy_init_receipt,
            direct_withdraw_init_receipt,
            adaptor_program,
            vault_asset_mint,
            vault_asset_total_value_unlocked_before,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_lp_total_accumulated_fees_before,
            vault_lp_total_accumulated_fees_after,
            vault_lp_dead_weight_before,
            vault_lp_dead_weight_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            strategy_position_value_before,
            strategy_position_value_after,
            withdrawn_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_withdraw_all, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_amount_asset_withdrawn,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_amount_lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.direct_withdraw_init_receipt,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_unlocked_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_dead_weight_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_lp_dead_weight_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawn_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawStrategyEventEvent(pub InstantWithdrawStrategyEvent);
impl InstantWithdrawStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSTANT_WITHDRAW_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [
    46, 57, 60, 20, 6, 160, 164, 247,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantWithdrawVaultEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
    pub user_amount_asset_withdrawn: u64,
    pub user_amount_lp_burned: u64,
    pub vault: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_total_value_unlocked_before: u64,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_lp_total_accumulated_fees_before: u64,
    pub vault_lp_total_accumulated_fees_after: u64,
    pub vault_lp_dead_weight_before: u64,
    pub vault_lp_dead_weight_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub withdrawn_ts: u64,
}
impl InstantWithdrawVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_asset_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_unlocked_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_dead_weight_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_dead_weight_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawn_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            amount,
            is_amount_in_lp,
            is_withdraw_all,
            user_amount_asset_withdrawn,
            user_amount_lp_burned,
            vault,
            vault_asset_mint,
            vault_asset_total_value_unlocked_before,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_lp_total_accumulated_fees_before,
            vault_lp_total_accumulated_fees_after,
            vault_lp_dead_weight_before,
            vault_lp_dead_weight_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            withdrawn_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_withdraw_all, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_amount_asset_withdrawn,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_amount_lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_unlocked_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_dead_weight_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_lp_dead_weight_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawn_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantWithdrawVaultEventEvent(pub InstantWithdrawVaultEvent);
impl InstantWithdrawVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_WITHDRAW_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = InstantWithdrawVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_WITHDRAW_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REMOVE_ADAPTOR_EVENT_EVENT_DISCM: [u8; 8] = [
    155, 178, 2, 29, 245, 86, 246, 153,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveAdaptorEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub adaptor_program: Pubkey,
    pub adaptor_add_receipt: Pubkey,
    pub removed_ts: u64,
}
impl RemoveAdaptorEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_add_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let removed_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            adaptor_program,
            adaptor_add_receipt,
            removed_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_add_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.removed_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAdaptorEventEvent(pub RemoveAdaptorEvent);
impl RemoveAdaptorEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ADAPTOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RemoveAdaptorEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ADAPTOR_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REQUEST_WITHDRAW_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [
    59, 94, 26, 38, 47, 131, 158, 162,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RequestWithdrawVaultEvent {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub requested_amount: u64,
    pub is_amount_in_lp: bool,
    pub is_withdraw_all: bool,
    pub request_withdraw_vault_receipt: Pubkey,
    pub amount_lp_escrowed: u64,
    pub amount_asset_to_withdraw_decimal_bits: u128,
    pub withdrawable_from_ts: u64,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_total_value_unlocked: u64,
    pub vault_asset_total_value: u64,
    pub vault_lp_supply_incl_fees: u64,
    pub requested_ts: u64,
}
impl RequestWithdrawVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let requested_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_amount_in_lp: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_all: bool = crate::borsh_de_or_default(&mut reader)?;
        let request_withdraw_vault_receipt: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amount_lp_escrowed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_asset_to_withdraw_decimal_bits: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawable_from_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_unlocked: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_supply_incl_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let requested_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            user,
            requested_amount,
            is_amount_in_lp,
            is_withdraw_all,
            request_withdraw_vault_receipt,
            amount_lp_escrowed,
            amount_asset_to_withdraw_decimal_bits,
            withdrawable_from_ts,
            vault_asset_mint,
            vault_asset_total_value_unlocked,
            vault_asset_total_value,
            vault_lp_supply_incl_fees,
            requested_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.requested_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_amount_in_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_withdraw_all, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.request_withdraw_vault_receipt,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.amount_lp_escrowed, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.amount_asset_to_withdraw_decimal_bits,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawable_from_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_unlocked,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_asset_total_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_lp_supply_incl_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.requested_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestWithdrawVaultEventEvent(pub RequestWithdrawVaultEvent);
impl RequestWithdrawVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_WITHDRAW_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RequestWithdrawVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_WITHDRAW_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_PROTOCOL_EVENT_EVENT_DISCM: [u8; 8] = [
    14, 227, 204, 217, 62, 46, 241, 237,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateProtocolEvent {
    pub admin: Pubkey,
    pub protocol: Pubkey,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
    pub updated_ts: u64,
}
impl UpdateProtocolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field: String = crate::borsh_de_or_default(&mut reader)?;
        let old_value: String = crate::borsh_de_or_default(&mut reader)?;
        let new_value: String = crate::borsh_de_or_default(&mut reader)?;
        let updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            protocol,
            field,
            old_value,
            new_value,
            updated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.updated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateProtocolEventEvent(pub UpdateProtocolEvent);
impl UpdateProtocolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PROTOCOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateProtocolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PROTOCOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_VAULT_ADAPTOR_POLICY_EVENT_EVENT_DISCM: [u8; 8] = [
    216, 190, 129, 182, 181, 123, 237, 189,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultAdaptorPolicyEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub field: String,
    pub old_value: u8,
    pub new_value: u8,
    pub updated_ts: u64,
}
impl UpdateVaultAdaptorPolicyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field: String = crate::borsh_de_or_default(&mut reader)?;
        let old_value: u8 = crate::borsh_de_or_default(&mut reader)?;
        let new_value: u8 = crate::borsh_de_or_default(&mut reader)?;
        let updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            field,
            old_value,
            new_value,
            updated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.updated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultAdaptorPolicyEventEvent(pub UpdateVaultAdaptorPolicyEvent);
impl UpdateVaultAdaptorPolicyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_ADAPTOR_POLICY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateVaultAdaptorPolicyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_ADAPTOR_POLICY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_VAULT_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [
    61, 92, 206, 151, 162, 40, 237, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultConfigEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
    pub updated_ts: u64,
}
impl UpdateVaultConfigEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field: String = crate::borsh_de_or_default(&mut reader)?;
        let old_value: String = crate::borsh_de_or_default(&mut reader)?;
        let new_value: String = crate::borsh_de_or_default(&mut reader)?;
        let updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            field,
            old_value,
            new_value,
            updated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.updated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultConfigEventEvent(pub UpdateVaultConfigEvent);
impl UpdateVaultConfigEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_CONFIG_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateVaultConfigEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_CONFIG_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [123, 31, 27, 189, 102, 1, 121, 57];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub vault_config_max_cap_before: u64,
    pub vault_config_max_cap_after: u64,
    pub vault_config_start_at_ts_before: u64,
    pub vault_config_start_at_ts_after: u64,
    pub vault_config_locked_profit_degradation_duration_before: u64,
    pub vault_config_locked_profit_degradation_duration_after: u64,
    pub vault_config_withdrawal_waiting_period_before: u64,
    pub vault_config_withdrawal_waiting_period_after: u64,
    pub vault_config_manager_performance_fee_before: u16,
    pub vault_config_manager_performance_fee_after: u16,
    pub vault_config_admin_performance_fee_before: u16,
    pub vault_config_admin_performance_fee_after: u16,
    pub vault_config_manager_management_fee_before: u16,
    pub vault_config_manager_management_fee_after: u16,
    pub vault_config_admin_management_fee_before: u16,
    pub vault_config_admin_management_fee_after: u16,
    pub vault_config_redemption_fee_before: u16,
    pub vault_config_redemption_fee_after: u16,
    pub vault_config_issuance_fee_before: u16,
    pub vault_config_issuance_fee_after: u16,
    pub updated_ts: u64,
}
impl UpdateVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_max_cap_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_max_cap_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_start_at_ts_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_start_at_ts_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_locked_profit_degradation_duration_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_locked_profit_degradation_duration_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_withdrawal_waiting_period_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_withdrawal_waiting_period_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_manager_performance_fee_before: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_manager_performance_fee_after: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_admin_performance_fee_before: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_admin_performance_fee_after: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_manager_management_fee_before: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_manager_management_fee_after: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_admin_management_fee_before: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_admin_management_fee_after: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_redemption_fee_before: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_redemption_fee_after: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_issuance_fee_before: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_config_issuance_fee_after: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            vault_config_max_cap_before,
            vault_config_max_cap_after,
            vault_config_start_at_ts_before,
            vault_config_start_at_ts_after,
            vault_config_locked_profit_degradation_duration_before,
            vault_config_locked_profit_degradation_duration_after,
            vault_config_withdrawal_waiting_period_before,
            vault_config_withdrawal_waiting_period_after,
            vault_config_manager_performance_fee_before,
            vault_config_manager_performance_fee_after,
            vault_config_admin_performance_fee_before,
            vault_config_admin_performance_fee_after,
            vault_config_manager_management_fee_before,
            vault_config_manager_management_fee_after,
            vault_config_admin_management_fee_before,
            vault_config_admin_management_fee_after,
            vault_config_redemption_fee_before,
            vault_config_redemption_fee_after,
            vault_config_issuance_fee_before,
            vault_config_issuance_fee_after,
            updated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_max_cap_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_config_max_cap_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_start_at_ts_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_start_at_ts_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_locked_profit_degradation_duration_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_locked_profit_degradation_duration_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_withdrawal_waiting_period_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_withdrawal_waiting_period_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_manager_performance_fee_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_manager_performance_fee_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_admin_performance_fee_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_admin_performance_fee_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_manager_management_fee_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_manager_management_fee_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_admin_management_fee_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_admin_management_fee_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_redemption_fee_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_redemption_fee_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_issuance_fee_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_config_issuance_fee_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.updated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultEventEvent(pub UpdateVaultEvent);
impl UpdateVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UPDATE_VAULT_PROTOCOL_FEE_EVENT_EVENT_DISCM: [u8; 8] = [
    172, 1, 173, 230, 228, 61, 77, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateVaultProtocolFeeEvent {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub field: String,
    pub old_value: u16,
    pub new_value: u16,
    pub updated_ts: u64,
}
impl UpdateVaultProtocolFeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field: String = crate::borsh_de_or_default(&mut reader)?;
        let old_value: u16 = crate::borsh_de_or_default(&mut reader)?;
        let new_value: u16 = crate::borsh_de_or_default(&mut reader)?;
        let updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            vault,
            field,
            old_value,
            new_value,
            updated_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.updated_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateVaultProtocolFeeEventEvent(pub UpdateVaultProtocolFeeEvent);
impl UpdateVaultProtocolFeeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_VAULT_PROTOCOL_FEE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UpdateVaultProtocolFeeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_VAULT_PROTOCOL_FEE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_STRATEGY_EVENT_EVENT_DISCM: [u8; 8] = [
    112, 45, 16, 172, 170, 33, 22, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawStrategyEvent {
    pub manager: Pubkey,
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub strategy_init_receipt: Pubkey,
    pub adaptor_program: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_amount_asset_withdrawn: u64,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub vault_asset_idle_ata_amount_before: u64,
    pub vault_asset_idle_ata_amount_after: u64,
    pub strategy_position_value_before: u64,
    pub strategy_position_value_after: u64,
    pub withdrawn_ts: u64,
}
impl WithdrawStrategyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_init_receipt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_amount_asset_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_idle_ata_amount_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_idle_ata_amount_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let strategy_position_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawn_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            manager,
            vault,
            strategy,
            strategy_init_receipt,
            adaptor_program,
            vault_asset_mint,
            vault_amount_asset_withdrawn,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            vault_asset_idle_ata_amount_before,
            vault_asset_idle_ata_amount_after,
            strategy_position_value_before,
            strategy_position_value_after,
            withdrawn_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_init_receipt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_amount_asset_withdrawn,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_idle_ata_amount_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_idle_ata_amount_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.strategy_position_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawn_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStrategyEventEvent(pub WithdrawStrategyEvent);
impl WithdrawStrategyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_STRATEGY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawStrategyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_STRATEGY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_VAULT_EVENT_EVENT_DISCM: [u8; 8] = [
    196, 123, 79, 215, 4, 214, 20, 197,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawVaultEvent {
    pub user: Pubkey,
    pub user_amount_asset_withdrawn: u64,
    pub user_amount_lp_burned: u64,
    pub vault: Pubkey,
    pub vault_asset_mint: Pubkey,
    pub vault_asset_total_value_unlocked_before: u64,
    pub vault_asset_total_value_before: u64,
    pub vault_asset_total_value_after: u64,
    pub vault_lp_supply_incl_fees_before: u64,
    pub vault_lp_supply_incl_fees_after: u64,
    pub vault_lp_total_accumulated_fees_before: u64,
    pub vault_lp_total_accumulated_fees_after: u64,
    pub vault_lp_dead_weight_before: u64,
    pub vault_lp_dead_weight_after: u64,
    pub vault_highest_asset_per_lp_decimal_bits_before: u128,
    pub vault_highest_asset_per_lp_decimal_bits_after: u128,
    pub withdrawn_ts: u64,
}
impl WithdrawVaultEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_asset_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_amount_lp_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_asset_total_value_unlocked_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_asset_total_value_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_supply_incl_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_before: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_total_accumulated_fees_after: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_lp_dead_weight_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_lp_dead_weight_after: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_highest_asset_per_lp_decimal_bits_before: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let vault_highest_asset_per_lp_decimal_bits_after: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawn_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            user_amount_asset_withdrawn,
            user_amount_lp_burned,
            vault,
            vault_asset_mint,
            vault_asset_total_value_unlocked_before,
            vault_asset_total_value_before,
            vault_asset_total_value_after,
            vault_lp_supply_incl_fees_before,
            vault_lp_supply_incl_fees_after,
            vault_lp_total_accumulated_fees_before,
            vault_lp_total_accumulated_fees_after,
            vault_lp_dead_weight_before,
            vault_lp_dead_weight_after,
            vault_highest_asset_per_lp_decimal_bits_before,
            vault_highest_asset_per_lp_decimal_bits_after,
            withdrawn_ts,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_amount_asset_withdrawn,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.user_amount_lp_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_unlocked_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_asset_total_value_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_supply_incl_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_total_accumulated_fees_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_lp_dead_weight_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.vault_lp_dead_weight_after, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_before,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.vault_highest_asset_per_lp_decimal_bits_after,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawn_ts, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawVaultEventEvent(pub WithdrawVaultEvent);
impl WithdrawVaultEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_VAULT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = WithdrawVaultEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_VAULT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
