use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LB_PAIR_ACCOUNT_DISCM: [u8; 8] = [33, 11, 49, 98, 181, 101, 177, 13];
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
pub struct LbPair {
    pub parameters: StaticParameters,
    pub v_parameters: VariableParameters,
    pub bump_seed: [u8; 1],
    pub bin_step_seed: [u8; 2],
    pub pair_type: u8,
    pub active_id: i32,
    pub bin_step: u16,
    pub status: u8,
    pub require_base_factor_seed: u8,
    pub base_factor_seed: [u8; 2],
    pub activation_type: u8,
    pub creator_pool_on_off_control: u8,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub protocol_fee: ProtocolFee,
    pub _padding_1: [u8; 32],
    pub reward_infos: [DlmmRewardInfo; 2],
    pub oracle: Pubkey,
    pub bin_array_bitmap: [u64; 16],
    pub last_updated_at: i64,
    pub _padding_2: [u8; 32],
    pub pre_activation_swap_address: Pubkey,
    pub base_key: Pubkey,
    pub activation_point: u64,
    pub pre_activation_duration: u64,
    pub _padding_3: [u8; 8],
    pub _padding_4: u64,
    pub creator: Pubkey,
    pub token_mint_x_program_flag: u8,
    pub token_mint_y_program_flag: u8,
    pub _reserved: [u8; 22],
}
impl LbPair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let parameters = if reader.is_empty() {
            Default::default()
        } else {
            <StaticParameters>::deserialize(&mut reader)?
        };
        let v_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <VariableParameters>::deserialize(&mut reader)?
        };
        let bump_seed: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_seed: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let pair_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let require_base_factor_seed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor_seed: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_pool_on_off_control: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_x_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee = if reader.is_empty() {
            Default::default()
        } else {
            <ProtocolFee>::deserialize(&mut reader)?
        };
        let _padding_1: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [DlmmRewardInfo; 2] = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_array_bitmap: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_2: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let pre_activation_swap_address: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pre_activation_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_3: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let _padding_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_x_program_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_y_program_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved: [u8; 22] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            parameters,
            v_parameters,
            bump_seed,
            bin_step_seed,
            pair_type,
            active_id,
            bin_step,
            status,
            require_base_factor_seed,
            base_factor_seed,
            activation_type,
            creator_pool_on_off_control,
            token_x_mint,
            token_y_mint,
            reserve_x,
            reserve_y,
            protocol_fee,
            _padding_1,
            reward_infos,
            oracle,
            bin_array_bitmap,
            last_updated_at,
            _padding_2,
            pre_activation_swap_address,
            base_key,
            activation_point,
            pre_activation_duration,
            _padding_3,
            _padding_4,
            creator,
            token_mint_x_program_flag,
            token_mint_y_program_flag,
            _reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.v_parameters, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pair_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.require_base_factor_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_factor_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_pool_on_off_control,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_x_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_array_bitmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_2, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.pre_activation_swap_address,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.base_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_activation_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_x_program_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_y_program_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LbPairAccount(pub LbPair);
impl LbPairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LB_PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LbPair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LB_PAIR_ACCOUNT_DISCM)?;
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
    pub pool_fees: PoolFeesStruct,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub whitelisted_vault: Pubkey,
    pub partner: Pubkey,
    pub liquidity: u128,
    pub _padding: u128,
    pub protocol_a_fee: u64,
    pub protocol_b_fee: u64,
    pub partner_a_fee: u64,
    pub partner_b_fee: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub activation_type: u8,
    pub pool_status: u8,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub collect_fee_mode: u8,
    pub pool_type: u8,
    pub version: u8,
    pub _padding_0: u8,
    pub fee_a_per_liquidity: [u8; 32],
    pub fee_b_per_liquidity: [u8; 32],
    pub permanent_lock_liquidity: u128,
    pub metrics: PoolMetrics,
    pub creator: Pubkey,
    pub _padding_1: [u64; 6],
    pub reward_infos: [CpAmmRewardInfo; 2],
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeesStruct>::deserialize(&mut reader)?
        };
        let token_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelisted_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: u128 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a_per_liquidity: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_per_liquidity: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let permanent_lock_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let metrics = if reader.is_empty() {
            Default::default()
        } else {
            <PoolMetrics>::deserialize(&mut reader)?
        };
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [u64; 6] = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [CpAmmRewardInfo; 2] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            token_a_mint,
            token_b_mint,
            token_a_vault,
            token_b_vault,
            whitelisted_vault,
            partner,
            liquidity,
            _padding,
            protocol_a_fee,
            protocol_b_fee,
            partner_a_fee,
            partner_b_fee,
            sqrt_min_price,
            sqrt_max_price,
            sqrt_price,
            activation_point,
            activation_type,
            pool_status,
            token_a_flag,
            token_b_flag,
            collect_fee_mode,
            pool_type,
            version,
            _padding_0,
            fee_a_per_liquidity,
            fee_b_per_liquidity,
            permanent_lock_liquidity,
            metrics,
            creator,
            _padding_1,
            reward_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelisted_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_a_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_b_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_a_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_b_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_min_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_max_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a_per_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_b_per_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permanent_lock_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metrics, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
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
pub const USER_LEDGER_ACCOUNT_DISCM: [u8; 8] = [185, 84, 101, 128, 8, 6, 160, 83];
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
pub struct UserLedger {
    pub owner: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
}
impl UserLedger {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owner, amount_a, amount_b })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserLedgerAccount(pub UserLedger);
impl UserLedgerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_LEDGER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserLedger::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_LEDGER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
