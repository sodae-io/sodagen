use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CRANK_FEE_WHITELIST_ACCOUNT_DISCM: [u8; 8] = [
    39, 105, 184, 30, 248, 231, 176, 133,
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
pub struct CrankFeeWhitelist {
    pub owner: Pubkey,
    pub padding: [u128; 5],
}
impl CrankFeeWhitelist {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owner, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CrankFeeWhitelistAccount(pub CrankFeeWhitelist);
impl CrankFeeWhitelistAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CRANK_FEE_WHITELIST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(CrankFeeWhitelist::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CRANK_FEE_WHITELIST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_ACCOUNT_DISCM: [u8; 8] = [31, 213, 123, 187, 186, 22, 218, 155];
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
pub struct Escrow {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub total_deposit: u64,
    pub claimed_token: u64,
    pub last_claimed_point: u64,
    pub refunded: u8,
    pub padding_1: [u8; 7],
    pub max_cap: u64,
    pub withdrawn_deposit_overflow: u64,
    pub padding: [u128; 1],
}
impl Escrow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let claimed_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_claimed_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let refunded: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawn_deposit_overflow: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 1] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            owner,
            total_deposit,
            claimed_token,
            last_claimed_point,
            refunded,
            padding_1,
            max_cap,
            withdrawn_deposit_overflow,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claimed_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_claimed_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.refunded, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawn_deposit_overflow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowAccount(pub Escrow);
impl EscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Escrow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FCFS_VAULT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    99, 243, 252, 122, 160, 175, 130, 52,
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
pub struct FcfsVaultConfig {
    pub max_depositing_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub depositing_duration_until_last_join_point: u64,
    pub individual_depositing_cap: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 175],
}
impl FcfsVaultConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositing_duration_until_last_join_point: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 175] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            max_depositing_cap,
            start_vesting_duration,
            end_vesting_duration,
            depositing_duration_until_last_join_point,
            individual_depositing_cap,
            escrow_fee,
            activation_type,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.max_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.depositing_duration_until_last_join_point,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.individual_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FcfsVaultConfigAccount(pub FcfsVaultConfig);
impl FcfsVaultConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FCFS_VAULT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FcfsVaultConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FCFS_VAULT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MERKLE_PROOF_METADATA_ACCOUNT_DISCM: [u8; 8] = [
    133, 24, 30, 217, 240, 20, 222, 100,
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
pub struct MerkleProofMetadata {
    pub vault: Pubkey,
    pub padding: [u64; 16],
    pub proof_url: String,
}
impl MerkleProofMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let proof_url: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { vault, padding, proof_url })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.proof_url, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MerkleProofMetadataAccount(pub MerkleProofMetadata);
impl MerkleProofMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERKLE_PROOF_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MerkleProofMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERKLE_PROOF_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MERKLE_ROOT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    103, 2, 222, 217, 73, 50, 187, 39,
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
pub struct MerkleRootConfig {
    pub root: [u8; 32],
    pub vault: Pubkey,
    pub version: u64,
    pub padding: [u64; 8],
}
impl MerkleRootConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let root: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            root,
            vault,
            version,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.root, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MerkleRootConfigAccount(pub MerkleRootConfig);
impl MerkleRootConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERKLE_ROOT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MerkleRootConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERKLE_ROOT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRORATA_VAULT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    93, 214, 205, 104, 119, 9, 51, 152,
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
pub struct ProrataVaultConfig {
    pub max_buying_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 191],
}
impl ProrataVaultConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 191] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            max_buying_cap,
            start_vesting_duration,
            end_vesting_duration,
            escrow_fee,
            activation_type,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.max_buying_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProrataVaultConfigAccount(pub ProrataVaultConfig);
impl ProrataVaultConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRORATA_VAULT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ProrataVaultConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRORATA_VAULT_CONFIG_ACCOUNT_DISCM)?;
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
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Vault {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub token_out_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint: Pubkey,
    pub base: Pubkey,
    pub owner: Pubkey,
    pub max_buying_cap: u64,
    pub total_deposit: u64,
    pub total_escrow: u64,
    pub swapped_amount: u64,
    pub bought_token: u64,
    pub total_refund: u64,
    pub total_claimed_token: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub bump: u8,
    pub pool_type: u8,
    pub vault_mode: u8,
    pub padding_0: [u8; 5],
    pub max_depositing_cap: u64,
    pub individual_depositing_cap: u64,
    pub depositing_point: u64,
    pub escrow_fee: u64,
    pub total_escrow_fee: u64,
    pub whitelist_mode: u8,
    pub activation_type: u8,
    pub padding_1: [u8; 6],
    pub vault_authority: Pubkey,
    pub padding: [u128; 5],
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_out_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_escrow: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swapped_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bought_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_refund: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositing_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let vault_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            token_vault,
            token_out_vault,
            quote_mint,
            base_mint,
            base,
            owner,
            max_buying_cap,
            total_deposit,
            total_escrow,
            swapped_amount,
            bought_token,
            total_refund,
            total_claimed_token,
            start_vesting_point,
            end_vesting_point,
            bump,
            pool_type,
            vault_mode,
            padding_0,
            max_depositing_cap,
            individual_depositing_cap,
            depositing_point,
            escrow_fee,
            total_escrow_fee,
            whitelist_mode,
            activation_type,
            padding_1,
            vault_authority,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_buying_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_escrow, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swapped_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bought_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_refund, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.start_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_vesting_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.individual_depositing_cap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.depositing_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_escrow_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelist_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
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
