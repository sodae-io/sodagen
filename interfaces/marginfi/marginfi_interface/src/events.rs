use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const DELEVERAGE_EVENT_EVENT_DISCM: [u8; 8] = [161, 8, 108, 204, 209, 198, 12, 30];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleverageEvent {
    pub marginfi_account: Pubkey,
    pub risk_admin: Pubkey,
    pub deleveragee_assets_seized: f64,
    pub deleveragee_liability_repaid: f64,
}
impl DeleverageEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let marginfi_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let risk_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deleveragee_assets_seized: f64 = crate::borsh_de_or_default(&mut reader)?;
        let deleveragee_liability_repaid: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            marginfi_account,
            risk_admin,
            deleveragee_assets_seized,
            deleveragee_liability_repaid,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.marginfi_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.risk_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deleveragee_assets_seized, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.deleveragee_liability_repaid,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleverageEventEvent(pub DeleverageEvent);
impl DeleverageEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELEVERAGE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeleverageEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELEVERAGE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DELEVERAGE_WITHDRAW_FLOW_EVENT_EVENT_DISCM: [u8; 8] = [
    109, 90, 139, 200, 10, 204, 84, 176,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleverageWithdrawFlowEvent {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub outflow_usd: u32,
    pub current_timestamp: i64,
}
impl DeleverageWithdrawFlowEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let outflow_usd: u32 = crate::borsh_de_or_default(&mut reader)?;
        let current_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            group,
            bank,
            mint,
            outflow_usd,
            current_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.group, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.outflow_usd, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleverageWithdrawFlowEventEvent(pub DeleverageWithdrawFlowEvent);
impl DeleverageWithdrawFlowEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELEVERAGE_WITHDRAW_FLOW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DeleverageWithdrawFlowEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELEVERAGE_WITHDRAW_FLOW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EDIT_STAKED_SETTINGS_EVENT_EVENT_DISCM: [u8; 8] = [
    29, 58, 155, 191, 75, 220, 145, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EditStakedSettingsEvent {
    pub group: Pubkey,
    pub settings: StakedSettingsEditConfig,
}
impl EditStakedSettingsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let settings = if reader.is_empty() {
            Default::default()
        } else {
            <StakedSettingsEditConfig>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { group, settings })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.group, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.settings, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EditStakedSettingsEventEvent(pub EditStakedSettingsEvent);
impl EditStakedSettingsEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EDIT_STAKED_SETTINGS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EditStakedSettingsEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EDIT_STAKED_SETTINGS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HEALTH_PULSE_EVENT_EVENT_DISCM: [u8; 8] = [183, 159, 218, 110, 61, 220, 65, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HealthPulseEvent {
    pub account: Pubkey,
    pub health_cache: HealthCache,
}
impl HealthPulseEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let health_cache = if reader.is_empty() {
            Default::default()
        } else {
            <HealthCache>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { account, health_cache })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.health_cache, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HealthPulseEventEvent(pub HealthPulseEvent);
impl HealthPulseEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HEALTH_PULSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = HealthPulseEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HEALTH_PULSE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const KEEPER_CLOSE_ORDER_EVENT_EVENT_DISCM: [u8; 8] = [
    46, 152, 11, 174, 92, 157, 77, 64,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeeperCloseOrderEvent {
    pub header: AccountEventHeader,
    pub order: Pubkey,
}
impl KeeperCloseOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, order })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct KeeperCloseOrderEventEvent(pub KeeperCloseOrderEvent);
impl KeeperCloseOrderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KEEPER_CLOSE_ORDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = KeeperCloseOrderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KEEPER_CLOSE_ORDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_ACCOUNT_BORROW_EVENT_EVENT_DISCM: [u8; 8] = [
    223, 96, 81, 10, 156, 99, 26, 59,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountBorrowEvent {
    pub header: AccountEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
impl LendingAccountBorrowEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, bank, mint, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountBorrowEventEvent(pub LendingAccountBorrowEvent);
impl LendingAccountBorrowEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_BORROW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingAccountBorrowEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_BORROW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_ACCOUNT_DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [
    161, 54, 237, 217, 105, 248, 122, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountDepositEvent {
    pub header: AccountEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
impl LendingAccountDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, bank, mint, amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountDepositEventEvent(pub LendingAccountDepositEvent);
impl LendingAccountDepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingAccountDepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_ACCOUNT_LIQUIDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    166, 160, 249, 154, 183, 39, 23, 242,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountLiquidateEvent {
    pub header: AccountEventHeader,
    pub liquidatee_marginfi_account: Pubkey,
    pub liquidatee_marginfi_account_authority: Pubkey,
    pub asset_bank: Pubkey,
    pub asset_mint: Pubkey,
    pub liability_bank: Pubkey,
    pub liability_mint: Pubkey,
    pub liquidatee_pre_health: f64,
    pub liquidatee_post_health: f64,
    pub pre_balances: LiquidationBalances,
    pub post_balances: LiquidationBalances,
}
impl LendingAccountLiquidateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let liquidatee_marginfi_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidatee_marginfi_account_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let asset_bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liability_bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liability_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidatee_pre_health: f64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidatee_post_health: f64 = crate::borsh_de_or_default(&mut reader)?;
        let pre_balances = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidationBalances>::deserialize(&mut reader)?
        };
        let post_balances = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidationBalances>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            header,
            liquidatee_marginfi_account,
            liquidatee_marginfi_account_authority,
            asset_bank,
            asset_mint,
            liability_bank,
            liability_mint,
            liquidatee_pre_health,
            liquidatee_post_health,
            pre_balances,
            post_balances,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.liquidatee_marginfi_account,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.liquidatee_marginfi_account_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.asset_bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liability_bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liability_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidatee_pre_health, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidatee_post_health, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_balances, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_balances, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountLiquidateEventEvent(pub LendingAccountLiquidateEvent);
impl LendingAccountLiquidateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_LIQUIDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingAccountLiquidateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_LIQUIDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_ACCOUNT_REPAY_EVENT_EVENT_DISCM: [u8; 8] = [
    16, 220, 55, 111, 7, 80, 16, 25,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountRepayEvent {
    pub header: AccountEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub close_balance: bool,
}
impl LendingAccountRepayEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let close_balance: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            amount,
            close_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.close_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountRepayEventEvent(pub LendingAccountRepayEvent);
impl LendingAccountRepayEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_REPAY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingAccountRepayEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_REPAY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_ACCOUNT_WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [
    3, 220, 148, 243, 33, 249, 54, 88,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingAccountWithdrawEvent {
    pub header: AccountEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub close_balance: bool,
}
impl LendingAccountWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let close_balance: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            amount,
            close_balance,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.close_balance, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingAccountWithdrawEventEvent(pub LendingAccountWithdrawEvent);
impl LendingAccountWithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_ACCOUNT_WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingAccountWithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_ACCOUNT_WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_ACCRUE_INTEREST_EVENT_EVENT_DISCM: [u8; 8] = [
    104, 117, 187, 156, 111, 154, 106, 186,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankAccrueInterestEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub delta: u64,
    pub fees_collected: f64,
    pub insurance_collected: f64,
}
impl LendingPoolBankAccrueInterestEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_collected: f64 = crate::borsh_de_or_default(&mut reader)?;
        let insurance_collected: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            delta,
            fees_collected,
            insurance_collected,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.insurance_collected, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankAccrueInterestEventEvent(
    pub LendingPoolBankAccrueInterestEvent,
);
impl LendingPoolBankAccrueInterestEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_ACCRUE_INTEREST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankAccrueInterestEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_ACCRUE_INTEREST_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_COLLECT_FEES_EVENT_EVENT_DISCM: [u8; 8] = [
    101, 119, 97, 250, 169, 175, 156, 253,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankCollectFeesEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub group_fees_collected: f64,
    pub group_fees_outstanding: f64,
    pub insurance_fees_collected: f64,
    pub insurance_fees_outstanding: f64,
}
impl LendingPoolBankCollectFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let group_fees_collected: f64 = crate::borsh_de_or_default(&mut reader)?;
        let group_fees_outstanding: f64 = crate::borsh_de_or_default(&mut reader)?;
        let insurance_fees_collected: f64 = crate::borsh_de_or_default(&mut reader)?;
        let insurance_fees_outstanding: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            group_fees_collected,
            group_fees_outstanding,
            insurance_fees_collected,
            insurance_fees_outstanding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.group_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.group_fees_outstanding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.insurance_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.insurance_fees_outstanding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankCollectFeesEventEvent(pub LendingPoolBankCollectFeesEvent);
impl LendingPoolBankCollectFeesEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_COLLECT_FEES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankCollectFeesEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_COLLECT_FEES_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_CONFIGURE_EVENT_EVENT_DISCM: [u8; 8] = [
    246, 35, 233, 110, 93, 152, 235, 40,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankConfigureEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub config: BankConfigOpt,
}
impl LendingPoolBankConfigureEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <BankConfigOpt>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { header, bank, mint, config })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankConfigureEventEvent(pub LendingPoolBankConfigureEvent);
impl LendingPoolBankConfigureEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_CONFIGURE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankConfigureEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_CONFIGURE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_CONFIGURE_FROZEN_EVENT_EVENT_DISCM: [u8; 8] = [
    24, 10, 55, 18, 49, 150, 157, 179,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankConfigureFrozenEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub deposit_limit: u64,
    pub borrow_limit: u64,
}
impl LendingPoolBankConfigureFrozenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            deposit_limit,
            borrow_limit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankConfigureFrozenEventEvent(
    pub LendingPoolBankConfigureFrozenEvent,
);
impl LendingPoolBankConfigureFrozenEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_CONFIGURE_FROZEN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankConfigureFrozenEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_CONFIGURE_FROZEN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_CONFIGURE_ORACLE_EVENT_EVENT_DISCM: [u8; 8] = [
    119, 140, 110, 253, 150, 64, 210, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankConfigureOracleEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub oracle_setup: u8,
    pub oracle: Pubkey,
}
impl LendingPoolBankConfigureOracleEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_setup: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            oracle_setup,
            oracle,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_setup, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankConfigureOracleEventEvent(
    pub LendingPoolBankConfigureOracleEvent,
);
impl LendingPoolBankConfigureOracleEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_CONFIGURE_ORACLE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankConfigureOracleEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_CONFIGURE_ORACLE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    236, 220, 201, 63, 239, 126, 136, 249,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankCreateEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
}
impl LendingPoolBankCreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, bank, mint })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankCreateEventEvent(pub LendingPoolBankCreateEvent);
impl LendingPoolBankCreateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_CREATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankCreateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_CREATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_HANDLE_BANKRUPTCY_EVENT_EVENT_DISCM: [u8; 8] = [
    166, 77, 41, 140, 36, 94, 10, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankHandleBankruptcyEvent {
    pub header: AccountEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub bad_debt: f64,
    pub covered_amount: f64,
    pub socialized_amount: f64,
}
impl LendingPoolBankHandleBankruptcyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bad_debt: f64 = crate::borsh_de_or_default(&mut reader)?;
        let covered_amount: f64 = crate::borsh_de_or_default(&mut reader)?;
        let socialized_amount: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            bad_debt,
            covered_amount,
            socialized_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bad_debt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.covered_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.socialized_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankHandleBankruptcyEventEvent(
    pub LendingPoolBankHandleBankruptcyEvent,
);
impl LendingPoolBankHandleBankruptcyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_HANDLE_BANKRUPTCY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankHandleBankruptcyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_HANDLE_BANKRUPTCY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_BANK_SET_FIXED_ORACLE_PRICE_EVENT_EVENT_DISCM: [u8; 8] = [
    65, 72, 8, 85, 229, 20, 90, 26,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolBankSetFixedOraclePriceEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub price: WrappedI80F48,
}
impl LendingPoolBankSetFixedOraclePriceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { header, bank, price })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolBankSetFixedOraclePriceEventEvent(
    pub LendingPoolBankSetFixedOraclePriceEvent,
);
impl LendingPoolBankSetFixedOraclePriceEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_BANK_SET_FIXED_ORACLE_PRICE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolBankSetFixedOraclePriceEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_BANK_SET_FIXED_ORACLE_PRICE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_SUPER_ADMIN_DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [
    99, 152, 211, 30, 58, 165, 210, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolSuperAdminDepositEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub transfer_amount: u64,
    pub vault_inflow_amount: u64,
}
impl LendingPoolSuperAdminDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_inflow_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            transfer_amount,
            vault_inflow_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.transfer_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_inflow_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolSuperAdminDepositEventEvent(pub LendingPoolSuperAdminDepositEvent);
impl LendingPoolSuperAdminDepositEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_SUPER_ADMIN_DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolSuperAdminDepositEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_SUPER_ADMIN_DEPOSIT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_SUPER_ADMIN_WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [
    107, 168, 232, 181, 144, 161, 252, 37,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LendingPoolSuperAdminWithdrawEvent {
    pub header: GroupEventHeader,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub vault_outflow_amount: u64,
}
impl LendingPoolSuperAdminWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_outflow_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            bank,
            mint,
            vault_outflow_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_outflow_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolSuperAdminWithdrawEventEvent(
    pub LendingPoolSuperAdminWithdrawEvent,
);
impl LendingPoolSuperAdminWithdrawEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_SUPER_ADMIN_WITHDRAW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LendingPoolSuperAdminWithdrawEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_SUPER_ADMIN_WITHDRAW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDATION_RECEIVER_EVENT_EVENT_DISCM: [u8; 8] = [
    40, 131, 224, 220, 151, 83, 24, 230,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidationReceiverEvent {
    pub marginfi_account: Pubkey,
    pub liquidation_receiver: Pubkey,
    pub liquidatee_assets_seized: f64,
    pub liquidatee_liability_repaid: f64,
    pub lamps_fee_paid: u32,
}
impl LiquidationReceiverEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let marginfi_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidatee_assets_seized: f64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidatee_liability_repaid: f64 = crate::borsh_de_or_default(&mut reader)?;
        let lamps_fee_paid: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            marginfi_account,
            liquidation_receiver,
            liquidatee_assets_seized,
            liquidatee_liability_repaid,
            lamps_fee_paid,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.marginfi_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidatee_assets_seized, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.liquidatee_liability_repaid,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.lamps_fee_paid, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidationReceiverEventEvent(pub LiquidationReceiverEvent);
impl LiquidationReceiverEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATION_RECEIVER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidationReceiverEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATION_RECEIVER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_ACCOUNT_CLOSE_ORDER_EVENT_EVENT_DISCM: [u8; 8] = [
    158, 34, 122, 98, 23, 146, 229, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountCloseOrderEvent {
    pub header: AccountEventHeader,
    pub order: Pubkey,
}
impl MarginfiAccountCloseOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, order })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountCloseOrderEventEvent(pub MarginfiAccountCloseOrderEvent);
impl MarginfiAccountCloseOrderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_CLOSE_ORDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiAccountCloseOrderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_CLOSE_ORDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_ACCOUNT_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    183, 5, 117, 104, 122, 199, 68, 51,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountCreateEvent {
    pub header: AccountEventHeader,
}
impl MarginfiAccountCreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { header })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountCreateEventEvent(pub MarginfiAccountCreateEvent);
impl MarginfiAccountCreateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_CREATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiAccountCreateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_CREATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_ACCOUNT_FREEZE_EVENT_EVENT_DISCM: [u8; 8] = [
    219, 219, 57, 178, 75, 86, 146, 122,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountFreezeEvent {
    pub header: AccountEventHeader,
    pub frozen: bool,
}
impl MarginfiAccountFreezeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let frozen: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, frozen })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.frozen, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountFreezeEventEvent(pub MarginfiAccountFreezeEvent);
impl MarginfiAccountFreezeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_FREEZE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiAccountFreezeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_FREEZE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_ACCOUNT_PLACE_ORDER_EVENT_EVENT_DISCM: [u8; 8] = [
    1, 105, 79, 28, 142, 242, 99, 145,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountPlaceOrderEvent {
    pub header: AccountEventHeader,
    pub order: Pubkey,
    pub trigger: OrderTriggerType,
    pub stop_loss: WrappedI80F48,
    pub take_profit: WrappedI80F48,
    pub tags: [u16; 2],
}
impl MarginfiAccountPlaceOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trigger: OrderTriggerType = crate::borsh_de_or_default(&mut reader)?;
        let stop_loss = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let take_profit = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let tags: [u16; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            order,
            trigger,
            stop_loss,
            take_profit,
            tags,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trigger, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stop_loss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.take_profit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tags, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountPlaceOrderEventEvent(pub MarginfiAccountPlaceOrderEvent);
impl MarginfiAccountPlaceOrderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_PLACE_ORDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiAccountPlaceOrderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_PLACE_ORDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_ACCOUNT_TRANSFER_TO_NEW_ACCOUNT_EVENT_DISCM: [u8; 8] = [
    59, 105, 171, 110, 223, 136, 80, 89,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiAccountTransferToNewAccount {
    pub header: AccountEventHeader,
    pub old_account: Pubkey,
    pub old_account_authority: Pubkey,
    pub new_account_authority: Pubkey,
}
impl MarginfiAccountTransferToNewAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let old_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let old_account_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_account_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            header,
            old_account,
            old_account_authority,
            new_account_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.old_account_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_account_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountTransferToNewAccountEvent(
    pub MarginfiAccountTransferToNewAccount,
);
impl MarginfiAccountTransferToNewAccountEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_TRANSFER_TO_NEW_ACCOUNT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiAccountTransferToNewAccount::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_TRANSFER_TO_NEW_ACCOUNT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_GROUP_CONFIGURE_EVENT_EVENT_DISCM: [u8; 8] = [
    241, 104, 172, 167, 41, 195, 199, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiGroupConfigureEvent {
    pub header: GroupEventHeader,
    pub admin: Option<Pubkey>,
    pub flags: u64,
}
impl MarginfiGroupConfigureEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        let admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, admin, flags })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flags, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiGroupConfigureEventEvent(pub MarginfiGroupConfigureEvent);
impl MarginfiGroupConfigureEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_GROUP_CONFIGURE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiGroupConfigureEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_GROUP_CONFIGURE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_GROUP_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    233, 125, 61, 14, 98, 240, 136, 253,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiGroupCreateEvent {
    pub header: GroupEventHeader,
}
impl MarginfiGroupCreateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <GroupEventHeader>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { header })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiGroupCreateEventEvent(pub MarginfiGroupCreateEvent);
impl MarginfiGroupCreateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_GROUP_CREATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MarginfiGroupCreateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_GROUP_CREATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RATE_LIMIT_FLOW_EVENT_EVENT_DISCM: [u8; 8] = [
    229, 5, 73, 200, 0, 107, 105, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RateLimitFlowEvent {
    pub group: Pubkey,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub flow_direction: u8,
    pub native_amount: u64,
    pub mint_decimals: u8,
    pub current_timestamp: i64,
}
impl RateLimitFlowEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let flow_direction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let native_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let current_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            group,
            bank,
            mint,
            flow_direction,
            native_amount,
            mint_decimals,
            current_timestamp,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.group, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flow_direction, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_timestamp, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RateLimitFlowEventEvent(pub RateLimitFlowEvent);
impl RateLimitFlowEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RATE_LIMIT_FLOW_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RateLimitFlowEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RATE_LIMIT_FLOW_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_KEEPER_CLOSE_FLAGS_EVENT_EVENT_DISCM: [u8; 8] = [
    193, 230, 93, 128, 117, 87, 96, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetKeeperCloseFlagsEvent {
    pub header: AccountEventHeader,
    pub bank_keys: Option<Vec<Pubkey>>,
}
impl SetKeeperCloseFlagsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <AccountEventHeader>::deserialize(&mut reader)?
        };
        let bank_keys: Option<Vec<Pubkey>> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { header, bank_keys })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank_keys, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetKeeperCloseFlagsEventEvent(pub SetKeeperCloseFlagsEvent);
impl SetKeeperCloseFlagsEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_KEEPER_CLOSE_FLAGS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetKeeperCloseFlagsEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_KEEPER_CLOSE_FLAGS_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
