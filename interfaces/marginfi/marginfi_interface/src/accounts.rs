use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BANK_ACCOUNT_DISCM: [u8; 8] = [142, 49, 166, 242, 50, 66, 97, 188];
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
pub struct Bank {
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub group: Pubkey,
    pub _pad0: [u8; 7],
    pub asset_share_value: WrappedI80F48,
    pub liability_share_value: WrappedI80F48,
    pub liquidity_vault: Pubkey,
    pub liquidity_vault_bump: u8,
    pub liquidity_vault_authority_bump: u8,
    pub insurance_vault: Pubkey,
    pub insurance_vault_bump: u8,
    pub insurance_vault_authority_bump: u8,
    pub _pad1: [u8; 4],
    pub collected_insurance_fees_outstanding: WrappedI80F48,
    pub fee_vault: Pubkey,
    pub fee_vault_bump: u8,
    pub fee_vault_authority_bump: u8,
    pub _pad2: [u8; 6],
    pub collected_group_fees_outstanding: WrappedI80F48,
    pub total_liability_shares: WrappedI80F48,
    pub total_asset_shares: WrappedI80F48,
    pub last_update: i64,
    pub config: BankConfig,
    pub flags: u64,
    pub emissions_rate: u64,
    pub emissions_remaining: WrappedI80F48,
    pub emissions_mint: Pubkey,
    pub collected_program_fees_outstanding: WrappedI80F48,
    pub emode: EmodeSettings,
    pub fees_destination_account: Pubkey,
    pub cache: BankCache,
    pub lending_position_count: i32,
    pub borrowing_position_count: i32,
    pub _padding_0: [u8; 16],
    pub integration_acc_1: Pubkey,
    pub integration_acc_2: Pubkey,
    pub integration_acc_3: Pubkey,
    pub rate_limiter: BankRateLimiter,
    pub _pad_0: [u8; 16],
    pub _padding_1: [[u64; 2]; 7],
}
impl Bank {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let asset_share_value = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_share_value = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liquidity_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_vault_authority_bump: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let insurance_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let insurance_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let insurance_vault_authority_bump: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let _pad1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let collected_insurance_fees_outstanding = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault_authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad2: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let collected_group_fees_outstanding = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let total_liability_shares = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let total_asset_shares = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let last_update: i64 = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <BankConfig>::deserialize(&mut reader)?
        };
        let flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_remaining = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let emissions_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collected_program_fees_outstanding = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let emode = if reader.is_empty() {
            Default::default()
        } else {
            <EmodeSettings>::deserialize(&mut reader)?
        };
        let fees_destination_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cache = if reader.is_empty() {
            Default::default()
        } else {
            <BankCache>::deserialize(&mut reader)?
        };
        let lending_position_count: i32 = crate::borsh_de_or_default(&mut reader)?;
        let borrowing_position_count: i32 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let integration_acc_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let integration_acc_2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let integration_acc_3: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rate_limiter = if reader.is_empty() {
            Default::default()
        } else {
            <BankRateLimiter>::deserialize(&mut reader)?
        };
        let _pad_0: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [[u64; 2]; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            mint_decimals,
            group,
            _pad0,
            asset_share_value,
            liability_share_value,
            liquidity_vault,
            liquidity_vault_bump,
            liquidity_vault_authority_bump,
            insurance_vault,
            insurance_vault_bump,
            insurance_vault_authority_bump,
            _pad1,
            collected_insurance_fees_outstanding,
            fee_vault,
            fee_vault_bump,
            fee_vault_authority_bump,
            _pad2,
            collected_group_fees_outstanding,
            total_liability_shares,
            total_asset_shares,
            last_update,
            config,
            flags,
            emissions_rate,
            emissions_remaining,
            emissions_mint,
            collected_program_fees_outstanding,
            emode,
            fees_destination_account,
            cache,
            lending_position_count,
            borrowing_position_count,
            _padding_0,
            integration_acc_1,
            integration_acc_2,
            integration_acc_3,
            rate_limiter,
            _pad_0,
            _padding_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.group, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_share_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liability_share_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_vault_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.liquidity_vault_authority_bump,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.insurance_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.insurance_vault_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.insurance_vault_authority_bump,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self._pad1, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collected_insurance_fees_outstanding,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._pad2, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collected_group_fees_outstanding,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_liability_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_asset_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emissions_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emissions_remaining, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emissions_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collected_program_fees_outstanding,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.emode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_destination_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_position_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrowing_position_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.integration_acc_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.integration_acc_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.integration_acc_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_limiter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._pad_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BankAccount(pub Bank);
impl BankAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BANK_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Bank::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BANK_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BANK_METADATA_ACCOUNT_DISCM: [u8; 8] = [49, 207, 31, 34, 67, 225, 169, 186];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BankMetadata {
    pub bank: Pubkey,
    pub placeholder: u64,
    #[serde(with = "crate::big_array_serde")]
    pub ticker: [u8; 64],
    #[serde(with = "crate::big_array_serde")]
    pub description: [u8; 128],
    #[serde(with = "crate::big_array_serde")]
    pub data_blob: [u8; 256],
    pub end_description_byte: u16,
    pub end_data_blob: u16,
    pub end_ticker_byte: u8,
    pub bump: u8,
    pub _pad0: [u8; 2],
}
impl BankMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let placeholder: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ticker = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let description = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let data_blob = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let end_description_byte: u16 = crate::borsh_de_or_default(&mut reader)?;
        let end_data_blob: u16 = crate::borsh_de_or_default(&mut reader)?;
        let end_ticker_byte: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bank,
            placeholder,
            ticker,
            description,
            data_blob,
            end_description_byte,
            end_data_blob,
            end_ticker_byte,
            bump,
            _pad0,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placeholder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.description, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.data_blob, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_description_byte, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_data_blob, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_ticker_byte, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._pad0, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BankMetadataAccount(pub BankMetadata);
impl BankMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BANK_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BankMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BANK_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EXECUTE_ORDER_RECORD_ACCOUNT_DISCM: [u8; 8] = [
    6, 100, 107, 60, 164, 226, 56, 97,
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
pub struct ExecuteOrderRecord {
    pub order: Pubkey,
    pub executor: Pubkey,
    pub balance_states: [ExecuteOrderBalanceRecord; 14],
    pub active_balance_count: u8,
    pub inactive_balance_count: u8,
    pub _reserved0: [u8; 6],
    pub order_start_health: WrappedI80F48,
}
impl ExecuteOrderRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let executor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let balance_states: [ExecuteOrderBalanceRecord; 14] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let active_balance_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let inactive_balance_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let order_start_health = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            order,
            executor,
            balance_states,
            active_balance_count,
            inactive_balance_count,
            _reserved0,
            order_start_health,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.order, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.executor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.balance_states, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_balance_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inactive_balance_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_start_health, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteOrderRecordAccount(pub ExecuteOrderRecord);
impl ExecuteOrderRecordAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_ORDER_RECORD_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ExecuteOrderRecord::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_ORDER_RECORD_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_STATE_ACCOUNT_DISCM: [u8; 8] = [63, 224, 16, 85, 193, 36, 235, 220];
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
pub struct FeeState {
    pub key: Pubkey,
    pub global_fee_admin: Pubkey,
    pub global_fee_wallet: Pubkey,
    pub placeholder0: u64,
    pub bank_init_flat_sol_fee: u32,
    pub bump_seed: u8,
    pub _padding0: [u8; 3],
    pub liquidation_max_fee: WrappedI80F48,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
    pub panic_state: PanicState,
    pub placeholder1: u64,
    pub liquidation_flat_sol_fee: u32,
    pub order_init_flat_sol_fee: u32,
    pub order_execution_max_fee: WrappedI80F48,
    pub _reserved1: [u8; 32],
}
impl FeeState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_fee_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let placeholder0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bank_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bump_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding0: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let program_fee_fixed = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let program_fee_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let panic_state = if reader.is_empty() {
            Default::default()
        } else {
            <PanicState>::deserialize(&mut reader)?
        };
        let placeholder1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let order_init_flat_sol_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let order_execution_max_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let _reserved1: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            key,
            global_fee_admin,
            global_fee_wallet,
            placeholder0,
            bank_init_flat_sol_fee,
            bump_seed,
            _padding0,
            liquidation_max_fee,
            program_fee_fixed,
            program_fee_rate,
            panic_state,
            placeholder1,
            liquidation_flat_sol_fee,
            order_init_flat_sol_fee,
            order_execution_max_fee,
            _reserved1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_fee_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_fee_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placeholder0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank_init_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_max_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.program_fee_fixed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.program_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.panic_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placeholder1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_init_flat_sol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_execution_max_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeStateAccount(pub FeeState);
impl FeeStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_POOL_ACCOUNT_DISCM: [u8; 8] = [135, 199, 82, 16, 249, 131, 182, 241];
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
pub struct LendingPool {
    pub mint: Pubkey,
    pub f_token_mint: Pubkey,
    pub lending_id: u16,
    pub decimals: u8,
    pub rewards_rate_model: Pubkey,
    pub liquidity_exchange_price: u64,
    pub token_exchange_price: u64,
    pub last_update_timestamp: u64,
    pub token_reserves_liquidity: Pubkey,
    pub supply_position_on_liquidity: Pubkey,
    pub bump: u8,
}
impl LendingPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let f_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lending_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_rate_model: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_exchange_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_reserves_liquidity: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supply_position_on_liquidity: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            f_token_mint,
            lending_id,
            decimals,
            rewards_rate_model,
            liquidity_exchange_price,
            token_exchange_price,
            last_update_timestamp,
            token_reserves_liquidity,
            supply_position_on_liquidity,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.f_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_rate_model, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_exchange_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_reserves_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.supply_position_on_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingPoolAccount(pub LendingPool);
impl LendingPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LendingPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDATION_RECORD_ACCOUNT_DISCM: [u8; 8] = [
    95, 116, 23, 132, 89, 210, 245, 162,
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
pub struct LiquidationRecord {
    pub key: Pubkey,
    pub marginfi_account: Pubkey,
    pub record_payer: Pubkey,
    pub liquidation_receiver: Pubkey,
    pub entries: [LiquidationEntry; 4],
    pub cache: LiquidationCache,
    #[serde(with = "crate::big_array_serde")]
    pub _reserved0: [u8; 64],
    pub _reserved2: [u8; 16],
    pub _reserved3: [u8; 8],
}
impl LiquidationRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let marginfi_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let record_payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let entries: [LiquidationEntry; 4] = crate::borsh_de_or_default(&mut reader)?;
        let cache = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidationCache>::deserialize(&mut reader)?
        };
        let _reserved0 = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let _reserved2: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let _reserved3: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            key,
            marginfi_account,
            record_payer,
            liquidation_receiver,
            entries,
            cache,
            _reserved0,
            _reserved2,
            _reserved3,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.marginfi_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.record_payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.entries, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved3, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidationRecordAccount(pub LiquidationRecord);
impl LiquidationRecordAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATION_RECORD_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LiquidationRecord::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATION_RECORD_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [
    67, 178, 130, 109, 126, 114, 28, 42,
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
pub struct MarginfiAccount {
    pub group: Pubkey,
    pub authority: Pubkey,
    pub lending_account: LendingAccount,
    pub account_flags: u64,
    pub emissions_destination_account: Pubkey,
    pub health_cache: HealthCache,
    pub migrated_from: Pubkey,
    pub migrated_to: Pubkey,
    pub last_update: u64,
    pub account_index: u16,
    pub third_party_index: u16,
    pub bump: u8,
    pub _pad0: [u8; 3],
    pub liquidation_record: Pubkey,
    pub _padding0: [u64; 7],
}
impl MarginfiAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lending_account = if reader.is_empty() {
            Default::default()
        } else {
            <LendingAccount>::deserialize(&mut reader)?
        };
        let account_flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_destination_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let health_cache = if reader.is_empty() {
            Default::default()
        } else {
            <HealthCache>::deserialize(&mut reader)?
        };
        let migrated_from: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migrated_to: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let third_party_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_record: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding0: [u64; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            group,
            authority,
            lending_account,
            account_flags,
            emissions_destination_account,
            health_cache,
            migrated_from,
            migrated_to,
            last_update,
            account_index,
            third_party_index,
            bump,
            _pad0,
            liquidation_record,
            _padding0,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.group, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.account_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.emissions_destination_account,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.health_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrated_from, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrated_to, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.account_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.third_party_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidation_record, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding0, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiAccountAccount(pub MarginfiAccount);
impl MarginfiAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarginfiAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARGINFI_GROUP_ACCOUNT_DISCM: [u8; 8] = [182, 23, 173, 240, 151, 206, 182, 67];
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
pub struct MarginfiGroup {
    pub admin: Pubkey,
    pub group_flags: u64,
    pub fee_state_cache: FeeStateCache,
    pub banks: u16,
    pub pad0: [u8; 6],
    pub emode_admin: Pubkey,
    pub delegate_curve_admin: Pubkey,
    pub delegate_limit_admin: Pubkey,
    pub delegate_emissions_admin: Pubkey,
    pub panic_state_cache: PanicStateCache,
    pub deleverage_withdraw_window_cache: WithdrawWindowCache,
    pub risk_admin: Pubkey,
    pub metadata_admin: Pubkey,
    pub emode_max_init_leverage: u32,
    pub emode_max_maint_leverage: u32,
    pub _padding: [u8; 8],
    pub rate_limiter: GroupRateLimiter,
    pub rate_limiter_last_admin_update_slot: u64,
    pub rate_limiter_last_admin_update_seq: u64,
    pub deleverage_withdraw_last_admin_update_slot: u64,
    pub deleverage_withdraw_last_admin_update_seq: u64,
    pub delegate_flow_admin: Pubkey,
    pub _padding_0: [[u64; 2]; 2],
    pub _padding_1: [[u64; 2]; 32],
}
impl MarginfiGroup {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let group_flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_state_cache = if reader.is_empty() {
            Default::default()
        } else {
            <FeeStateCache>::deserialize(&mut reader)?
        };
        let banks: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let emode_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_curve_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_limit_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_emissions_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let panic_state_cache = if reader.is_empty() {
            Default::default()
        } else {
            <PanicStateCache>::deserialize(&mut reader)?
        };
        let deleverage_withdraw_window_cache = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawWindowCache>::deserialize(&mut reader)?
        };
        let risk_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let emode_max_init_leverage: u32 = crate::borsh_de_or_default(&mut reader)?;
        let emode_max_maint_leverage: u32 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let rate_limiter = if reader.is_empty() {
            Default::default()
        } else {
            <GroupRateLimiter>::deserialize(&mut reader)?
        };
        let rate_limiter_last_admin_update_slot: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let rate_limiter_last_admin_update_seq: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deleverage_withdraw_last_admin_update_slot: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deleverage_withdraw_last_admin_update_seq: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let delegate_flow_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: [[u64; 2]; 2] = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [[u64; 2]; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            group_flags,
            fee_state_cache,
            banks,
            pad0,
            emode_admin,
            delegate_curve_admin,
            delegate_limit_admin,
            delegate_emissions_admin,
            panic_state_cache,
            deleverage_withdraw_window_cache,
            risk_admin,
            metadata_admin,
            emode_max_init_leverage,
            emode_max_maint_leverage,
            _padding,
            rate_limiter,
            rate_limiter_last_admin_update_slot,
            rate_limiter_last_admin_update_seq,
            deleverage_withdraw_last_admin_update_slot,
            deleverage_withdraw_last_admin_update_seq,
            delegate_flow_admin,
            _padding_0,
            _padding_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.group_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_state_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.banks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emode_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate_curve_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate_limit_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate_emissions_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.panic_state_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.deleverage_withdraw_window_cache,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.risk_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metadata_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emode_max_init_leverage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emode_max_maint_leverage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_limiter, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.rate_limiter_last_admin_update_slot,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.rate_limiter_last_admin_update_seq,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.deleverage_withdraw_last_admin_update_slot,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.deleverage_withdraw_last_admin_update_seq,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.delegate_flow_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiGroupAccount(pub MarginfiGroup);
impl MarginfiGroupAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_GROUP_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarginfiGroup::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_GROUP_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINIMAL_OBLIGATION_ACCOUNT_DISCM: [u8; 8] = [
    168, 206, 141, 106, 88, 76, 172, 167,
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
pub struct MinimalObligation {
    pub tag: u64,
    pub last_update_slot: u64,
    pub last_update_stale: u8,
    pub last_update_price_status: u8,
    pub last_update_placeholder: [u8; 6],
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [MinimalObligationCollateral; 8],
    pub lowest_reserve_deposit_liquidation_ltv: u64,
    pub deposited_value_sf: [u8; 16],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part1: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part2: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part3: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part4: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part5a: [u8; 64],
    pub padding_part5c: [u8; 24],
}
impl MinimalObligation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tag: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_stale: u8 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_price_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_placeholder: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let lending_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposits: [MinimalObligationCollateral; 8] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lowest_reserve_deposit_liquidation_ltv: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposited_value_sf: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let padding_part1 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part2 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part3 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part4 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part5a = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part5c: [u8; 24] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            tag,
            last_update_slot,
            last_update_stale,
            last_update_price_status,
            last_update_placeholder,
            lending_market,
            owner,
            deposits,
            lowest_reserve_deposit_liquidation_ltv,
            deposited_value_sf,
            padding_part1,
            padding_part2,
            padding_part3,
            padding_part4,
            padding_part5a,
            padding_part5c,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.tag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_stale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_price_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_placeholder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.lowest_reserve_deposit_liquidation_ltv,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.deposited_value_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part5a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part5c, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MinimalObligationAccount(pub MinimalObligation);
impl MinimalObligationAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINIMAL_OBLIGATION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MinimalObligation::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINIMAL_OBLIGATION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINIMAL_RESERVE_ACCOUNT_DISCM: [u8; 8] = [43, 242, 204, 202, 26, 247, 59, 127];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MinimalReserve {
    pub version: u64,
    pub slot: u64,
    pub stale: u8,
    pub price_status: u8,
    pub placeholder: [u8; 6],
    pub lending_market: Pubkey,
    pub farm_collateral: Pubkey,
    pub farm_debt: Pubkey,
    pub mint_pubkey: Pubkey,
    pub supply_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub available_amount: u64,
    pub borrowed_amount_sf: [u8; 16],
    pub market_price_sf: [u8; 16],
    pub market_price_last_updated_ts: u64,
    pub mint_decimals: u64,
    pub deposit_limit_crossed_timestamp: u64,
    pub borrow_limit_crossed_timestamp: u64,
    #[serde(with = "crate::big_array_serde")]
    pub cumulative_borrow_rate_bsf: [u8; 48],
    pub accumulated_protocol_fees_sf: [u8; 16],
    pub accumulated_referrer_fees_sf: [u8; 16],
    pub pending_referrer_fees_sf: [u8; 16],
    pub absolute_referral_rate_sf: [u8; 16],
    pub token_program: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding2_part1: [u8; 256],
    #[serde(with = "crate::big_array_serde")]
    pub padding2_part2: [u8; 128],
    pub padding2_part3: [u8; 24],
    #[serde(with = "crate::big_array_serde")]
    pub padding3: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part1: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part2: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part3: [u8; 128],
    #[serde(with = "crate::big_array_serde")]
    pub padding_part4: [u8; 48],
    pub collateral_mint_pubkey: Pubkey,
    pub mint_total_supply: u64,
    pub collateral_supply_vault: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding1_reserve_collateral: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding2_reserve_collateral: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding4_part1: [u8; 4096],
    #[serde(with = "crate::big_array_serde")]
    pub padding4_part2: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub padding4_part3: [u8; 256],
    #[serde(with = "crate::big_array_serde")]
    pub padding4_part4: [u8; 64],
    pub padding4_part5: [u8; 32],
    pub padding4_part6: [u8; 8],
}
impl MinimalReserve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stale: u8 = crate::borsh_de_or_default(&mut reader)?;
        let price_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let placeholder: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let lending_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_collateral: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_debt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supply_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let available_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_amount_sf: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let market_price_sf: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let market_price_last_updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_limit_crossed_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_limit_crossed_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_borrow_rate_bsf = <[u8; 48] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let accumulated_protocol_fees_sf: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let accumulated_referrer_fees_sf: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pending_referrer_fees_sf: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let absolute_referral_rate_sf: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding2_part1 = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding2_part2 = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding2_part3: [u8; 24] = crate::borsh_de_or_default(&mut reader)?;
        let padding3 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part1 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part2 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part3 = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding_part4 = <[u8; 48] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let collateral_mint_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_supply_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding1_reserve_collateral = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding2_reserve_collateral = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding4_part1 = <[u8; 4096] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding4_part2 = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding4_part3 = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding4_part4 = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding4_part5: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let padding4_part6: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            version,
            slot,
            stale,
            price_status,
            placeholder,
            lending_market,
            farm_collateral,
            farm_debt,
            mint_pubkey,
            supply_vault,
            fee_vault,
            available_amount,
            borrowed_amount_sf,
            market_price_sf,
            market_price_last_updated_ts,
            mint_decimals,
            deposit_limit_crossed_timestamp,
            borrow_limit_crossed_timestamp,
            cumulative_borrow_rate_bsf,
            accumulated_protocol_fees_sf,
            accumulated_referrer_fees_sf,
            pending_referrer_fees_sf,
            absolute_referral_rate_sf,
            token_program,
            padding2_part1,
            padding2_part2,
            padding2_part3,
            padding3,
            padding_part1,
            padding_part2,
            padding_part3,
            padding_part4,
            collateral_mint_pubkey,
            mint_total_supply,
            collateral_supply_vault,
            padding1_reserve_collateral,
            padding2_reserve_collateral,
            padding4_part1,
            padding4_part2,
            padding4_part3,
            padding4_part4,
            padding4_part5,
            padding4_part6,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placeholder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_debt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.available_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrowed_amount_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_price_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.market_price_last_updated_ts,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.deposit_limit_crossed_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.borrow_limit_crossed_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.cumulative_borrow_rate_bsf, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.accumulated_protocol_fees_sf,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.accumulated_referrer_fees_sf,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pending_referrer_fees_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.absolute_referral_rate_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2_part1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2_part2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2_part3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_part4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_mint_pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_supply_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.padding1_reserve_collateral,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.padding2_reserve_collateral,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding4_part1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4_part2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4_part3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4_part4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4_part5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4_part6, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MinimalReserveAccount(pub MinimalReserve);
impl MinimalReserveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINIMAL_RESERVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MinimalReserve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINIMAL_RESERVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINIMAL_SPOT_MARKET_ACCOUNT_DISCM: [u8; 8] = [
    100, 177, 8, 107, 168, 65, 65, 39,
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
pub struct MinimalSpotMarket {
    pub pubkey: Pubkey,
    pub oracle: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub _padding1: [[u64; 4]; 9],
    pub _padding2: [u8; 8],
    pub deposit_balance: [u8; 16],
    pub borrow_balance: [u8; 16],
    pub cumulative_deposit_interest: [u8; 16],
    pub cumulative_borrow_interest: [u8; 16],
    pub _padding3: [u64; 9],
    pub last_interest_ts: u64,
    pub _padding4: [u64; 13],
    pub decimals: u32,
    pub market_index: u16,
    pub _padding5: [u16; 24],
    pub _padding6: [u8; 1],
    pub pool_id: u8,
    pub _padding7: [u64; 5],
}
impl MinimalSpotMarket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [[u64; 4]; 9] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let deposit_balance: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let borrow_balance: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposit_interest: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_borrow_interest: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let _padding3: [u64; 9] = crate::borsh_de_or_default(&mut reader)?;
        let last_interest_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding4: [u64; 13] = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u32 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let _padding5: [u16; 24] = crate::borsh_de_or_default(&mut reader)?;
        let _padding6: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding7: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            oracle,
            mint,
            vault,
            _padding1,
            _padding2,
            deposit_balance,
            borrow_balance,
            cumulative_deposit_interest,
            cumulative_borrow_interest,
            _padding3,
            last_interest_ts,
            _padding4,
            decimals,
            market_index,
            _padding5,
            _padding6,
            pool_id,
            _padding7,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.cumulative_deposit_interest,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.cumulative_borrow_interest, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_interest_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding6, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding7, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MinimalSpotMarketAccount(pub MinimalSpotMarket);
impl MinimalSpotMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINIMAL_SPOT_MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MinimalSpotMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINIMAL_SPOT_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINIMAL_USER_ACCOUNT_DISCM: [u8; 8] = [159, 117, 95, 227, 239, 151, 58, 236];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MinimalUser {
    pub authority: Pubkey,
    pub delegate: Pubkey,
    pub name: [u8; 32],
    pub spot_positions: [SpotPosition; 8],
    #[serde(with = "crate::big_array_serde")]
    pub _padding1: [u64; 256],
    #[serde(with = "crate::big_array_serde")]
    pub _padding2: [u64; 128],
    #[serde(with = "crate::big_array_serde")]
    pub _padding3: [u64; 64],
    pub _padding4: [u64; 32],
    pub _padding5: [u64; 8],
    pub _padding6: [u64; 2],
    pub _padding7: [u16; 1],
    pub sub_account_id: u16,
    pub status: UserStatus,
    pub _padding8: [u8; 27],
}
impl MinimalUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let spot_positions: [SpotPosition; 8] = crate::borsh_de_or_default(&mut reader)?;
        let _padding1 = <[u64; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let _padding2 = <[u64; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let _padding3 = <[u64; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let _padding4: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        let _padding5: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let _padding6: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let _padding7: [u16; 1] = crate::borsh_de_or_default(&mut reader)?;
        let sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let status: UserStatus = crate::borsh_de_or_default(&mut reader)?;
        let _padding8: [u8; 27] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            delegate,
            name,
            spot_positions,
            _padding1,
            _padding2,
            _padding3,
            _padding4,
            _padding5,
            _padding6,
            _padding7,
            sub_account_id,
            status,
            _padding8,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.spot_positions, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding6, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding7, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sub_account_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding8, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MinimalUserAccount(pub MinimalUser);
impl MinimalUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINIMAL_USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MinimalUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINIMAL_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
    pub marginfi_account: Pubkey,
    pub stop_loss: WrappedI80F48,
    pub take_profit: WrappedI80F48,
    pub placeholder: u64,
    pub max_slippage: u32,
    pub pad0: [u8; 4],
    pub tags: [u16; 2],
    pub pad1: [u8; 4],
    pub _tags_padding: [u8; 32],
    pub trigger: OrderTriggerType,
    pub bump: u8,
    pub pad2: [u8; 6],
    pub _reserved1: [[u8; 32]; 4],
}
impl Order {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let marginfi_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
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
        let placeholder: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_slippage: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let tags: [u16; 2] = crate::borsh_de_or_default(&mut reader)?;
        let pad1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let _tags_padding: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let trigger: OrderTriggerType = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad2: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let _reserved1: [[u8; 32]; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            marginfi_account,
            stop_loss,
            take_profit,
            placeholder,
            max_slippage,
            pad0,
            tags,
            pad1,
            _tags_padding,
            trigger,
            bump,
            pad2,
            _reserved1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.marginfi_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stop_loss, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.take_profit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.placeholder, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._tags_padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trigger, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved1, &mut writer)?;
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
pub const STAKED_SETTINGS_ACCOUNT_DISCM: [u8; 8] = [157, 140, 6, 77, 89, 173, 173, 125];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct StakedSettings {
    pub key: Pubkey,
    pub marginfi_group: Pubkey,
    pub oracle: Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub risk_tier: RiskTier,
    pub _pad0: [u8; 5],
    pub _reserved0: [u8; 8],
    pub _reserved1: [u8; 32],
    #[serde(with = "crate::big_array_serde")]
    pub _reserved2: [u8; 64],
}
impl StakedSettings {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let marginfi_group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let _reserved0: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let _reserved1: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let _reserved2 = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            key,
            marginfi_group,
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            total_asset_value_init_limit,
            oracle_max_age,
            risk_tier,
            _pad0,
            _reserved0,
            _reserved1,
            _reserved2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.marginfi_group, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_weight_init, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_weight_maint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_asset_value_init_limit,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.oracle_max_age, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.risk_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakedSettingsAccount(pub StakedSettings);
impl StakedSettingsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKED_SETTINGS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StakedSettings::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKED_SETTINGS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
