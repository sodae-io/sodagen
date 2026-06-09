use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CRANK_FEE_WHITELIST_CLOSED_EVENT_DISCM: [u8; 8] = [
    157, 171, 85, 155, 37, 20, 41, 114,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CrankFeeWhitelistClosed {
    pub cranker: Pubkey,
}
impl CrankFeeWhitelistClosed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cranker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { cranker })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.cranker, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CrankFeeWhitelistClosedEvent(pub CrankFeeWhitelistClosed);
impl CrankFeeWhitelistClosedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CRANK_FEE_WHITELIST_CLOSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CrankFeeWhitelistClosed::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CRANK_FEE_WHITELIST_CLOSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CRANK_FEE_WHITELIST_CREATED_EVENT_DISCM: [u8; 8] = [
    176, 138, 32, 77, 129, 74, 137, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CrankFeeWhitelistCreated {
    pub cranker: Pubkey,
}
impl CrankFeeWhitelistCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cranker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { cranker })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.cranker, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CrankFeeWhitelistCreatedEvent(pub CrankFeeWhitelistCreated);
impl CrankFeeWhitelistCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CRANK_FEE_WHITELIST_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CrankFeeWhitelistCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CRANK_FEE_WHITELIST_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_CLAIM_TOKEN_EVENT_DISCM: [u8; 8] = [179, 72, 71, 30, 59, 19, 170, 3];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EscrowClaimToken {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub vault_total_claimed_token: u64,
}
impl EscrowClaimToken {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_total_claimed_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            escrow,
            owner,
            amount,
            vault_total_claimed_token,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_total_claimed_token, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowClaimTokenEvent(pub EscrowClaimToken);
impl EscrowClaimTokenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_CLAIM_TOKEN_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EscrowClaimToken::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_CLAIM_TOKEN_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_CLOSED_EVENT_DISCM: [u8; 8] = [109, 20, 57, 51, 217, 118, 3, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EscrowClosed {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub vault_total_escrow: u64,
}
impl EscrowClosed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_total_escrow: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            escrow,
            owner,
            vault_total_escrow,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_total_escrow, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowClosedEvent(pub EscrowClosed);
impl EscrowClosedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_CLOSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EscrowClosed::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_CLOSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_CREATED_EVENT_DISCM: [u8; 8] = [70, 127, 105, 102, 92, 97, 7, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EscrowCreated {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub vault_total_escrow: u64,
    pub escrow_fee: u64,
}
impl EscrowCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_total_escrow: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            escrow,
            owner,
            vault_total_escrow,
            escrow_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_total_escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowCreatedEvent(pub EscrowCreated);
impl EscrowCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EscrowCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_DEPOSIT_EVENT_DISCM: [u8; 8] = [43, 90, 49, 176, 134, 148, 50, 32];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EscrowDeposit {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub vault_total_deposit: u64,
}
impl EscrowDeposit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_total_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            escrow,
            owner,
            amount,
            vault_total_deposit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_total_deposit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowDepositEvent(pub EscrowDeposit);
impl EscrowDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_DEPOSIT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EscrowDeposit::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_DEPOSIT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_REMAINING_WITHDRAW_EVENT_DISCM: [u8; 8] = [
    113, 14, 156, 89, 113, 79, 88, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EscrowRemainingWithdraw {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub vault_remaining_deposit: u64,
}
impl EscrowRemainingWithdraw {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_remaining_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            escrow,
            owner,
            amount,
            vault_remaining_deposit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_remaining_deposit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowRemainingWithdrawEvent(pub EscrowRemainingWithdraw);
impl EscrowRemainingWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_REMAINING_WITHDRAW_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EscrowRemainingWithdraw::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_REMAINING_WITHDRAW_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_WITHDRAW_EVENT_DISCM: [u8; 8] = [171, 17, 164, 116, 122, 66, 183, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EscrowWithdraw {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub vault_total_deposit: u64,
}
impl EscrowWithdraw {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_total_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            escrow,
            owner,
            amount,
            vault_total_deposit,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_total_deposit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowWithdrawEvent(pub EscrowWithdraw);
impl EscrowWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_WITHDRAW_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = EscrowWithdraw::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_WITHDRAW_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FCFS_VAULT_CREATED_EVENT_DISCM: [u8; 8] = [
    73, 153, 165, 103, 151, 182, 184, 136,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FcfsVaultCreated {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub max_depositing_cap: u64,
    pub pool: Pubkey,
    pub pool_type: u8,
    pub depositing_point: u64,
    pub individual_depositing_cap: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
}
impl FcfsVaultCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let depositing_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_mint,
            quote_mint,
            start_vesting_point,
            end_vesting_point,
            max_depositing_cap,
            pool,
            pool_type,
            depositing_point,
            individual_depositing_cap,
            escrow_fee,
            activation_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositing_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.individual_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FcfsVaultCreatedEvent(pub FcfsVaultCreated);
impl FcfsVaultCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FCFS_VAULT_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FcfsVaultCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FCFS_VAULT_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FCFS_VAULT_PARAMETERS_UPDATED_EVENT_DISCM: [u8; 8] = [
    78, 112, 112, 62, 193, 209, 231, 226,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FcfsVaultParametersUpdated {
    pub vault: Pubkey,
    pub max_depositing_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub depositing_point: u64,
    pub individual_depositing_cap: u64,
}
impl FcfsVaultParametersUpdated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositing_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            max_depositing_cap,
            start_vesting_point,
            end_vesting_point,
            depositing_point,
            individual_depositing_cap,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositing_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.individual_depositing_cap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FcfsVaultParametersUpdatedEvent(pub FcfsVaultParametersUpdated);
impl FcfsVaultParametersUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FCFS_VAULT_PARAMETERS_UPDATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FcfsVaultParametersUpdated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FCFS_VAULT_PARAMETERS_UPDATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MERKLE_PROOF_METADATA_CREATED_EVENT_DISCM: [u8; 8] = [
    186, 42, 131, 176, 244, 128, 196, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerkleProofMetadataCreated {
    pub vault: Pubkey,
    pub proof_url: String,
}
impl MerkleProofMetadataCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let proof_url: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { vault, proof_url })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proof_url, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MerkleProofMetadataCreatedEvent(pub MerkleProofMetadataCreated);
impl MerkleProofMetadataCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERKLE_PROOF_METADATA_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MerkleProofMetadataCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERKLE_PROOF_METADATA_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MERKLE_ROOT_CONFIG_CREATED_EVENT_DISCM: [u8; 8] = [
    121, 112, 42, 76, 144, 131, 142, 90,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerkleRootConfigCreated {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub vault: Pubkey,
    pub version: u64,
    pub root: [u8; 32],
}
impl MerkleRootConfigCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        let root: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            config,
            vault,
            version,
            root,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.root, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MerkleRootConfigCreatedEvent(pub MerkleRootConfigCreated);
impl MerkleRootConfigCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERKLE_ROOT_CONFIG_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MerkleRootConfigCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERKLE_ROOT_CONFIG_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRORATA_VAULT_CREATED_EVENT_DISCM: [u8; 8] = [
    181, 255, 162, 226, 203, 199, 193, 6,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProrataVaultCreated {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub max_buying_cap: u64,
    pub pool: Pubkey,
    pub pool_type: u8,
    pub escrow_fee: u64,
    pub activation_type: u8,
}
impl ProrataVaultCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_mint,
            quote_mint,
            start_vesting_point,
            end_vesting_point,
            max_buying_cap,
            pool,
            pool_type,
            escrow_fee,
            activation_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_buying_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProrataVaultCreatedEvent(pub ProrataVaultCreated);
impl ProrataVaultCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRORATA_VAULT_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ProrataVaultCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRORATA_VAULT_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRORATA_VAULT_PARAMETERS_UPDATED_EVENT_DISCM: [u8; 8] = [
    24, 147, 160, 237, 132, 87, 15, 206,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProrataVaultParametersUpdated {
    pub vault: Pubkey,
    pub max_buying_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
}
impl ProrataVaultParametersUpdated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            max_buying_cap,
            start_vesting_point,
            end_vesting_point,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_buying_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_point, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProrataVaultParametersUpdatedEvent(pub ProrataVaultParametersUpdated);
impl ProrataVaultParametersUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRORATA_VAULT_PARAMETERS_UPDATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = ProrataVaultParametersUpdated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRORATA_VAULT_PARAMETERS_UPDATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_FILL_EVENT_DISCM: [u8; 8] = [116, 212, 73, 222, 33, 244, 134, 148];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapFill {
    pub vault: Pubkey,
    pub pair: Pubkey,
    pub fill_amount: u64,
    pub purchased_amount: u64,
    pub unfilled_amount: u64,
}
impl SwapFill {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fill_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let purchased_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unfilled_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            pair,
            fill_amount,
            purchased_amount,
            unfilled_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fill_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.purchased_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unfilled_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapFillEvent(pub SwapFill);
impl SwapFillEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_FILL_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapFill::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_FILL_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
