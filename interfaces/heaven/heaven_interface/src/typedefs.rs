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
pub struct AddLiquidityParams {
    pub max_token_a: u64,
    pub max_token_b: u64,
    pub min_lp_tokens: u64,
}
impl AddLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_token_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_lp_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_token_a,
            max_token_b,
            min_lp_tokens,
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
pub enum AdminUpdateLiquidityPoolState {
    CreatorTradingFeeClaimStatus(CreatorTradingFeeClaimStatus),
    CreatorTradingFeeDistribution(CreatorTradingFeeDistribution),
    CreatorTradingFeeReceiver(Pubkey),
    FeeConfigurationMode(FeeConfigurationMode),
    SlotOffsetBasedFees { fee_type: FeeType, fees: SlotFeeBrackets },
    MarketCapBasedFees { fee_type: FeeType, fees: FeeBrackets },
    ToggleSwapPermission(bool),
    SetCreatorTradingFeeTradingVolumeThreshold(f64),
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
pub struct BuyParams {
    pub max_sol_spend: u64,
    pub minimum_amount_out: u64,
    pub encoded_user_defined_event_data: String,
}
impl BuyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_sol_spend: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let encoded_user_defined_event_data: String = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            max_sol_spend,
            minimum_amount_out,
            encoded_user_defined_event_data,
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
pub struct CreateProLiquidityPoolParams {
    pub encoded_user_defined_event_data: String,
    pub initial_token_a_amount: u64,
    pub initial_token_b_amount: u64,
    pub creator_trading_fee: FeeBracketsParams,
    pub reflection_trading_fee: Option<FeeBracketsParams>,
    pub liquidity_provider_trading_fee: Option<FeeBracketsParams>,
    pub creator_slot_trading_fee: Option<SlotFeeBracketsParams>,
    pub enable_same_slot_trading: bool,
    pub enable_sandwich_resistant_mode: bool,
    pub enable_deposit_liquidity: bool,
    pub enable_withdraw_liquidity: bool,
    pub enable_swap: bool,
    pub enable_update_creator_trading_fee: bool,
}
impl CreateProLiquidityPoolParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let encoded_user_defined_event_data: String = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeBracketsParams>::deserialize(&mut reader)?
        };
        let reflection_trading_fee: Option<FeeBracketsParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidity_provider_trading_fee: Option<FeeBracketsParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_slot_trading_fee: Option<SlotFeeBracketsParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let enable_same_slot_trading: bool = crate::borsh_de_or_default(&mut reader)?;
        let enable_sandwich_resistant_mode: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let enable_deposit_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let enable_withdraw_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let enable_swap: bool = crate::borsh_de_or_default(&mut reader)?;
        let enable_update_creator_trading_fee: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            encoded_user_defined_event_data,
            initial_token_a_amount,
            initial_token_b_amount,
            creator_trading_fee,
            reflection_trading_fee,
            liquidity_provider_trading_fee,
            creator_slot_trading_fee,
            enable_same_slot_trading,
            enable_sandwich_resistant_mode,
            enable_deposit_liquidity,
            enable_withdraw_liquidity,
            enable_swap,
            enable_update_creator_trading_fee,
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
pub struct CreateStandardLiquidityPoolParams {
    pub encoded_user_defined_event_data: String,
    pub initial_purchase_amount: u64,
    pub max_sol_spend: u64,
}
impl CreateStandardLiquidityPoolParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let encoded_user_defined_event_data: String = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_purchase_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_sol_spend: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            encoded_user_defined_event_data,
            initial_purchase_amount,
            max_sol_spend,
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
pub enum CreatorTradingFeeClaimStatus {
    #[default]
    Unclaimed,
    Submitted,
    Processed,
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
pub enum CreatorTradingFeeDistribution {
    #[default]
    Community,
    Creator,
    Blocked,
    Shared,
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
pub struct FeeBracket {
    pub market_cap_upper_bound: u64,
    pub buy_fee_bps: u32,
    pub sell_fee_bps: u32,
}
impl FeeBracket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_cap_upper_bound: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buy_fee_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let sell_fee_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market_cap_upper_bound,
            buy_fee_bps,
            sell_fee_bps,
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
pub struct FeeBrackets {
    pub brackets: [FeeBracket; 4],
    pub count: u8,
    pub _padding: [u8; 7],
}
impl FeeBrackets {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let brackets: [FeeBracket; 4] = crate::borsh_de_or_default(&mut reader)?;
        let count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { brackets, count, _padding })
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
pub struct FeeBracketsParams {
    pub brackets: Vec<FeeBracket>,
    pub count: u8,
}
impl FeeBracketsParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let brackets: Vec<FeeBracket> = crate::borsh_de_or_default(&mut reader)?;
        let count: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { brackets, count })
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
pub enum FeeConfigurationMode {
    #[default]
    Global,
    Local,
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
pub enum FeeType {
    #[default]
    ProtocolFee,
    LiquidityProviderFee,
    CreatorFee,
    CreatorFeeProtocolFee,
    ReflectionFee,
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
pub struct LiquidityPoolAllowlist {
    pub swap: u8,
    pub remove_liquidity: u8,
    pub deposit_liquidity: u8,
    pub same_slot_trading: u8,
    pub update_creator_trading_fee: u8,
    pub padding1: [u8; 2],
}
impl LiquidityPoolAllowlist {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap: u8 = crate::borsh_de_or_default(&mut reader)?;
        let remove_liquidity: u8 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_liquidity: u8 = crate::borsh_de_or_default(&mut reader)?;
        let same_slot_trading: u8 = crate::borsh_de_or_default(&mut reader)?;
        let update_creator_trading_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            swap,
            remove_liquidity,
            deposit_liquidity,
            same_slot_trading,
            update_creator_trading_fee,
            padding1,
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
pub struct LiquidityPoolFeatureFlags {
    pub sandwich_resistant_mode: u8,
    pub padding1: [u8; 7],
}
impl LiquidityPoolFeatureFlags {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sandwich_resistant_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sandwich_resistant_mode,
            padding1,
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
pub struct LiquidityPoolInfo {
    pub creator: Pubkey,
    pub update_authority: Pubkey,
    pub open_at: u64,
    pub created_at: u64,
    pub protocol_config_version: u16,
    pub r_type: u8,
    pub pool_authority_bump: u8,
    pub temp_sol_holder_bump: u8,
    pub _pad: [u8; 3],
}
impl LiquidityPoolInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let update_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_config_version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let r_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_authority_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let temp_sol_holder_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            update_authority,
            open_at,
            created_at,
            protocol_config_version,
            r_type,
            pool_authority_bump,
            temp_sol_holder_bump,
            _pad,
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
pub struct LiquidityPoolLpTokenInfo {
    pub supply: LiquidityPoolLpTokenSupply,
    pub decimals: u8,
    pub _pad: [u8; 7],
}
impl LiquidityPoolLpTokenInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let supply = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolLpTokenSupply>::deserialize(&mut reader)?
        };
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { supply, decimals, _pad })
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
pub struct LiquidityPoolLpTokenSupply {
    pub initial: u64,
    pub total: u64,
    pub unlocked: u64,
    pub locked: u64,
    pub burnt: u64,
}
impl LiquidityPoolLpTokenSupply {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initial: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unlocked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burnt: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initial,
            total,
            unlocked,
            locked,
            burnt,
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
pub struct LiquidityPoolMarketCapBasedFees {
    pub protocol_trading_fee: FeeBrackets,
    pub liquidity_provider_trading_fee: FeeBrackets,
    pub creator_trading_fee: FeeBrackets,
    pub creator_trading_fee_protocol_fee: FeeBrackets,
    pub reflection_trading_fee: FeeBrackets,
}
impl LiquidityPoolMarketCapBasedFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol_trading_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeBrackets>::deserialize(&mut reader)?
        };
        let liquidity_provider_trading_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeBrackets>::deserialize(&mut reader)?
        };
        let creator_trading_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeBrackets>::deserialize(&mut reader)?
        };
        let creator_trading_fee_protocol_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeBrackets>::deserialize(&mut reader)?
        };
        let reflection_trading_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FeeBrackets>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            protocol_trading_fee,
            liquidity_provider_trading_fee,
            creator_trading_fee,
            creator_trading_fee_protocol_fee,
            reflection_trading_fee,
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
pub struct LiquidityPoolReserve {
    pub token_a: u64,
    pub token_b: u64,
    pub snapshot_slot: u64,
    pub snapshot_a: u64,
    pub snapshot_b: u64,
    pub accumulated_fee_per_lp_token: u64,
    pub initial_a: u64,
    pub initial_b: u64,
    pub leader_slot_window: u8,
    pub _pad: [u8; 7],
}
impl LiquidityPoolReserve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let accumulated_fee_per_lp_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let leader_slot_window: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_a,
            token_b,
            snapshot_slot,
            snapshot_a,
            snapshot_b,
            accumulated_fee_per_lp_token,
            initial_a,
            initial_b,
            leader_slot_window,
            _pad,
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
pub struct LiquidityPoolSlotOffsetBasedFees {
    pub protocol_trading_fee: SlotFeeBrackets,
    pub liquidity_provider_trading_fee: SlotFeeBrackets,
    pub creator_trading_fee: SlotFeeBrackets,
    pub creator_trading_fee_protocol_fee: SlotFeeBrackets,
    pub reflection_trading_fee: SlotFeeBrackets,
    pub _pad: [u8; 6],
}
impl LiquidityPoolSlotOffsetBasedFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol_trading_fee = <SlotFeeBrackets>::deserialize(&mut reader)?;
        let liquidity_provider_trading_fee = <SlotFeeBrackets>::deserialize(
            &mut reader,
        )?;
        let creator_trading_fee = <SlotFeeBrackets>::deserialize(&mut reader)?;
        let creator_trading_fee_protocol_fee = <SlotFeeBrackets>::deserialize(
            &mut reader,
        )?;
        let reflection_trading_fee = <SlotFeeBrackets>::deserialize(&mut reader)?;
        let _pad: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol_trading_fee,
            liquidity_provider_trading_fee,
            creator_trading_fee,
            creator_trading_fee_protocol_fee,
            reflection_trading_fee,
            _pad,
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
pub struct LiquidityPoolTokenInfo {
    pub mint: Pubkey,
    pub decimals: u8,
    pub owner: Pubkey,
}
impl LiquidityPoolTokenInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mint, decimals, owner })
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
pub enum LiquidityPoolType {
    #[default]
    None,
    Pro,
    Standard,
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
pub struct ProtocolConfigParams {
    pub create_pool_fee: u64,
    pub allow_create_pool: bool,
    pub supported_pool_type: LiquidityPoolType,
    pub market_cap_based_fees: LiquidityPoolMarketCapBasedFees,
    pub initial_token_b_amount: f64,
    pub initial_token_a_amount: u64,
    pub default_leader_slot_window: u8,
    pub auto_staking_enabled: bool,
    pub sandwich_resistence_enabled: bool,
    pub buffer_bps: u16,
    pub auto_staking_threshold_bps: u16,
    pub token_a_decimals: u8,
    pub max_creator_trading_fee: u32,
    pub max_supply_per_wallet: u64,
    pub creator_trading_fee_trading_volume_threshold: f64,
    pub migration_market_cap_threshold: u16,
}
impl ProtocolConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let create_pool_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let allow_create_pool: bool = crate::borsh_de_or_default(&mut reader)?;
        let supported_pool_type: LiquidityPoolType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let market_cap_based_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityPoolMarketCapBasedFees>::deserialize(&mut reader)?
        };
        let initial_token_b_amount: f64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let default_leader_slot_window: u8 = crate::borsh_de_or_default(&mut reader)?;
        let auto_staking_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let sandwich_resistence_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let buffer_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let auto_staking_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_creator_trading_fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply_per_wallet: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fee_trading_volume_threshold: f64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let migration_market_cap_threshold: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            create_pool_fee,
            allow_create_pool,
            supported_pool_type,
            market_cap_based_fees,
            initial_token_b_amount,
            initial_token_a_amount,
            default_leader_slot_window,
            auto_staking_enabled,
            sandwich_resistence_enabled,
            buffer_bps,
            auto_staking_threshold_bps,
            token_a_decimals,
            max_creator_trading_fee,
            max_supply_per_wallet,
            creator_trading_fee_trading_volume_threshold,
            migration_market_cap_threshold,
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
pub struct RemoveLiquidityParams {
    pub lp_tokens_to_burn: u64,
    pub min_token_a: u64,
    pub min_token_b: u64,
}
impl RemoveLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_tokens_to_burn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_tokens_to_burn,
            min_token_a,
            min_token_b,
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
pub struct SellParams {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub encoded_user_defined_event_data: String,
}
impl SellParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let encoded_user_defined_event_data: String = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            minimum_amount_out,
            encoded_user_defined_event_data,
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
pub struct SlotFeeBracket {
    pub buy_fee_bps: u16,
    pub sell_fee_bps: u16,
    pub slot_offset_upperbound: u16,
}
impl SlotFeeBracket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let buy_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let sell_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let slot_offset_upperbound: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            buy_fee_bps,
            sell_fee_bps,
            slot_offset_upperbound,
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
pub struct SlotFeeBrackets {
    #[serde(with = "crate::big_array_serde")]
    pub brackets: [SlotFeeBracket; 42],
    pub max_slot_offset: u16,
    pub max_fee_bps: u16,
    pub count: u8,
    pub enabled: u8,
    pub _padding: [u8; 4],
}
impl SlotFeeBrackets {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let brackets = <[SlotFeeBracket; 42] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let max_slot_offset: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            brackets,
            max_slot_offset,
            max_fee_bps,
            count,
            enabled,
            _padding,
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
pub struct SlotFeeBracketsParams {
    pub brackets: Vec<SlotFeeBracket>,
    pub max_slot_offset: u16,
    pub max_fee_bps: u16,
    pub count: u8,
    pub enabled: u8,
}
impl SlotFeeBracketsParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let brackets: Vec<SlotFeeBracket> = crate::borsh_de_or_default(&mut reader)?;
        let max_slot_offset: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            brackets,
            max_slot_offset,
            max_fee_bps,
            count,
            enabled,
        })
    }
}
