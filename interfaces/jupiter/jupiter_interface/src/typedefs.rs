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
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
impl RemainingAccountsInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slices: Vec<RemainingAccountsSlice> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { slices })
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
pub struct RemainingAccountsSlice {
    pub accounts_type: u8,
    pub length: u8,
}
impl RemainingAccountsSlice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accounts_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let length: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { accounts_type, length })
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
pub enum AccountsType {
    #[default]
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}
impl TryFrom<u8> for AccountsType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::TransferHookA),
            1u8 => Ok(Self::TransferHookB),
            2u8 => Ok(Self::TransferHookReward),
            3u8 => Ok(Self::TransferHookInput),
            4u8 => Ok(Self::TransferHookIntermediate),
            5u8 => Ok(Self::TransferHookOutput),
            6u8 => Ok(Self::SupplementalTickArrays),
            7u8 => Ok(Self::SupplementalTickArraysOne),
            8u8 => Ok(Self::SupplementalTickArraysTwo),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum DefiTunaAccountsType {
    #[default]
    TransferHookA,
    TransferHookB,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}
impl TryFrom<u8> for DefiTunaAccountsType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::TransferHookA),
            1u8 => Ok(Self::TransferHookB),
            2u8 => Ok(Self::TransferHookInput),
            3u8 => Ok(Self::TransferHookIntermediate),
            4u8 => Ok(Self::TransferHookOutput),
            5u8 => Ok(Self::SupplementalTickArrays),
            6u8 => Ok(Self::SupplementalTickArraysOne),
            7u8 => Ok(Self::SupplementalTickArraysTwo),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}
