use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADAPTOR_ADD_RECEIPT_ACCOUNT_DISCM: [u8; 8] = [
    105, 99, 219, 155, 77, 241, 7, 119,
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
pub struct AdaptorAddReceipt {
    pub vault: Pubkey,
    pub adaptor_program: Pubkey,
    pub version: u8,
    pub bump: u8,
    pub padding0: [u8; 7],
    pub last_updated_epoch: u64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 56],
}
impl AdaptorAddReceipt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 56] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            adaptor_program,
            version,
            bump,
            padding0,
            last_updated_epoch,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdaptorAddReceiptAccount(pub AdaptorAddReceipt);
impl AdaptorAddReceiptAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADAPTOR_ADD_RECEIPT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AdaptorAddReceipt::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADAPTOR_ADD_RECEIPT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DIRECT_WITHDRAW_INIT_RECEIPT_ACCOUNT_DISCM: [u8; 8] = [
    206, 77, 207, 208, 25, 244, 81, 172,
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
pub struct DirectWithdrawInitReceipt {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub adaptor_program: Pubkey,
    pub instruction_discriminator: Vec<u8>,
    pub additional_args: Option<Vec<u8>>,
    pub allow_user_args: bool,
    pub version: u8,
    pub bump: u8,
}
impl DirectWithdrawInitReceipt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let instruction_discriminator: Vec<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let additional_args: Option<Vec<u8>> = crate::borsh_de_or_default(&mut reader)?;
        let allow_user_args: bool = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            strategy,
            adaptor_program,
            instruction_discriminator,
            additional_args,
            allow_user_args,
            version,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.instruction_discriminator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.additional_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_user_args, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DirectWithdrawInitReceiptAccount(pub DirectWithdrawInitReceipt);
impl DirectWithdrawInitReceiptAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DIRECT_WITHDRAW_INIT_RECEIPT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DirectWithdrawInitReceipt::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DIRECT_WITHDRAW_INIT_RECEIPT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROTOCOL_ACCOUNT_DISCM: [u8; 8] = [45, 39, 101, 43, 115, 72, 131, 40];
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
pub struct Protocol {
    pub admin: Pubkey,
    pub operational_state: u16,
    pub padding1: [u8; 2],
    pub bump: u8,
    pub padding0: [u8; 1],
    pub pending_admin: Pubkey,
    pub treasury: Pubkey,
}
impl Protocol {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let pending_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            operational_state,
            padding1,
            bump,
            padding0,
            pending_admin,
            treasury,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operational_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolAccount(pub Protocol);
impl ProtocolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Protocol::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REQUEST_WITHDRAW_VAULT_RECEIPT_ACCOUNT_DISCM: [u8; 8] = [
    203, 81, 223, 141, 175, 108, 101, 114,
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
pub struct RequestWithdrawVaultReceipt {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub amount_lp_escrowed: u64,
    pub amount_asset_to_withdraw_decimal_bits: u128,
    pub withdrawable_from_ts: u64,
    pub bump: u8,
    pub version: u8,
}
impl RequestWithdrawVaultReceipt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_lp_escrowed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_asset_to_withdraw_decimal_bits: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawable_from_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            user,
            amount_lp_escrowed,
            amount_asset_to_withdraw_decimal_bits,
            withdrawable_from_ts,
            bump,
            version,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_lp_escrowed, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.amount_asset_to_withdraw_decimal_bits,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdrawable_from_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestWithdrawVaultReceiptAccount(pub RequestWithdrawVaultReceipt);
impl RequestWithdrawVaultReceiptAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_WITHDRAW_VAULT_RECEIPT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RequestWithdrawVaultReceipt::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_WITHDRAW_VAULT_RECEIPT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STRATEGY_INIT_RECEIPT_ACCOUNT_DISCM: [u8; 8] = [
    51, 8, 192, 253, 115, 78, 112, 214,
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
pub struct StrategyInitReceipt {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub adaptor_program: Pubkey,
    pub position_value: u64,
    pub last_updated_ts: u64,
    pub version: u8,
    pub bump: u8,
    pub vault_strategy_auth_bump: u8,
    pub padding0: [u8; 5],
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 64],
}
impl StrategyInitReceipt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let adaptor_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_strategy_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            strategy,
            adaptor_program,
            position_value,
            last_updated_ts,
            version,
            bump,
            vault_strategy_auth_bump,
            padding0,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptor_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_value, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_strategy_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyInitReceiptAccount(pub StrategyInitReceipt);
impl StrategyInitReceiptAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STRATEGY_INIT_RECEIPT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StrategyInitReceipt::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STRATEGY_INIT_RECEIPT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VAULT_ACCOUNT_DISCM: [u8; 8] = [211, 8, 232, 43, 2, 152, 117, 119];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Vault {
    pub name: [u8; 32],
    #[serde(with = "crate::big_array_serde")]
    pub description: [u8; 64],
    pub asset: VaultAsset,
    pub lp: VaultLp,
    pub pending_admin: Pubkey,
    pub manager: Pubkey,
    pub admin: Pubkey,
    pub vault_configuration: VaultConfiguration,
    pub fee_configuration: FeeConfiguration,
    pub fee_update: FeeUpdate,
    pub fee_state: FeeState,
    pub dead_weight: u64,
    pub high_water_mark: HighWaterMark,
    pub last_updated_ts: u64,
    pub version: u8,
    pub allow_any_adaptor: u8,
    pub padding0: [u8; 6],
    pub locked_profit_state: LockedProfitState,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 240],
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let description = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let asset = <VaultAsset>::deserialize(&mut reader)?;
        let lp = if reader.is_empty() {
            Default::default()
        } else {
            <VaultLp>::deserialize(&mut reader)?
        };
        let pending_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_configuration = <VaultConfiguration>::deserialize(&mut reader)?;
        let fee_configuration = if reader.is_empty() {
            Default::default()
        } else {
            <FeeConfiguration>::deserialize(&mut reader)?
        };
        let fee_update = if reader.is_empty() {
            Default::default()
        } else {
            <FeeUpdate>::deserialize(&mut reader)?
        };
        let fee_state = if reader.is_empty() {
            Default::default()
        } else {
            <FeeState>::deserialize(&mut reader)?
        };
        let dead_weight: u64 = crate::borsh_de_or_default(&mut reader)?;
        let high_water_mark = if reader.is_empty() {
            Default::default()
        } else {
            <HighWaterMark>::deserialize(&mut reader)?
        };
        let last_updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let allow_any_adaptor: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let locked_profit_state = if reader.is_empty() {
            Default::default()
        } else {
            <LockedProfitState>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 240] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            name,
            description,
            asset,
            lp,
            pending_admin,
            manager,
            admin,
            vault_configuration,
            fee_configuration,
            fee_update,
            fee_state,
            dead_weight,
            high_water_mark,
            last_updated_ts,
            version,
            allow_any_adaptor,
            padding0,
            locked_profit_state,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.description, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_configuration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_configuration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dead_weight, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.high_water_mark, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allow_any_adaptor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_profit_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultAccount(pub Vault);
impl VaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Vault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
