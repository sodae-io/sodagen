use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BIN_ARRAY_ACCOUNT_DISCM: [u8; 8] = [92, 142, 92, 220, 5, 148, 70, 181];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BinArray {
    pub index: i64,
    pub version: u8,
    pub _padding_1: [u8; 7],
    pub lb_pair: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub bins: [Bin; 70],
}
impl BinArray {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: i64 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bins = <[Bin; 70] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            index,
            version,
            _padding_1,
            lb_pair,
            bins,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bins, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinArrayAccount(pub BinArray);
impl BinArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BinArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BIN_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM: [u8; 8] = [
    80, 111, 124, 113, 55, 237, 18, 5,
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
pub struct BinArrayBitmapExtension {
    pub lb_pair: Pubkey,
    pub positive_bin_array_bitmap: [[u64; 8]; 12],
    pub negative_bin_array_bitmap: [[u64; 8]; 12],
}
impl BinArrayBitmapExtension {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let positive_bin_array_bitmap: [[u64; 8]; 12] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let negative_bin_array_bitmap: [[u64; 8]; 12] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            positive_bin_array_bitmap,
            negative_bin_array_bitmap,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.positive_bin_array_bitmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.negative_bin_array_bitmap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BinArrayBitmapExtensionAccount(pub BinArrayBitmapExtension);
impl BinArrayBitmapExtensionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BIN_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BinArrayBitmapExtension::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BIN_ARRAY_BITMAP_EXTENSION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CLAIM_FEE_OPERATOR_ACCOUNT_DISCM: [u8; 8] = [
    166, 48, 134, 86, 34, 200, 188, 150,
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
pub struct ClaimFeeOperator {
    pub operator: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub _padding: [u8; 128],
}
impl ClaimFeeOperator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { operator, _padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeOperatorAccount(pub ClaimFeeOperator);
impl ClaimFeeOperatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_OPERATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ClaimFeeOperator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_OPERATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DUMMY_ZC_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [94, 107, 238, 80, 208, 48, 180, 8];
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
pub struct DummyZcAccount {
    pub position_bin_data: PositionBinData,
    pub limit_order_bin_data: LimitOrderBinData,
}
impl DummyZcAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_bin_data = if reader.is_empty() {
            Default::default()
        } else {
            <PositionBinData>::deserialize(&mut reader)?
        };
        let limit_order_bin_data = if reader.is_empty() {
            Default::default()
        } else {
            <LimitOrderBinData>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            position_bin_data,
            limit_order_bin_data,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_bin_data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_order_bin_data, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DummyZcAccountAccount(pub DummyZcAccount);
impl DummyZcAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DUMMY_ZC_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DummyZcAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DUMMY_ZC_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
    pub reward_infos: [RewardInfo; 2],
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
    pub version: u8,
    pub _reserved: [u8; 21],
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
        let reward_infos: [RewardInfo; 2] = crate::borsh_de_or_default(&mut reader)?;
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
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved: [u8; 21] = crate::borsh_de_or_default(&mut reader)?;
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
            version,
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
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
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
pub const LIMIT_ORDER_ACCOUNT_DISCM: [u8; 8] = [137, 183, 212, 91, 115, 29, 141, 227];
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
pub struct LimitOrder {
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub bin_count: u16,
    pub _padding_0: [u8; 14],
    pub padding_1: [u64; 4],
}
impl LimitOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bin_count: u16 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: [u8; 14] = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            owner,
            bin_count,
            _padding_0,
            padding_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bin_count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LimitOrderAccount(pub LimitOrder);
impl LimitOrderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIMIT_ORDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LimitOrder::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIMIT_ORDER_ACCOUNT_DISCM)?;
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
    pub signer: Pubkey,
    pub permission: u128,
    pub padding: [u64; 2],
}
impl Operator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            signer,
            permission,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.signer, &mut writer)?;
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
pub const ORACLE_ACCOUNT_DISCM: [u8; 8] = [139, 194, 131, 179, 140, 179, 229, 244];
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
pub struct Oracle {
    pub idx: u64,
    pub active_size: u64,
    pub length: u64,
}
impl Oracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let idx: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let length: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { idx, active_size, length })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.idx, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.length, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OracleAccount(pub Oracle);
impl OracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Oracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_V2_ACCOUNT_DISCM: [u8; 8] = [117, 176, 212, 199, 245, 180, 133, 182];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PositionV2 {
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub liquidity_shares: [u128; 70],
    #[serde(with = "crate::big_array_serde")]
    pub reward_infos: [UserRewardInfo; 70],
    #[serde(with = "crate::big_array_serde")]
    pub fee_infos: [FeeInfo; 70],
    pub lower_bin_id: i32,
    pub upper_bin_id: i32,
    pub last_updated_at: i64,
    pub total_claimed_fee_x_amount: u64,
    pub total_claimed_fee_y_amount: u64,
    pub total_claimed_rewards: [u64; 2],
    pub operator: Pubkey,
    pub lock_release_point: u64,
    pub _padding_0: u8,
    pub fee_owner: Pubkey,
    pub version: u8,
    pub permissionless_operation_bits: u8,
    #[serde(with = "crate::big_array_serde")]
    pub _reserved: [u8; 85],
}
impl PositionV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lb_pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_shares = <[u128; 70] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let reward_infos = <[UserRewardInfo; 70] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let fee_infos = <[FeeInfo; 70] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let lower_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_fee_x_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_fee_y_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_rewards: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lock_release_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let permissionless_operation_bits: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved = <[u8; 85] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            lb_pair,
            owner,
            liquidity_shares,
            reward_infos,
            fee_infos,
            lower_bin_id,
            upper_bin_id,
            last_updated_at,
            total_claimed_fee_x_amount,
            total_claimed_fee_y_amount,
            total_claimed_rewards,
            operator,
            lock_release_point,
            _padding_0,
            fee_owner,
            version,
            permissionless_operation_bits,
            _reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.lb_pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_updated_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_fee_x_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_fee_y_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_rewards, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lock_release_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.permissionless_operation_bits,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self._reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionV2Account(pub PositionV2);
impl PositionV2Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_V2_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionV2::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_V2_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRESET_PARAMETER_ACCOUNT_DISCM: [u8; 8] = [
    242, 62, 244, 34, 181, 112, 58, 170,
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
pub struct PresetParameter {
    pub bin_step: u16,
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub protocol_share: u16,
}
impl PresetParameter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bin_step,
            base_factor,
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
            min_bin_id,
            max_bin_id,
            protocol_share,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filter_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decay_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduction_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.variable_fee_control, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_volatility_accumulator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_bin_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_share, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PresetParameterAccount(pub PresetParameter);
impl PresetParameterAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRESET_PARAMETER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PresetParameter::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRESET_PARAMETER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PRESET_PARAMETER2_ACCOUNT_DISCM: [u8; 8] = [
    171, 236, 148, 115, 162, 113, 222, 174,
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
pub struct PresetParameter2 {
    pub bin_step: u16,
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub reduction_factor: u16,
    pub protocol_share: u16,
    pub index: u16,
    pub base_fee_power_factor: u8,
    pub concrete_function_type: u8,
    pub collect_fee_mode: u8,
    pub padding_0: [u8; 7],
    pub padding_1: [u64; 19],
}
impl PresetParameter2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        let concrete_function_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u64; 19] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bin_step,
            base_factor,
            filter_period,
            decay_period,
            variable_fee_control,
            max_volatility_accumulator,
            reduction_factor,
            protocol_share,
            index,
            base_fee_power_factor,
            concrete_function_type,
            collect_fee_mode,
            padding_0,
            padding_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bin_step, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filter_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decay_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.variable_fee_control, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_volatility_accumulator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduction_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fee_power_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.concrete_function_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PresetParameter2Account(pub PresetParameter2);
impl PresetParameter2Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PRESET_PARAMETER2_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PresetParameter2::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PRESET_PARAMETER2_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenBadge {
    pub token_mint: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub _padding: [u8; 128],
}
impl TokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { token_mint, _padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenBadgeAccount(pub TokenBadge);
impl TokenBadgeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_BADGE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenBadge::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_BADGE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
