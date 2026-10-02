use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BUFFER_STATE_ACCOUNT_DISCM: [u8; 8] = [90, 178, 221, 223, 231, 223, 64, 105];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BufferState {
    pub onyc_mint: Pubkey,
    pub gross_apr: u64,
    pub previous_supply: u64,
    pub management_fee_basis_points: u16,
    pub performance_fee_basis_points: u16,
    pub performance_fee_high_watermark: u64,
    pub performance_fee_high_watermark_enabled: bool,
    pub last_accrual_timestamp: i64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 135],
}
impl BufferState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let gross_apr: u64 = crate::borsh_de_or_default(&mut reader)?;
        let previous_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_high_watermark: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let performance_fee_high_watermark_enabled: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_accrual_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 135] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            onyc_mint,
            gross_apr,
            previous_supply,
            management_fee_basis_points,
            performance_fee_basis_points,
            performance_fee_high_watermark,
            performance_fee_high_watermark_enabled,
            last_accrual_timestamp,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.gross_apr, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.previous_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.management_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.performance_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.performance_fee_high_watermark,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.performance_fee_high_watermark_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_accrual_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BufferStateAccount(pub BufferState);
impl BufferStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUFFER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BufferState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUFFER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_ACCOUNT_DISCM: [u8; 8] = [
    253, 157, 119, 194, 173, 214, 166, 155,
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
pub struct CirculatingSupplyExcludedAccounts {
    pub owners: [Pubkey; 20],
    pub bump: u8,
    pub reserved: [u8; 31],
}
impl CirculatingSupplyExcludedAccounts {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owners: [Pubkey; 20] = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 31] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owners, bump, reserved })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owners, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CirculatingSupplyExcludedAccountsAccount(
    pub CirculatingSupplyExcludedAccounts,
);
impl CirculatingSupplyExcludedAccountsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CirculatingSupplyExcludedAccounts::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CIRCULATING_SUPPLY_EXCLUDED_ACCOUNTS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CIRCULATING_SUPPLY_EXCLUDED_BALANCE_ACCOUNT_DISCM: [u8; 8] = [
    148, 113, 150, 192, 170, 44, 10, 140,
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
pub struct CirculatingSupplyExcludedBalance {
    pub amount: u64,
    pub last_updated_at: i64,
    pub last_updated_slot: u64,
    pub bump: u8,
    pub reserved: [u8; 31],
}
impl CirculatingSupplyExcludedBalance {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 31] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            last_updated_at,
            last_updated_slot,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CirculatingSupplyExcludedBalanceAccount(pub CirculatingSupplyExcludedBalance);
impl CirculatingSupplyExcludedBalanceAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CIRCULATING_SUPPLY_EXCLUDED_BALANCE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CirculatingSupplyExcludedBalance::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CIRCULATING_SUPPLY_EXCLUDED_BALANCE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIGURABLE_VAULT_ACCOUNT_DISCM: [u8; 8] = [
    208, 230, 235, 106, 163, 86, 250, 199,
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
pub struct ConfigurableVault {
    pub kind: u8,
    pub withdrawal_destination: Pubkey,
    pub bump: u8,
    pub reserved: [u8; 31],
}
impl ConfigurableVault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let kind: u8 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_destination: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 31] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            kind,
            withdrawal_destination,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.kind, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_destination, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigurableVaultAccount(pub ConfigurableVault);
impl ConfigurableVaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIGURABLE_VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ConfigurableVault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIGURABLE_VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_STATS_ACCOUNT_DISCM: [u8; 8] = [240, 45, 182, 233, 92, 118, 209, 83];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MarketStats {
    pub apy: u64,
    pub circulating_supply: u64,
    pub nav: u64,
    pub nav_adjustment: i64,
    pub tvl: u64,
    pub last_updated_at: i64,
    pub last_updated_slot: u64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 95],
}
impl MarketStats {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let apy: u64 = crate::borsh_de_or_default(&mut reader)?;
        let circulating_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let nav: u64 = crate::borsh_de_or_default(&mut reader)?;
        let nav_adjustment: i64 = crate::borsh_de_or_default(&mut reader)?;
        let tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 95] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            apy,
            circulating_supply,
            nav,
            nav_adjustment,
            tvl,
            last_updated_at,
            last_updated_slot,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.apy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circulating_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nav_adjustment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tvl, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketStatsAccount(pub MarketStats);
