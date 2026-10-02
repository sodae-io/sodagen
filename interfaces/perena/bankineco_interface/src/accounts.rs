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
    pub padding1: [u8; 6],
    pub config: BankConfig,
    pub status: BankStatus,
    pub accounting: BankAccounting,
    pub mint: BankMint,
    pub vaults: VaultManagement,
    pub padding2: [u64; 32],
}
impl BankState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bank_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
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
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            bank_index,
            padding1,
            config,
            status,
            accounting,
            mint,
            vaults,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vaults, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
    pub padding1: [u8; 6],
    pub bank: Pubkey,
    pub vault: Pubkey,
    pub config: OracleConfig,
    pub data: OracleData,
    pub result: OracleResult,
    pub padding2: [u64; 32],
}
impl OracleGenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let veto: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
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
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            veto,
            padding1,
            bank,
            vault,
            config,
            data,
            result,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.veto, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.result, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
    pub padding1: [u8; 7],
    pub details: TeamDetails,
    pub accounting: TeamAccounting,
    pub padding2: [u64; 32],
}
impl TeamGenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
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
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            padding1,
            details,
            accounting,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.details, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
    pub padding1: [u8; 7],
    pub bank: Pubkey,
    pub config: TrancheConfig,
    pub accounting: TrancheAccounting,
    pub padding2: [u64; 9],
}
impl TrancheState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
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
        let padding2: [u64; 9] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            padding1,
            bank,
            config,
            accounting,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
    pub bump: u8,
    pub mint_decimals: u8,
    pub vault_id: u16,
    pub circuit_breaker_active: CommonPodBool,
    pub tranching_enabled: CommonPodBool,
    pub losses_enabled: CommonPodBool,
    pub padding_bool: CommonPodBool,
    pub mint: Pubkey,
    pub config: CommonVaultConfig,
    pub roles: VaultRoles,
    pub pending_role_updates: PendingVaultRoleUpdates,
    pub accounting: CommonVaultAccounting,
    pub holdings: [VaultHolding; 12],
    pub external_liquidity: [ExternalLiquiditySlot; 8],
    pub mint_token_program: TokenProgram,
    pub padding_mint_token_program: [u8; 7],
    #[serde(with = "crate::big_array_serde")]
    pub padding2: [u64; 64],
    pub padding2b: [u64; 32],
    pub padding2c: [u64; 5],
    #[serde(with = "crate::big_array_serde")]
    pub padding3: [u64; 128],
    #[serde(with = "crate::big_array_serde")]
    pub padding4: [u64; 128],
    #[serde(with = "crate::big_array_serde")]
    pub padding5: [u64; 64],
    pub padding6: [u64; 16],
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let circuit_breaker_active = if reader.is_empty() {
            Default::default()
        } else {
            <CommonPodBool>::deserialize(&mut reader)?
        };
        let tranching_enabled = if reader.is_empty() {
            Default::default()
        } else {
            <CommonPodBool>::deserialize(&mut reader)?
        };
        let losses_enabled = if reader.is_empty() {
            Default::default()
        } else {
            <CommonPodBool>::deserialize(&mut reader)?
        };
        let padding_bool = if reader.is_empty() {
            Default::default()
        } else {
            <CommonPodBool>::deserialize(&mut reader)?
        };
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <CommonVaultConfig>::deserialize(&mut reader)?
        };
        let roles = if reader.is_empty() {
            Default::default()
        } else {
            <VaultRoles>::deserialize(&mut reader)?
        };
        let pending_role_updates = if reader.is_empty() {
            Default::default()
        } else {
            <PendingVaultRoleUpdates>::deserialize(&mut reader)?
        };
        let accounting = if reader.is_empty() {
            Default::default()
        } else {
            <CommonVaultAccounting>::deserialize(&mut reader)?
        };
        let holdings: [VaultHolding; 12] = crate::borsh_de_or_default(&mut reader)?;
        let external_liquidity = <[ExternalLiquiditySlot; 8] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let mint_token_program: TokenProgram = crate::borsh_de_or_default(&mut reader)?;
        let padding_mint_token_program: [u8; 7] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding2 = <[u64; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding2b: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        let padding2c: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        let padding3 = <[u64; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding4 = <[u64; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding5 = <[u64; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding6: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            mint_decimals,
            vault_id,
            circuit_breaker_active,
            tranching_enabled,
            losses_enabled,
            padding_bool,
            mint,
            config,
            roles,
            pending_role_updates,
            accounting,
            holdings,
            external_liquidity,
            mint_token_program,
            padding_mint_token_program,
            padding2,
            padding2b,
            padding2c,
            padding3,
            padding4,
            padding5,
            padding6,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.circuit_breaker_active, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tranching_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.losses_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_bool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.roles, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_role_updates, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.holdings, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.external_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_token_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_mint_token_program, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2c, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding6, &mut writer)?;
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
    pub reserved_vault_type: u8,
    pub vault_index: u8,
    pub yielding_token_index: u8,
    pub bump: u8,
    pub padding1: [u8; 4],
    pub config: VaultVaultConfig,
    pub status: VaultStatus,
    pub accounting: VaultVaultAccounting,
    pub padding2: [u64; 32],
}
impl VaultGenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reserved_vault_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_token_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <VaultVaultConfig>::deserialize(&mut reader)?
        };
        let status = if reader.is_empty() {
            Default::default()
        } else {
            <VaultStatus>::deserialize(&mut reader)?
        };
        let accounting = if reader.is_empty() {
            Default::default()
        } else {
            <VaultVaultAccounting>::deserialize(&mut reader)?
        };
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            reserved_vault_type,
            vault_index,
            yielding_token_index,
            bump,
            padding1,
            config,
            status,
            accounting,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.reserved_vault_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yielding_token_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.accounting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
pub const VAULT_ORACLE_ACCOUNT_DISCM: [u8; 8] = [55, 234, 122, 181, 207, 61, 41, 204];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct VaultOracle {
    pub bump: u8,
    pub oracle_type: VaultOracleType,
    pub padding1: [u8; 6],
    pub vault: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub data: [u8; 2048],
}
impl VaultOracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_type: VaultOracleType = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let data = <[u8; 2048] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            oracle_type,
            padding1,
            vault,
            data,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultOracleAccount(pub VaultOracle);
impl VaultOracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VaultOracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VAULT_TRANCHE_STATE_ACCOUNT_DISCM: [u8; 8] = [
    58, 61, 35, 146, 194, 219, 153, 37,
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
pub struct VaultTrancheState {
    pub bump: u8,
    pub padding1: [u8; 7],
    pub vault: Pubkey,
    pub config: VaultTrancheConfig,
    pub junior: VaultTrancheAccounting,
    pub senior: VaultTrancheAccounting,
    pub last_settled_ts: i64,
    pub senior_apy_anchor_share_price: u64,
    pub senior_apy_anchor_ts: i64,
    pub padding2: [u64; 29],
}
impl VaultTrancheState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <VaultTrancheConfig>::deserialize(&mut reader)?
        };
        let junior = if reader.is_empty() {
            Default::default()
        } else {
            <VaultTrancheAccounting>::deserialize(&mut reader)?
        };
        let senior = if reader.is_empty() {
            Default::default()
        } else {
            <VaultTrancheAccounting>::deserialize(&mut reader)?
        };
        let last_settled_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let senior_apy_anchor_share_price: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let senior_apy_anchor_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 29] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            padding1,
            vault,
            config,
            junior,
            senior,
            last_settled_ts,
            senior_apy_anchor_share_price,
            senior_apy_anchor_ts,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.junior, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.senior, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_settled_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.senior_apy_anchor_share_price,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.senior_apy_anchor_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultTrancheStateAccount(pub VaultTrancheState);