impl RoutePlanStep {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap: Swap = crate::borsh_de_or_default(&mut reader)?;
        let percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let input_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let output_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            swap,
            percent,
            input_index,
            output_index,
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
pub struct RoutePlanStepV2 {
    pub swap: Swap,
    pub bps: u16,
    pub input_index: u8,
    pub output_index: u8,
}
impl RoutePlanStepV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap: Swap = crate::borsh_de_or_default(&mut reader)?;
        let bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let input_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let output_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            swap,
            bps,
            input_index,
            output_index,
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
pub enum Side {
    #[default]
    Bid,
    Ask,
}
impl TryFrom<u8> for Side {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Bid),
            1u8 => Ok(Self::Ask),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum BisonFiPredictSide {
    #[default]
    Yes,
    No,
}
impl TryFrom<u8> for BisonFiPredictSide {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Yes),
            1u8 => Ok(Self::No),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum Swap {
    #[default]
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema { a_to_b: bool },
    Lifinity,
    Mercurial,
    Cykura,
    Serum { side: Side },
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin { side: Side },
    AldrinV2 { side: Side },
    Whirlpool { a_to_b: bool },
    Invariant { x_to_y: bool },
    Meteora,
    GooseFx,
    DeltaFi { stable: bool },
    Balansol,
    MarcoPolo { x_to_y: bool },
    Dradex { side: Side },
    LifinityV2,
    RaydiumClmm,
    Openbook { side: Side },
    Phoenix { side: Side },
    Symmetry { from_token_id: u64, to_token_id: u64 },
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake { bridge_stake_seed: u32 },
    GooseFxv2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2 { side: Side },
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake { bridge_stake_seed: u32 },
    Clone { pool_index: u8, quantity_is_input: bool, quantity_is_collateral: bool },
    SanctumS {
        src_lst_value_calc_accs: u8,
        dst_lst_value_calc_accs: u8,
        src_lst_index: u32,
        dst_lst_index: u32,
    },
    SanctumSAddLiquidity { lst_value_calc_accs: u8, lst_index: u32 },
    SanctumSRemoveLiquidity { lst_value_calc_accs: u8, lst_index: u32 },
    RaydiumCp,
    WhirlpoolSwapV2 {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    OneIntro,
    PumpWrappedBuy,
    PumpWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    Obric { x_to_y: bool },
    FoxBuyFromEstimatedCost,
    FoxClaimPartial { is_y: bool },
    SolFi { is_quote_to_base: bool },
    SolayerDelegateNoInit,
    SolayerUndelegateNoInit,
    TokenMill { side: Side },
    DaosFunBuy,
    DaosFunSell,
    ZeroFi,
    StakeDexWithdrawWrappedSol,
    VirtualsBuy,
    VirtualsSell,
    Perena { in_index: u8, out_index: u8 },
    PumpSwapBuy,
    PumpSwapSell,
    Gamma,
    MeteoraDlmmSwapV2 { remaining_accounts_info: RemainingAccountsInfo },
    Woofi,
    MeteoraDammV2,
    MeteoraDynamicBondingCurveSwap,
    StabbleStableSwapV2,
    StabbleWeightedSwapV2,
    RaydiumLaunchlabBuy { share_fee_rate: u64 },
    RaydiumLaunchlabSell { share_fee_rate: u64 },
    BoopdotfunWrappedBuy,
    BoopdotfunWrappedSell,
    Plasma { side: Side },
    GoonFi { is_bid: bool, blacklist_bump: u8 },
    HumidiFi { swap_id: u64, is_base_to_quote: bool },
    MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
    TesseraV { side: Side },
    PumpWrappedBuyV2,
    PumpWrappedSellV2,
    PumpSwapBuyV2,
    PumpSwapSellV2,
    Heaven { a_to_b: bool },
    SolFiV2 { is_quote_to_base: bool },
    Aquifer,
    PumpWrappedBuyV3,
    PumpWrappedSellV3,
    PumpSwapBuyV3,
    PumpSwapSellV3,
    JupiterLendDeposit,
    JupiterLendRedeem,
    DefiTuna { a_to_b: bool, remaining_accounts_info: Option<RemainingAccountsInfo> },
    AlphaQ { a_to_b: bool },
    RaydiumV2,
    SarosDlmm { swap_for_y: bool },
    Futarchy { side: Side },
    MeteoraDammV2WithRemainingAccounts,
    Obsidian,
    WhaleStreet { side: Side },
    DynamicV1 { candidate_swaps: Vec<CandidateSwap>, best_position: Option<u8> },
    PumpWrappedBuyV4,
    PumpWrappedSellV4,
    CarrotIssue,
    CarrotRedeem,
    Manifest { side: Side },
    BisonFi { a_to_b: bool },
    HumidiFiV2 { swap_id: u64, is_base_to_quote: bool },
    PerenaStar { is_mint: bool },
    JupiterRfqV2 { side: Side, fill_data: Vec<u8> },
    GoonFiV2 { is_bid: bool },
    Scorch { swap_id: u128 },
    VaultLiquidUnstake { lst_amounts: [u64; 5], seed: u64 },
    XOrca,
    Quantum { side: Side },
    WhaleStreetV2 { side: Side, auth_amount_in: u64, auth: u64 },
    Riptide { amount_is_token_a: bool },
    RunnerRodeo,
    TaurusFi { is_base_in: bool },
    Omnipair,
    MSwap,
    Hylo { swap_type: HyloSwapType },
    VoltrDeposit,
    VoltrWithdraw,
    SanctumSv2 {
        src_lst_value_calc_accs: u8,
        dst_lst_value_calc_accs: u8,
        src_lst_index: u32,
        dst_lst_index: u32,
    },
    LemmingsFi { is_base_in: bool },
    ScaleVmmBuy,
    ScaleVmmSell,
    ScaleAmmBuy,
    ScaleAmmSell,
    BisonFiV2 { a_to_b: bool },
    Trends,
    HumaDeposit,
    HumaInstantWithdraw,
    Kipseli { is_base_to_quote: bool },
    DynamicV2 {
        candidate_swaps: Vec<CandidateSwapWithBps>,
        max_split_quote_calls: u8,
        max_split_candidates: u8,
    },
    PumpSwapBuyV3WithCashbackClaim,
    PumpSwapSellV3WithCashbackClaim,
    PumpWrappedBuyV4WithCashbackClaim,
    PumpWrappedSellV4WithCashbackClaim,
    GoonFiV3 { is_bid: bool },
    PumpWrappedBuyV5 { claim_cashback: bool },
    PumpWrappedSellV5 { claim_cashback: bool },
    ZeroFiSwapV2,
    BisonFiPredict { side: BisonFiPredictSide, is_buy: bool },
    ByrealDynamicV3,
    Flux { swap_id: u64, base_to_quote: bool },
    VaultLiquidSellLst,
    VaultLiquidBuyLst { lst_amount: u64 },
    KipseliV2 { is_base_to_quote: bool },
    Deriverse { side: Side, instr_id: u32 },
    Hadron { is_x: bool },
    BinaryFi,
    Metric { zero_for_one: bool },
    JupiterLendDexSwap { swap0to1: bool },
    Gatorswap { base_to_quote: bool },
    Flint { is_global: bool, taker_buy: bool },
    Denali { base_to_quote: bool },
    PerenaStarV2Deposit,
    PerenaStarV2WithdrawFromExternal { external_liquidity_source: u8 },
    SanctumSols { swap_type: SanctumSolsSwapType },
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
pub struct CandidateSwapWithBps {
    pub candidate_swap: CandidateSwap,
    pub bps: u32,
}
impl CandidateSwapWithBps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let candidate_swap = <CandidateSwap as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { candidate_swap, bps })
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
pub enum CandidateSwap {
    HumidiFi { swap_id: u64, is_base_to_quote: bool },
    TesseraV { side: Side },
    HumidiFiV2 { swap_id: u64, is_base_to_quote: bool },
    RaydiumV2,
    RaydiumClmm,
    Whirlpool { a_to_b: bool },
    ZeroFi,
    BisonFiV2 { a_to_b: bool },
    GoonFiV2 { is_bid: bool },
    GoonFiV3 { is_bid: bool },
    WhirlpoolV2 { a_to_b: bool, remaining_accounts_info: Option<RemainingAccountsInfo> },
    ZeroFiSwapV2,
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
pub enum SanctumSolsSwapType {
    #[default]
    Mint,
    Claim,
    ClaimHolding,
}
impl TryFrom<u8> for SanctumSolsSwapType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Mint),
            1u8 => Ok(Self::Claim),
            2u8 => Ok(Self::ClaimHolding),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum HyloSwapType {
    #[default]
    MintStable,
    RedeemStable,
    MintLever,
    RedeemLever,
    SwapStableToLever,
    SwapLeverToStable,
    StabilityPoolDeposit,
    StabilityPoolWithdraw,
}
impl TryFrom<u8> for HyloSwapType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MintStable),
            1u8 => Ok(Self::RedeemStable),
            2u8 => Ok(Self::MintLever),
            3u8 => Ok(Self::RedeemLever),
            4u8 => Ok(Self::SwapStableToLever),
            5u8 => Ok(Self::SwapLeverToStable),
            6u8 => Ok(Self::StabilityPoolDeposit),
            7u8 => Ok(Self::StabilityPoolWithdraw),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub struct SwapEventV2 {
    pub input_mint: Pubkey,
    pub input_amount: u64,
    pub output_mint: Pubkey,
    pub output_amount: u64,
    pub amm: Pubkey,
}
impl SwapEventV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amm: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            input_mint,
            input_amount,
            output_mint,
            output_amount,
            amm,
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
pub enum CandidateSwapResult {
    OutAmount(u64),
    ProgramError(u64),
}
