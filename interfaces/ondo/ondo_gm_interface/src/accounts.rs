use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ATTESTATION_ACCOUNT_DISCM: [u8; 8] = [152, 125, 183, 86, 36, 146, 121, 73];
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
pub struct Attestation {
    pub attestation_id: [u8; 16],
    pub creator: Pubkey,
    pub created_at: i64,
    pub bump: u8,
}
impl Attestation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let attestation_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            attestation_id,
            creator,
            created_at,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.attestation_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AttestationAccount(pub Attestation);
impl AttestationAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ATTESTATION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Attestation::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ATTESTATION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GM_TOKEN_MANAGER_STATE_ACCOUNT_DISCM: [u8; 8] = [
    120, 3, 104, 163, 207, 32, 156, 24,
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
pub struct GMTokenManagerState {
    pub execution_id: Option<u128>,
    pub factory_paused: bool,
    pub redemption_paused: bool,
    pub minting_paused: bool,
    pub bump: u8,
    pub attestation_signer_secp: [u8; 20],
    pub trading_hours_offset: i64,
}
impl GMTokenManagerState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let execution_id: Option<u128> = crate::borsh_de_or_default(&mut reader)?;
        let factory_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let redemption_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let minting_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let attestation_signer_secp: [u8; 20] = crate::borsh_de_or_default(&mut reader)?;
        let trading_hours_offset: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            execution_id,
            factory_paused,
            redemption_paused,
            minting_paused,
            bump,
            attestation_signer_secp,
            trading_hours_offset,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.execution_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.factory_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minting_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.attestation_signer_secp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trading_hours_offset, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GMTokenManagerStateAccount(pub GMTokenManagerState);
impl GMTokenManagerStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GM_TOKEN_MANAGER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GMTokenManagerState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GM_TOKEN_MANAGER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ONDO_USER_ACCOUNT_DISCM: [u8; 8] = [20, 5, 255, 14, 176, 93, 189, 142];
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
pub struct OndoUser {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub rate_limit: Option<u64>,
    pub limit_window: Option<u64>,
    pub mint_capacity_used: Option<u64>,
    pub mint_last_updated: Option<i64>,
    pub redeem_capacity_used: Option<u64>,
    pub redeem_last_updated: Option<i64>,
    pub bump: u8,
}
impl OndoUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let mint_capacity_used: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let mint_last_updated: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let redeem_capacity_used: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let redeem_last_updated: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            mint,
            rate_limit,
            limit_window,
            mint_capacity_used,
            mint_last_updated,
            redeem_capacity_used,
            redeem_last_updated,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_capacity_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_last_updated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_capacity_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_last_updated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OndoUserAccount(pub OndoUser);
impl OndoUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ONDO_USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OndoUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ONDO_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLE_SANITY_CHECK_ACCOUNT_DISCM: [u8; 8] = [
    27, 186, 33, 40, 113, 34, 206, 76,
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
pub struct OracleSanityCheck {
    pub mint: Pubkey,
    pub last_price: u64,
    pub allowed_deviation_bps: u64,
    pub max_time_delay: i64,
    pub price_last_updated: i64,
    pub bump: u8,
}
impl OracleSanityCheck {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let allowed_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_time_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
        let price_last_updated: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            last_price,
            allowed_deviation_bps,
            max_time_delay,
            price_last_updated,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allowed_deviation_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_time_delay, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_last_updated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OracleSanityCheckAccount(pub OracleSanityCheck);
impl OracleSanityCheckAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_SANITY_CHECK_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OracleSanityCheck::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_SANITY_CHECK_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ROLES_ACCOUNT_DISCM: [u8; 8] = [177, 37, 17, 201, 242, 158, 212, 65];
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
pub struct Roles {
    pub address: Pubkey,
    pub role: RoleType,
    pub bump: u8,
}
impl Roles {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { address, role, bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RolesAccount(pub Roles);
impl RolesAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROLES_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Roles::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROLES_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_LIMIT_ACCOUNT_DISCM: [u8; 8] = [130, 254, 128, 120, 255, 3, 217, 51];
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
pub struct TokenLimit {
    pub mint: Pubkey,
    pub rate_limit: Option<u64>,
    pub limit_window: Option<u64>,
    pub mint_capacity_used: Option<u64>,
    pub mint_last_updated: Option<i64>,
    pub redeem_capacity_used: Option<u64>,
    pub redeem_last_updated: Option<i64>,
    pub redemption_paused: bool,
    pub minting_paused: bool,
    pub default_user_rate_limit: Option<u64>,
    pub default_user_limit_window: Option<u64>,
    pub bump: u8,
}
impl TokenLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rate_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let mint_capacity_used: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let mint_last_updated: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let redeem_capacity_used: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let redeem_last_updated: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let redemption_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let minting_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let default_user_rate_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let default_user_limit_window: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            rate_limit,
            limit_window,
            mint_capacity_used,
            mint_last_updated,
            redeem_capacity_used,
            redeem_last_updated,
            redemption_paused,
            minting_paused,
            default_user_rate_limit,
            default_user_limit_window,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_capacity_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_last_updated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_capacity_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redeem_last_updated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.redemption_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minting_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_user_rate_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_user_limit_window, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenLimitAccount(pub TokenLimit);
impl TokenLimitAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_LIMIT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenLimit::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_LIMIT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const US_DON_MANAGER_STATE_ACCOUNT_DISCM: [u8; 8] = [4, 39, 95, 41, 51, 43, 174, 91];
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
pub struct USDonManagerState {
    pub owner: Pubkey,
    pub usdon_mint: Pubkey,
    pub oracle_price_enabled: bool,
    pub oracle_price_max_age: u64,
    pub usdc_price_update: Pubkey,
    pub usdc_vault: Pubkey,
    pub usdon_vault: Pubkey,
    pub bump: u8,
}
impl USDonManagerState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let usdon_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price_max_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        let usdc_price_update: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let usdc_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let usdon_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            usdon_mint,
            oracle_price_enabled,
            oracle_price_max_age,
            usdc_price_update,
            usdc_vault,
            usdon_vault,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdon_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_price_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_price_max_age, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_price_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdc_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.usdon_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct USDonManagerStateAccount(pub USDonManagerState);
impl USDonManagerStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != US_DON_MANAGER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(USDonManagerState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&US_DON_MANAGER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHITELIST_ACCOUNT_DISCM: [u8; 8] = [204, 176, 52, 79, 146, 121, 54, 247];
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
pub struct Whitelist {
    pub user: Pubkey,
}
impl Whitelist {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { user })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhitelistAccount(pub Whitelist);
impl WhitelistAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHITELIST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Whitelist::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHITELIST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
