use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
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
pub enum AmmCreatorFeeOn {
    #[default]
    QuoteToken,
    BothToken,
}
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
pub struct BondingCurveParam {
    pub migrate_type: u8,
    pub migrate_cpmm_fee_on: u8,
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}
impl BondingCurveParam {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let migrate_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_cpmm_fee_on: u8 = crate::borsh_de_or_default(&mut reader)?;
        let supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_base_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_fund_raising: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_locked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unlock_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            migrate_type,
            migrate_cpmm_fee_on,
            supply,
            total_base_sell,
            total_quote_fund_raising,
            total_locked_amount,
            cliff_period,
            unlock_period,
        })
    }
}
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
pub struct ConstantCurve {
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}
impl ConstantCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_base_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_fund_raising: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            supply,
            total_base_sell,
            total_quote_fund_raising,
            migrate_type,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum CurveParams {
    Constant { data: ConstantCurve },
    Fixed { data: FixedCurve },
    Linear { data: LinearCurve },
}
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
pub struct FixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}
impl FixedCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_fund_raising: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            supply,
            total_quote_fund_raising,
            migrate_type,
        })
    }
}
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
pub struct LinearCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}
impl LinearCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_fund_raising: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            supply,
            total_quote_fund_raising,
            migrate_type,
        })
    }
}
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
pub struct MigrateNftInfo {
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
}
impl MigrateNftInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let platform_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burn_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            platform_scale,
            creator_scale,
            burn_scale,
        })
    }
}
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
pub struct MintParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl MintParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            decimals,
            name,
            symbol,
            uri,
        })
    }
}
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
pub struct PlatformConfigInfo {
    pub fee_wallet: Pubkey,
    pub nft_wallet: Pubkey,
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
    pub transfer_fee_extension_auth: Pubkey,
    pub creator_fee_rate: u64,
    pub platform_vesting_scale: u64,
    pub vesting_wallet: Pubkey,
}
impl PlatformConfigInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let nft_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migrate_nft_info = if reader.is_empty() {
            Default::default()
        } else {
            <MigrateNftInfo>::deserialize(&mut reader)?
        };
        let fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let web: String = crate::borsh_de_or_default(&mut reader)?;
        let img: String = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_extension_auth: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let platform_vesting_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_wallet,
            nft_wallet,
            migrate_nft_info,
            fee_rate,
            name,
            web,
            img,
            transfer_fee_extension_auth,
            creator_fee_rate,
            platform_vesting_scale,
            vesting_wallet,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum PlatformConfigParam {
    FeeWallet(Pubkey),
    NftWallet(Pubkey),
    MigrateNftInfo(MigrateNftInfo),
    FeeRate(u64),
    Name(String),
    Web(String),
    Img(String),
    CpSwapConfig,
    AllInfo(PlatformConfigInfo),
    VestingWallet(Pubkey),
    PlatformVestingScale(u64),
    PlatformCpCreator(Pubkey),
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PlatformCurveParam {
    pub epoch: u64,
    pub index: u8,
    pub global_config: Pubkey,
    pub bonding_curve_param: BondingCurveParam,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u64; 50],
}
impl PlatformCurveParam {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let global_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bonding_curve_param = if reader.is_empty() {
            Default::default()
        } else {
            <BondingCurveParam>::deserialize(&mut reader)?
        };
        let padding = <[u64; 50] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            epoch,
            index,
            global_config,
            bonding_curve_param,
            padding,
        })
    }
}
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
pub struct PlatformParams {
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
    pub creator_fee_rate: u64,
    pub platform_vesting_scale: u64,
}
impl PlatformParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let migrate_nft_info = if reader.is_empty() {
            Default::default()
        } else {
            <MigrateNftInfo>::deserialize(&mut reader)?
        };
        let fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let web: String = crate::borsh_de_or_default(&mut reader)?;
        let img: String = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let platform_vesting_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            migrate_nft_info,
            fee_rate,
            name,
            web,
            img,
            creator_fee_rate,
            platform_vesting_scale,
        })
    }
}
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
pub enum PoolStatus {
    #[default]
    Fund,
    Migrate,
    Trade,
}
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
pub enum TradeDirection {
    #[default]
    Buy,
    Sell,
}
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
pub struct TransferFeeExtensionParams {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}
impl TransferFeeExtensionParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let transfer_fee_basis_points: u16 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            transfer_fee_basis_points,
            maximum_fee,
        })
    }
}
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
pub struct VestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}
impl VestingParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_locked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unlock_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_locked_amount,
            cliff_period,
            unlock_period,
        })
    }
}
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
pub struct VestingSchedule {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
    pub start_time: u64,
    pub allocated_share_amount: u64,
}
impl VestingSchedule {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_locked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unlock_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let allocated_share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_locked_amount,
            cliff_period,
            unlock_period,
            start_time,
            allocated_share_amount,
        })
    }
}
