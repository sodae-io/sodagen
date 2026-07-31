use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const GM_TOKEN_DEPLOYED_EVENT_DISCM: [u8; 8] = [155, 46, 81, 223, 209, 203, 89, 46];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GmTokenDeployed {
    pub gm_token: Pubkey,
}
impl GmTokenDeployed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let gm_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { gm_token })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.gm_token, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GmTokenDeployedEvent(pub GmTokenDeployed);
impl GmTokenDeployedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GM_TOKEN_DEPLOYED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GmTokenDeployed::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GM_TOKEN_DEPLOYED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GM_TOKEN_MINTING_PAUSED_EVENT_DISCM: [u8; 8] = [
    215, 17, 82, 34, 76, 211, 151, 22,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GmTokenMintingPaused {
    pub is_paused: bool,
    pub token: Pubkey,
    pub pauser: Pubkey,
}
impl GmTokenMintingPaused {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pauser: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused, token, pauser })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pauser, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GmTokenMintingPausedEvent(pub GmTokenMintingPaused);
impl GmTokenMintingPausedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GM_TOKEN_MINTING_PAUSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GmTokenMintingPaused::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GM_TOKEN_MINTING_PAUSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GM_TOKEN_PAUSED_EVENT_DISCM: [u8; 8] = [147, 68, 57, 240, 27, 46, 44, 201];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GmTokenPaused {
    pub is_paused: bool,
    pub token: Pubkey,
    pub pauser: Pubkey,
}
impl GmTokenPaused {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pauser: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused, token, pauser })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pauser, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GmTokenPausedEvent(pub GmTokenPaused);
impl GmTokenPausedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GM_TOKEN_PAUSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GmTokenPaused::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GM_TOKEN_PAUSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const GM_TOKEN_REDEMPTION_PAUSED_EVENT_DISCM: [u8; 8] = [
    246, 249, 230, 73, 152, 65, 61, 214,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GmTokenRedemptionPaused {
    pub is_paused: bool,
    pub token: Pubkey,
    pub pauser: Pubkey,
}
impl GmTokenRedemptionPaused {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pauser: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused, token, pauser })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pauser, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GmTokenRedemptionPausedEvent(pub GmTokenRedemptionPaused);
impl GmTokenRedemptionPausedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GM_TOKEN_REDEMPTION_PAUSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = GmTokenRedemptionPaused::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GM_TOKEN_REDEMPTION_PAUSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RATE_LIMIT_TOKEN_SET_EVENT_DISCM: [u8; 8] = [
    133, 18, 77, 154, 53, 236, 197, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RateLimitTokenSet {
    pub token: Pubkey,
    pub limit: Option<u64>,
    pub limit_window: Option<u64>,
}
impl RateLimitTokenSet {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let limit_window: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token, limit, limit_window })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_window, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RateLimitTokenSetEvent(pub RateLimitTokenSet);
impl RateLimitTokenSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RATE_LIMIT_TOKEN_SET_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RateLimitTokenSet::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RATE_LIMIT_TOKEN_SET_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RATE_LIMIT_USER_SET_EVENT_DISCM: [u8; 8] = [
    175, 49, 222, 84, 192, 105, 171, 129,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RateLimitUserSet {
    pub user: Pubkey,
    pub limit: u64,
}
impl RateLimitUserSet {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { user, limit })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RateLimitUserSetEvent(pub RateLimitUserSet);
impl RateLimitUserSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RATE_LIMIT_USER_SET_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RateLimitUserSet::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RATE_LIMIT_USER_SET_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ROLE_GRANTED_EVENT_DISCM: [u8; 8] = [220, 183, 89, 228, 143, 63, 246, 58];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RoleGranted {
    pub role: RoleType,
    pub grantee: Pubkey,
    pub granter: Pubkey,
}
impl RoleGranted {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let grantee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let granter: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { role, grantee, granter })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.grantee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.granter, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RoleGrantedEvent(pub RoleGranted);
impl RoleGrantedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROLE_GRANTED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RoleGranted::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROLE_GRANTED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ROLE_REVOKED_EVENT_DISCM: [u8; 8] = [167, 183, 52, 229, 126, 206, 62, 61];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RoleRevoked {
    pub role: RoleType,
    pub grantee: Pubkey,
    pub revoker: Pubkey,
}
impl RoleRevoked {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let role: RoleType = crate::borsh_de_or_default(&mut reader)?;
        let grantee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let revoker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { role, grantee, revoker })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.role, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.grantee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revoker, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RoleRevokedEvent(pub RoleRevoked);
impl RoleRevokedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROLE_REVOKED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = RoleRevoked::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROLE_REVOKED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SANITY_CHECK_SET_EVENT_DISCM: [u8; 8] = [9, 207, 117, 155, 44, 153, 31, 234];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SanityCheckSet {
    pub mint: Pubkey,
    pub allowed_deviation_bps: u64,
    pub max_time_delay: i64,
}
impl SanityCheckSet {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let allowed_deviation_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_time_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            allowed_deviation_bps,
            max_time_delay,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allowed_deviation_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_time_delay, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SanityCheckSetEvent(pub SanityCheckSet);
impl SanityCheckSetEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SANITY_CHECK_SET_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SanityCheckSet::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SANITY_CHECK_SET_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SANITY_CHECK_UPDATED_EVENT_DISCM: [u8; 8] = [
    173, 97, 103, 183, 67, 230, 249, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SanityCheckUpdated {
    pub mint: Pubkey,
    pub last_price: Option<u64>,
    pub allowed_deviation_bps: Option<u64>,
    pub max_time_delay: Option<i64>,
}
impl SanityCheckUpdated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let allowed_deviation_bps: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_time_delay: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            last_price,
            allowed_deviation_bps,
            max_time_delay,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allowed_deviation_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_time_delay, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SanityCheckUpdatedEvent(pub SanityCheckUpdated);
impl SanityCheckUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SANITY_CHECK_UPDATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SanityCheckUpdated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SANITY_CHECK_UPDATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_FACTORY_PAUSED_EVENT_DISCM: [u8; 8] = [
    24, 185, 192, 62, 218, 128, 226, 103,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenFactoryPaused {
    pub is_paused: bool,
    pub pauser: Pubkey,
}
impl TokenFactoryPaused {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let pauser: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused, pauser })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pauser, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenFactoryPausedEvent(pub TokenFactoryPaused);
impl TokenFactoryPausedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_FACTORY_PAUSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TokenFactoryPaused::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_FACTORY_PAUSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_MANAGER_MINTING_PAUSED_EVENT_DISCM: [u8; 8] = [
    96, 11, 118, 59, 8, 78, 247, 254,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenManagerMintingPaused {
    pub is_paused: bool,
    pub pauser: Pubkey,
}
impl TokenManagerMintingPaused {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let pauser: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused, pauser })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pauser, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenManagerMintingPausedEvent(pub TokenManagerMintingPaused);
impl TokenManagerMintingPausedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_MANAGER_MINTING_PAUSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TokenManagerMintingPaused::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_MANAGER_MINTING_PAUSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_MANAGER_REDEMPTION_PAUSED_EVENT_DISCM: [u8; 8] = [
    66, 222, 165, 60, 32, 59, 135, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenManagerRedemptionPaused {
    pub is_paused: bool,
    pub pauser: Pubkey,
}
impl TokenManagerRedemptionPaused {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let pauser: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { is_paused, pauser })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pauser, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenManagerRedemptionPausedEvent(pub TokenManagerRedemptionPaused);
impl TokenManagerRedemptionPausedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_MANAGER_REDEMPTION_PAUSED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TokenManagerRedemptionPaused::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_MANAGER_REDEMPTION_PAUSED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKENS_RETRIEVED_EVENT_DISCM: [u8; 8] = [217, 35, 1, 89, 38, 122, 145, 187];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokensRetrieved {
    pub token: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub authority: Pubkey,
}
impl TokensRetrieved {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let to: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token,
            to,
            amount,
            authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.to, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokensRetrievedEvent(pub TokensRetrieved);
impl TokensRetrievedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKENS_RETRIEVED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TokensRetrieved::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKENS_RETRIEVED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADE_EXECUTED_EVENT_DISCM: [u8; 8] = [41, 110, 64, 129, 60, 79, 179, 80];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeExecuted {
    pub execution_id: u128,
}
impl TradeExecuted {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let execution_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { execution_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.execution_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeExecutedEvent(pub TradeExecuted);
impl TradeExecutedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_EXECUTED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeExecuted::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_EXECUTED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_ADDED_TO_WHITELIST_EVENT_DISCM: [u8; 8] = [
    206, 139, 181, 72, 116, 111, 42, 42,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserAddedToWhitelist {
    pub user: Pubkey,
    pub added_by: Pubkey,
}
impl UserAddedToWhitelist {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let added_by: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { user, added_by })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.added_by, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserAddedToWhitelistEvent(pub UserAddedToWhitelist);
impl UserAddedToWhitelistEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_ADDED_TO_WHITELIST_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserAddedToWhitelist::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_ADDED_TO_WHITELIST_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_REMOVED_FROM_WHITELIST_EVENT_DISCM: [u8; 8] = [
    106, 223, 245, 11, 213, 150, 2, 15,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserRemovedFromWhitelist {
    pub user: Pubkey,
    pub removed_by: Pubkey,
}
impl UserRemovedFromWhitelist {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let removed_by: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { user, removed_by })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.removed_by, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserRemovedFromWhitelistEvent(pub UserRemovedFromWhitelist);
impl UserRemovedFromWhitelistEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_REMOVED_FROM_WHITELIST_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = UserRemovedFromWhitelist::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_REMOVED_FROM_WHITELIST_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
