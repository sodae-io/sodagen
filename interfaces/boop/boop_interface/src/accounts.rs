use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const AMM_CONFIG_ACCOUNT_DISCM: [u8; 8] = [218, 244, 33, 104, 203, 203, 43, 111];
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
pub struct AmmConfig {
    pub bump: u8,
    pub disable_create_pool: bool,
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub protocol_owner: Pubkey,
    pub fund_owner: Pubkey,
    pub padding: [u64; 16],
}
impl AmmConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let disable_create_pool: bool = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fund_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let create_pool_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fund_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            disable_create_pool,
            index,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
            create_pool_fee,
            protocol_owner,
            fund_owner,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_create_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_pool_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fund_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmConfigAccount(pub AmmConfig);
impl AmmConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AmmConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BONDING_CURVE_ACCOUNT_DISCM: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];
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
pub struct BondingCurve {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub sol_reserves: u64,
    pub token_reserves: u64,
    pub damping_term: u8,
    pub swap_fee_basis_points: u8,
    pub token_for_stakers_basis_points: u16,
    pub status: BondingCurveStatus,
}
impl BondingCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let graduation_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let graduation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let damping_term: u8 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_basis_points: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_for_stakers_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let status: BondingCurveStatus = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            mint,
            virtual_sol_reserves,
            virtual_token_reserves,
            graduation_target,
            graduation_fee,
            sol_reserves,
            token_reserves,
            damping_term,
            swap_fee_basis_points,
            token_for_stakers_basis_points,
            status,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.graduation_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.graduation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.damping_term, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.token_for_stakers_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BondingCurveAccount(pub BondingCurve);
impl BondingCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BONDING_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BondingCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BONDING_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
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
pub struct Config {
    pub is_paused: bool,
    pub authority: Pubkey,
    pub pending_authority: Pubkey,
    pub operators: Vec<Pubkey>,
    pub protocol_fee_recipient: Pubkey,
    pub token_distributor: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub damping_term: u8,
    pub token_for_stakers_basis_points: u16,
    pub swap_fee_basis_points: u8,
    pub token_amount_for_raydium_liquidity: u64,
    pub max_graduation_price_deviation_basis_points: u16,
    pub max_swap_amount_for_pool_price_correction_basis_points: u16,
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operators: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_distributor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let graduation_target: u64 = crate::borsh_de_or_default(&mut reader)?;
        let graduation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let damping_term: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_for_stakers_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let swap_fee_basis_points: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_amount_for_raydium_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_graduation_price_deviation_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_swap_amount_for_pool_price_correction_basis_points: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            is_paused,
            authority,
            pending_authority,
            operators,
            protocol_fee_recipient,
            token_distributor,
            virtual_sol_reserves,
            virtual_token_reserves,
            graduation_target,
            graduation_fee,
            damping_term,
            token_for_stakers_basis_points,
            swap_fee_basis_points,
            token_amount_for_raydium_liquidity,
            max_graduation_price_deviation_basis_points,
            max_swap_amount_for_pool_price_correction_basis_points,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.is_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operators, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_distributor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.graduation_target, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.graduation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.damping_term, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.token_for_stakers_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.swap_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.token_amount_for_raydium_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_graduation_price_deviation_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_swap_amount_for_pool_price_correction_basis_points,
            &mut writer,
        )?;
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
pub const LOCKED_CP_LIQUIDITY_STATE_ACCOUNT_DISCM: [u8; 8] = [
    25, 10, 238, 197, 207, 234, 73, 22,
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
pub struct LockedCpLiquidityState {
    pub locked_lp_amount: u64,
    pub claimed_lp_amount: u64,
    pub unclaimed_lp_amount: u64,
    pub last_lp: u64,
    pub last_k: u128,
    pub recent_epoch: u64,
    pub pool_id: Pubkey,
    pub fee_nft_mint: Pubkey,
    pub locked_owner: Pubkey,
    pub locked_lp_mint: Pubkey,
    pub padding: [u64; 8],
}
impl LockedCpLiquidityState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let locked_lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let claimed_lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unclaimed_lp_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_lp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_k: u128 = crate::borsh_de_or_default(&mut reader)?;
        let recent_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let locked_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let locked_lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            locked_lp_amount,
            claimed_lp_amount,
            unclaimed_lp_amount,
            last_lp,
            last_k,
            recent_epoch,
            pool_id,
            fee_nft_mint,
            locked_owner,
            locked_lp_mint,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.locked_lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claimed_lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unclaimed_lp_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_k, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recent_epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockedCpLiquidityStateAccount(pub LockedCpLiquidityState);
impl LockedCpLiquidityStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCKED_CP_LIQUIDITY_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LockedCpLiquidityState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCKED_CP_LIQUIDITY_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
