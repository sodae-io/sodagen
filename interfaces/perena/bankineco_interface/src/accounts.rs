use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BANK_STATE_ACCOUNT_DISCM: [u8; 8] = [16, 169, 126, 99, 35, 169, 73, 200];
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
pub struct BankState {
    pub bump: u8,
    pub bank_index: u8,
    pub _padding1: [u8; 6],
    pub config: BankConfig,
    pub status: BankStatus,
    pub accounting: BankAccounting,
    pub mint: BankMint,
    pub vaults: VaultManagement,
    pub _padding2: [u64; 32],
}
impl BankState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bank_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <BankConfig>::deserialize(&mut reader)?
        };
        let status = if reader.is_empty() {
            Default::default()
        } else {
            <BankStatus>::deserialize(&mut reader)?
        };
        let accounting = if reader.is_empty() {
            Default::default()
        } else {
            <BankAccounting>::deserialize(&mut reader)?
        };
        let mint = if reader.is_empty() {
            Default::default()
        } else {
            <BankMint>::deserialize(&mut reader)?
        };
        let vaults = if reader.is_empty() {
            Default::default()
        } else {
            <VaultManagement>::deserialize(&mut reader)?
        };
        let _padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            bank_index,
            _padding1,
            config,
            status,
            accounting,
            mint,
            vaults,
            _padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vaults, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BankStateAccount(pub BankState);
impl BankStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BANK_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BankState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BANK_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLE_GEN_STATE_ACCOUNT_DISCM: [u8; 8] = [
    130, 253, 66, 122, 94, 135, 208, 214,
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
pub struct OracleGenState {
    pub bump: u8,
    pub veto: u8,
    pub _padding1: [u8; 6],
    pub bank: Pubkey,
    pub vault: Pubkey,
    pub config: OracleConfig,
    pub data: OracleData,
    pub result: OracleResult,
    pub _padding2: [u64; 32],
}
impl OracleGenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let veto: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <OracleConfig>::deserialize(&mut reader)?
        };
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <OracleData>::deserialize(&mut reader)?
        };
        let result = if reader.is_empty() {
            Default::default()
        } else {
            <OracleResult>::deserialize(&mut reader)?
        };
        let _padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            veto,
            _padding1,
            bank,
            vault,
            config,
            data,
            result,
            _padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.veto, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.result, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OracleGenStateAccount(pub OracleGenState);
impl OracleGenStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_GEN_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OracleGenState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_GEN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TEAM_GEN_STATE_ACCOUNT_DISCM: [u8; 8] = [30, 252, 138, 15, 198, 186, 51, 100];
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
pub struct TeamGenState {
    pub bump: u8,
    pub _padding1: [u8; 7],
    pub details: TeamDetails,
    pub accounting: TeamAccounting,
    pub _padding2: [u64; 32],
}
impl TeamGenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let details = if reader.is_empty() {
            Default::default()
        } else {
            <TeamDetails>::deserialize(&mut reader)?
        };
        let accounting = if reader.is_empty() {
            Default::default()
        } else {
            <TeamAccounting>::deserialize(&mut reader)?
        };
        let _padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            _padding1,
            details,
            accounting,
            _padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.details, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TeamGenStateAccount(pub TeamGenState);
impl TeamGenStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TEAM_GEN_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TeamGenState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TEAM_GEN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRANCHE_STATE_ACCOUNT_DISCM: [u8; 8] = [212, 231, 254, 24, 238, 63, 92, 105];
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
pub struct TrancheState {
    pub bump: u8,
    pub _padding1: [u8; 7],
    pub bank: Pubkey,
    pub config: TrancheConfig,
    pub accounting: TrancheAccounting,
    pub _padding2: [u64; 9],
}
impl TrancheState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <TrancheConfig>::deserialize(&mut reader)?
        };
        let accounting = if reader.is_empty() {
            Default::default()
        } else {
            <TrancheAccounting>::deserialize(&mut reader)?
        };
        let _padding2: [u64; 9] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            _padding1,
            bank,
            config,
            accounting,
            _padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TrancheStateAccount(pub TrancheState);
impl TrancheStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANCHE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TrancheState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANCHE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VAULT_GEN_STATE_ACCOUNT_DISCM: [u8; 8] = [238, 187, 81, 47, 96, 228, 197, 55];
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
pub struct VaultGenState {
    pub vault_type: VaultType,
    pub vault_index: u8,
    pub yielding_token_index: u8,
    pub bump: u8,
    pub _padding1: [u8; 4],
    pub config: VaultConfig,
    pub status: VaultStatus,
    pub accounting: VaultAccounting,
    pub _padding2: [u64; 32],
}
impl VaultGenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_type: VaultType = crate::borsh_de_or_default(&mut reader)?;
        let vault_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_token_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <VaultConfig>::deserialize(&mut reader)?
        };
        let status = if reader.is_empty() {
            Default::default()
        } else {
            <VaultStatus>::deserialize(&mut reader)?
        };
        let accounting = if reader.is_empty() {
            Default::default()
        } else {
            <VaultAccounting>::deserialize(&mut reader)?
        };
        let _padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault_type,
            vault_index,
            yielding_token_index,
            bump,
            _padding1,
            config,
            status,
            accounting,
            _padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yielding_token_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultGenStateAccount(pub VaultGenState);
impl VaultGenStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_GEN_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VaultGenState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_GEN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAWAL_QUEUE_ACCOUNT_DISCM: [u8; 8] = [
    54, 56, 158, 88, 232, 203, 241, 163,
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
pub struct WithdrawalQueue {
    pub bump: u8,
    pub queue_id: u8,
    pub _padding1: [u8; 6],
    pub owner: Pubkey,
    pub bank: Pubkey,
    pub escrow_mint: Pubkey,
    pub target_mint: Pubkey,
    pub payer: Pubkey,
    pub amount: u64,
    pub request_ts: i64,
    pub expiry_ts: i64,
    pub _padding2: [u64; 12],
}
impl WithdrawalQueue {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let target_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let expiry_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            queue_id,
            _padding1,
            owner,
            bank,
            escrow_mint,
            target_mint,
            payer,
            amount,
            request_ts,
            expiry_ts,
            _padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.queue_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expiry_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawalQueueAccount(pub WithdrawalQueue);
impl WithdrawalQueueAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAWAL_QUEUE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WithdrawalQueue::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAWAL_QUEUE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