impl MarketStatsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_STATS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarketStats::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_STATS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OFFER_ACCOUNT_DISCM: [u8; 8] = [215, 88, 60, 71, 170, 162, 73, 229];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Offer {
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub vectors: [OfferVector; 10],
    pub fee_basis_points: u16,
    pub bump: u8,
    pub needs_approval: u8,
    pub allow_permissionless: u8,
    pub disabled: u8,
    pub fee_basis_points_permissionless: u16,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl Offer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vectors: [OfferVector; 10] = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let needs_approval: u8 = crate::borsh_de_or_default(&mut reader)?;
        let allow_permissionless: u8 = crate::borsh_de_or_default(&mut reader)?;
        let disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points_permissionless: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            token_in_mint,
            token_out_mint,
            vectors,
            fee_basis_points,
            bump,
            needs_approval,
            allow_permissionless,
            disabled,
            fee_basis_points_permissionless,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vectors, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.needs_approval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_permissionless, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disabled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_basis_points_permissionless,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OfferAccount(pub Offer);
impl OfferAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OFFER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Offer::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OFFER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PERMISSIONLESS_AUTHORITY_ACCOUNT_DISCM: [u8; 8] = [
    241, 34, 5, 97, 43, 102, 149, 52,
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
pub struct PermissionlessAuthority {
    pub name: String,
}
impl PermissionlessAuthority {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionlessAuthorityAccount(pub PermissionlessAuthority);
impl PermissionlessAuthorityAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERMISSIONLESS_AUTHORITY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PermissionlessAuthority::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERMISSIONLESS_AUTHORITY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROP_AMM_PAIR_STATE_ACCOUNT_DISCM: [u8; 8] = [
    83, 138, 171, 182, 7, 98, 212, 149,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PropAmmPairState {
    pub offer: Pubkey,
    pub asset_mint: Pubkey,
    pub onyc_mint: Pubkey,
    pub enabled: bool,
    pub curve_peg_haircut_bps: u16,
    pub curve_exponent_scaled: u32,
    pub cadence_threshold: u32,
    pub cadence_wave_scaled: u32,
    pub epoch_duration_seconds: i64,
    pub wall_sensitivity_scaled: u32,
    pub minimum_sell_haircut_onyc: u64,
    pub curr_sell_value_stable: u64,
    pub curr_buy_value_stable: u64,
    pub prev_net_sell_value_stable: u64,
    pub curr_sell_trade_count: u32,
    pub epoch_start: i64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 284],
}
impl PropAmmPairState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let curve_peg_haircut_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let curve_exponent_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let cadence_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let cadence_wave_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_duration_seconds: i64 = crate::borsh_de_or_default(&mut reader)?;
        let wall_sensitivity_scaled: u32 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_sell_haircut_onyc: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curr_sell_value_stable: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curr_buy_value_stable: u64 = crate::borsh_de_or_default(&mut reader)?;
        let prev_net_sell_value_stable: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curr_sell_trade_count: u32 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_start: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 284] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            offer,
            asset_mint,
            onyc_mint,
            enabled,
            curve_peg_haircut_bps,
            curve_exponent_scaled,
            cadence_threshold,
            cadence_wave_scaled,
            epoch_duration_seconds,
            wall_sensitivity_scaled,
            minimum_sell_haircut_onyc,
            curr_sell_value_stable,
            curr_buy_value_stable,
            prev_net_sell_value_stable,
            curr_sell_trade_count,
            epoch_start,
            bump,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_peg_haircut_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_exponent_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cadence_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cadence_wave_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch_duration_seconds, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wall_sensitivity_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minimum_sell_haircut_onyc, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curr_sell_value_stable, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curr_buy_value_stable, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.prev_net_sell_value_stable, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curr_sell_trade_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.epoch_start, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PropAmmPairStateAccount(pub PropAmmPairState);
impl PropAmmPairStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROP_AMM_PAIR_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PropAmmPairState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROP_AMM_PAIR_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_OFFER_ACCOUNT_DISCM: [u8; 8] = [
    170, 229, 178, 15, 184, 107, 140, 41,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RedemptionOffer {
    pub offer: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub executed_redemptions: u128,
    pub requested_redemptions: u128,
    pub fee_basis_points: u16,
    pub gap: u64,
    pub bump: u8,
    pub vault_target_bps: u16,
    pub disabled: u8,
    pub fee_basis_points_prop_amm_sell: u16,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 104],
}
impl RedemptionOffer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let executed_redemptions: u128 = crate::borsh_de_or_default(&mut reader)?;
        let requested_redemptions: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let gap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_target_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points_prop_amm_sell: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reserved = <[u8; 104] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            offer,
            token_in_mint,
            token_out_mint,
            executed_redemptions,
            requested_redemptions,
            fee_basis_points,
            gap,
            bump,
            vault_target_bps,
            disabled,
            fee_basis_points_prop_amm_sell,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executed_redemptions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.requested_redemptions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.gap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_target_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disabled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_basis_points_prop_amm_sell,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionOfferAccount(pub RedemptionOffer);
impl RedemptionOfferAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_OFFER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RedemptionOffer::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_OFFER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REDEMPTION_REQUEST_ACCOUNT_DISCM: [u8; 8] = [
    117, 157, 214, 214, 64, 160, 31, 58,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct RedemptionRequest {
    pub offer: Pubkey,
    pub request_id: String,
    pub redeemer: Pubkey,
    pub amount: u64,
    pub bump: u8,
    pub fulfilled_amount: u64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 91],
}
impl RedemptionRequest {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let request_id: String = crate::borsh_de_or_default(&mut reader)?;
        let redeemer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fulfilled_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 91] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            offer,
            request_id,
            redeemer,
            amount,
            bump,
            fulfilled_amount,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeemer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fulfilled_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedemptionRequestAccount(pub RedemptionRequest);
impl RedemptionRequestAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEMPTION_REQUEST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RedemptionRequest::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEMPTION_REQUEST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct State {
    pub boss: Pubkey,
    pub proposed_boss: Pubkey,
    pub is_killed: bool,
    pub onyc_mint: Pubkey,
    pub admins: [Pubkey; 20],
    pub approver1: Pubkey,
    pub approver2: Pubkey,
    pub bump: u8,
    pub max_supply: u64,
    pub worker: Pubkey,
    pub max_mint_amount: u64,
    pub main_offer: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 56],
}
impl State {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proposed_boss: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_killed: bool = crate::borsh_de_or_default(&mut reader)?;
        let onyc_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admins: [Pubkey; 20] = crate::borsh_de_or_default(&mut reader)?;
        let approver1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let approver2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let worker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_mint_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let main_offer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 56] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            boss,
            proposed_boss,
            is_killed,
            onyc_mint,
            admins,
            approver1,
            approver2,
            bump,
            max_supply,
            worker,
            max_mint_amount,
            main_offer,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proposed_boss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_killed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onyc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admins, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.approver1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.approver2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.worker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_mint_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.main_offer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