impl VaultTrancheStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_TRANCHE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VaultTrancheState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_TRANCHE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VAULT_WITHDRAWAL_QUEUE_ACCOUNT_DISCM: [u8; 8] = [
    61, 98, 117, 253, 197, 198, 20, 168,
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
pub struct VaultWithdrawalQueue {
    pub header: WithdrawalQueueHeader,
    pub parties: WithdrawalQueueParties,
    pub mints: WithdrawalQueueMints,
    pub request: WithdrawalQueueRequest,
}
impl VaultWithdrawalQueue {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalQueueHeader>::deserialize(&mut reader)?
        };
        let parties = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalQueueParties>::deserialize(&mut reader)?
        };
        let mints = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalQueueMints>::deserialize(&mut reader)?
        };
        let request = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalQueueRequest>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            header,
            parties,
            mints,
            request,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.parties, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mints, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultWithdrawalQueueAccount(pub VaultWithdrawalQueue);
impl VaultWithdrawalQueueAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VAULT_WITHDRAWAL_QUEUE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VaultWithdrawalQueue::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VAULT_WITHDRAWAL_QUEUE_ACCOUNT_DISCM)?;
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
    pub padding1: [u8; 6],
    pub owner: Pubkey,
    pub bank: Pubkey,
    pub escrow_mint: Pubkey,
    pub target_mint: Pubkey,
    pub payer: Pubkey,
    pub amount: u64,
    pub request_ts: i64,
    pub expiry_ts: i64,
    pub padding2: [u64; 12],
}
impl WithdrawalQueue {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let target_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let expiry_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            queue_id,
            padding1,
            owner,
            bank,
            escrow_mint,
            target_mint,
            payer,
            amount,
            request_ts,
            expiry_ts,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.queue_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bank, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.payer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.request_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expiry_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
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
pub const FEE_VAULT_ACCOUNT_DISCM: [u8; 8] = [192, 178, 69, 232, 58, 149, 157, 132];
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
pub struct FeeVault {
    pub bump: u8,
    pub padding0: [u8; 7],
    pub vault: Pubkey,
    pub mint_fees_owed: u64,
    pub mint_fees_collected: u64,
    pub burn_fees_owed: u64,
    pub burn_fees_collected: u64,
    pub performance_fees_owed: u64,
    pub performance_fees_collected: u64,
    pub protocol_fees_owed: u64,
    pub protocol_fees_collected: u64,
    pub padding1: [u64; 16],
}
impl FeeVault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_fees_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burn_fees_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burn_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fees_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_collected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            padding0,
            vault,
            mint_fees_owed,
            mint_fees_collected,
            burn_fees_owed,
            burn_fees_collected,
            performance_fees_owed,
            performance_fees_collected,
            protocol_fees_owed,
            protocol_fees_collected,
            padding1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_fees_owed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.burn_fees_owed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.burn_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.performance_fees_owed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.performance_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_owed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_collected, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeVaultAccount(pub FeeVault);
impl FeeVaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_VAULT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeVault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
