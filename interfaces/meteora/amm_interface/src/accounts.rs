use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Config {
    pub pool_fees: PoolFees,
    pub activation_duration: u64,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub partner_fee_numerator: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 219],
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFees>::deserialize(&mut reader)?
        };
        let activation_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 219] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            activation_duration,
            vault_config_key,
            pool_creator_authority,
            activation_type,
            partner_fee_numerator,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccount(pub Config);
impl ConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Config::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOCK_ESCROW_ACCOUNT_DISCM: [u8; 8] = [190, 106, 121, 6, 200, 182, 21, 75];
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
pub struct LockEscrow {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub escrow_vault: Pubkey,
    pub bump: u8,
    pub total_locked_amount: u64,
    pub lp_per_token: u128,
    pub unclaimed_fee_pending: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}
impl LockEscrow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_locked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_per_token: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unclaimed_fee_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            escrow_vault,
            bump,
            total_locked_amount,
            lp_per_token,
            unclaimed_fee_pending,
            a_fee,
            b_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_locked_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_per_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unclaimed_fee_pending, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.b_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockEscrowAccount(pub LockEscrow);
impl LockEscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_ESCROW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LockEscrow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_ESCROW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPERATOR_ACCOUNT_DISCM: [u8; 8] = [219, 31, 188, 145, 69, 139, 204, 117];
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
pub struct Operator {
    pub whitelisted_address: Pubkey,
    pub permission: u128,
    pub padding: [u64; 2],
}
impl Operator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whitelisted_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whitelisted_address,
            permission,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whitelisted_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorAccount(pub Operator);
impl OperatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPERATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Operator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPERATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
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
pub struct Pool {
    pub lp_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_bump: u8,
    pub enabled: bool,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub fee_last_updated_at: u64,
    pub padding0: [u8; 24],
    pub fees: PoolFees,
    pub pool_type: PoolType,
    pub stake: Pubkey,
    pub total_locked_lp: u64,
    pub bootstrapping: Bootstrapping,
    pub partner_info: PartnerInfo,
    pub padding: Padding,
    pub curve_type: CurveType,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let a_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let b_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let a_vault_lp: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let b_vault_lp: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let a_vault_lp_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let protocol_token_a_fee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_token_b_fee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_last_updated_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 24] = crate::borsh_de_or_default(&mut reader)?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFees>::deserialize(&mut reader)?
        };
        let pool_type: PoolType = crate::borsh_de_or_default(&mut reader)?;
        let stake: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_locked_lp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bootstrapping = if reader.is_empty() {
            Default::default()
        } else {
            <Bootstrapping>::deserialize(&mut reader)?
        };
        let partner_info = if reader.is_empty() {
            Default::default()
        } else {
            <PartnerInfo>::deserialize(&mut reader)?
        };
        let padding = if reader.is_empty() {
            Default::default()
        } else {
            <Padding>::deserialize(&mut reader)?
        };
        let curve_type: CurveType = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_mint,
            token_a_mint,
            token_b_mint,
            a_vault,
            b_vault,
            a_vault_lp,
            b_vault_lp,
            a_vault_lp_bump,
            enabled,
            protocol_token_a_fee,
            protocol_token_b_fee,
            fee_last_updated_at,
            padding0,
            fees,
            pool_type,
            stake,
            total_locked_lp,
            bootstrapping,
            partner_info,
            padding,
            curve_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.b_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_vault_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.b_vault_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_vault_lp_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_token_a_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_token_b_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_last_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stake, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_locked_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bootstrapping, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_info, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
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
    pub enabled: u8,
    pub bumps: VaultBumps,
    pub total_amount: u64,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub token_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub strategies: [Pubkey; 30],
    pub base: Pubkey,
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub locked_profit_tracker: LockedProfitTracker,
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bumps = if reader.is_empty() {
            Default::default()
        } else {
            <VaultBumps>::deserialize(&mut reader)?
        };
        let total_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategies: [Pubkey; 30] = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let locked_profit_tracker = if reader.is_empty() {
            Default::default()
        } else {
            <LockedProfitTracker>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            enabled,
            bumps,
            total_amount,
            token_vault,
            fee_vault,
            token_mint,
            lp_mint,
            strategies,
            base,
            admin,
            operator,
            locked_profit_tracker,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategies, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_profit_tracker, &mut writer)?;
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
