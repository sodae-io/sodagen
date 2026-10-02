use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum HyloExchangeProgramIx {
    AcceptAddressUpdate(AcceptAddressUpdateIxArgs),
    ApproveAddressUpdate(ApproveAddressUpdateIxArgs),
    CancelAddressUpdate(CancelAddressUpdateIxArgs),
    ConvertLeverToStableExo(ConvertLeverToStableExoIxArgs),
    ConvertLeverToStableLst(ConvertLeverToStableLstIxArgs),
    ConvertStableToLeverExo(ConvertStableToLeverExoIxArgs),
    ConvertStableToLeverLst(ConvertStableToLeverLstIxArgs),
    GenesisMintExo(GenesisMintExoIxArgs),
    HarvestBorrowRate,
    HarvestYield,
    InitializeLstRegistry(InitializeLstRegistryIxArgs),
    InitializeLstRegistryCalculators,
    InitializeLstVirtualStablecoin,
    InitializeMints(InitializeMintsIxArgs),
    InitializePoolDrawdownExo,
    InitializePoolDrawdownLst,
    InitializeProtocol(InitializeProtocolIxArgs),
    InitializeUsdc(InitializeUsdcIxArgs),
    MintLevercoinExo(MintLevercoinExoIxArgs),
    MintLevercoinLst(MintLevercoinLstIxArgs),
    MintStablecoinExo(MintStablecoinExoIxArgs),
    MintStablecoinLst(MintStablecoinLstIxArgs),
    MintStablecoinUsdc(MintStablecoinUsdcIxArgs),
    PauseExoPair,
    PauseLstPair,
    PauseProtocol,
    PauseUsdcPair,
    ProposeAddressUpdate(ProposeAddressUpdateIxArgs),
    RedeemLevercoinExo(RedeemLevercoinExoIxArgs),
    RedeemLevercoinLst(RedeemLevercoinLstIxArgs),
    RedeemStablecoinExo(RedeemStablecoinExoIxArgs),
    RedeemStablecoinLst(RedeemStablecoinLstIxArgs),
    RedeemStablecoinUsdc(RedeemStablecoinUsdcIxArgs),
    RegisterExo(RegisterExoIxArgs),
    RegisterLst(RegisterLstIxArgs),
    SettleVirtualStablecoinExo,
    SettleVirtualStablecoinLst,
    SettleVirtualStablecoinUsdc,
    SwapExoToUsdc(SwapExoToUsdcIxArgs),
    SwapExoToUsdcAll(SwapExoToUsdcAllIxArgs),
    SwapLstToLst(SwapLstToLstIxArgs),
    SwapLstToUsdc(SwapLstToUsdcIxArgs),
    SwapLstToUsdcAll(SwapLstToUsdcAllIxArgs),
    SwapUsdcToExo(SwapUsdcToExoIxArgs),
    SwapUsdcToLst(SwapUsdcToLstIxArgs),
    UnpauseExoPair,
    UnpauseLstPair,
    UnpauseProtocol,
    UnpauseUsdcPair,
    UpdateExoBorrowRateCurve(UpdateExoBorrowRateCurveIxArgs),
    UpdateExoBorrowRateFee(UpdateExoBorrowRateFeeIxArgs),
    UpdateExoBuyCurve(UpdateExoBuyCurveIxArgs),
    UpdateExoLevercoinFees(UpdateExoLevercoinFeesIxArgs),
    UpdateExoLevercoinMarketCapLimit(UpdateExoLevercoinMarketCapLimitIxArgs),
    UpdateExoOracle(UpdateExoOracleIxArgs),
    UpdateExoOracleConfTolerance(UpdateExoOracleConfToleranceIxArgs),
    UpdateExoOracleInterval(UpdateExoOracleIntervalIxArgs),
    UpdateExoSellCurve(UpdateExoSellCurveIxArgs),
    UpdateExoStablecoinMintThreshold(UpdateExoStablecoinMintThresholdIxArgs),
    UpdateLevercoinFees(UpdateLevercoinFeesIxArgs),
    UpdateLstBuyCurveConfig(UpdateLstBuyCurveConfigIxArgs),
    UpdateLstPrices,
    UpdateLstRebalanceFee(UpdateLstRebalanceFeeIxArgs),
    UpdateLstSellCurveConfig(UpdateLstSellCurveConfigIxArgs),
    UpdateLstStablecoinMintThreshold(UpdateLstStablecoinMintThresholdIxArgs),
    UpdateLstSwapFee(UpdateLstSwapFeeIxArgs),
    UpdateOracleConfTolerance(UpdateOracleConfToleranceIxArgs),
    UpdateOracleInterval(UpdateOracleIntervalIxArgs),
    UpdateParTolerance(UpdateParToleranceIxArgs),
    UpdateSolUsdOracle(UpdateSolUsdOracleIxArgs),
    UpdateUsdcMintFee(UpdateUsdcMintFeeIxArgs),
    UpdateUsdcOracleConfTolerance(UpdateUsdcOracleConfToleranceIxArgs),
    UpdateUsdcOracleInterval(UpdateUsdcOracleIntervalIxArgs),
    UpdateUsdcRedeemFee(UpdateUsdcRedeemFeeIxArgs),
    UpdateYieldHarvestConfig(UpdateYieldHarvestConfigIxArgs),
    WithdrawFees,
}
impl HyloExchangeProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ACCEPT_ADDRESS_UPDATE_IX_DISCM) {
            let mut reader = &buf[ACCEPT_ADDRESS_UPDATE_IX_DISCM.len()..];
            let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AcceptAddressUpdate(AcceptAddressUpdateIxArgs {
                    address_field,
                }),
            );
        }
        if buf.starts_with(&APPROVE_ADDRESS_UPDATE_IX_DISCM) {
            let mut reader = &buf[APPROVE_ADDRESS_UPDATE_IX_DISCM.len()..];
            let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ApproveAddressUpdate(ApproveAddressUpdateIxArgs {
                    address_field,
                }),
            );
        }
        if buf.starts_with(&CANCEL_ADDRESS_UPDATE_IX_DISCM) {
            let mut reader = &buf[CANCEL_ADDRESS_UPDATE_IX_DISCM.len()..];
            let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CancelAddressUpdate(CancelAddressUpdateIxArgs {
                    address_field,
                }),
            );
        }
        if buf.starts_with(&CONVERT_LEVER_TO_STABLE_EXO_IX_DISCM) {
            let mut reader = &buf[CONVERT_LEVER_TO_STABLE_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConvertLeverToStableExo(ConvertLeverToStableExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&CONVERT_LEVER_TO_STABLE_LST_IX_DISCM) {
            let mut reader = &buf[CONVERT_LEVER_TO_STABLE_LST_IX_DISCM.len()..];
            let amount_levercoin: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConvertLeverToStableLst(ConvertLeverToStableLstIxArgs {
                    amount_levercoin,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&CONVERT_STABLE_TO_LEVER_EXO_IX_DISCM) {
            let mut reader = &buf[CONVERT_STABLE_TO_LEVER_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConvertStableToLeverExo(ConvertStableToLeverExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&CONVERT_STABLE_TO_LEVER_LST_IX_DISCM) {
            let mut reader = &buf[CONVERT_STABLE_TO_LEVER_LST_IX_DISCM.len()..];
            let amount_stablecoin: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::ConvertStableToLeverLst(ConvertStableToLeverLstIxArgs {
                    amount_stablecoin,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&GENESIS_MINT_EXO_IX_DISCM) {
            let mut reader = &buf[GENESIS_MINT_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::GenesisMintExo(GenesisMintExoIxArgs { amount }));
        }
        if buf.starts_with(&HARVEST_BORROW_RATE_IX_DISCM) {
            return Ok(Self::HarvestBorrowRate);
        }
        if buf.starts_with(&HARVEST_YIELD_IX_DISCM) {
            return Ok(Self::HarvestYield);
        }
        if buf.starts_with(&INITIALIZE_LST_REGISTRY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_LST_REGISTRY_IX_DISCM.len()..];
            let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeLstRegistry(InitializeLstRegistryIxArgs {
                    slot,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_LST_REGISTRY_CALCULATORS_IX_DISCM) {
            return Ok(Self::InitializeLstRegistryCalculators);
        }
        if buf.starts_with(&INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_DISCM) {
            return Ok(Self::InitializeLstVirtualStablecoin);
        }
        if buf.starts_with(&INITIALIZE_MINTS_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_MINTS_IX_DISCM.len()..];
            let stablecoin_metadata = if reader.is_empty() {
                Default::default()
            } else {
                <TokenMetadata>::deserialize(&mut reader)?
            };
            let levercoin_metadata = if reader.is_empty() {
                Default::default()
            } else {
                <TokenMetadata>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeMints(InitializeMintsIxArgs {
                    stablecoin_metadata,
                    levercoin_metadata,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_POOL_DRAWDOWN_EXO_IX_DISCM) {
            return Ok(Self::InitializePoolDrawdownExo);
        }
        if buf.starts_with(&INITIALIZE_POOL_DRAWDOWN_LST_IX_DISCM) {
            return Ok(Self::InitializePoolDrawdownLst);
        }
        if buf.starts_with(&INITIALIZE_PROTOCOL_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_PROTOCOL_IX_DISCM.len()..];
            let pause_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            let stablecoin_mint_threshold = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let levercoin_fees = if reader.is_empty() {
                Default::default()
            } else {
                <LevercoinFees>::deserialize(&mut reader)?
            };
            let yield_harvest_config = if reader.is_empty() {
                Default::default()
            } else {
                <YieldHarvestConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeProtocol(InitializeProtocolIxArgs {
                    pause_authority,
                    oracle_interval_secs,
                    stablecoin_mint_threshold,
                    levercoin_fees,
                    yield_harvest_config,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_USDC_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_USDC_IX_DISCM.len()..];
            let mint_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let redeem_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            let oracle_conf_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let par_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeUsdc(InitializeUsdcIxArgs {
                    mint_fee,
                    redeem_fee,
                    oracle_interval_secs,
                    oracle_conf_tolerance,
                    par_tolerance,
                }),
            );
        }
        if buf.starts_with(&MINT_LEVERCOIN_EXO_IX_DISCM) {
            let mut reader = &buf[MINT_LEVERCOIN_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MintLevercoinExo(MintLevercoinExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&MINT_LEVERCOIN_LST_IX_DISCM) {
            let mut reader = &buf[MINT_LEVERCOIN_LST_IX_DISCM.len()..];
            let amount_lst_to_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MintLevercoinLst(MintLevercoinLstIxArgs {
                    amount_lst_to_deposit,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&MINT_STABLECOIN_EXO_IX_DISCM) {
            let mut reader = &buf[MINT_STABLECOIN_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MintStablecoinExo(MintStablecoinExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&MINT_STABLECOIN_LST_IX_DISCM) {
            let mut reader = &buf[MINT_STABLECOIN_LST_IX_DISCM.len()..];
            let amount_lst_to_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MintStablecoinLst(MintStablecoinLstIxArgs {
                    amount_lst_to_deposit,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&MINT_STABLECOIN_USDC_IX_DISCM) {
            let mut reader = &buf[MINT_STABLECOIN_USDC_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::MintStablecoinUsdc(MintStablecoinUsdcIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&PAUSE_EXO_PAIR_IX_DISCM) {
            return Ok(Self::PauseExoPair);
        }
        if buf.starts_with(&PAUSE_LST_PAIR_IX_DISCM) {
            return Ok(Self::PauseLstPair);
        }
        if buf.starts_with(&PAUSE_PROTOCOL_IX_DISCM) {
            return Ok(Self::PauseProtocol);
        }
        if buf.starts_with(&PAUSE_USDC_PAIR_IX_DISCM) {
            return Ok(Self::PauseUsdcPair);
        }
        if buf.starts_with(&PROPOSE_ADDRESS_UPDATE_IX_DISCM) {
            let mut reader = &buf[PROPOSE_ADDRESS_UPDATE_IX_DISCM.len()..];
            let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
            let ttl_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ProposeAddressUpdate(ProposeAddressUpdateIxArgs {
                    address_field,
                    ttl_secs,
                }),
            );
        }
        if buf.starts_with(&REDEEM_LEVERCOIN_EXO_IX_DISCM) {
            let mut reader = &buf[REDEEM_LEVERCOIN_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RedeemLevercoinExo(RedeemLevercoinExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&REDEEM_LEVERCOIN_LST_IX_DISCM) {
            let mut reader = &buf[REDEEM_LEVERCOIN_LST_IX_DISCM.len()..];
            let amount_to_redeem: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RedeemLevercoinLst(RedeemLevercoinLstIxArgs {
                    amount_to_redeem,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&REDEEM_STABLECOIN_EXO_IX_DISCM) {
            let mut reader = &buf[REDEEM_STABLECOIN_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RedeemStablecoinExo(RedeemStablecoinExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&REDEEM_STABLECOIN_LST_IX_DISCM) {
            let mut reader = &buf[REDEEM_STABLECOIN_LST_IX_DISCM.len()..];
            let amount_to_redeem: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RedeemStablecoinLst(RedeemStablecoinLstIxArgs {
                    amount_to_redeem,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&REDEEM_STABLECOIN_USDC_IX_DISCM) {
            let mut reader = &buf[REDEEM_STABLECOIN_USDC_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::RedeemStablecoinUsdc(RedeemStablecoinUsdcIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&REGISTER_EXO_IX_DISCM) {
            let mut reader = &buf[REGISTER_EXO_IX_DISCM.len()..];
            let oracle_feed_id: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
            let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            let oracle_conf_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let stablecoin_mint_threshold = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let borrow_rate_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <BorrowRateCurveConfig>::deserialize(&mut reader)?
            };
            let borrow_rate_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            let levercoin_fees = if reader.is_empty() {
                Default::default()
            } else {
                <LevercoinFees>::deserialize(&mut reader)?
            };
            let sell_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceCurveConfig>::deserialize(&mut reader)?
            };
            let buy_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceCurveConfig>::deserialize(&mut reader)?
            };
            let metadata = if reader.is_empty() {
                Default::default()
            } else {
                <TokenMetadata>::deserialize(&mut reader)?
            };
            let levercoin_market_cap_limit = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::RegisterExo(RegisterExoIxArgs {
                    oracle_feed_id,
                    oracle_interval_secs,
                    oracle_conf_tolerance,
                    stablecoin_mint_threshold,
                    borrow_rate_curve_config,
                    borrow_rate_fee,
                    levercoin_fees,
                    sell_curve_config,
                    buy_curve_config,
                    metadata,
                    levercoin_market_cap_limit,
                }),
            );
        }
        if buf.starts_with(&REGISTER_LST_IX_DISCM) {
            let mut reader = &buf[REGISTER_LST_IX_DISCM.len()..];
            let rebalance_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(Self::RegisterLst(RegisterLstIxArgs { rebalance_fee }));
        }
        if buf.starts_with(&SETTLE_VIRTUAL_STABLECOIN_EXO_IX_DISCM) {
            return Ok(Self::SettleVirtualStablecoinExo);
        }
        if buf.starts_with(&SETTLE_VIRTUAL_STABLECOIN_LST_IX_DISCM) {
            return Ok(Self::SettleVirtualStablecoinLst);
        }
        if buf.starts_with(&SETTLE_VIRTUAL_STABLECOIN_USDC_IX_DISCM) {
            return Ok(Self::SettleVirtualStablecoinUsdc);
        }
        if buf.starts_with(&SWAP_EXO_TO_USDC_IX_DISCM) {
            let mut reader = &buf[SWAP_EXO_TO_USDC_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapExoToUsdc(SwapExoToUsdcIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&SWAP_EXO_TO_USDC_ALL_IX_DISCM) {
            let mut reader = &buf[SWAP_EXO_TO_USDC_ALL_IX_DISCM.len()..];
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapExoToUsdcAll(SwapExoToUsdcAllIxArgs {
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&SWAP_LST_TO_LST_IX_DISCM) {
            let mut reader = &buf[SWAP_LST_TO_LST_IX_DISCM.len()..];
            let amount_lst_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapLstToLst(SwapLstToLstIxArgs {
                    amount_lst_a,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&SWAP_LST_TO_USDC_IX_DISCM) {
            let mut reader = &buf[SWAP_LST_TO_USDC_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapLstToUsdc(SwapLstToUsdcIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&SWAP_LST_TO_USDC_ALL_IX_DISCM) {
            let mut reader = &buf[SWAP_LST_TO_USDC_ALL_IX_DISCM.len()..];
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapLstToUsdcAll(SwapLstToUsdcAllIxArgs {
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&SWAP_USDC_TO_EXO_IX_DISCM) {
            let mut reader = &buf[SWAP_USDC_TO_EXO_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapUsdcToExo(SwapUsdcToExoIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&SWAP_USDC_TO_LST_IX_DISCM) {
            let mut reader = &buf[SWAP_USDC_TO_LST_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SwapUsdcToLst(SwapUsdcToLstIxArgs {
                    amount,
                    slippage_config,
                }),
            );
        }
        if buf.starts_with(&UNPAUSE_EXO_PAIR_IX_DISCM) {
            return Ok(Self::UnpauseExoPair);
        }
        if buf.starts_with(&UNPAUSE_LST_PAIR_IX_DISCM) {
            return Ok(Self::UnpauseLstPair);
        }
        if buf.starts_with(&UNPAUSE_PROTOCOL_IX_DISCM) {
            return Ok(Self::UnpauseProtocol);
        }
        if buf.starts_with(&UNPAUSE_USDC_PAIR_IX_DISCM) {
            return Ok(Self::UnpauseUsdcPair);
        }
        if buf.starts_with(&UPDATE_EXO_BORROW_RATE_CURVE_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_BORROW_RATE_CURVE_IX_DISCM.len()..];
            let new_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <BorrowRateCurveConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoBorrowRateCurve(UpdateExoBorrowRateCurveIxArgs {
                    new_curve_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_BORROW_RATE_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_BORROW_RATE_FEE_IX_DISCM.len()..];
            let new_borrow_rate_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoBorrowRateFee(UpdateExoBorrowRateFeeIxArgs {
                    new_borrow_rate_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_BUY_CURVE_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_BUY_CURVE_IX_DISCM.len()..];
            let new_buy_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceCurveConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoBuyCurve(UpdateExoBuyCurveIxArgs {
                    new_buy_curve_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_LEVERCOIN_FEES_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_LEVERCOIN_FEES_IX_DISCM.len()..];
            let new_levercoin_fees = if reader.is_empty() {
                Default::default()
            } else {
                <LevercoinFees>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoLevercoinFees(UpdateExoLevercoinFeesIxArgs {
                    new_levercoin_fees,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_DISCM
                .len()..];
            let new_levercoin_market_cap_limit = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoLevercoinMarketCapLimit(UpdateExoLevercoinMarketCapLimitIxArgs {
                    new_levercoin_market_cap_limit,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_ORACLE_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_ORACLE_IX_DISCM.len()..];
            let new_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateExoOracle(UpdateExoOracleIxArgs {
                    new_oracle,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_DISCM.len()..];
            let new_oracle_conf_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoOracleConfTolerance(UpdateExoOracleConfToleranceIxArgs {
                    new_oracle_conf_tolerance,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_ORACLE_INTERVAL_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_ORACLE_INTERVAL_IX_DISCM.len()..];
            let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateExoOracleInterval(UpdateExoOracleIntervalIxArgs {
                    new_oracle_interval_secs,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_SELL_CURVE_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_SELL_CURVE_IX_DISCM.len()..];
            let new_sell_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceCurveConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoSellCurve(UpdateExoSellCurveIxArgs {
                    new_sell_curve_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_DISCM) {
            let mut reader = &buf[UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_DISCM.len()..];
            let new_stablecoin_mint_threshold = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateExoStablecoinMintThreshold(UpdateExoStablecoinMintThresholdIxArgs {
                    new_stablecoin_mint_threshold,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LEVERCOIN_FEES_IX_DISCM) {
            let mut reader = &buf[UPDATE_LEVERCOIN_FEES_IX_DISCM.len()..];
            let new_levercoin_fees = if reader.is_empty() {
                Default::default()
            } else {
                <LevercoinFees>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateLevercoinFees(UpdateLevercoinFeesIxArgs {
                    new_levercoin_fees,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LST_BUY_CURVE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_LST_BUY_CURVE_CONFIG_IX_DISCM.len()..];
            let new_buy_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceCurveConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateLstBuyCurveConfig(UpdateLstBuyCurveConfigIxArgs {
                    new_buy_curve_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LST_PRICES_IX_DISCM) {
            return Ok(Self::UpdateLstPrices);
        }
        if buf.starts_with(&UPDATE_LST_REBALANCE_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_LST_REBALANCE_FEE_IX_DISCM.len()..];
            let new_rebalance_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateLstRebalanceFee(UpdateLstRebalanceFeeIxArgs {
                    new_rebalance_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LST_SELL_CURVE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_LST_SELL_CURVE_CONFIG_IX_DISCM.len()..];
            let new_sell_curve_config = if reader.is_empty() {
                Default::default()
            } else {
                <RebalanceCurveConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateLstSellCurveConfig(UpdateLstSellCurveConfigIxArgs {
                    new_sell_curve_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_DISCM) {
            let mut reader = &buf[UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_DISCM.len()..];
            let new_stablecoin_mint_threshold = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateLstStablecoinMintThreshold(UpdateLstStablecoinMintThresholdIxArgs {
                    new_stablecoin_mint_threshold,
                }),
            );
        }
        if buf.starts_with(&UPDATE_LST_SWAP_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_LST_SWAP_FEE_IX_DISCM.len()..];
            let new_lst_swap_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateLstSwapFee(UpdateLstSwapFeeIxArgs {
                    new_lst_swap_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORACLE_CONF_TOLERANCE_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORACLE_CONF_TOLERANCE_IX_DISCM.len()..];
            let new_oracle_conf_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateOracleConfTolerance(UpdateOracleConfToleranceIxArgs {
                    new_oracle_conf_tolerance,
                }),
            );
        }
        if buf.starts_with(&UPDATE_ORACLE_INTERVAL_IX_DISCM) {
            let mut reader = &buf[UPDATE_ORACLE_INTERVAL_IX_DISCM.len()..];
            let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateOracleInterval(UpdateOracleIntervalIxArgs {
                    new_oracle_interval_secs,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PAR_TOLERANCE_IX_DISCM) {
            let mut reader = &buf[UPDATE_PAR_TOLERANCE_IX_DISCM.len()..];
            let new_par_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateParTolerance(UpdateParToleranceIxArgs {
                    new_par_tolerance,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SOL_USD_ORACLE_IX_DISCM) {
            let mut reader = &buf[UPDATE_SOL_USD_ORACLE_IX_DISCM.len()..];
            let new_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateSolUsdOracle(UpdateSolUsdOracleIxArgs {
                    new_oracle,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USDC_MINT_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_USDC_MINT_FEE_IX_DISCM.len()..];
            let new_mint_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateUsdcMintFee(UpdateUsdcMintFeeIxArgs {
                    new_mint_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_DISCM) {
            let mut reader = &buf[UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_DISCM.len()..];
            let new_oracle_conf_tolerance = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateUsdcOracleConfTolerance(UpdateUsdcOracleConfToleranceIxArgs {
                    new_oracle_conf_tolerance,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USDC_ORACLE_INTERVAL_IX_DISCM) {
            let mut reader = &buf[UPDATE_USDC_ORACLE_INTERVAL_IX_DISCM.len()..];
            let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateUsdcOracleInterval(UpdateUsdcOracleIntervalIxArgs {
                    new_oracle_interval_secs,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USDC_REDEEM_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_USDC_REDEEM_FEE_IX_DISCM.len()..];
            let new_redeem_fee = if reader.is_empty() {
                Default::default()
            } else {
                <UFixValue64>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateUsdcRedeemFee(UpdateUsdcRedeemFeeIxArgs {
                    new_redeem_fee,
                }),
            );
        }
        if buf.starts_with(&UPDATE_YIELD_HARVEST_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_YIELD_HARVEST_CONFIG_IX_DISCM.len()..];
            let new_yield_harvest_config = if reader.is_empty() {
                Default::default()
            } else {
                <YieldHarvestConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateYieldHarvestConfig(UpdateYieldHarvestConfigIxArgs {
                    new_yield_harvest_config,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FEES_IX_DISCM) {
            return Ok(Self::WithdrawFees);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AcceptAddressUpdate(args) => {
                writer.write_all(&ACCEPT_ADDRESS_UPDATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.address_field, &mut writer)?;
                Ok(())
            }
            Self::ApproveAddressUpdate(args) => {
                writer.write_all(&APPROVE_ADDRESS_UPDATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.address_field, &mut writer)?;
                Ok(())
            }
            Self::CancelAddressUpdate(args) => {
                writer.write_all(&CANCEL_ADDRESS_UPDATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.address_field, &mut writer)?;
                Ok(())
            }
            Self::ConvertLeverToStableExo(args) => {
                writer.write_all(&CONVERT_LEVER_TO_STABLE_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::ConvertLeverToStableLst(args) => {
                writer.write_all(&CONVERT_LEVER_TO_STABLE_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_levercoin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::ConvertStableToLeverExo(args) => {
                writer.write_all(&CONVERT_STABLE_TO_LEVER_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::ConvertStableToLeverLst(args) => {
                writer.write_all(&CONVERT_STABLE_TO_LEVER_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_stablecoin, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::GenesisMintExo(args) => {
                writer.write_all(&GENESIS_MINT_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::HarvestBorrowRate => writer.write_all(&HARVEST_BORROW_RATE_IX_DISCM),
            Self::HarvestYield => writer.write_all(&HARVEST_YIELD_IX_DISCM),
            Self::InitializeLstRegistry(args) => {
                writer.write_all(&INITIALIZE_LST_REGISTRY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.slot, &mut writer)?;
                Ok(())
            }
            Self::InitializeLstRegistryCalculators => {
                writer.write_all(&INITIALIZE_LST_REGISTRY_CALCULATORS_IX_DISCM)
            }
            Self::InitializeLstVirtualStablecoin => {
                writer.write_all(&INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_DISCM)
            }
            Self::InitializeMints(args) => {
                writer.write_all(&INITIALIZE_MINTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.stablecoin_metadata,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.levercoin_metadata, &mut writer)?;
                Ok(())
            }
            Self::InitializePoolDrawdownExo => {
                writer.write_all(&INITIALIZE_POOL_DRAWDOWN_EXO_IX_DISCM)
            }
            Self::InitializePoolDrawdownLst => {
                writer.write_all(&INITIALIZE_POOL_DRAWDOWN_LST_IX_DISCM)
            }
            Self::InitializeProtocol(args) => {
                writer.write_all(&INITIALIZE_PROTOCOL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pause_authority, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_interval_secs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.stablecoin_mint_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.levercoin_fees, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.yield_harvest_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeUsdc(args) => {
                writer.write_all(&INITIALIZE_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mint_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.redeem_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_interval_secs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_conf_tolerance,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.par_tolerance, &mut writer)?;
                Ok(())
            }
            Self::MintLevercoinExo(args) => {
                writer.write_all(&MINT_LEVERCOIN_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::MintLevercoinLst(args) => {
                writer.write_all(&MINT_LEVERCOIN_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.amount_lst_to_deposit,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::MintStablecoinExo(args) => {
                writer.write_all(&MINT_STABLECOIN_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::MintStablecoinLst(args) => {
                writer.write_all(&MINT_STABLECOIN_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.amount_lst_to_deposit,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::MintStablecoinUsdc(args) => {
                writer.write_all(&MINT_STABLECOIN_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::PauseExoPair => writer.write_all(&PAUSE_EXO_PAIR_IX_DISCM),
            Self::PauseLstPair => writer.write_all(&PAUSE_LST_PAIR_IX_DISCM),
            Self::PauseProtocol => writer.write_all(&PAUSE_PROTOCOL_IX_DISCM),
            Self::PauseUsdcPair => writer.write_all(&PAUSE_USDC_PAIR_IX_DISCM),
            Self::ProposeAddressUpdate(args) => {
                writer.write_all(&PROPOSE_ADDRESS_UPDATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.address_field, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.ttl_secs, &mut writer)?;
                Ok(())
            }
            Self::RedeemLevercoinExo(args) => {
                writer.write_all(&REDEEM_LEVERCOIN_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::RedeemLevercoinLst(args) => {
                writer.write_all(&REDEEM_LEVERCOIN_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_to_redeem, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::RedeemStablecoinExo(args) => {
                writer.write_all(&REDEEM_STABLECOIN_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::RedeemStablecoinLst(args) => {
                writer.write_all(&REDEEM_STABLECOIN_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_to_redeem, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::RedeemStablecoinUsdc(args) => {
                writer.write_all(&REDEEM_STABLECOIN_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::RegisterExo(args) => {
                writer.write_all(&REGISTER_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.oracle_feed_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_interval_secs,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_conf_tolerance,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.stablecoin_mint_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.borrow_rate_curve_config,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.borrow_rate_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.levercoin_fees, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.sell_curve_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.buy_curve_config, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.metadata, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.levercoin_market_cap_limit,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::RegisterLst(args) => {
                writer.write_all(&REGISTER_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.rebalance_fee, &mut writer)?;
                Ok(())
            }
            Self::SettleVirtualStablecoinExo => {
                writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_EXO_IX_DISCM)
            }
            Self::SettleVirtualStablecoinLst => {
                writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_LST_IX_DISCM)
            }
            Self::SettleVirtualStablecoinUsdc => {
                writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_USDC_IX_DISCM)
            }
            Self::SwapExoToUsdc(args) => {
                writer.write_all(&SWAP_EXO_TO_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::SwapExoToUsdcAll(args) => {
                writer.write_all(&SWAP_EXO_TO_USDC_ALL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::SwapLstToLst(args) => {
                writer.write_all(&SWAP_LST_TO_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_lst_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::SwapLstToUsdc(args) => {
                writer.write_all(&SWAP_LST_TO_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::SwapLstToUsdcAll(args) => {
                writer.write_all(&SWAP_LST_TO_USDC_ALL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::SwapUsdcToExo(args) => {
                writer.write_all(&SWAP_USDC_TO_EXO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::SwapUsdcToLst(args) => {
                writer.write_all(&SWAP_USDC_TO_LST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.slippage_config, &mut writer)?;
                Ok(())
            }
            Self::UnpauseExoPair => writer.write_all(&UNPAUSE_EXO_PAIR_IX_DISCM),
            Self::UnpauseLstPair => writer.write_all(&UNPAUSE_LST_PAIR_IX_DISCM),
            Self::UnpauseProtocol => writer.write_all(&UNPAUSE_PROTOCOL_IX_DISCM),
            Self::UnpauseUsdcPair => writer.write_all(&UNPAUSE_USDC_PAIR_IX_DISCM),
            Self::UpdateExoBorrowRateCurve(args) => {
                writer.write_all(&UPDATE_EXO_BORROW_RATE_CURVE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_curve_config, &mut writer)?;
                Ok(())
            }
            Self::UpdateExoBorrowRateFee(args) => {
                writer.write_all(&UPDATE_EXO_BORROW_RATE_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_borrow_rate_fee,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateExoBuyCurve(args) => {
                writer.write_all(&UPDATE_EXO_BUY_CURVE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_buy_curve_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateExoLevercoinFees(args) => {
                writer.write_all(&UPDATE_EXO_LEVERCOIN_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_levercoin_fees, &mut writer)?;
                Ok(())
            }
            Self::UpdateExoLevercoinMarketCapLimit(args) => {
                writer.write_all(&UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_levercoin_market_cap_limit,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateExoOracle(args) => {
                writer.write_all(&UPDATE_EXO_ORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_oracle, &mut writer)?;
                Ok(())
            }
            Self::UpdateExoOracleConfTolerance(args) => {
                writer.write_all(&UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_oracle_conf_tolerance,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateExoOracleInterval(args) => {
                writer.write_all(&UPDATE_EXO_ORACLE_INTERVAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_oracle_interval_secs,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateExoSellCurve(args) => {
                writer.write_all(&UPDATE_EXO_SELL_CURVE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_sell_curve_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateExoStablecoinMintThreshold(args) => {
                writer.write_all(&UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_stablecoin_mint_threshold,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateLevercoinFees(args) => {
                writer.write_all(&UPDATE_LEVERCOIN_FEES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_levercoin_fees, &mut writer)?;
                Ok(())
            }
            Self::UpdateLstBuyCurveConfig(args) => {
                writer.write_all(&UPDATE_LST_BUY_CURVE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_buy_curve_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateLstPrices => writer.write_all(&UPDATE_LST_PRICES_IX_DISCM),
            Self::UpdateLstRebalanceFee(args) => {
                writer.write_all(&UPDATE_LST_REBALANCE_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_rebalance_fee, &mut writer)?;
                Ok(())
            }
            Self::UpdateLstSellCurveConfig(args) => {
                writer.write_all(&UPDATE_LST_SELL_CURVE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_sell_curve_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateLstStablecoinMintThreshold(args) => {
                writer.write_all(&UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_stablecoin_mint_threshold,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateLstSwapFee(args) => {
                writer.write_all(&UPDATE_LST_SWAP_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_lst_swap_fee, &mut writer)?;
                Ok(())
            }
            Self::UpdateOracleConfTolerance(args) => {
                writer.write_all(&UPDATE_ORACLE_CONF_TOLERANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_oracle_conf_tolerance,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateOracleInterval(args) => {
                writer.write_all(&UPDATE_ORACLE_INTERVAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_oracle_interval_secs,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateParTolerance(args) => {
                writer.write_all(&UPDATE_PAR_TOLERANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_par_tolerance, &mut writer)?;
                Ok(())
            }
            Self::UpdateSolUsdOracle(args) => {
                writer.write_all(&UPDATE_SOL_USD_ORACLE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_oracle, &mut writer)?;
                Ok(())
            }
            Self::UpdateUsdcMintFee(args) => {
                writer.write_all(&UPDATE_USDC_MINT_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_mint_fee, &mut writer)?;
                Ok(())
            }
            Self::UpdateUsdcOracleConfTolerance(args) => {
                writer.write_all(&UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_oracle_conf_tolerance,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateUsdcOracleInterval(args) => {
                writer.write_all(&UPDATE_USDC_ORACLE_INTERVAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_oracle_interval_secs,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateUsdcRedeemFee(args) => {
                writer.write_all(&UPDATE_USDC_REDEEM_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_redeem_fee, &mut writer)?;
                Ok(())
            }
            Self::UpdateYieldHarvestConfig(args) => {
                writer.write_all(&UPDATE_YIELD_HARVEST_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.new_yield_harvest_config,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::WithdrawFees => writer.write_all(&WITHDRAW_FEES_IX_DISCM),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AcceptAddressUpdateAccounts<'me, 'info> {
    pub new_address: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptAddressUpdateKeys {
    pub new_address: Pubkey,
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub proposal: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AcceptAddressUpdateAccounts<'_, '_>> for AcceptAddressUpdateKeys {
    fn from(accounts: AcceptAddressUpdateAccounts) -> Self {
        Self {
            new_address: *accounts.new_address.key,
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            proposal: *accounts.proposal.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AcceptAddressUpdateKeys>
for [AccountMeta; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptAddressUpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.new_address,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]> for AcceptAddressUpdateKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            new_address: pubkeys[0],
            admin: pubkeys[1],
            hylo: pubkeys[2],
            proposal: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<AcceptAddressUpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptAddressUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.new_address.clone(),
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.proposal.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]>
for AcceptAddressUpdateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_address: &arr[0],
            admin: &arr[1],
            hylo: &arr[2],
            proposal: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const ACCEPT_ADDRESS_UPDATE_IX_DISCM: [u8; 8usize] = [
    56, 65, 20, 139, 208, 230, 213, 188,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptAddressUpdateIxArgs {
    pub address_field: AddressField,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptAddressUpdateIxData(pub AcceptAddressUpdateIxArgs);
impl From<AcceptAddressUpdateIxArgs> for AcceptAddressUpdateIxData {
    fn from(args: AcceptAddressUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl AcceptAddressUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_ADDRESS_UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AcceptAddressUpdateIxArgs {
                address_field,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_ADDRESS_UPDATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.address_field, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_address_update_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptAddressUpdateKeys,
    args: AcceptAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: AcceptAddressUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn accept_address_update_ix(
    keys: AcceptAddressUpdateKeys,
    args: AcceptAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    accept_address_update_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn accept_address_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptAddressUpdateAccounts<'_, '_>,
    args: AcceptAddressUpdateIxArgs,
) -> ProgramResult {
    let keys: AcceptAddressUpdateKeys = accounts.into();
    let ix = accept_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_address_update_invoke(
    accounts: AcceptAddressUpdateAccounts<'_, '_>,
    args: AcceptAddressUpdateIxArgs,
) -> ProgramResult {
    accept_address_update_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn accept_address_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptAddressUpdateAccounts<'_, '_>,
    args: AcceptAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptAddressUpdateKeys = accounts.into();
    let ix = accept_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_address_update_invoke_signed(
    accounts: AcceptAddressUpdateAccounts<'_, '_>,
    args: AcceptAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_address_update_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn accept_address_update_verify_account_keys(
    accounts: AcceptAddressUpdateAccounts<'_, '_>,
    keys: AcceptAddressUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.new_address.key, keys.new_address),
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_address_update_verify_writable_privileges<'me, 'info>(
    accounts: AcceptAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.hylo, accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_address_update_verify_signer_privileges<'me, 'info>(
    accounts: AcceptAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.new_address] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_address_update_verify_account_privileges<'me, 'info>(
    accounts: AcceptAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_address_update_verify_writable_privileges(accounts)?;
    accept_address_update_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ApproveAddressUpdateAccounts<'me, 'info> {
    pub upgrade_authority: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub new_address: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
    pub hylo_exchange: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApproveAddressUpdateKeys {
    pub upgrade_authority: Pubkey,
    pub proposal: Pubkey,
    pub new_address: Pubkey,
    pub program_data: Pubkey,
    pub hylo_exchange: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ApproveAddressUpdateAccounts<'_, '_>> for ApproveAddressUpdateKeys {
    fn from(accounts: ApproveAddressUpdateAccounts) -> Self {
        Self {
            upgrade_authority: *accounts.upgrade_authority.key,
            proposal: *accounts.proposal.key,
            new_address: *accounts.new_address.key,
            program_data: *accounts.program_data.key,
            hylo_exchange: *accounts.hylo_exchange.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ApproveAddressUpdateKeys>
for [AccountMeta; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: ApproveAddressUpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.upgrade_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo_exchange,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]>
for ApproveAddressUpdateKeys {
    fn from(pubkeys: [Pubkey; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            upgrade_authority: pubkeys[0],
            proposal: pubkeys[1],
            new_address: pubkeys[2],
            program_data: pubkeys[3],
            hylo_exchange: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<ApproveAddressUpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ApproveAddressUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.upgrade_authority.clone(),
            accounts.proposal.clone(),
            accounts.new_address.clone(),
            accounts.program_data.clone(),
            accounts.hylo_exchange.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]>
for ApproveAddressUpdateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            upgrade_authority: &arr[0],
            proposal: &arr[1],
            new_address: &arr[2],
            program_data: &arr[3],
            hylo_exchange: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const APPROVE_ADDRESS_UPDATE_IX_DISCM: [u8; 8usize] = [
    35, 60, 186, 56, 181, 237, 165, 27,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApproveAddressUpdateIxArgs {
    pub address_field: AddressField,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveAddressUpdateIxData(pub ApproveAddressUpdateIxArgs);
impl From<ApproveAddressUpdateIxArgs> for ApproveAddressUpdateIxData {
    fn from(args: ApproveAddressUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl ApproveAddressUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPROVE_ADDRESS_UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ApproveAddressUpdateIxArgs {
                address_field,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPROVE_ADDRESS_UPDATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.address_field, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn approve_address_update_ix_with_program_id(
    program_id: Pubkey,
    keys: ApproveAddressUpdateKeys,
    args: ApproveAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; APPROVE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ApproveAddressUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_address_update_ix(
    keys: ApproveAddressUpdateKeys,
    args: ApproveAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    approve_address_update_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn approve_address_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ApproveAddressUpdateAccounts<'_, '_>,
    args: ApproveAddressUpdateIxArgs,
) -> ProgramResult {
    let keys: ApproveAddressUpdateKeys = accounts.into();
    let ix = approve_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn approve_address_update_invoke(
    accounts: ApproveAddressUpdateAccounts<'_, '_>,
    args: ApproveAddressUpdateIxArgs,
) -> ProgramResult {
    approve_address_update_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn approve_address_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ApproveAddressUpdateAccounts<'_, '_>,
    args: ApproveAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ApproveAddressUpdateKeys = accounts.into();
    let ix = approve_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn approve_address_update_invoke_signed(
    accounts: ApproveAddressUpdateAccounts<'_, '_>,
    args: ApproveAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    approve_address_update_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn approve_address_update_verify_account_keys(
    accounts: ApproveAddressUpdateAccounts<'_, '_>,
    keys: ApproveAddressUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.upgrade_authority.key, keys.upgrade_authority),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.new_address.key, keys.new_address),
        (*accounts.program_data.key, keys.program_data),
        (*accounts.hylo_exchange.key, keys.hylo_exchange),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn approve_address_update_verify_writable_privileges<'me, 'info>(
    accounts: ApproveAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn approve_address_update_verify_signer_privileges<'me, 'info>(
    accounts: ApproveAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.upgrade_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn approve_address_update_verify_account_privileges<'me, 'info>(
    accounts: ApproveAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    approve_address_update_verify_writable_privileges(accounts)?;
    approve_address_update_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelAddressUpdateAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelAddressUpdateKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub proposal: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CancelAddressUpdateAccounts<'_, '_>> for CancelAddressUpdateKeys {
    fn from(accounts: CancelAddressUpdateAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            proposal: *accounts.proposal.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CancelAddressUpdateKeys>
for [AccountMeta; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelAddressUpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]> for CancelAddressUpdateKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            proposal: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CancelAddressUpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelAddressUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.proposal.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]>
for CancelAddressUpdateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            proposal: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CANCEL_ADDRESS_UPDATE_IX_DISCM: [u8; 8usize] = [
    181, 122, 207, 111, 131, 188, 184, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelAddressUpdateIxArgs {
    pub address_field: AddressField,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAddressUpdateIxData(pub CancelAddressUpdateIxArgs);
impl From<CancelAddressUpdateIxArgs> for CancelAddressUpdateIxData {
    fn from(args: CancelAddressUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl CancelAddressUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ADDRESS_UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CancelAddressUpdateIxArgs {
                address_field,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ADDRESS_UPDATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.address_field, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_address_update_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelAddressUpdateKeys,
    args: CancelAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelAddressUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_address_update_ix(
    keys: CancelAddressUpdateKeys,
    args: CancelAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    cancel_address_update_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn cancel_address_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelAddressUpdateAccounts<'_, '_>,
    args: CancelAddressUpdateIxArgs,
) -> ProgramResult {
    let keys: CancelAddressUpdateKeys = accounts.into();
    let ix = cancel_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_address_update_invoke(
    accounts: CancelAddressUpdateAccounts<'_, '_>,
    args: CancelAddressUpdateIxArgs,
) -> ProgramResult {
    cancel_address_update_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn cancel_address_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelAddressUpdateAccounts<'_, '_>,
    args: CancelAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelAddressUpdateKeys = accounts.into();
    let ix = cancel_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_address_update_invoke_signed(
    accounts: CancelAddressUpdateAccounts<'_, '_>,
    args: CancelAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_address_update_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_address_update_verify_account_keys(
    accounts: CancelAddressUpdateAccounts<'_, '_>,
    keys: CancelAddressUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_address_update_verify_writable_privileges<'me, 'info>(
    accounts: CancelAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_address_update_verify_signer_privileges<'me, 'info>(
    accounts: CancelAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_address_update_verify_account_privileges<'me, 'info>(
    accounts: CancelAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_address_update_verify_writable_privileges(accounts)?;
    cancel_address_update_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct ConvertLeverToStableExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConvertLeverToStableExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub levercoin_auth: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ConvertLeverToStableExoAccounts<'_, '_>> for ConvertLeverToStableExoKeys {
    fn from(accounts: ConvertLeverToStableExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_mint: *accounts.collateral_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ConvertLeverToStableExoKeys>
for [AccountMeta; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: ConvertLeverToStableExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN]>
for ConvertLeverToStableExoKeys {
    fn from(pubkeys: [Pubkey; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            levercoin_auth: pubkeys[3],
            stablecoin_auth: pubkeys[4],
            vault_auth: pubkeys[5],
            fee_auth: pubkeys[6],
            collateral_vault: pubkeys[7],
            fee_vault: pubkeys[8],
            user_levercoin_ta: pubkeys[9],
            user_stablecoin_ta: pubkeys[10],
            stablecoin_mint: pubkeys[11],
            levercoin_mint: pubkeys[12],
            collateral_mint: pubkeys[13],
            collateral_usd_pyth_feed: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<ConvertLeverToStableExoAccounts<'_, 'info>>
for [AccountInfo<'info>; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConvertLeverToStableExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.levercoin_auth.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN]>
for ConvertLeverToStableExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            levercoin_auth: &arr[3],
            stablecoin_auth: &arr[4],
            vault_auth: &arr[5],
            fee_auth: &arr[6],
            collateral_vault: &arr[7],
            fee_vault: &arr[8],
            user_levercoin_ta: &arr[9],
            user_stablecoin_ta: &arr[10],
            stablecoin_mint: &arr[11],
            levercoin_mint: &arr[12],
            collateral_mint: &arr[13],
            collateral_usd_pyth_feed: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const CONVERT_LEVER_TO_STABLE_EXO_IX_DISCM: [u8; 8usize] = [
    75, 77, 23, 21, 3, 87, 20, 128,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertLeverToStableExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertLeverToStableExoIxData(pub ConvertLeverToStableExoIxArgs);
impl From<ConvertLeverToStableExoIxArgs> for ConvertLeverToStableExoIxData {
    fn from(args: ConvertLeverToStableExoIxArgs) -> Self {
        Self(args)
    }
}
impl ConvertLeverToStableExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_LEVER_TO_STABLE_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ConvertLeverToStableExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_LEVER_TO_STABLE_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn convert_lever_to_stable_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: ConvertLeverToStableExoKeys,
    args: ConvertLeverToStableExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONVERT_LEVER_TO_STABLE_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConvertLeverToStableExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_lever_to_stable_exo_ix(
    keys: ConvertLeverToStableExoKeys,
    args: ConvertLeverToStableExoIxArgs,
) -> std::io::Result<Instruction> {
    convert_lever_to_stable_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn convert_lever_to_stable_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConvertLeverToStableExoAccounts<'_, '_>,
    args: ConvertLeverToStableExoIxArgs,
) -> ProgramResult {
    let keys: ConvertLeverToStableExoKeys = accounts.into();
    let ix = convert_lever_to_stable_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn convert_lever_to_stable_exo_invoke(
    accounts: ConvertLeverToStableExoAccounts<'_, '_>,
    args: ConvertLeverToStableExoIxArgs,
) -> ProgramResult {
    convert_lever_to_stable_exo_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn convert_lever_to_stable_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConvertLeverToStableExoAccounts<'_, '_>,
    args: ConvertLeverToStableExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConvertLeverToStableExoKeys = accounts.into();
    let ix = convert_lever_to_stable_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn convert_lever_to_stable_exo_invoke_signed(
    accounts: ConvertLeverToStableExoAccounts<'_, '_>,
    args: ConvertLeverToStableExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    convert_lever_to_stable_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn convert_lever_to_stable_exo_verify_account_keys(
    accounts: ConvertLeverToStableExoAccounts<'_, '_>,
    keys: ConvertLeverToStableExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn convert_lever_to_stable_exo_verify_writable_privileges<'me, 'info>(
    accounts: ConvertLeverToStableExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.exo_pair,
        accounts.fee_vault,
        accounts.user_levercoin_ta,
        accounts.user_stablecoin_ta,
        accounts.stablecoin_mint,
        accounts.levercoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn convert_lever_to_stable_exo_verify_signer_privileges<'me, 'info>(
    accounts: ConvertLeverToStableExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn convert_lever_to_stable_exo_verify_account_privileges<'me, 'info>(
    accounts: ConvertLeverToStableExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    convert_lever_to_stable_exo_verify_writable_privileges(accounts)?;
    convert_lever_to_stable_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ConvertLeverToStableLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConvertLeverToStableLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub levercoin_mint: Pubkey,
    pub levercoin_auth: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ConvertLeverToStableLstAccounts<'_, '_>> for ConvertLeverToStableLstKeys {
    fn from(accounts: ConvertLeverToStableLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            fee_auth: *accounts.fee_auth.key,
            fee_vault: *accounts.fee_vault.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ConvertLeverToStableLstKeys>
for [AccountMeta; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: ConvertLeverToStableLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN]>
for ConvertLeverToStableLstKeys {
    fn from(pubkeys: [Pubkey; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            sol_usd_pyth_feed: pubkeys[2],
            stablecoin_mint: pubkeys[3],
            stablecoin_auth: pubkeys[4],
            fee_auth: pubkeys[5],
            fee_vault: pubkeys[6],
            user_stablecoin_ta: pubkeys[7],
            levercoin_mint: pubkeys[8],
            levercoin_auth: pubkeys[9],
            user_levercoin_ta: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ConvertLeverToStableLstAccounts<'_, 'info>>
for [AccountInfo<'info>; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConvertLeverToStableLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.levercoin_mint.clone(),
            accounts.levercoin_auth.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN]>
for ConvertLeverToStableLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            sol_usd_pyth_feed: &arr[2],
            stablecoin_mint: &arr[3],
            stablecoin_auth: &arr[4],
            fee_auth: &arr[5],
            fee_vault: &arr[6],
            user_stablecoin_ta: &arr[7],
            levercoin_mint: &arr[8],
            levercoin_auth: &arr[9],
            user_levercoin_ta: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CONVERT_LEVER_TO_STABLE_LST_IX_DISCM: [u8; 8usize] = [
    203, 251, 143, 181, 230, 117, 64, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertLeverToStableLstIxArgs {
    pub amount_levercoin: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertLeverToStableLstIxData(pub ConvertLeverToStableLstIxArgs);
impl From<ConvertLeverToStableLstIxArgs> for ConvertLeverToStableLstIxData {
    fn from(args: ConvertLeverToStableLstIxArgs) -> Self {
        Self(args)
    }
}
impl ConvertLeverToStableLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_LEVER_TO_STABLE_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_levercoin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ConvertLeverToStableLstIxArgs {
                amount_levercoin,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_LEVER_TO_STABLE_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_levercoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn convert_lever_to_stable_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: ConvertLeverToStableLstKeys,
    args: ConvertLeverToStableLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONVERT_LEVER_TO_STABLE_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConvertLeverToStableLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_lever_to_stable_lst_ix(
    keys: ConvertLeverToStableLstKeys,
    args: ConvertLeverToStableLstIxArgs,
) -> std::io::Result<Instruction> {
    convert_lever_to_stable_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn convert_lever_to_stable_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConvertLeverToStableLstAccounts<'_, '_>,
    args: ConvertLeverToStableLstIxArgs,
) -> ProgramResult {
    let keys: ConvertLeverToStableLstKeys = accounts.into();
    let ix = convert_lever_to_stable_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn convert_lever_to_stable_lst_invoke(
    accounts: ConvertLeverToStableLstAccounts<'_, '_>,
    args: ConvertLeverToStableLstIxArgs,
) -> ProgramResult {
    convert_lever_to_stable_lst_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn convert_lever_to_stable_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConvertLeverToStableLstAccounts<'_, '_>,
    args: ConvertLeverToStableLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConvertLeverToStableLstKeys = accounts.into();
    let ix = convert_lever_to_stable_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn convert_lever_to_stable_lst_invoke_signed(
    accounts: ConvertLeverToStableLstAccounts<'_, '_>,
    args: ConvertLeverToStableLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    convert_lever_to_stable_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn convert_lever_to_stable_lst_verify_account_keys(
    accounts: ConvertLeverToStableLstAccounts<'_, '_>,
    keys: ConvertLeverToStableLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn convert_lever_to_stable_lst_verify_writable_privileges<'me, 'info>(
    accounts: ConvertLeverToStableLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.hylo,
        accounts.stablecoin_mint,
        accounts.fee_vault,
        accounts.user_stablecoin_ta,
        accounts.levercoin_mint,
        accounts.user_levercoin_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn convert_lever_to_stable_lst_verify_signer_privileges<'me, 'info>(
    accounts: ConvertLeverToStableLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn convert_lever_to_stable_lst_verify_account_privileges<'me, 'info>(
    accounts: ConvertLeverToStableLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    convert_lever_to_stable_lst_verify_writable_privileges(accounts)?;
    convert_lever_to_stable_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct ConvertStableToLeverExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConvertStableToLeverExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub levercoin_auth: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ConvertStableToLeverExoAccounts<'_, '_>> for ConvertStableToLeverExoKeys {
    fn from(accounts: ConvertStableToLeverExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_mint: *accounts.collateral_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ConvertStableToLeverExoKeys>
for [AccountMeta; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: ConvertStableToLeverExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN]>
for ConvertStableToLeverExoKeys {
    fn from(pubkeys: [Pubkey; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            levercoin_auth: pubkeys[3],
            stablecoin_auth: pubkeys[4],
            vault_auth: pubkeys[5],
            fee_auth: pubkeys[6],
            collateral_vault: pubkeys[7],
            fee_vault: pubkeys[8],
            user_levercoin_ta: pubkeys[9],
            user_stablecoin_ta: pubkeys[10],
            stablecoin_mint: pubkeys[11],
            levercoin_mint: pubkeys[12],
            collateral_mint: pubkeys[13],
            collateral_usd_pyth_feed: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<ConvertStableToLeverExoAccounts<'_, 'info>>
for [AccountInfo<'info>; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConvertStableToLeverExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.levercoin_auth.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN]>
for ConvertStableToLeverExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            levercoin_auth: &arr[3],
            stablecoin_auth: &arr[4],
            vault_auth: &arr[5],
            fee_auth: &arr[6],
            collateral_vault: &arr[7],
            fee_vault: &arr[8],
            user_levercoin_ta: &arr[9],
            user_stablecoin_ta: &arr[10],
            stablecoin_mint: &arr[11],
            levercoin_mint: &arr[12],
            collateral_mint: &arr[13],
            collateral_usd_pyth_feed: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const CONVERT_STABLE_TO_LEVER_EXO_IX_DISCM: [u8; 8usize] = [
    223, 38, 252, 226, 12, 251, 194, 215,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertStableToLeverExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertStableToLeverExoIxData(pub ConvertStableToLeverExoIxArgs);
impl From<ConvertStableToLeverExoIxArgs> for ConvertStableToLeverExoIxData {
    fn from(args: ConvertStableToLeverExoIxArgs) -> Self {
        Self(args)
    }
}
impl ConvertStableToLeverExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_STABLE_TO_LEVER_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ConvertStableToLeverExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_STABLE_TO_LEVER_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn convert_stable_to_lever_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: ConvertStableToLeverExoKeys,
    args: ConvertStableToLeverExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONVERT_STABLE_TO_LEVER_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConvertStableToLeverExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_stable_to_lever_exo_ix(
    keys: ConvertStableToLeverExoKeys,
    args: ConvertStableToLeverExoIxArgs,
) -> std::io::Result<Instruction> {
    convert_stable_to_lever_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn convert_stable_to_lever_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConvertStableToLeverExoAccounts<'_, '_>,
    args: ConvertStableToLeverExoIxArgs,
) -> ProgramResult {
    let keys: ConvertStableToLeverExoKeys = accounts.into();
    let ix = convert_stable_to_lever_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn convert_stable_to_lever_exo_invoke(
    accounts: ConvertStableToLeverExoAccounts<'_, '_>,
    args: ConvertStableToLeverExoIxArgs,
) -> ProgramResult {
    convert_stable_to_lever_exo_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn convert_stable_to_lever_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConvertStableToLeverExoAccounts<'_, '_>,
    args: ConvertStableToLeverExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConvertStableToLeverExoKeys = accounts.into();
    let ix = convert_stable_to_lever_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn convert_stable_to_lever_exo_invoke_signed(
    accounts: ConvertStableToLeverExoAccounts<'_, '_>,
    args: ConvertStableToLeverExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    convert_stable_to_lever_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn convert_stable_to_lever_exo_verify_account_keys(
    accounts: ConvertStableToLeverExoAccounts<'_, '_>,
    keys: ConvertStableToLeverExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn convert_stable_to_lever_exo_verify_writable_privileges<'me, 'info>(
    accounts: ConvertStableToLeverExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.exo_pair,
        accounts.fee_vault,
        accounts.user_levercoin_ta,
        accounts.user_stablecoin_ta,
        accounts.stablecoin_mint,
        accounts.levercoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn convert_stable_to_lever_exo_verify_signer_privileges<'me, 'info>(
    accounts: ConvertStableToLeverExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn convert_stable_to_lever_exo_verify_account_privileges<'me, 'info>(
    accounts: ConvertStableToLeverExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    convert_stable_to_lever_exo_verify_writable_privileges(accounts)?;
    convert_stable_to_lever_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ConvertStableToLeverLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConvertStableToLeverLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub levercoin_mint: Pubkey,
    pub levercoin_auth: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ConvertStableToLeverLstAccounts<'_, '_>> for ConvertStableToLeverLstKeys {
    fn from(accounts: ConvertStableToLeverLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            fee_auth: *accounts.fee_auth.key,
            fee_vault: *accounts.fee_vault.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ConvertStableToLeverLstKeys>
for [AccountMeta; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: ConvertStableToLeverLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN]>
for ConvertStableToLeverLstKeys {
    fn from(pubkeys: [Pubkey; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            sol_usd_pyth_feed: pubkeys[2],
            stablecoin_mint: pubkeys[3],
            stablecoin_auth: pubkeys[4],
            fee_auth: pubkeys[5],
            fee_vault: pubkeys[6],
            user_stablecoin_ta: pubkeys[7],
            levercoin_mint: pubkeys[8],
            levercoin_auth: pubkeys[9],
            user_levercoin_ta: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ConvertStableToLeverLstAccounts<'_, 'info>>
for [AccountInfo<'info>; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: ConvertStableToLeverLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.levercoin_mint.clone(),
            accounts.levercoin_auth.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN]>
for ConvertStableToLeverLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            sol_usd_pyth_feed: &arr[2],
            stablecoin_mint: &arr[3],
            stablecoin_auth: &arr[4],
            fee_auth: &arr[5],
            fee_vault: &arr[6],
            user_stablecoin_ta: &arr[7],
            levercoin_mint: &arr[8],
            levercoin_auth: &arr[9],
            user_levercoin_ta: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CONVERT_STABLE_TO_LEVER_LST_IX_DISCM: [u8; 8usize] = [
    70, 153, 236, 141, 152, 102, 238, 157,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvertStableToLeverLstIxArgs {
    pub amount_stablecoin: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertStableToLeverLstIxData(pub ConvertStableToLeverLstIxArgs);
impl From<ConvertStableToLeverLstIxArgs> for ConvertStableToLeverLstIxData {
    fn from(args: ConvertStableToLeverLstIxArgs) -> Self {
        Self(args)
    }
}
impl ConvertStableToLeverLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONVERT_STABLE_TO_LEVER_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_stablecoin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(ConvertStableToLeverLstIxArgs {
                amount_stablecoin,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONVERT_STABLE_TO_LEVER_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_stablecoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn convert_stable_to_lever_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: ConvertStableToLeverLstKeys,
    args: ConvertStableToLeverLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONVERT_STABLE_TO_LEVER_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConvertStableToLeverLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_stable_to_lever_lst_ix(
    keys: ConvertStableToLeverLstKeys,
    args: ConvertStableToLeverLstIxArgs,
) -> std::io::Result<Instruction> {
    convert_stable_to_lever_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn convert_stable_to_lever_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConvertStableToLeverLstAccounts<'_, '_>,
    args: ConvertStableToLeverLstIxArgs,
) -> ProgramResult {
    let keys: ConvertStableToLeverLstKeys = accounts.into();
    let ix = convert_stable_to_lever_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn convert_stable_to_lever_lst_invoke(
    accounts: ConvertStableToLeverLstAccounts<'_, '_>,
    args: ConvertStableToLeverLstIxArgs,
) -> ProgramResult {
    convert_stable_to_lever_lst_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn convert_stable_to_lever_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConvertStableToLeverLstAccounts<'_, '_>,
    args: ConvertStableToLeverLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConvertStableToLeverLstKeys = accounts.into();
    let ix = convert_stable_to_lever_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn convert_stable_to_lever_lst_invoke_signed(
    accounts: ConvertStableToLeverLstAccounts<'_, '_>,
    args: ConvertStableToLeverLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    convert_stable_to_lever_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn convert_stable_to_lever_lst_verify_account_keys(
    accounts: ConvertStableToLeverLstAccounts<'_, '_>,
    keys: ConvertStableToLeverLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn convert_stable_to_lever_lst_verify_writable_privileges<'me, 'info>(
    accounts: ConvertStableToLeverLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.hylo,
        accounts.stablecoin_mint,
        accounts.fee_vault,
        accounts.user_stablecoin_ta,
        accounts.levercoin_mint,
        accounts.user_levercoin_ta,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn convert_stable_to_lever_lst_verify_signer_privileges<'me, 'info>(
    accounts: ConvertStableToLeverLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn convert_stable_to_lever_lst_verify_account_privileges<'me, 'info>(
    accounts: ConvertStableToLeverLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    convert_stable_to_lever_lst_verify_writable_privileges(accounts)?;
    convert_stable_to_lever_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GENESIS_MINT_EXO_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct GenesisMintExoAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub dead: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub admin_collateral_ta: &'me AccountInfo<'info>,
    pub dead_levercoin_ta: &'me AccountInfo<'info>,
    pub dead_stablecoin_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenesisMintExoKeys {
    pub admin: Pubkey,
    pub dead: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub levercoin_auth: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub admin_collateral_ta: Pubkey,
    pub dead_levercoin_ta: Pubkey,
    pub dead_stablecoin_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<GenesisMintExoAccounts<'_, '_>> for GenesisMintExoKeys {
    fn from(accounts: GenesisMintExoAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            dead: *accounts.dead.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            admin_collateral_ta: *accounts.admin_collateral_ta.key,
            dead_levercoin_ta: *accounts.dead_levercoin_ta.key,
            dead_stablecoin_ta: *accounts.dead_stablecoin_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<GenesisMintExoKeys> for [AccountMeta; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: GenesisMintExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dead,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dead_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dead_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN]> for GenesisMintExoKeys {
    fn from(pubkeys: [Pubkey; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            dead: pubkeys[1],
            hylo: pubkeys[2],
            exo_pair: pubkeys[3],
            levercoin_auth: pubkeys[4],
            stablecoin_auth: pubkeys[5],
            vault_auth: pubkeys[6],
            collateral_vault: pubkeys[7],
            admin_collateral_ta: pubkeys[8],
            dead_levercoin_ta: pubkeys[9],
            dead_stablecoin_ta: pubkeys[10],
            collateral_mint: pubkeys[11],
            levercoin_mint: pubkeys[12],
            stablecoin_mint: pubkeys[13],
            collateral_usd_pyth_feed: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<GenesisMintExoAccounts<'_, 'info>>
for [AccountInfo<'info>; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: GenesisMintExoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.dead.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.levercoin_auth.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.admin_collateral_ta.clone(),
            accounts.dead_levercoin_ta.clone(),
            accounts.dead_stablecoin_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN]>
for GenesisMintExoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            dead: &arr[1],
            hylo: &arr[2],
            exo_pair: &arr[3],
            levercoin_auth: &arr[4],
            stablecoin_auth: &arr[5],
            vault_auth: &arr[6],
            collateral_vault: &arr[7],
            admin_collateral_ta: &arr[8],
            dead_levercoin_ta: &arr[9],
            dead_stablecoin_ta: &arr[10],
            collateral_mint: &arr[11],
            levercoin_mint: &arr[12],
            stablecoin_mint: &arr[13],
            collateral_usd_pyth_feed: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const GENESIS_MINT_EXO_IX_DISCM: [u8; 8usize] = [140, 83, 64, 243, 219, 44, 0, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenesisMintExoIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GenesisMintExoIxData(pub GenesisMintExoIxArgs);
impl From<GenesisMintExoIxArgs> for GenesisMintExoIxData {
    fn from(args: GenesisMintExoIxArgs) -> Self {
        Self(args)
    }
}
impl GenesisMintExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GENESIS_MINT_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(GenesisMintExoIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GENESIS_MINT_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn genesis_mint_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: GenesisMintExoKeys,
    args: GenesisMintExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GENESIS_MINT_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: GenesisMintExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn genesis_mint_exo_ix(
    keys: GenesisMintExoKeys,
    args: GenesisMintExoIxArgs,
) -> std::io::Result<Instruction> {
    genesis_mint_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn genesis_mint_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GenesisMintExoAccounts<'_, '_>,
    args: GenesisMintExoIxArgs,
) -> ProgramResult {
    let keys: GenesisMintExoKeys = accounts.into();
    let ix = genesis_mint_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn genesis_mint_exo_invoke(
    accounts: GenesisMintExoAccounts<'_, '_>,
    args: GenesisMintExoIxArgs,
) -> ProgramResult {
    genesis_mint_exo_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn genesis_mint_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GenesisMintExoAccounts<'_, '_>,
    args: GenesisMintExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GenesisMintExoKeys = accounts.into();
    let ix = genesis_mint_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn genesis_mint_exo_invoke_signed(
    accounts: GenesisMintExoAccounts<'_, '_>,
    args: GenesisMintExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    genesis_mint_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn genesis_mint_exo_verify_account_keys(
    accounts: GenesisMintExoAccounts<'_, '_>,
    keys: GenesisMintExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.dead.key, keys.dead),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.admin_collateral_ta.key, keys.admin_collateral_ta),
        (*accounts.dead_levercoin_ta.key, keys.dead_levercoin_ta),
        (*accounts.dead_stablecoin_ta.key, keys.dead_stablecoin_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn genesis_mint_exo_verify_writable_privileges<'me, 'info>(
    accounts: GenesisMintExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.exo_pair,
        accounts.collateral_vault,
        accounts.admin_collateral_ta,
        accounts.dead_levercoin_ta,
        accounts.dead_stablecoin_ta,
        accounts.levercoin_mint,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn genesis_mint_exo_verify_signer_privileges<'me, 'info>(
    accounts: GenesisMintExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn genesis_mint_exo_verify_account_privileges<'me, 'info>(
    accounts: GenesisMintExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    genesis_mint_exo_verify_writable_privileges(accounts)?;
    genesis_mint_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct HarvestBorrowRateAccounts<'me, 'info> {
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub stablecoin_fee_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub stablecoin_fee_vault: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub hylo_earn_pool: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HarvestBorrowRateKeys {
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub levercoin_auth: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub stablecoin_fee_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub stablecoin_fee_vault: Pubkey,
    pub collateral_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub hylo_earn_pool: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<HarvestBorrowRateAccounts<'_, '_>> for HarvestBorrowRateKeys {
    fn from(accounts: HarvestBorrowRateAccounts) -> Self {
        Self {
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            stablecoin_fee_auth: *accounts.stablecoin_fee_auth.key,
            pool_auth: *accounts.pool_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            stablecoin_fee_vault: *accounts.stablecoin_fee_vault.key,
            collateral_mint: *accounts.collateral_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            hylo_earn_pool: *accounts.hylo_earn_pool.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<HarvestBorrowRateKeys> for [AccountMeta; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: HarvestBorrowRateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo_earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN]> for HarvestBorrowRateKeys {
    fn from(pubkeys: [Pubkey; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hylo: pubkeys[0],
            exo_pair: pubkeys[1],
            levercoin_auth: pubkeys[2],
            stablecoin_auth: pubkeys[3],
            vault_auth: pubkeys[4],
            stablecoin_fee_auth: pubkeys[5],
            pool_auth: pubkeys[6],
            collateral_vault: pubkeys[7],
            stablecoin_pool: pubkeys[8],
            stablecoin_fee_vault: pubkeys[9],
            collateral_mint: pubkeys[10],
            stablecoin_mint: pubkeys[11],
            levercoin_mint: pubkeys[12],
            collateral_usd_pyth_feed: pubkeys[13],
            hylo_earn_pool: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<HarvestBorrowRateAccounts<'_, 'info>>
for [AccountInfo<'info>; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: HarvestBorrowRateAccounts<'_, 'info>) -> Self {
        [
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.levercoin_auth.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.stablecoin_fee_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.stablecoin_fee_vault.clone(),
            accounts.collateral_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.hylo_earn_pool.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN]>
for HarvestBorrowRateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hylo: &arr[0],
            exo_pair: &arr[1],
            levercoin_auth: &arr[2],
            stablecoin_auth: &arr[3],
            vault_auth: &arr[4],
            stablecoin_fee_auth: &arr[5],
            pool_auth: &arr[6],
            collateral_vault: &arr[7],
            stablecoin_pool: &arr[8],
            stablecoin_fee_vault: &arr[9],
            collateral_mint: &arr[10],
            stablecoin_mint: &arr[11],
            levercoin_mint: &arr[12],
            collateral_usd_pyth_feed: &arr[13],
            hylo_earn_pool: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const HARVEST_BORROW_RATE_IX_DISCM: [u8; 8usize] = [
    164, 102, 122, 69, 68, 217, 66, 111,
];
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestBorrowRateIxData;
impl HarvestBorrowRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_BORROW_RATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_BORROW_RATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn harvest_borrow_rate_ix_with_program_id(
    program_id: Pubkey,
    keys: HarvestBorrowRateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; HARVEST_BORROW_RATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: HarvestBorrowRateIxData.try_to_vec()?,
    })
}
pub fn harvest_borrow_rate_ix(
    keys: HarvestBorrowRateKeys,
) -> std::io::Result<Instruction> {
    harvest_borrow_rate_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn harvest_borrow_rate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: HarvestBorrowRateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: HarvestBorrowRateKeys = accounts.into();
    let ix = harvest_borrow_rate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn harvest_borrow_rate_invoke(
    accounts: HarvestBorrowRateAccounts<'_, '_>,
) -> ProgramResult {
    harvest_borrow_rate_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn harvest_borrow_rate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: HarvestBorrowRateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: HarvestBorrowRateKeys = accounts.into();
    let ix = harvest_borrow_rate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn harvest_borrow_rate_invoke_signed(
    accounts: HarvestBorrowRateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    harvest_borrow_rate_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn harvest_borrow_rate_verify_account_keys(
    accounts: HarvestBorrowRateAccounts<'_, '_>,
    keys: HarvestBorrowRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.stablecoin_fee_auth.key, keys.stablecoin_fee_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.stablecoin_fee_vault.key, keys.stablecoin_fee_vault),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.hylo_earn_pool.key, keys.hylo_earn_pool),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn harvest_borrow_rate_verify_writable_privileges<'me, 'info>(
    accounts: HarvestBorrowRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.exo_pair,
        accounts.stablecoin_pool,
        accounts.stablecoin_fee_vault,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn harvest_borrow_rate_verify_account_privileges<'me, 'info>(
    accounts: HarvestBorrowRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    harvest_borrow_rate_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const HARVEST_YIELD_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct HarvestYieldAccounts<'me, 'info> {
    pub hylo: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub stablecoin_fee_auth: &'me AccountInfo<'info>,
    pub stablecoin_fee_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub hylo_earn_pool: &'me AccountInfo<'info>,
    pub lst_registry: &'me AccountInfo<'info>,
    pub lut_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HarvestYieldKeys {
    pub hylo: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub stablecoin_fee_auth: Pubkey,
    pub stablecoin_fee_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub pool_auth: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub hylo_earn_pool: Pubkey,
    pub lst_registry: Pubkey,
    pub lut_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<HarvestYieldAccounts<'_, '_>> for HarvestYieldKeys {
    fn from(accounts: HarvestYieldAccounts) -> Self {
        Self {
            hylo: *accounts.hylo.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            stablecoin_fee_auth: *accounts.stablecoin_fee_auth.key,
            stablecoin_fee_vault: *accounts.stablecoin_fee_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            pool_auth: *accounts.pool_auth.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            hylo_earn_pool: *accounts.hylo_earn_pool.key,
            lst_registry: *accounts.lst_registry.key,
            lut_program: *accounts.lut_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<HarvestYieldKeys> for [AccountMeta; HARVEST_YIELD_IX_ACCOUNTS_LEN] {
    fn from(keys: HarvestYieldKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo_earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_registry,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lut_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; HARVEST_YIELD_IX_ACCOUNTS_LEN]> for HarvestYieldKeys {
    fn from(pubkeys: [Pubkey; HARVEST_YIELD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hylo: pubkeys[0],
            stablecoin_mint: pubkeys[1],
            stablecoin_auth: pubkeys[2],
            stablecoin_fee_auth: pubkeys[3],
            stablecoin_fee_vault: pubkeys[4],
            stablecoin_pool: pubkeys[5],
            pool_auth: pubkeys[6],
            sol_usd_pyth_feed: pubkeys[7],
            hylo_earn_pool: pubkeys[8],
            lst_registry: pubkeys[9],
            lut_program: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<HarvestYieldAccounts<'_, 'info>>
for [AccountInfo<'info>; HARVEST_YIELD_IX_ACCOUNTS_LEN] {
    fn from(accounts: HarvestYieldAccounts<'_, 'info>) -> Self {
        [
            accounts.hylo.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.stablecoin_fee_auth.clone(),
            accounts.stablecoin_fee_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.pool_auth.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.hylo_earn_pool.clone(),
            accounts.lst_registry.clone(),
            accounts.lut_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; HARVEST_YIELD_IX_ACCOUNTS_LEN]>
for HarvestYieldAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; HARVEST_YIELD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hylo: &arr[0],
            stablecoin_mint: &arr[1],
            stablecoin_auth: &arr[2],
            stablecoin_fee_auth: &arr[3],
            stablecoin_fee_vault: &arr[4],
            stablecoin_pool: &arr[5],
            pool_auth: &arr[6],
            sol_usd_pyth_feed: &arr[7],
            hylo_earn_pool: &arr[8],
            lst_registry: &arr[9],
            lut_program: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const HARVEST_YIELD_IX_DISCM: [u8; 8usize] = [28, 200, 150, 200, 69, 56, 38, 133];
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestYieldIxData;
impl HarvestYieldIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HARVEST_YIELD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HARVEST_YIELD_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn harvest_yield_ix_with_program_id(
    program_id: Pubkey,
    keys: HarvestYieldKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; HARVEST_YIELD_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: HarvestYieldIxData.try_to_vec()?,
    })
}
pub fn harvest_yield_ix(keys: HarvestYieldKeys) -> std::io::Result<Instruction> {
    harvest_yield_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn harvest_yield_invoke_with_program_id(
    program_id: Pubkey,
    accounts: HarvestYieldAccounts<'_, '_>,
) -> ProgramResult {
    let keys: HarvestYieldKeys = accounts.into();
    let ix = harvest_yield_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn harvest_yield_invoke(accounts: HarvestYieldAccounts<'_, '_>) -> ProgramResult {
    harvest_yield_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn harvest_yield_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: HarvestYieldAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: HarvestYieldKeys = accounts.into();
    let ix = harvest_yield_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn harvest_yield_invoke_signed(
    accounts: HarvestYieldAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    harvest_yield_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn harvest_yield_verify_account_keys(
    accounts: HarvestYieldAccounts<'_, '_>,
    keys: HarvestYieldKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hylo.key, keys.hylo),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.stablecoin_fee_auth.key, keys.stablecoin_fee_auth),
        (*accounts.stablecoin_fee_vault.key, keys.stablecoin_fee_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.hylo_earn_pool.key, keys.hylo_earn_pool),
        (*accounts.lst_registry.key, keys.lst_registry),
        (*accounts.lut_program.key, keys.lut_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn harvest_yield_verify_writable_privileges<'me, 'info>(
    accounts: HarvestYieldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.hylo,
        accounts.stablecoin_mint,
        accounts.stablecoin_fee_vault,
        accounts.stablecoin_pool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn harvest_yield_verify_account_privileges<'me, 'info>(
    accounts: HarvestYieldAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    harvest_yield_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLstRegistryAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub registry_auth: &'me AccountInfo<'info>,
    pub lst_registry: &'me AccountInfo<'info>,
    pub lut_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLstRegistryKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub registry_auth: Pubkey,
    pub lst_registry: Pubkey,
    pub lut_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeLstRegistryAccounts<'_, '_>> for InitializeLstRegistryKeys {
    fn from(accounts: InitializeLstRegistryAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            registry_auth: *accounts.registry_auth.key,
            lst_registry: *accounts.lst_registry.key,
            lut_program: *accounts.lut_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeLstRegistryKeys>
for [AccountMeta; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLstRegistryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.registry_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lut_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN]>
for InitializeLstRegistryKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            registry_auth: pubkeys[2],
            lst_registry: pubkeys[3],
            lut_program: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeLstRegistryAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLstRegistryAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.registry_auth.clone(),
            accounts.lst_registry.clone(),
            accounts.lut_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN]>
for InitializeLstRegistryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            registry_auth: &arr[2],
            lst_registry: &arr[3],
            lut_program: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INITIALIZE_LST_REGISTRY_IX_DISCM: [u8; 8usize] = [
    148, 180, 49, 29, 133, 6, 9, 59,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeLstRegistryIxArgs {
    pub slot: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLstRegistryIxData(pub InitializeLstRegistryIxArgs);
impl From<InitializeLstRegistryIxArgs> for InitializeLstRegistryIxData {
    fn from(args: InitializeLstRegistryIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeLstRegistryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LST_REGISTRY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeLstRegistryIxArgs {
                slot,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LST_REGISTRY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.slot, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lst_registry_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLstRegistryKeys,
    args: InitializeLstRegistryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LST_REGISTRY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeLstRegistryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_lst_registry_ix(
    keys: InitializeLstRegistryKeys,
    args: InitializeLstRegistryIxArgs,
) -> std::io::Result<Instruction> {
    initialize_lst_registry_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn initialize_lst_registry_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLstRegistryAccounts<'_, '_>,
    args: InitializeLstRegistryIxArgs,
) -> ProgramResult {
    let keys: InitializeLstRegistryKeys = accounts.into();
    let ix = initialize_lst_registry_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lst_registry_invoke(
    accounts: InitializeLstRegistryAccounts<'_, '_>,
    args: InitializeLstRegistryIxArgs,
) -> ProgramResult {
    initialize_lst_registry_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_lst_registry_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLstRegistryAccounts<'_, '_>,
    args: InitializeLstRegistryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLstRegistryKeys = accounts.into();
    let ix = initialize_lst_registry_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lst_registry_invoke_signed(
    accounts: InitializeLstRegistryAccounts<'_, '_>,
    args: InitializeLstRegistryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lst_registry_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_lst_registry_verify_account_keys(
    accounts: InitializeLstRegistryAccounts<'_, '_>,
    keys: InitializeLstRegistryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.registry_auth.key, keys.registry_auth),
        (*accounts.lst_registry.key, keys.lst_registry),
        (*accounts.lut_program.key, keys.lut_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_lst_registry_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLstRegistryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.hylo, accounts.lst_registry] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lst_registry_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLstRegistryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lst_registry_verify_account_privileges<'me, 'info>(
    accounts: InitializeLstRegistryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lst_registry_verify_writable_privileges(accounts)?;
    initialize_lst_registry_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLstRegistryCalculatorsAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub lst_registry_auth: &'me AccountInfo<'info>,
    pub lst_registry: &'me AccountInfo<'info>,
    pub lut_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLstRegistryCalculatorsKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub lst_registry_auth: Pubkey,
    pub lst_registry: Pubkey,
    pub lut_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeLstRegistryCalculatorsAccounts<'_, '_>>
for InitializeLstRegistryCalculatorsKeys {
    fn from(accounts: InitializeLstRegistryCalculatorsAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            lst_registry_auth: *accounts.lst_registry_auth.key,
            lst_registry: *accounts.lst_registry.key,
            lut_program: *accounts.lut_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeLstRegistryCalculatorsKeys>
for [AccountMeta; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLstRegistryCalculatorsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_registry_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lut_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN]>
for InitializeLstRegistryCalculatorsKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            lst_registry_auth: pubkeys[2],
            lst_registry: pubkeys[3],
            lut_program: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeLstRegistryCalculatorsAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLstRegistryCalculatorsAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.lst_registry_auth.clone(),
            accounts.lst_registry.clone(),
            accounts.lut_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN]>
for InitializeLstRegistryCalculatorsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            lst_registry_auth: &arr[2],
            lst_registry: &arr[3],
            lut_program: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INITIALIZE_LST_REGISTRY_CALCULATORS_IX_DISCM: [u8; 8usize] = [
    109, 166, 101, 171, 217, 202, 83, 166,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLstRegistryCalculatorsIxData;
impl InitializeLstRegistryCalculatorsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LST_REGISTRY_CALCULATORS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LST_REGISTRY_CALCULATORS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lst_registry_calculators_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLstRegistryCalculatorsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LST_REGISTRY_CALCULATORS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeLstRegistryCalculatorsIxData.try_to_vec()?,
    })
}
pub fn initialize_lst_registry_calculators_ix(
    keys: InitializeLstRegistryCalculatorsKeys,
) -> std::io::Result<Instruction> {
    initialize_lst_registry_calculators_ix_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        keys,
    )
}
pub fn initialize_lst_registry_calculators_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLstRegistryCalculatorsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeLstRegistryCalculatorsKeys = accounts.into();
    let ix = initialize_lst_registry_calculators_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lst_registry_calculators_invoke(
    accounts: InitializeLstRegistryCalculatorsAccounts<'_, '_>,
) -> ProgramResult {
    initialize_lst_registry_calculators_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_lst_registry_calculators_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLstRegistryCalculatorsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLstRegistryCalculatorsKeys = accounts.into();
    let ix = initialize_lst_registry_calculators_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lst_registry_calculators_invoke_signed(
    accounts: InitializeLstRegistryCalculatorsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lst_registry_calculators_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_lst_registry_calculators_verify_account_keys(
    accounts: InitializeLstRegistryCalculatorsAccounts<'_, '_>,
    keys: InitializeLstRegistryCalculatorsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.lst_registry_auth.key, keys.lst_registry_auth),
        (*accounts.lst_registry.key, keys.lst_registry),
        (*accounts.lut_program.key, keys.lut_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_lst_registry_calculators_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLstRegistryCalculatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.lst_registry] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lst_registry_calculators_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLstRegistryCalculatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lst_registry_calculators_verify_account_privileges<'me, 'info>(
    accounts: InitializeLstRegistryCalculatorsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lst_registry_calculators_verify_writable_privileges(accounts)?;
    initialize_lst_registry_calculators_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLstVirtualStablecoinAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLstVirtualStablecoinKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeLstVirtualStablecoinAccounts<'_, '_>>
for InitializeLstVirtualStablecoinKeys {
    fn from(accounts: InitializeLstVirtualStablecoinAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeLstVirtualStablecoinKeys>
for [AccountMeta; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLstVirtualStablecoinKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN]>
for InitializeLstVirtualStablecoinKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            stablecoin_mint: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeLstVirtualStablecoinAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLstVirtualStablecoinAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN]>
for InitializeLstVirtualStablecoinAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            stablecoin_mint: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_DISCM: [u8; 8usize] = [
    184, 38, 47, 39, 48, 5, 249, 109,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLstVirtualStablecoinIxData;
impl InitializeLstVirtualStablecoinIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lst_virtual_stablecoin_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLstVirtualStablecoinKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LST_VIRTUAL_STABLECOIN_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeLstVirtualStablecoinIxData.try_to_vec()?,
    })
}
pub fn initialize_lst_virtual_stablecoin_ix(
    keys: InitializeLstVirtualStablecoinKeys,
) -> std::io::Result<Instruction> {
    initialize_lst_virtual_stablecoin_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn initialize_lst_virtual_stablecoin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLstVirtualStablecoinAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeLstVirtualStablecoinKeys = accounts.into();
    let ix = initialize_lst_virtual_stablecoin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lst_virtual_stablecoin_invoke(
    accounts: InitializeLstVirtualStablecoinAccounts<'_, '_>,
) -> ProgramResult {
    initialize_lst_virtual_stablecoin_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_lst_virtual_stablecoin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLstVirtualStablecoinAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLstVirtualStablecoinKeys = accounts.into();
    let ix = initialize_lst_virtual_stablecoin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lst_virtual_stablecoin_invoke_signed(
    accounts: InitializeLstVirtualStablecoinAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lst_virtual_stablecoin_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_lst_virtual_stablecoin_verify_account_keys(
    accounts: InitializeLstVirtualStablecoinAccounts<'_, '_>,
    keys: InitializeLstVirtualStablecoinKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_lst_virtual_stablecoin_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLstVirtualStablecoinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lst_virtual_stablecoin_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLstVirtualStablecoinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lst_virtual_stablecoin_verify_account_privileges<'me, 'info>(
    accounts: InitializeLstVirtualStablecoinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lst_virtual_stablecoin_verify_writable_privileges(accounts)?;
    initialize_lst_virtual_stablecoin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_MINTS_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMintsAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub stablecoin_metadata: &'me AccountInfo<'info>,
    pub levercoin_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeMintsKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub levercoin_auth: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub stablecoin_metadata: Pubkey,
    pub levercoin_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeMintsAccounts<'_, '_>> for InitializeMintsKeys {
    fn from(accounts: InitializeMintsAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            stablecoin_metadata: *accounts.stablecoin_metadata.key,
            levercoin_metadata: *accounts.levercoin_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeMintsKeys> for [AccountMeta; INITIALIZE_MINTS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMintsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_MINTS_IX_ACCOUNTS_LEN]> for InitializeMintsKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MINTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            stablecoin_auth: pubkeys[2],
            levercoin_auth: pubkeys[3],
            stablecoin_mint: pubkeys[4],
            levercoin_mint: pubkeys[5],
            stablecoin_metadata: pubkeys[6],
            levercoin_metadata: pubkeys[7],
            metadata_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            rent: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<InitializeMintsAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_MINTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeMintsAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.levercoin_auth.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.stablecoin_metadata.clone(),
            accounts.levercoin_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MINTS_IX_ACCOUNTS_LEN]>
for InitializeMintsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MINTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            stablecoin_auth: &arr[2],
            levercoin_auth: &arr[3],
            stablecoin_mint: &arr[4],
            levercoin_mint: &arr[5],
            stablecoin_metadata: &arr[6],
            levercoin_metadata: &arr[7],
            metadata_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            rent: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const INITIALIZE_MINTS_IX_DISCM: [u8; 8usize] = [189, 84, 85, 142, 177, 200, 57, 22];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeMintsIxArgs {
    pub stablecoin_metadata: TokenMetadata,
    pub levercoin_metadata: TokenMetadata,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMintsIxData(pub InitializeMintsIxArgs);
impl From<InitializeMintsIxArgs> for InitializeMintsIxData {
    fn from(args: InitializeMintsIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMintsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_MINTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let stablecoin_metadata = if reader.is_empty() {
            Default::default()
        } else {
            <TokenMetadata>::deserialize(&mut reader)?
        };
        let levercoin_metadata = if reader.is_empty() {
            Default::default()
        } else {
            <TokenMetadata>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeMintsIxArgs {
                stablecoin_metadata,
                levercoin_metadata,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_MINTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.stablecoin_metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.levercoin_metadata, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_mints_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeMintsKeys,
    args: InitializeMintsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MINTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMintsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_mints_ix(
    keys: InitializeMintsKeys,
    args: InitializeMintsIxArgs,
) -> std::io::Result<Instruction> {
    initialize_mints_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn initialize_mints_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMintsAccounts<'_, '_>,
    args: InitializeMintsIxArgs,
) -> ProgramResult {
    let keys: InitializeMintsKeys = accounts.into();
    let ix = initialize_mints_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_mints_invoke(
    accounts: InitializeMintsAccounts<'_, '_>,
    args: InitializeMintsIxArgs,
) -> ProgramResult {
    initialize_mints_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn initialize_mints_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMintsAccounts<'_, '_>,
    args: InitializeMintsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMintsKeys = accounts.into();
    let ix = initialize_mints_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_mints_invoke_signed(
    accounts: InitializeMintsAccounts<'_, '_>,
    args: InitializeMintsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_mints_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_mints_verify_account_keys(
    accounts: InitializeMintsAccounts<'_, '_>,
    keys: InitializeMintsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.stablecoin_metadata.key, keys.stablecoin_metadata),
        (*accounts.levercoin_metadata.key, keys.levercoin_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_mints_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMintsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.hylo,
        accounts.stablecoin_mint,
        accounts.levercoin_mint,
        accounts.stablecoin_metadata,
        accounts.levercoin_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_mints_verify_signer_privileges<'me, 'info>(
    accounts: InitializeMintsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_mints_verify_account_privileges<'me, 'info>(
    accounts: InitializeMintsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_mints_verify_writable_privileges(accounts)?;
    initialize_mints_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolDrawdownExoAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolDrawdownExoKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePoolDrawdownExoAccounts<'_, '_>> for InitializePoolDrawdownExoKeys {
    fn from(accounts: InitializePoolDrawdownExoAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePoolDrawdownExoKeys>
for [AccountMeta; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolDrawdownExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN]>
for InitializePoolDrawdownExoKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializePoolDrawdownExoAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolDrawdownExoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN]>
for InitializePoolDrawdownExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const INITIALIZE_POOL_DRAWDOWN_EXO_IX_DISCM: [u8; 8usize] = [
    6, 152, 26, 242, 102, 212, 96, 134,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolDrawdownExoIxData;
impl InitializePoolDrawdownExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_DRAWDOWN_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_DRAWDOWN_EXO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_drawdown_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolDrawdownExoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_DRAWDOWN_EXO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePoolDrawdownExoIxData.try_to_vec()?,
    })
}
pub fn initialize_pool_drawdown_exo_ix(
    keys: InitializePoolDrawdownExoKeys,
) -> std::io::Result<Instruction> {
    initialize_pool_drawdown_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn initialize_pool_drawdown_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolDrawdownExoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePoolDrawdownExoKeys = accounts.into();
    let ix = initialize_pool_drawdown_exo_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_drawdown_exo_invoke(
    accounts: InitializePoolDrawdownExoAccounts<'_, '_>,
) -> ProgramResult {
    initialize_pool_drawdown_exo_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_pool_drawdown_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolDrawdownExoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolDrawdownExoKeys = accounts.into();
    let ix = initialize_pool_drawdown_exo_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_drawdown_exo_invoke_signed(
    accounts: InitializePoolDrawdownExoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_drawdown_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_pool_drawdown_exo_verify_account_keys(
    accounts: InitializePoolDrawdownExoAccounts<'_, '_>,
    keys: InitializePoolDrawdownExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_drawdown_exo_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolDrawdownExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_drawdown_exo_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolDrawdownExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_drawdown_exo_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolDrawdownExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_drawdown_exo_verify_writable_privileges(accounts)?;
    initialize_pool_drawdown_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializePoolDrawdownLstAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolDrawdownLstKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePoolDrawdownLstAccounts<'_, '_>> for InitializePoolDrawdownLstKeys {
    fn from(accounts: InitializePoolDrawdownLstAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePoolDrawdownLstKeys>
for [AccountMeta; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolDrawdownLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN]>
for InitializePoolDrawdownLstKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializePoolDrawdownLstAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolDrawdownLstAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN]>
for InitializePoolDrawdownLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const INITIALIZE_POOL_DRAWDOWN_LST_IX_DISCM: [u8; 8usize] = [
    60, 100, 3, 198, 190, 66, 21, 158,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolDrawdownLstIxData;
impl InitializePoolDrawdownLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_DRAWDOWN_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_DRAWDOWN_LST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_pool_drawdown_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolDrawdownLstKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_DRAWDOWN_LST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializePoolDrawdownLstIxData.try_to_vec()?,
    })
}
pub fn initialize_pool_drawdown_lst_ix(
    keys: InitializePoolDrawdownLstKeys,
) -> std::io::Result<Instruction> {
    initialize_pool_drawdown_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn initialize_pool_drawdown_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolDrawdownLstAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializePoolDrawdownLstKeys = accounts.into();
    let ix = initialize_pool_drawdown_lst_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_pool_drawdown_lst_invoke(
    accounts: InitializePoolDrawdownLstAccounts<'_, '_>,
) -> ProgramResult {
    initialize_pool_drawdown_lst_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn initialize_pool_drawdown_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolDrawdownLstAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolDrawdownLstKeys = accounts.into();
    let ix = initialize_pool_drawdown_lst_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_pool_drawdown_lst_invoke_signed(
    accounts: InitializePoolDrawdownLstAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_drawdown_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_pool_drawdown_lst_verify_account_keys(
    accounts: InitializePoolDrawdownLstAccounts<'_, '_>,
    keys: InitializePoolDrawdownLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_pool_drawdown_lst_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolDrawdownLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_pool_drawdown_lst_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolDrawdownLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_pool_drawdown_lst_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolDrawdownLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_drawdown_lst_verify_writable_privileges(accounts)?;
    initialize_pool_drawdown_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeProtocolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub upgrade_authority: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub treasury: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
    pub hylo_exchange: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeProtocolKeys {
    pub admin: Pubkey,
    pub upgrade_authority: Pubkey,
    pub hylo: Pubkey,
    pub treasury: Pubkey,
    pub system_program: Pubkey,
    pub program_data: Pubkey,
    pub hylo_exchange: Pubkey,
}
impl From<InitializeProtocolAccounts<'_, '_>> for InitializeProtocolKeys {
    fn from(accounts: InitializeProtocolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            upgrade_authority: *accounts.upgrade_authority.key,
            hylo: *accounts.hylo.key,
            treasury: *accounts.treasury.key,
            system_program: *accounts.system_program.key,
            program_data: *accounts.program_data.key,
            hylo_exchange: *accounts.hylo_exchange.key,
        }
    }
}
impl From<InitializeProtocolKeys>
for [AccountMeta; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.upgrade_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo_exchange,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN]> for InitializeProtocolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            upgrade_authority: pubkeys[1],
            hylo: pubkeys[2],
            treasury: pubkeys[3],
            system_program: pubkeys[4],
            program_data: pubkeys[5],
            hylo_exchange: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeProtocolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.upgrade_authority.clone(),
            accounts.hylo.clone(),
            accounts.treasury.clone(),
            accounts.system_program.clone(),
            accounts.program_data.clone(),
            accounts.hylo_exchange.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN]>
for InitializeProtocolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            upgrade_authority: &arr[1],
            hylo: &arr[2],
            treasury: &arr[3],
            system_program: &arr[4],
            program_data: &arr[5],
            hylo_exchange: &arr[6],
        }
    }
}
pub const INITIALIZE_PROTOCOL_IX_DISCM: [u8; 8usize] = [
    188, 233, 252, 106, 134, 146, 202, 91,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeProtocolIxArgs {
    pub pause_authority: Pubkey,
    pub oracle_interval_secs: u64,
    pub stablecoin_mint_threshold: UFixValue64,
    pub levercoin_fees: LevercoinFees,
    pub yield_harvest_config: YieldHarvestConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeProtocolIxData(pub InitializeProtocolIxArgs);
impl From<InitializeProtocolIxArgs> for InitializeProtocolIxData {
    fn from(args: InitializeProtocolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pause_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        let yield_harvest_config = if reader.is_empty() {
            Default::default()
        } else {
            <YieldHarvestConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeProtocolIxArgs {
                pause_authority,
                oracle_interval_secs,
                stablecoin_mint_threshold,
                levercoin_fees,
                yield_harvest_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PROTOCOL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pause_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.stablecoin_mint_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.levercoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.yield_harvest_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeProtocolKeys,
    args: InitializeProtocolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeProtocolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_protocol_ix(
    keys: InitializeProtocolKeys,
    args: InitializeProtocolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_protocol_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn initialize_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeProtocolAccounts<'_, '_>,
    args: InitializeProtocolIxArgs,
) -> ProgramResult {
    let keys: InitializeProtocolKeys = accounts.into();
    let ix = initialize_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_protocol_invoke(
    accounts: InitializeProtocolAccounts<'_, '_>,
    args: InitializeProtocolIxArgs,
) -> ProgramResult {
    initialize_protocol_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn initialize_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeProtocolAccounts<'_, '_>,
    args: InitializeProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeProtocolKeys = accounts.into();
    let ix = initialize_protocol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_protocol_invoke_signed(
    accounts: InitializeProtocolAccounts<'_, '_>,
    args: InitializeProtocolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_protocol_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_protocol_verify_account_keys(
    accounts: InitializeProtocolAccounts<'_, '_>,
    keys: InitializeProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.upgrade_authority.key, keys.upgrade_authority),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.treasury.key, keys.treasury),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program_data.key, keys.program_data),
        (*accounts.hylo_exchange.key, keys.hylo_exchange),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_protocol_verify_writable_privileges<'me, 'info>(
    accounts: InitializeProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_protocol_verify_signer_privileges<'me, 'info>(
    accounts: InitializeProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.upgrade_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_protocol_verify_account_privileges<'me, 'info>(
    accounts: InitializeProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_protocol_verify_writable_privileges(accounts)?;
    initialize_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_USDC_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUsdcAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub usdc_fee_auth: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub usdc_fee_vault: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUsdcKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub usdc_fee_auth: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub usdc_fee_vault: Pubkey,
    pub usdc_mint: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeUsdcAccounts<'_, '_>> for InitializeUsdcKeys {
    fn from(accounts: InitializeUsdcAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            usdc_fee_auth: *accounts.usdc_fee_auth.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            usdc_fee_vault: *accounts.usdc_fee_vault.key,
            usdc_mint: *accounts.usdc_mint.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeUsdcKeys> for [AccountMeta; INITIALIZE_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_USDC_IX_ACCOUNTS_LEN]> for InitializeUsdcKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            usdc_vault_auth: pubkeys[3],
            usdc_fee_auth: pubkeys[4],
            usdc_collateral_vault: pubkeys[5],
            usdc_fee_vault: pubkeys[6],
            usdc_mint: pubkeys[7],
            usdc_usd_pyth_feed: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializeUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.usdc_fee_auth.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.usdc_fee_vault.clone(),
            accounts.usdc_mint.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_USDC_IX_ACCOUNTS_LEN]>
for InitializeUsdcAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            usdc_vault_auth: &arr[3],
            usdc_fee_auth: &arr[4],
            usdc_collateral_vault: &arr[5],
            usdc_fee_vault: &arr[6],
            usdc_mint: &arr[7],
            usdc_usd_pyth_feed: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIALIZE_USDC_IX_DISCM: [u8; 8usize] = [78, 227, 1, 15, 38, 179, 241, 189];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeUsdcIxArgs {
    pub mint_fee: UFixValue64,
    pub redeem_fee: UFixValue64,
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub par_tolerance: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUsdcIxData(pub InitializeUsdcIxArgs);
impl From<InitializeUsdcIxArgs> for InitializeUsdcIxData {
    fn from(args: InitializeUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mint_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let redeem_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let par_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeUsdcIxArgs {
                mint_fee,
                redeem_fee,
                oracle_interval_secs,
                oracle_conf_tolerance,
                par_tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mint_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.redeem_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.par_tolerance, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeUsdcKeys,
    args: InitializeUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_usdc_ix(
    keys: InitializeUsdcKeys,
    args: InitializeUsdcIxArgs,
) -> std::io::Result<Instruction> {
    initialize_usdc_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn initialize_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUsdcAccounts<'_, '_>,
    args: InitializeUsdcIxArgs,
) -> ProgramResult {
    let keys: InitializeUsdcKeys = accounts.into();
    let ix = initialize_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_usdc_invoke(
    accounts: InitializeUsdcAccounts<'_, '_>,
    args: InitializeUsdcIxArgs,
) -> ProgramResult {
    initialize_usdc_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn initialize_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeUsdcAccounts<'_, '_>,
    args: InitializeUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeUsdcKeys = accounts.into();
    let ix = initialize_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_usdc_invoke_signed(
    accounts: InitializeUsdcAccounts<'_, '_>,
    args: InitializeUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_usdc_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_usdc_verify_account_keys(
    accounts: InitializeUsdcAccounts<'_, '_>,
    keys: InitializeUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.usdc_fee_auth.key, keys.usdc_fee_auth),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.usdc_fee_vault.key, keys.usdc_fee_vault),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_usdc_verify_writable_privileges<'me, 'info>(
    accounts: InitializeUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.usdc_pair,
        accounts.usdc_collateral_vault,
        accounts.usdc_fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_usdc_verify_signer_privileges<'me, 'info>(
    accounts: InitializeUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_usdc_verify_account_privileges<'me, 'info>(
    accounts: InitializeUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_usdc_verify_writable_privileges(accounts)?;
    initialize_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct MintLevercoinExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintLevercoinExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub levercoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintLevercoinExoAccounts<'_, '_>> for MintLevercoinExoKeys {
    fn from(accounts: MintLevercoinExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintLevercoinExoKeys> for [AccountMeta; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: MintLevercoinExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]> for MintLevercoinExoKeys {
    fn from(pubkeys: [Pubkey; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            levercoin_auth: pubkeys[3],
            vault_auth: pubkeys[4],
            fee_auth: pubkeys[5],
            collateral_vault: pubkeys[6],
            fee_vault: pubkeys[7],
            user_collateral_ta: pubkeys[8],
            user_levercoin_ta: pubkeys[9],
            collateral_mint: pubkeys[10],
            levercoin_mint: pubkeys[11],
            collateral_usd_pyth_feed: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<MintLevercoinExoAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintLevercoinExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.levercoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]>
for MintLevercoinExoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            levercoin_auth: &arr[3],
            vault_auth: &arr[4],
            fee_auth: &arr[5],
            collateral_vault: &arr[6],
            fee_vault: &arr[7],
            user_collateral_ta: &arr[8],
            user_levercoin_ta: &arr[9],
            collateral_mint: &arr[10],
            levercoin_mint: &arr[11],
            collateral_usd_pyth_feed: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const MINT_LEVERCOIN_EXO_IX_DISCM: [u8; 8usize] = [
    206, 156, 247, 99, 235, 199, 166, 79,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintLevercoinExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintLevercoinExoIxData(pub MintLevercoinExoIxArgs);
impl From<MintLevercoinExoIxArgs> for MintLevercoinExoIxData {
    fn from(args: MintLevercoinExoIxArgs) -> Self {
        Self(args)
    }
}
impl MintLevercoinExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_LEVERCOIN_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MintLevercoinExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_LEVERCOIN_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_levercoin_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: MintLevercoinExoKeys,
    args: MintLevercoinExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_LEVERCOIN_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintLevercoinExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_levercoin_exo_ix(
    keys: MintLevercoinExoKeys,
    args: MintLevercoinExoIxArgs,
) -> std::io::Result<Instruction> {
    mint_levercoin_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn mint_levercoin_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintLevercoinExoAccounts<'_, '_>,
    args: MintLevercoinExoIxArgs,
) -> ProgramResult {
    let keys: MintLevercoinExoKeys = accounts.into();
    let ix = mint_levercoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_levercoin_exo_invoke(
    accounts: MintLevercoinExoAccounts<'_, '_>,
    args: MintLevercoinExoIxArgs,
) -> ProgramResult {
    mint_levercoin_exo_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn mint_levercoin_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintLevercoinExoAccounts<'_, '_>,
    args: MintLevercoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintLevercoinExoKeys = accounts.into();
    let ix = mint_levercoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_levercoin_exo_invoke_signed(
    accounts: MintLevercoinExoAccounts<'_, '_>,
    args: MintLevercoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_levercoin_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_levercoin_exo_verify_account_keys(
    accounts: MintLevercoinExoAccounts<'_, '_>,
    keys: MintLevercoinExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_levercoin_exo_verify_writable_privileges<'me, 'info>(
    accounts: MintLevercoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.collateral_vault,
        accounts.fee_vault,
        accounts.user_collateral_ta,
        accounts.user_levercoin_ta,
        accounts.levercoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_levercoin_exo_verify_signer_privileges<'me, 'info>(
    accounts: MintLevercoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_levercoin_exo_verify_account_privileges<'me, 'info>(
    accounts: MintLevercoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_levercoin_exo_verify_writable_privileges(accounts)?;
    mint_levercoin_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct MintLevercoinLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintLevercoinLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub fee_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub levercoin_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub lst_vault: Pubkey,
    pub lst_header: Pubkey,
    pub user_lst_ta: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub lst_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintLevercoinLstAccounts<'_, '_>> for MintLevercoinLstKeys {
    fn from(accounts: MintLevercoinLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            fee_auth: *accounts.fee_auth.key,
            vault_auth: *accounts.vault_auth.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            fee_vault: *accounts.fee_vault.key,
            lst_vault: *accounts.lst_vault.key,
            lst_header: *accounts.lst_header.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            lst_mint: *accounts.lst_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintLevercoinLstKeys> for [AccountMeta; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: MintLevercoinLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN]> for MintLevercoinLstKeys {
    fn from(pubkeys: [Pubkey; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            fee_auth: pubkeys[2],
            vault_auth: pubkeys[3],
            levercoin_auth: pubkeys[4],
            fee_vault: pubkeys[5],
            lst_vault: pubkeys[6],
            lst_header: pubkeys[7],
            user_lst_ta: pubkeys[8],
            user_levercoin_ta: pubkeys[9],
            lst_mint: pubkeys[10],
            levercoin_mint: pubkeys[11],
            sol_usd_pyth_feed: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<MintLevercoinLstAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintLevercoinLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.fee_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.levercoin_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.lst_vault.clone(),
            accounts.lst_header.clone(),
            accounts.user_lst_ta.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.lst_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN]>
for MintLevercoinLstAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            fee_auth: &arr[2],
            vault_auth: &arr[3],
            levercoin_auth: &arr[4],
            fee_vault: &arr[5],
            lst_vault: &arr[6],
            lst_header: &arr[7],
            user_lst_ta: &arr[8],
            user_levercoin_ta: &arr[9],
            lst_mint: &arr[10],
            levercoin_mint: &arr[11],
            sol_usd_pyth_feed: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const MINT_LEVERCOIN_LST_IX_DISCM: [u8; 8usize] = [
    184, 75, 99, 100, 153, 64, 243, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintLevercoinLstIxArgs {
    pub amount_lst_to_deposit: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintLevercoinLstIxData(pub MintLevercoinLstIxArgs);
impl From<MintLevercoinLstIxArgs> for MintLevercoinLstIxData {
    fn from(args: MintLevercoinLstIxArgs) -> Self {
        Self(args)
    }
}
impl MintLevercoinLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_LEVERCOIN_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_lst_to_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MintLevercoinLstIxArgs {
                amount_lst_to_deposit,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_LEVERCOIN_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_lst_to_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_levercoin_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: MintLevercoinLstKeys,
    args: MintLevercoinLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_LEVERCOIN_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintLevercoinLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_levercoin_lst_ix(
    keys: MintLevercoinLstKeys,
    args: MintLevercoinLstIxArgs,
) -> std::io::Result<Instruction> {
    mint_levercoin_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn mint_levercoin_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintLevercoinLstAccounts<'_, '_>,
    args: MintLevercoinLstIxArgs,
) -> ProgramResult {
    let keys: MintLevercoinLstKeys = accounts.into();
    let ix = mint_levercoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_levercoin_lst_invoke(
    accounts: MintLevercoinLstAccounts<'_, '_>,
    args: MintLevercoinLstIxArgs,
) -> ProgramResult {
    mint_levercoin_lst_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn mint_levercoin_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintLevercoinLstAccounts<'_, '_>,
    args: MintLevercoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintLevercoinLstKeys = accounts.into();
    let ix = mint_levercoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_levercoin_lst_invoke_signed(
    accounts: MintLevercoinLstAccounts<'_, '_>,
    args: MintLevercoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_levercoin_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_levercoin_lst_verify_account_keys(
    accounts: MintLevercoinLstAccounts<'_, '_>,
    keys: MintLevercoinLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_levercoin_lst_verify_writable_privileges<'me, 'info>(
    accounts: MintLevercoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.hylo,
        accounts.fee_vault,
        accounts.lst_vault,
        accounts.user_lst_ta,
        accounts.user_levercoin_ta,
        accounts.levercoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_levercoin_lst_verify_signer_privileges<'me, 'info>(
    accounts: MintLevercoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_levercoin_lst_verify_account_privileges<'me, 'info>(
    accounts: MintLevercoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_levercoin_lst_verify_writable_privileges(accounts)?;
    mint_levercoin_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct MintStablecoinExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintStablecoinExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintStablecoinExoAccounts<'_, '_>> for MintStablecoinExoKeys {
    fn from(accounts: MintStablecoinExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintStablecoinExoKeys> for [AccountMeta; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: MintStablecoinExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN]> for MintStablecoinExoKeys {
    fn from(pubkeys: [Pubkey; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            stablecoin_auth: pubkeys[3],
            vault_auth: pubkeys[4],
            fee_auth: pubkeys[5],
            collateral_vault: pubkeys[6],
            fee_vault: pubkeys[7],
            user_collateral_ta: pubkeys[8],
            user_stablecoin_ta: pubkeys[9],
            collateral_mint: pubkeys[10],
            stablecoin_mint: pubkeys[11],
            collateral_usd_pyth_feed: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<MintStablecoinExoAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintStablecoinExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN]>
for MintStablecoinExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            stablecoin_auth: &arr[3],
            vault_auth: &arr[4],
            fee_auth: &arr[5],
            collateral_vault: &arr[6],
            fee_vault: &arr[7],
            user_collateral_ta: &arr[8],
            user_stablecoin_ta: &arr[9],
            collateral_mint: &arr[10],
            stablecoin_mint: &arr[11],
            collateral_usd_pyth_feed: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const MINT_STABLECOIN_EXO_IX_DISCM: [u8; 8usize] = [
    52, 121, 187, 108, 227, 237, 51, 22,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintStablecoinExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintStablecoinExoIxData(pub MintStablecoinExoIxArgs);
impl From<MintStablecoinExoIxArgs> for MintStablecoinExoIxData {
    fn from(args: MintStablecoinExoIxArgs) -> Self {
        Self(args)
    }
}
impl MintStablecoinExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_STABLECOIN_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MintStablecoinExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_STABLECOIN_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_stablecoin_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: MintStablecoinExoKeys,
    args: MintStablecoinExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_STABLECOIN_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintStablecoinExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_stablecoin_exo_ix(
    keys: MintStablecoinExoKeys,
    args: MintStablecoinExoIxArgs,
) -> std::io::Result<Instruction> {
    mint_stablecoin_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn mint_stablecoin_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintStablecoinExoAccounts<'_, '_>,
    args: MintStablecoinExoIxArgs,
) -> ProgramResult {
    let keys: MintStablecoinExoKeys = accounts.into();
    let ix = mint_stablecoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_stablecoin_exo_invoke(
    accounts: MintStablecoinExoAccounts<'_, '_>,
    args: MintStablecoinExoIxArgs,
) -> ProgramResult {
    mint_stablecoin_exo_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn mint_stablecoin_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintStablecoinExoAccounts<'_, '_>,
    args: MintStablecoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintStablecoinExoKeys = accounts.into();
    let ix = mint_stablecoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_stablecoin_exo_invoke_signed(
    accounts: MintStablecoinExoAccounts<'_, '_>,
    args: MintStablecoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_stablecoin_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_stablecoin_exo_verify_account_keys(
    accounts: MintStablecoinExoAccounts<'_, '_>,
    keys: MintStablecoinExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_exo_verify_writable_privileges<'me, 'info>(
    accounts: MintStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.exo_pair,
        accounts.collateral_vault,
        accounts.fee_vault,
        accounts.user_collateral_ta,
        accounts.user_stablecoin_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_exo_verify_signer_privileges<'me, 'info>(
    accounts: MintStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_exo_verify_account_privileges<'me, 'info>(
    accounts: MintStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_stablecoin_exo_verify_writable_privileges(accounts)?;
    mint_stablecoin_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct MintStablecoinLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintStablecoinLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub fee_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub lst_vault: Pubkey,
    pub lst_header: Pubkey,
    pub user_lst_ta: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub lst_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintStablecoinLstAccounts<'_, '_>> for MintStablecoinLstKeys {
    fn from(accounts: MintStablecoinLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            fee_auth: *accounts.fee_auth.key,
            vault_auth: *accounts.vault_auth.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            fee_vault: *accounts.fee_vault.key,
            lst_vault: *accounts.lst_vault.key,
            lst_header: *accounts.lst_header.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            lst_mint: *accounts.lst_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintStablecoinLstKeys> for [AccountMeta; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: MintStablecoinLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN]> for MintStablecoinLstKeys {
    fn from(pubkeys: [Pubkey; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            fee_auth: pubkeys[2],
            vault_auth: pubkeys[3],
            stablecoin_auth: pubkeys[4],
            fee_vault: pubkeys[5],
            lst_vault: pubkeys[6],
            lst_header: pubkeys[7],
            user_lst_ta: pubkeys[8],
            user_stablecoin_ta: pubkeys[9],
            lst_mint: pubkeys[10],
            stablecoin_mint: pubkeys[11],
            sol_usd_pyth_feed: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<MintStablecoinLstAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintStablecoinLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.fee_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.lst_vault.clone(),
            accounts.lst_header.clone(),
            accounts.user_lst_ta.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.lst_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN]>
for MintStablecoinLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            fee_auth: &arr[2],
            vault_auth: &arr[3],
            stablecoin_auth: &arr[4],
            fee_vault: &arr[5],
            lst_vault: &arr[6],
            lst_header: &arr[7],
            user_lst_ta: &arr[8],
            user_stablecoin_ta: &arr[9],
            lst_mint: &arr[10],
            stablecoin_mint: &arr[11],
            sol_usd_pyth_feed: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const MINT_STABLECOIN_LST_IX_DISCM: [u8; 8usize] = [
    6, 208, 28, 175, 229, 35, 145, 226,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintStablecoinLstIxArgs {
    pub amount_lst_to_deposit: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintStablecoinLstIxData(pub MintStablecoinLstIxArgs);
impl From<MintStablecoinLstIxArgs> for MintStablecoinLstIxData {
    fn from(args: MintStablecoinLstIxArgs) -> Self {
        Self(args)
    }
}
impl MintStablecoinLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_STABLECOIN_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_lst_to_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MintStablecoinLstIxArgs {
                amount_lst_to_deposit,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_STABLECOIN_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_lst_to_deposit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_stablecoin_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: MintStablecoinLstKeys,
    args: MintStablecoinLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_STABLECOIN_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintStablecoinLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_stablecoin_lst_ix(
    keys: MintStablecoinLstKeys,
    args: MintStablecoinLstIxArgs,
) -> std::io::Result<Instruction> {
    mint_stablecoin_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn mint_stablecoin_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintStablecoinLstAccounts<'_, '_>,
    args: MintStablecoinLstIxArgs,
) -> ProgramResult {
    let keys: MintStablecoinLstKeys = accounts.into();
    let ix = mint_stablecoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_stablecoin_lst_invoke(
    accounts: MintStablecoinLstAccounts<'_, '_>,
    args: MintStablecoinLstIxArgs,
) -> ProgramResult {
    mint_stablecoin_lst_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn mint_stablecoin_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintStablecoinLstAccounts<'_, '_>,
    args: MintStablecoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintStablecoinLstKeys = accounts.into();
    let ix = mint_stablecoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_stablecoin_lst_invoke_signed(
    accounts: MintStablecoinLstAccounts<'_, '_>,
    args: MintStablecoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_stablecoin_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_stablecoin_lst_verify_account_keys(
    accounts: MintStablecoinLstAccounts<'_, '_>,
    keys: MintStablecoinLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_lst_verify_writable_privileges<'me, 'info>(
    accounts: MintStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.hylo,
        accounts.fee_vault,
        accounts.lst_vault,
        accounts.user_lst_ta,
        accounts.user_stablecoin_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_lst_verify_signer_privileges<'me, 'info>(
    accounts: MintStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_lst_verify_account_privileges<'me, 'info>(
    accounts: MintStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_stablecoin_lst_verify_writable_privileges(accounts)?;
    mint_stablecoin_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct MintStablecoinUsdcAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub usdc_fee_auth: &'me AccountInfo<'info>,
    pub stablecoin_fee_auth: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub usdc_fee_vault: &'me AccountInfo<'info>,
    pub stablecoin_fee_vault: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintStablecoinUsdcKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub usdc_fee_auth: Pubkey,
    pub stablecoin_fee_auth: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub usdc_fee_vault: Pubkey,
    pub stablecoin_fee_vault: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MintStablecoinUsdcAccounts<'_, '_>> for MintStablecoinUsdcKeys {
    fn from(accounts: MintStablecoinUsdcAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            usdc_fee_auth: *accounts.usdc_fee_auth.key,
            stablecoin_fee_auth: *accounts.stablecoin_fee_auth.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            usdc_fee_vault: *accounts.usdc_fee_vault.key,
            stablecoin_fee_vault: *accounts.stablecoin_fee_vault.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MintStablecoinUsdcKeys>
for [AccountMeta; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: MintStablecoinUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN]> for MintStablecoinUsdcKeys {
    fn from(pubkeys: [Pubkey; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            stablecoin_auth: pubkeys[3],
            usdc_vault_auth: pubkeys[4],
            usdc_fee_auth: pubkeys[5],
            stablecoin_fee_auth: pubkeys[6],
            usdc_collateral_vault: pubkeys[7],
            usdc_fee_vault: pubkeys[8],
            stablecoin_fee_vault: pubkeys[9],
            user_stablecoin_ta: pubkeys[10],
            user_usdc_ta: pubkeys[11],
            stablecoin_mint: pubkeys[12],
            usdc_mint: pubkeys[13],
            usdc_usd_pyth_feed: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<MintStablecoinUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintStablecoinUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.usdc_fee_auth.clone(),
            accounts.stablecoin_fee_auth.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.usdc_fee_vault.clone(),
            accounts.stablecoin_fee_vault.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN]>
for MintStablecoinUsdcAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            stablecoin_auth: &arr[3],
            usdc_vault_auth: &arr[4],
            usdc_fee_auth: &arr[5],
            stablecoin_fee_auth: &arr[6],
            usdc_collateral_vault: &arr[7],
            usdc_fee_vault: &arr[8],
            stablecoin_fee_vault: &arr[9],
            user_stablecoin_ta: &arr[10],
            user_usdc_ta: &arr[11],
            stablecoin_mint: &arr[12],
            usdc_mint: &arr[13],
            usdc_usd_pyth_feed: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const MINT_STABLECOIN_USDC_IX_DISCM: [u8; 8usize] = [
    14, 14, 163, 26, 224, 182, 159, 155,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintStablecoinUsdcIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintStablecoinUsdcIxData(pub MintStablecoinUsdcIxArgs);
impl From<MintStablecoinUsdcIxArgs> for MintStablecoinUsdcIxData {
    fn from(args: MintStablecoinUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl MintStablecoinUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_STABLECOIN_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(MintStablecoinUsdcIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_STABLECOIN_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_stablecoin_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: MintStablecoinUsdcKeys,
    args: MintStablecoinUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_STABLECOIN_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintStablecoinUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_stablecoin_usdc_ix(
    keys: MintStablecoinUsdcKeys,
    args: MintStablecoinUsdcIxArgs,
) -> std::io::Result<Instruction> {
    mint_stablecoin_usdc_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn mint_stablecoin_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintStablecoinUsdcAccounts<'_, '_>,
    args: MintStablecoinUsdcIxArgs,
) -> ProgramResult {
    let keys: MintStablecoinUsdcKeys = accounts.into();
    let ix = mint_stablecoin_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_stablecoin_usdc_invoke(
    accounts: MintStablecoinUsdcAccounts<'_, '_>,
    args: MintStablecoinUsdcIxArgs,
) -> ProgramResult {
    mint_stablecoin_usdc_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn mint_stablecoin_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintStablecoinUsdcAccounts<'_, '_>,
    args: MintStablecoinUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintStablecoinUsdcKeys = accounts.into();
    let ix = mint_stablecoin_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_stablecoin_usdc_invoke_signed(
    accounts: MintStablecoinUsdcAccounts<'_, '_>,
    args: MintStablecoinUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_stablecoin_usdc_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mint_stablecoin_usdc_verify_account_keys(
    accounts: MintStablecoinUsdcAccounts<'_, '_>,
    keys: MintStablecoinUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.usdc_fee_auth.key, keys.usdc_fee_auth),
        (*accounts.stablecoin_fee_auth.key, keys.stablecoin_fee_auth),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.usdc_fee_vault.key, keys.usdc_fee_vault),
        (*accounts.stablecoin_fee_vault.key, keys.stablecoin_fee_vault),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_usdc_verify_writable_privileges<'me, 'info>(
    accounts: MintStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.usdc_pair,
        accounts.usdc_collateral_vault,
        accounts.usdc_fee_vault,
        accounts.stablecoin_fee_vault,
        accounts.user_stablecoin_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_usdc_verify_signer_privileges<'me, 'info>(
    accounts: MintStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_stablecoin_usdc_verify_account_privileges<'me, 'info>(
    accounts: MintStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_stablecoin_usdc_verify_writable_privileges(accounts)?;
    mint_stablecoin_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct PauseExoPairAccounts<'me, 'info> {
    pub pause_authority: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseExoPairKeys {
    pub pause_authority: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PauseExoPairAccounts<'_, '_>> for PauseExoPairKeys {
    fn from(accounts: PauseExoPairAccounts) -> Self {
        Self {
            pause_authority: *accounts.pause_authority.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PauseExoPairKeys> for [AccountMeta; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseExoPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pause_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]> for PauseExoPairKeys {
    fn from(pubkeys: [Pubkey; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<PauseExoPairAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseExoPairAccounts<'_, 'info>) -> Self {
        [
            accounts.pause_authority.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]>
for PauseExoPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const PAUSE_EXO_PAIR_IX_DISCM: [u8; 8usize] = [32, 154, 250, 122, 162, 64, 211, 180];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseExoPairIxData;
impl PauseExoPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_EXO_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_EXO_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_exo_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseExoPairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_EXO_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseExoPairIxData.try_to_vec()?,
    })
}
pub fn pause_exo_pair_ix(keys: PauseExoPairKeys) -> std::io::Result<Instruction> {
    pause_exo_pair_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn pause_exo_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseExoPairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseExoPairKeys = accounts.into();
    let ix = pause_exo_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_exo_pair_invoke(accounts: PauseExoPairAccounts<'_, '_>) -> ProgramResult {
    pause_exo_pair_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn pause_exo_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseExoPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseExoPairKeys = accounts.into();
    let ix = pause_exo_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_exo_pair_invoke_signed(
    accounts: PauseExoPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_exo_pair_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_exo_pair_verify_account_keys(
    accounts: PauseExoPairAccounts<'_, '_>,
    keys: PauseExoPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pause_authority.key, keys.pause_authority),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_exo_pair_verify_writable_privileges<'me, 'info>(
    accounts: PauseExoPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_exo_pair_verify_signer_privileges<'me, 'info>(
    accounts: PauseExoPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pause_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_exo_pair_verify_account_privileges<'me, 'info>(
    accounts: PauseExoPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_exo_pair_verify_writable_privileges(accounts)?;
    pause_exo_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_LST_PAIR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PauseLstPairAccounts<'me, 'info> {
    pub pause_authority: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseLstPairKeys {
    pub pause_authority: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PauseLstPairAccounts<'_, '_>> for PauseLstPairKeys {
    fn from(accounts: PauseLstPairAccounts) -> Self {
        Self {
            pause_authority: *accounts.pause_authority.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PauseLstPairKeys> for [AccountMeta; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseLstPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pause_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN]> for PauseLstPairKeys {
    fn from(pubkeys: [Pubkey; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<PauseLstPairAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseLstPairAccounts<'_, 'info>) -> Self {
        [
            accounts.pause_authority.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN]>
for PauseLstPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const PAUSE_LST_PAIR_IX_DISCM: [u8; 8usize] = [64, 52, 201, 255, 25, 34, 220, 137];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseLstPairIxData;
impl PauseLstPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_LST_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_LST_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_lst_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseLstPairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_LST_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseLstPairIxData.try_to_vec()?,
    })
}
pub fn pause_lst_pair_ix(keys: PauseLstPairKeys) -> std::io::Result<Instruction> {
    pause_lst_pair_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn pause_lst_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseLstPairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseLstPairKeys = accounts.into();
    let ix = pause_lst_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_lst_pair_invoke(accounts: PauseLstPairAccounts<'_, '_>) -> ProgramResult {
    pause_lst_pair_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn pause_lst_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseLstPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseLstPairKeys = accounts.into();
    let ix = pause_lst_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_lst_pair_invoke_signed(
    accounts: PauseLstPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_lst_pair_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_lst_pair_verify_account_keys(
    accounts: PauseLstPairAccounts<'_, '_>,
    keys: PauseLstPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pause_authority.key, keys.pause_authority),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_lst_pair_verify_writable_privileges<'me, 'info>(
    accounts: PauseLstPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_lst_pair_verify_signer_privileges<'me, 'info>(
    accounts: PauseLstPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pause_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_lst_pair_verify_account_privileges<'me, 'info>(
    accounts: PauseLstPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_lst_pair_verify_writable_privileges(accounts)?;
    pause_lst_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_PROTOCOL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PauseProtocolAccounts<'me, 'info> {
    pub pause_authority: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseProtocolKeys {
    pub pause_authority: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PauseProtocolAccounts<'_, '_>> for PauseProtocolKeys {
    fn from(accounts: PauseProtocolAccounts) -> Self {
        Self {
            pause_authority: *accounts.pause_authority.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PauseProtocolKeys> for [AccountMeta; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pause_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN]> for PauseProtocolKeys {
    fn from(pubkeys: [Pubkey; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<PauseProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseProtocolAccounts<'_, 'info>) -> Self {
        [
            accounts.pause_authority.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN]>
for PauseProtocolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const PAUSE_PROTOCOL_IX_DISCM: [u8; 8usize] = [144, 95, 0, 107, 119, 39, 248, 141];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseProtocolIxData;
impl PauseProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_PROTOCOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseProtocolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseProtocolIxData.try_to_vec()?,
    })
}
pub fn pause_protocol_ix(keys: PauseProtocolKeys) -> std::io::Result<Instruction> {
    pause_protocol_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn pause_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseProtocolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseProtocolKeys = accounts.into();
    let ix = pause_protocol_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_protocol_invoke(accounts: PauseProtocolAccounts<'_, '_>) -> ProgramResult {
    pause_protocol_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn pause_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseProtocolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseProtocolKeys = accounts.into();
    let ix = pause_protocol_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_protocol_invoke_signed(
    accounts: PauseProtocolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_protocol_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_protocol_verify_account_keys(
    accounts: PauseProtocolAccounts<'_, '_>,
    keys: PauseProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pause_authority.key, keys.pause_authority),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_protocol_verify_writable_privileges<'me, 'info>(
    accounts: PauseProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_protocol_verify_signer_privileges<'me, 'info>(
    accounts: PauseProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pause_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_protocol_verify_account_privileges<'me, 'info>(
    accounts: PauseProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_protocol_verify_writable_privileges(accounts)?;
    pause_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PauseUsdcPairAccounts<'me, 'info> {
    pub pause_authority: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseUsdcPairKeys {
    pub pause_authority: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PauseUsdcPairAccounts<'_, '_>> for PauseUsdcPairKeys {
    fn from(accounts: PauseUsdcPairAccounts) -> Self {
        Self {
            pause_authority: *accounts.pause_authority.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PauseUsdcPairKeys> for [AccountMeta; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseUsdcPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pause_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]> for PauseUsdcPairKeys {
    fn from(pubkeys: [Pubkey; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<PauseUsdcPairAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseUsdcPairAccounts<'_, 'info>) -> Self {
        [
            accounts.pause_authority.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]>
for PauseUsdcPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pause_authority: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const PAUSE_USDC_PAIR_IX_DISCM: [u8; 8usize] = [247, 209, 82, 95, 96, 170, 13, 146];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseUsdcPairIxData;
impl PauseUsdcPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_USDC_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_USDC_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_usdc_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseUsdcPairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_USDC_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseUsdcPairIxData.try_to_vec()?,
    })
}
pub fn pause_usdc_pair_ix(keys: PauseUsdcPairKeys) -> std::io::Result<Instruction> {
    pause_usdc_pair_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn pause_usdc_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseUsdcPairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseUsdcPairKeys = accounts.into();
    let ix = pause_usdc_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_usdc_pair_invoke(accounts: PauseUsdcPairAccounts<'_, '_>) -> ProgramResult {
    pause_usdc_pair_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn pause_usdc_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseUsdcPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseUsdcPairKeys = accounts.into();
    let ix = pause_usdc_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_usdc_pair_invoke_signed(
    accounts: PauseUsdcPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_usdc_pair_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_usdc_pair_verify_account_keys(
    accounts: PauseUsdcPairAccounts<'_, '_>,
    keys: PauseUsdcPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pause_authority.key, keys.pause_authority),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_usdc_pair_verify_writable_privileges<'me, 'info>(
    accounts: PauseUsdcPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_usdc_pair_verify_signer_privileges<'me, 'info>(
    accounts: PauseUsdcPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pause_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_usdc_pair_verify_account_privileges<'me, 'info>(
    accounts: PauseUsdcPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_usdc_pair_verify_writable_privileges(accounts)?;
    pause_usdc_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ProposeAddressUpdateAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub new_address: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProposeAddressUpdateKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub proposal: Pubkey,
    pub new_address: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ProposeAddressUpdateAccounts<'_, '_>> for ProposeAddressUpdateKeys {
    fn from(accounts: ProposeAddressUpdateAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            proposal: *accounts.proposal.key,
            new_address: *accounts.new_address.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ProposeAddressUpdateKeys>
for [AccountMeta; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: ProposeAddressUpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]>
for ProposeAddressUpdateKeys {
    fn from(pubkeys: [Pubkey; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            proposal: pubkeys[2],
            new_address: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<ProposeAddressUpdateAccounts<'_, 'info>>
for [AccountInfo<'info>; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ProposeAddressUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.proposal.clone(),
            accounts.new_address.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN]>
for ProposeAddressUpdateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            proposal: &arr[2],
            new_address: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const PROPOSE_ADDRESS_UPDATE_IX_DISCM: [u8; 8usize] = [
    183, 253, 18, 185, 155, 67, 166, 35,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposeAddressUpdateIxArgs {
    pub address_field: AddressField,
    pub ttl_secs: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposeAddressUpdateIxData(pub ProposeAddressUpdateIxArgs);
impl From<ProposeAddressUpdateIxArgs> for ProposeAddressUpdateIxData {
    fn from(args: ProposeAddressUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl ProposeAddressUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSE_ADDRESS_UPDATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let address_field: AddressField = crate::borsh_de_or_default(&mut reader)?;
        let ttl_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ProposeAddressUpdateIxArgs {
                address_field,
                ttl_secs,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSE_ADDRESS_UPDATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.address_field, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.ttl_secs, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn propose_address_update_ix_with_program_id(
    program_id: Pubkey,
    keys: ProposeAddressUpdateKeys,
    args: ProposeAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROPOSE_ADDRESS_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ProposeAddressUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn propose_address_update_ix(
    keys: ProposeAddressUpdateKeys,
    args: ProposeAddressUpdateIxArgs,
) -> std::io::Result<Instruction> {
    propose_address_update_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn propose_address_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProposeAddressUpdateAccounts<'_, '_>,
    args: ProposeAddressUpdateIxArgs,
) -> ProgramResult {
    let keys: ProposeAddressUpdateKeys = accounts.into();
    let ix = propose_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn propose_address_update_invoke(
    accounts: ProposeAddressUpdateAccounts<'_, '_>,
    args: ProposeAddressUpdateIxArgs,
) -> ProgramResult {
    propose_address_update_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn propose_address_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProposeAddressUpdateAccounts<'_, '_>,
    args: ProposeAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProposeAddressUpdateKeys = accounts.into();
    let ix = propose_address_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn propose_address_update_invoke_signed(
    accounts: ProposeAddressUpdateAccounts<'_, '_>,
    args: ProposeAddressUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    propose_address_update_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn propose_address_update_verify_account_keys(
    accounts: ProposeAddressUpdateAccounts<'_, '_>,
    keys: ProposeAddressUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.new_address.key, keys.new_address),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn propose_address_update_verify_writable_privileges<'me, 'info>(
    accounts: ProposeAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn propose_address_update_verify_signer_privileges<'me, 'info>(
    accounts: ProposeAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn propose_address_update_verify_account_privileges<'me, 'info>(
    accounts: ProposeAddressUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    propose_address_update_verify_writable_privileges(accounts)?;
    propose_address_update_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RedeemLevercoinExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemLevercoinExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemLevercoinExoAccounts<'_, '_>> for RedeemLevercoinExoKeys {
    fn from(accounts: RedeemLevercoinExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemLevercoinExoKeys>
for [AccountMeta; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemLevercoinExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]> for RedeemLevercoinExoKeys {
    fn from(pubkeys: [Pubkey; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            vault_auth: pubkeys[3],
            fee_auth: pubkeys[4],
            collateral_vault: pubkeys[5],
            fee_vault: pubkeys[6],
            user_levercoin_ta: pubkeys[7],
            user_collateral_ta: pubkeys[8],
            collateral_mint: pubkeys[9],
            levercoin_mint: pubkeys[10],
            collateral_usd_pyth_feed: pubkeys[11],
            token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<RedeemLevercoinExoAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemLevercoinExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN]>
for RedeemLevercoinExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            vault_auth: &arr[3],
            fee_auth: &arr[4],
            collateral_vault: &arr[5],
            fee_vault: &arr[6],
            user_levercoin_ta: &arr[7],
            user_collateral_ta: &arr[8],
            collateral_mint: &arr[9],
            levercoin_mint: &arr[10],
            collateral_usd_pyth_feed: &arr[11],
            token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const REDEEM_LEVERCOIN_EXO_IX_DISCM: [u8; 8usize] = [
    51, 98, 223, 202, 13, 97, 183, 2,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemLevercoinExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemLevercoinExoIxData(pub RedeemLevercoinExoIxArgs);
impl From<RedeemLevercoinExoIxArgs> for RedeemLevercoinExoIxData {
    fn from(args: RedeemLevercoinExoIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemLevercoinExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_LEVERCOIN_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(RedeemLevercoinExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_LEVERCOIN_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_levercoin_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemLevercoinExoKeys,
    args: RedeemLevercoinExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_LEVERCOIN_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemLevercoinExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_levercoin_exo_ix(
    keys: RedeemLevercoinExoKeys,
    args: RedeemLevercoinExoIxArgs,
) -> std::io::Result<Instruction> {
    redeem_levercoin_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn redeem_levercoin_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemLevercoinExoAccounts<'_, '_>,
    args: RedeemLevercoinExoIxArgs,
) -> ProgramResult {
    let keys: RedeemLevercoinExoKeys = accounts.into();
    let ix = redeem_levercoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_levercoin_exo_invoke(
    accounts: RedeemLevercoinExoAccounts<'_, '_>,
    args: RedeemLevercoinExoIxArgs,
) -> ProgramResult {
    redeem_levercoin_exo_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn redeem_levercoin_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemLevercoinExoAccounts<'_, '_>,
    args: RedeemLevercoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemLevercoinExoKeys = accounts.into();
    let ix = redeem_levercoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_levercoin_exo_invoke_signed(
    accounts: RedeemLevercoinExoAccounts<'_, '_>,
    args: RedeemLevercoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_levercoin_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_levercoin_exo_verify_account_keys(
    accounts: RedeemLevercoinExoAccounts<'_, '_>,
    keys: RedeemLevercoinExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_levercoin_exo_verify_writable_privileges<'me, 'info>(
    accounts: RedeemLevercoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.collateral_vault,
        accounts.fee_vault,
        accounts.user_levercoin_ta,
        accounts.user_collateral_ta,
        accounts.levercoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_levercoin_exo_verify_signer_privileges<'me, 'info>(
    accounts: RedeemLevercoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_levercoin_exo_verify_account_privileges<'me, 'info>(
    accounts: RedeemLevercoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_levercoin_exo_verify_writable_privileges(accounts)?;
    redeem_levercoin_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RedeemLevercoinLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub user_levercoin_ta: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemLevercoinLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub fee_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub lst_vault: Pubkey,
    pub lst_header: Pubkey,
    pub user_levercoin_ta: Pubkey,
    pub user_lst_ta: Pubkey,
    pub levercoin_mint: Pubkey,
    pub lst_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemLevercoinLstAccounts<'_, '_>> for RedeemLevercoinLstKeys {
    fn from(accounts: RedeemLevercoinLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            fee_auth: *accounts.fee_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_vault: *accounts.fee_vault.key,
            lst_vault: *accounts.lst_vault.key,
            lst_header: *accounts.lst_header.key,
            user_levercoin_ta: *accounts.user_levercoin_ta.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            lst_mint: *accounts.lst_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemLevercoinLstKeys>
for [AccountMeta; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemLevercoinLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_levercoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN]> for RedeemLevercoinLstKeys {
    fn from(pubkeys: [Pubkey; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            fee_auth: pubkeys[2],
            vault_auth: pubkeys[3],
            fee_vault: pubkeys[4],
            lst_vault: pubkeys[5],
            lst_header: pubkeys[6],
            user_levercoin_ta: pubkeys[7],
            user_lst_ta: pubkeys[8],
            levercoin_mint: pubkeys[9],
            lst_mint: pubkeys[10],
            sol_usd_pyth_feed: pubkeys[11],
            token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<RedeemLevercoinLstAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemLevercoinLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.fee_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.lst_vault.clone(),
            accounts.lst_header.clone(),
            accounts.user_levercoin_ta.clone(),
            accounts.user_lst_ta.clone(),
            accounts.levercoin_mint.clone(),
            accounts.lst_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN]>
for RedeemLevercoinLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            fee_auth: &arr[2],
            vault_auth: &arr[3],
            fee_vault: &arr[4],
            lst_vault: &arr[5],
            lst_header: &arr[6],
            user_levercoin_ta: &arr[7],
            user_lst_ta: &arr[8],
            levercoin_mint: &arr[9],
            lst_mint: &arr[10],
            sol_usd_pyth_feed: &arr[11],
            token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const REDEEM_LEVERCOIN_LST_IX_DISCM: [u8; 8usize] = [
    34, 147, 66, 197, 160, 82, 147, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemLevercoinLstIxArgs {
    pub amount_to_redeem: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemLevercoinLstIxData(pub RedeemLevercoinLstIxArgs);
impl From<RedeemLevercoinLstIxArgs> for RedeemLevercoinLstIxData {
    fn from(args: RedeemLevercoinLstIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemLevercoinLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_LEVERCOIN_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_to_redeem: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(RedeemLevercoinLstIxArgs {
                amount_to_redeem,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_LEVERCOIN_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_to_redeem, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_levercoin_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemLevercoinLstKeys,
    args: RedeemLevercoinLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_LEVERCOIN_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemLevercoinLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_levercoin_lst_ix(
    keys: RedeemLevercoinLstKeys,
    args: RedeemLevercoinLstIxArgs,
) -> std::io::Result<Instruction> {
    redeem_levercoin_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn redeem_levercoin_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemLevercoinLstAccounts<'_, '_>,
    args: RedeemLevercoinLstIxArgs,
) -> ProgramResult {
    let keys: RedeemLevercoinLstKeys = accounts.into();
    let ix = redeem_levercoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_levercoin_lst_invoke(
    accounts: RedeemLevercoinLstAccounts<'_, '_>,
    args: RedeemLevercoinLstIxArgs,
) -> ProgramResult {
    redeem_levercoin_lst_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn redeem_levercoin_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemLevercoinLstAccounts<'_, '_>,
    args: RedeemLevercoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemLevercoinLstKeys = accounts.into();
    let ix = redeem_levercoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_levercoin_lst_invoke_signed(
    accounts: RedeemLevercoinLstAccounts<'_, '_>,
    args: RedeemLevercoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_levercoin_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_levercoin_lst_verify_account_keys(
    accounts: RedeemLevercoinLstAccounts<'_, '_>,
    keys: RedeemLevercoinLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.user_levercoin_ta.key, keys.user_levercoin_ta),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_levercoin_lst_verify_writable_privileges<'me, 'info>(
    accounts: RedeemLevercoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.hylo,
        accounts.fee_vault,
        accounts.lst_vault,
        accounts.user_levercoin_ta,
        accounts.user_lst_ta,
        accounts.levercoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_levercoin_lst_verify_signer_privileges<'me, 'info>(
    accounts: RedeemLevercoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_levercoin_lst_verify_account_privileges<'me, 'info>(
    accounts: RedeemLevercoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_levercoin_lst_verify_writable_privileges(accounts)?;
    redeem_levercoin_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RedeemStablecoinExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemStablecoinExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemStablecoinExoAccounts<'_, '_>> for RedeemStablecoinExoKeys {
    fn from(accounts: RedeemStablecoinExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemStablecoinExoKeys>
for [AccountMeta; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemStablecoinExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN]> for RedeemStablecoinExoKeys {
    fn from(pubkeys: [Pubkey; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            vault_auth: pubkeys[3],
            fee_auth: pubkeys[4],
            collateral_vault: pubkeys[5],
            fee_vault: pubkeys[6],
            user_stablecoin_ta: pubkeys[7],
            user_collateral_ta: pubkeys[8],
            collateral_mint: pubkeys[9],
            stablecoin_mint: pubkeys[10],
            collateral_usd_pyth_feed: pubkeys[11],
            token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<RedeemStablecoinExoAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemStablecoinExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN]>
for RedeemStablecoinExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            vault_auth: &arr[3],
            fee_auth: &arr[4],
            collateral_vault: &arr[5],
            fee_vault: &arr[6],
            user_stablecoin_ta: &arr[7],
            user_collateral_ta: &arr[8],
            collateral_mint: &arr[9],
            stablecoin_mint: &arr[10],
            collateral_usd_pyth_feed: &arr[11],
            token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const REDEEM_STABLECOIN_EXO_IX_DISCM: [u8; 8usize] = [
    171, 15, 190, 196, 96, 41, 58, 91,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStablecoinExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStablecoinExoIxData(pub RedeemStablecoinExoIxArgs);
impl From<RedeemStablecoinExoIxArgs> for RedeemStablecoinExoIxData {
    fn from(args: RedeemStablecoinExoIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemStablecoinExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STABLECOIN_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(RedeemStablecoinExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STABLECOIN_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_stablecoin_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemStablecoinExoKeys,
    args: RedeemStablecoinExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_STABLECOIN_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemStablecoinExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_stablecoin_exo_ix(
    keys: RedeemStablecoinExoKeys,
    args: RedeemStablecoinExoIxArgs,
) -> std::io::Result<Instruction> {
    redeem_stablecoin_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn redeem_stablecoin_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStablecoinExoAccounts<'_, '_>,
    args: RedeemStablecoinExoIxArgs,
) -> ProgramResult {
    let keys: RedeemStablecoinExoKeys = accounts.into();
    let ix = redeem_stablecoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_stablecoin_exo_invoke(
    accounts: RedeemStablecoinExoAccounts<'_, '_>,
    args: RedeemStablecoinExoIxArgs,
) -> ProgramResult {
    redeem_stablecoin_exo_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn redeem_stablecoin_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStablecoinExoAccounts<'_, '_>,
    args: RedeemStablecoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemStablecoinExoKeys = accounts.into();
    let ix = redeem_stablecoin_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_stablecoin_exo_invoke_signed(
    accounts: RedeemStablecoinExoAccounts<'_, '_>,
    args: RedeemStablecoinExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_stablecoin_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_stablecoin_exo_verify_account_keys(
    accounts: RedeemStablecoinExoAccounts<'_, '_>,
    keys: RedeemStablecoinExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_exo_verify_writable_privileges<'me, 'info>(
    accounts: RedeemStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.exo_pair,
        accounts.collateral_vault,
        accounts.fee_vault,
        accounts.user_stablecoin_ta,
        accounts.user_collateral_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_exo_verify_signer_privileges<'me, 'info>(
    accounts: RedeemStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_exo_verify_account_privileges<'me, 'info>(
    accounts: RedeemStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_stablecoin_exo_verify_writable_privileges(accounts)?;
    redeem_stablecoin_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RedeemStablecoinLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemStablecoinLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub fee_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub lst_vault: Pubkey,
    pub lst_header: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub user_lst_ta: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub lst_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemStablecoinLstAccounts<'_, '_>> for RedeemStablecoinLstKeys {
    fn from(accounts: RedeemStablecoinLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            fee_auth: *accounts.fee_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_vault: *accounts.fee_vault.key,
            lst_vault: *accounts.lst_vault.key,
            lst_header: *accounts.lst_header.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            lst_mint: *accounts.lst_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemStablecoinLstKeys>
for [AccountMeta; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemStablecoinLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN]> for RedeemStablecoinLstKeys {
    fn from(pubkeys: [Pubkey; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            fee_auth: pubkeys[2],
            vault_auth: pubkeys[3],
            fee_vault: pubkeys[4],
            lst_vault: pubkeys[5],
            lst_header: pubkeys[6],
            user_stablecoin_ta: pubkeys[7],
            user_lst_ta: pubkeys[8],
            stablecoin_mint: pubkeys[9],
            lst_mint: pubkeys[10],
            sol_usd_pyth_feed: pubkeys[11],
            token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<RedeemStablecoinLstAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemStablecoinLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.fee_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.lst_vault.clone(),
            accounts.lst_header.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.user_lst_ta.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.lst_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN]>
for RedeemStablecoinLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            fee_auth: &arr[2],
            vault_auth: &arr[3],
            fee_vault: &arr[4],
            lst_vault: &arr[5],
            lst_header: &arr[6],
            user_stablecoin_ta: &arr[7],
            user_lst_ta: &arr[8],
            stablecoin_mint: &arr[9],
            lst_mint: &arr[10],
            sol_usd_pyth_feed: &arr[11],
            token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const REDEEM_STABLECOIN_LST_IX_DISCM: [u8; 8usize] = [
    12, 230, 189, 126, 110, 233, 234, 195,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStablecoinLstIxArgs {
    pub amount_to_redeem: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStablecoinLstIxData(pub RedeemStablecoinLstIxArgs);
impl From<RedeemStablecoinLstIxArgs> for RedeemStablecoinLstIxData {
    fn from(args: RedeemStablecoinLstIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemStablecoinLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STABLECOIN_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_to_redeem: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(RedeemStablecoinLstIxArgs {
                amount_to_redeem,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STABLECOIN_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_to_redeem, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_stablecoin_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemStablecoinLstKeys,
    args: RedeemStablecoinLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_STABLECOIN_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemStablecoinLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_stablecoin_lst_ix(
    keys: RedeemStablecoinLstKeys,
    args: RedeemStablecoinLstIxArgs,
) -> std::io::Result<Instruction> {
    redeem_stablecoin_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn redeem_stablecoin_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStablecoinLstAccounts<'_, '_>,
    args: RedeemStablecoinLstIxArgs,
) -> ProgramResult {
    let keys: RedeemStablecoinLstKeys = accounts.into();
    let ix = redeem_stablecoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_stablecoin_lst_invoke(
    accounts: RedeemStablecoinLstAccounts<'_, '_>,
    args: RedeemStablecoinLstIxArgs,
) -> ProgramResult {
    redeem_stablecoin_lst_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn redeem_stablecoin_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStablecoinLstAccounts<'_, '_>,
    args: RedeemStablecoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemStablecoinLstKeys = accounts.into();
    let ix = redeem_stablecoin_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_stablecoin_lst_invoke_signed(
    accounts: RedeemStablecoinLstAccounts<'_, '_>,
    args: RedeemStablecoinLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_stablecoin_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_stablecoin_lst_verify_account_keys(
    accounts: RedeemStablecoinLstAccounts<'_, '_>,
    keys: RedeemStablecoinLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_lst_verify_writable_privileges<'me, 'info>(
    accounts: RedeemStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.hylo,
        accounts.fee_vault,
        accounts.lst_vault,
        accounts.user_stablecoin_ta,
        accounts.user_lst_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_lst_verify_signer_privileges<'me, 'info>(
    accounts: RedeemStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_lst_verify_account_privileges<'me, 'info>(
    accounts: RedeemStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_stablecoin_lst_verify_writable_privileges(accounts)?;
    redeem_stablecoin_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct RedeemStablecoinUsdcAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub usdc_fee_auth: &'me AccountInfo<'info>,
    pub stablecoin_fee_auth: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub usdc_fee_vault: &'me AccountInfo<'info>,
    pub stablecoin_fee_vault: &'me AccountInfo<'info>,
    pub user_stablecoin_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemStablecoinUsdcKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub usdc_fee_auth: Pubkey,
    pub stablecoin_fee_auth: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub usdc_fee_vault: Pubkey,
    pub stablecoin_fee_vault: Pubkey,
    pub user_stablecoin_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RedeemStablecoinUsdcAccounts<'_, '_>> for RedeemStablecoinUsdcKeys {
    fn from(accounts: RedeemStablecoinUsdcAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_auth: *accounts.stablecoin_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            usdc_fee_auth: *accounts.usdc_fee_auth.key,
            stablecoin_fee_auth: *accounts.stablecoin_fee_auth.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            usdc_fee_vault: *accounts.usdc_fee_vault.key,
            stablecoin_fee_vault: *accounts.stablecoin_fee_vault.key,
            user_stablecoin_ta: *accounts.user_stablecoin_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RedeemStablecoinUsdcKeys>
for [AccountMeta; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemStablecoinUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stablecoin_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN]>
for RedeemStablecoinUsdcKeys {
    fn from(pubkeys: [Pubkey; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            stablecoin_auth: pubkeys[3],
            usdc_vault_auth: pubkeys[4],
            usdc_fee_auth: pubkeys[5],
            stablecoin_fee_auth: pubkeys[6],
            usdc_collateral_vault: pubkeys[7],
            usdc_fee_vault: pubkeys[8],
            stablecoin_fee_vault: pubkeys[9],
            user_stablecoin_ta: pubkeys[10],
            user_usdc_ta: pubkeys[11],
            stablecoin_mint: pubkeys[12],
            usdc_mint: pubkeys[13],
            usdc_usd_pyth_feed: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<RedeemStablecoinUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemStablecoinUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.usdc_fee_auth.clone(),
            accounts.stablecoin_fee_auth.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.usdc_fee_vault.clone(),
            accounts.stablecoin_fee_vault.clone(),
            accounts.user_stablecoin_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN]>
for RedeemStablecoinUsdcAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            stablecoin_auth: &arr[3],
            usdc_vault_auth: &arr[4],
            usdc_fee_auth: &arr[5],
            stablecoin_fee_auth: &arr[6],
            usdc_collateral_vault: &arr[7],
            usdc_fee_vault: &arr[8],
            stablecoin_fee_vault: &arr[9],
            user_stablecoin_ta: &arr[10],
            user_usdc_ta: &arr[11],
            stablecoin_mint: &arr[12],
            usdc_mint: &arr[13],
            usdc_usd_pyth_feed: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const REDEEM_STABLECOIN_USDC_IX_DISCM: [u8; 8usize] = [
    170, 237, 98, 104, 60, 82, 72, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemStablecoinUsdcIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemStablecoinUsdcIxData(pub RedeemStablecoinUsdcIxArgs);
impl From<RedeemStablecoinUsdcIxArgs> for RedeemStablecoinUsdcIxData {
    fn from(args: RedeemStablecoinUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemStablecoinUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_STABLECOIN_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(RedeemStablecoinUsdcIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_STABLECOIN_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_stablecoin_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemStablecoinUsdcKeys,
    args: RedeemStablecoinUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_STABLECOIN_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemStablecoinUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_stablecoin_usdc_ix(
    keys: RedeemStablecoinUsdcKeys,
    args: RedeemStablecoinUsdcIxArgs,
) -> std::io::Result<Instruction> {
    redeem_stablecoin_usdc_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn redeem_stablecoin_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStablecoinUsdcAccounts<'_, '_>,
    args: RedeemStablecoinUsdcIxArgs,
) -> ProgramResult {
    let keys: RedeemStablecoinUsdcKeys = accounts.into();
    let ix = redeem_stablecoin_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_stablecoin_usdc_invoke(
    accounts: RedeemStablecoinUsdcAccounts<'_, '_>,
    args: RedeemStablecoinUsdcIxArgs,
) -> ProgramResult {
    redeem_stablecoin_usdc_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn redeem_stablecoin_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemStablecoinUsdcAccounts<'_, '_>,
    args: RedeemStablecoinUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemStablecoinUsdcKeys = accounts.into();
    let ix = redeem_stablecoin_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_stablecoin_usdc_invoke_signed(
    accounts: RedeemStablecoinUsdcAccounts<'_, '_>,
    args: RedeemStablecoinUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_stablecoin_usdc_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_stablecoin_usdc_verify_account_keys(
    accounts: RedeemStablecoinUsdcAccounts<'_, '_>,
    keys: RedeemStablecoinUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_auth.key, keys.stablecoin_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.usdc_fee_auth.key, keys.usdc_fee_auth),
        (*accounts.stablecoin_fee_auth.key, keys.stablecoin_fee_auth),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.usdc_fee_vault.key, keys.usdc_fee_vault),
        (*accounts.stablecoin_fee_vault.key, keys.stablecoin_fee_vault),
        (*accounts.user_stablecoin_ta.key, keys.user_stablecoin_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_usdc_verify_writable_privileges<'me, 'info>(
    accounts: RedeemStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.usdc_pair,
        accounts.usdc_collateral_vault,
        accounts.usdc_fee_vault,
        accounts.stablecoin_fee_vault,
        accounts.user_stablecoin_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_usdc_verify_signer_privileges<'me, 'info>(
    accounts: RedeemStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_stablecoin_usdc_verify_account_privileges<'me, 'info>(
    accounts: RedeemStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_stablecoin_usdc_verify_writable_privileges(accounts)?;
    redeem_stablecoin_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REGISTER_EXO_IX_ACCOUNTS_LEN: usize = 19;
#[derive(Copy, Clone, Debug)]
pub struct RegisterExoAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub levercoin_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub levercoin_metadata: &'me AccountInfo<'info>,
    pub exo_usd_pyth_feed: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegisterExoKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub levercoin_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub fee_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub collateral_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub levercoin_metadata: Pubkey,
    pub exo_usd_pyth_feed: Pubkey,
    pub metadata_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RegisterExoAccounts<'_, '_>> for RegisterExoKeys {
    fn from(accounts: RegisterExoAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            levercoin_auth: *accounts.levercoin_auth.key,
            vault_auth: *accounts.vault_auth.key,
            fee_auth: *accounts.fee_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            fee_vault: *accounts.fee_vault.key,
            collateral_mint: *accounts.collateral_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            levercoin_metadata: *accounts.levercoin_metadata.key,
            exo_usd_pyth_feed: *accounts.exo_usd_pyth_feed.key,
            metadata_program: *accounts.metadata_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RegisterExoKeys> for [AccountMeta; REGISTER_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: RegisterExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.exo_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REGISTER_EXO_IX_ACCOUNTS_LEN]> for RegisterExoKeys {
    fn from(pubkeys: [Pubkey; REGISTER_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            levercoin_auth: pubkeys[3],
            vault_auth: pubkeys[4],
            fee_auth: pubkeys[5],
            collateral_vault: pubkeys[6],
            fee_vault: pubkeys[7],
            collateral_mint: pubkeys[8],
            levercoin_mint: pubkeys[9],
            levercoin_metadata: pubkeys[10],
            exo_usd_pyth_feed: pubkeys[11],
            metadata_program: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            rent: pubkeys[15],
            system_program: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
        }
    }
}
impl<'info> From<RegisterExoAccounts<'_, 'info>>
for [AccountInfo<'info>; REGISTER_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: RegisterExoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.levercoin_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.fee_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.fee_vault.clone(),
            accounts.collateral_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.levercoin_metadata.clone(),
            accounts.exo_usd_pyth_feed.clone(),
            accounts.metadata_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REGISTER_EXO_IX_ACCOUNTS_LEN]>
for RegisterExoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REGISTER_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            levercoin_auth: &arr[3],
            vault_auth: &arr[4],
            fee_auth: &arr[5],
            collateral_vault: &arr[6],
            fee_vault: &arr[7],
            collateral_mint: &arr[8],
            levercoin_mint: &arr[9],
            levercoin_metadata: &arr[10],
            exo_usd_pyth_feed: &arr[11],
            metadata_program: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
            rent: &arr[15],
            system_program: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
        }
    }
}
pub const REGISTER_EXO_IX_DISCM: [u8; 8usize] = [255, 198, 39, 217, 189, 131, 248, 73];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterExoIxArgs {
    pub oracle_feed_id: [u8; 32],
    pub oracle_interval_secs: u64,
    pub oracle_conf_tolerance: UFixValue64,
    pub stablecoin_mint_threshold: UFixValue64,
    pub borrow_rate_curve_config: BorrowRateCurveConfig,
    pub borrow_rate_fee: UFixValue64,
    pub levercoin_fees: LevercoinFees,
    pub sell_curve_config: RebalanceCurveConfig,
    pub buy_curve_config: RebalanceCurveConfig,
    pub metadata: TokenMetadata,
    pub levercoin_market_cap_limit: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterExoIxData(pub RegisterExoIxArgs);
impl From<RegisterExoIxArgs> for RegisterExoIxData {
    fn from(args: RegisterExoIxArgs) -> Self {
        Self(args)
    }
}
impl RegisterExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REGISTER_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let oracle_feed_id: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let borrow_rate_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurveConfig>::deserialize(&mut reader)?
        };
        let borrow_rate_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        let sell_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let buy_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let metadata = if reader.is_empty() {
            Default::default()
        } else {
            <TokenMetadata>::deserialize(&mut reader)?
        };
        let levercoin_market_cap_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(RegisterExoIxArgs {
                oracle_feed_id,
                oracle_interval_secs,
                oracle_conf_tolerance,
                stablecoin_mint_threshold,
                borrow_rate_curve_config,
                borrow_rate_fee,
                levercoin_fees,
                sell_curve_config,
                buy_curve_config,
                metadata,
                levercoin_market_cap_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REGISTER_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_feed_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.stablecoin_mint_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.borrow_rate_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.borrow_rate_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.levercoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sell_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.buy_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.metadata, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.levercoin_market_cap_limit,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn register_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: RegisterExoKeys,
    args: RegisterExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REGISTER_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: RegisterExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn register_exo_ix(
    keys: RegisterExoKeys,
    args: RegisterExoIxArgs,
) -> std::io::Result<Instruction> {
    register_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn register_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RegisterExoAccounts<'_, '_>,
    args: RegisterExoIxArgs,
) -> ProgramResult {
    let keys: RegisterExoKeys = accounts.into();
    let ix = register_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn register_exo_invoke(
    accounts: RegisterExoAccounts<'_, '_>,
    args: RegisterExoIxArgs,
) -> ProgramResult {
    register_exo_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn register_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RegisterExoAccounts<'_, '_>,
    args: RegisterExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RegisterExoKeys = accounts.into();
    let ix = register_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn register_exo_invoke_signed(
    accounts: RegisterExoAccounts<'_, '_>,
    args: RegisterExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    register_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn register_exo_verify_account_keys(
    accounts: RegisterExoAccounts<'_, '_>,
    keys: RegisterExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.levercoin_auth.key, keys.levercoin_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.levercoin_metadata.key, keys.levercoin_metadata),
        (*accounts.exo_usd_pyth_feed.key, keys.exo_usd_pyth_feed),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn register_exo_verify_writable_privileges<'me, 'info>(
    accounts: RegisterExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.exo_pair,
        accounts.collateral_vault,
        accounts.fee_vault,
        accounts.levercoin_mint,
        accounts.levercoin_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn register_exo_verify_signer_privileges<'me, 'info>(
    accounts: RegisterExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn register_exo_verify_account_privileges<'me, 'info>(
    accounts: RegisterExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    register_exo_verify_writable_privileges(accounts)?;
    register_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REGISTER_LST_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct RegisterLstAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub registry_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub lst_registry: &'me AccountInfo<'info>,
    pub lst_stake_pool_state: &'me AccountInfo<'info>,
    pub sanctum_calculator_program: &'me AccountInfo<'info>,
    pub sanctum_calculator_state: &'me AccountInfo<'info>,
    pub stake_pool_program_data: &'me AccountInfo<'info>,
    pub stake_pool_program: &'me AccountInfo<'info>,
    pub lut_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegisterLstKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub lst_header: Pubkey,
    pub fee_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub registry_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub lst_vault: Pubkey,
    pub lst_mint: Pubkey,
    pub lst_registry: Pubkey,
    pub lst_stake_pool_state: Pubkey,
    pub sanctum_calculator_program: Pubkey,
    pub sanctum_calculator_state: Pubkey,
    pub stake_pool_program_data: Pubkey,
    pub stake_pool_program: Pubkey,
    pub lut_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RegisterLstAccounts<'_, '_>> for RegisterLstKeys {
    fn from(accounts: RegisterLstAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            lst_header: *accounts.lst_header.key,
            fee_auth: *accounts.fee_auth.key,
            vault_auth: *accounts.vault_auth.key,
            registry_auth: *accounts.registry_auth.key,
            fee_vault: *accounts.fee_vault.key,
            lst_vault: *accounts.lst_vault.key,
            lst_mint: *accounts.lst_mint.key,
            lst_registry: *accounts.lst_registry.key,
            lst_stake_pool_state: *accounts.lst_stake_pool_state.key,
            sanctum_calculator_program: *accounts.sanctum_calculator_program.key,
            sanctum_calculator_state: *accounts.sanctum_calculator_state.key,
            stake_pool_program_data: *accounts.stake_pool_program_data.key,
            stake_pool_program: *accounts.stake_pool_program.key,
            lut_program: *accounts.lut_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RegisterLstKeys> for [AccountMeta; REGISTER_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: RegisterLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.registry_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_stake_pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sanctum_calculator_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sanctum_calculator_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_program_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lut_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REGISTER_LST_IX_ACCOUNTS_LEN]> for RegisterLstKeys {
    fn from(pubkeys: [Pubkey; REGISTER_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            lst_header: pubkeys[2],
            fee_auth: pubkeys[3],
            vault_auth: pubkeys[4],
            registry_auth: pubkeys[5],
            fee_vault: pubkeys[6],
            lst_vault: pubkeys[7],
            lst_mint: pubkeys[8],
            lst_registry: pubkeys[9],
            lst_stake_pool_state: pubkeys[10],
            sanctum_calculator_program: pubkeys[11],
            sanctum_calculator_state: pubkeys[12],
            stake_pool_program_data: pubkeys[13],
            stake_pool_program: pubkeys[14],
            lut_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            token_program: pubkeys[17],
            system_program: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
        }
    }
}
impl<'info> From<RegisterLstAccounts<'_, 'info>>
for [AccountInfo<'info>; REGISTER_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: RegisterLstAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.lst_header.clone(),
            accounts.fee_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.registry_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.lst_vault.clone(),
            accounts.lst_mint.clone(),
            accounts.lst_registry.clone(),
            accounts.lst_stake_pool_state.clone(),
            accounts.sanctum_calculator_program.clone(),
            accounts.sanctum_calculator_state.clone(),
            accounts.stake_pool_program_data.clone(),
            accounts.stake_pool_program.clone(),
            accounts.lut_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REGISTER_LST_IX_ACCOUNTS_LEN]>
for RegisterLstAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REGISTER_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            lst_header: &arr[2],
            fee_auth: &arr[3],
            vault_auth: &arr[4],
            registry_auth: &arr[5],
            fee_vault: &arr[6],
            lst_vault: &arr[7],
            lst_mint: &arr[8],
            lst_registry: &arr[9],
            lst_stake_pool_state: &arr[10],
            sanctum_calculator_program: &arr[11],
            sanctum_calculator_state: &arr[12],
            stake_pool_program_data: &arr[13],
            stake_pool_program: &arr[14],
            lut_program: &arr[15],
            associated_token_program: &arr[16],
            token_program: &arr[17],
            system_program: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
        }
    }
}
pub const REGISTER_LST_IX_DISCM: [u8; 8usize] = [167, 114, 254, 24, 190, 221, 82, 107];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterLstIxArgs {
    pub rebalance_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterLstIxData(pub RegisterLstIxArgs);
impl From<RegisterLstIxArgs> for RegisterLstIxData {
    fn from(args: RegisterLstIxArgs) -> Self {
        Self(args)
    }
}
impl RegisterLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REGISTER_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let rebalance_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(Self(RegisterLstIxArgs { rebalance_fee }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REGISTER_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn register_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: RegisterLstKeys,
    args: RegisterLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REGISTER_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: RegisterLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn register_lst_ix(
    keys: RegisterLstKeys,
    args: RegisterLstIxArgs,
) -> std::io::Result<Instruction> {
    register_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn register_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RegisterLstAccounts<'_, '_>,
    args: RegisterLstIxArgs,
) -> ProgramResult {
    let keys: RegisterLstKeys = accounts.into();
    let ix = register_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn register_lst_invoke(
    accounts: RegisterLstAccounts<'_, '_>,
    args: RegisterLstIxArgs,
) -> ProgramResult {
    register_lst_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn register_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RegisterLstAccounts<'_, '_>,
    args: RegisterLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RegisterLstKeys = accounts.into();
    let ix = register_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn register_lst_invoke_signed(
    accounts: RegisterLstAccounts<'_, '_>,
    args: RegisterLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    register_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn register_lst_verify_account_keys(
    accounts: RegisterLstAccounts<'_, '_>,
    keys: RegisterLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.registry_auth.key, keys.registry_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.lst_registry.key, keys.lst_registry),
        (*accounts.lst_stake_pool_state.key, keys.lst_stake_pool_state),
        (*accounts.sanctum_calculator_program.key, keys.sanctum_calculator_program),
        (*accounts.sanctum_calculator_state.key, keys.sanctum_calculator_state),
        (*accounts.stake_pool_program_data.key, keys.stake_pool_program_data),
        (*accounts.stake_pool_program.key, keys.stake_pool_program),
        (*accounts.lut_program.key, keys.lut_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn register_lst_verify_writable_privileges<'me, 'info>(
    accounts: RegisterLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin,
        accounts.lst_header,
        accounts.fee_vault,
        accounts.lst_vault,
        accounts.lst_registry,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn register_lst_verify_signer_privileges<'me, 'info>(
    accounts: RegisterLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn register_lst_verify_account_privileges<'me, 'info>(
    accounts: RegisterLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    register_lst_verify_writable_privileges(accounts)?;
    register_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SettleVirtualStablecoinExoAccounts<'me, 'info> {
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinExoKeys {
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub pool_config: Pubkey,
    pub settlement_auth: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub collateral_vault: Pubkey,
    pub collateral_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SettleVirtualStablecoinExoAccounts<'_, '_>>
for SettleVirtualStablecoinExoKeys {
    fn from(accounts: SettleVirtualStablecoinExoAccounts) -> Self {
        Self {
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            pool_config: *accounts.pool_config.key,
            settlement_auth: *accounts.settlement_auth.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            pool_auth: *accounts.pool_auth.key,
            vault_auth: *accounts.vault_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            collateral_vault: *accounts.collateral_vault.key,
            collateral_mint: *accounts.collateral_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SettleVirtualStablecoinExoKeys>
for [AccountMeta; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleVirtualStablecoinExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN]>
for SettleVirtualStablecoinExoKeys {
    fn from(pubkeys: [Pubkey; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hylo: pubkeys[0],
            exo_pair: pubkeys[1],
            pool_config: pubkeys[2],
            settlement_auth: pubkeys[3],
            stablecoin_mint_auth: pubkeys[4],
            pool_auth: pubkeys[5],
            vault_auth: pubkeys[6],
            stablecoin_pool: pubkeys[7],
            collateral_vault: pubkeys[8],
            collateral_mint: pubkeys[9],
            stablecoin_mint: pubkeys[10],
            collateral_usd_pyth_feed: pubkeys[11],
            token_program: pubkeys[12],
            earn_pool: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<SettleVirtualStablecoinExoAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleVirtualStablecoinExoAccounts<'_, 'info>) -> Self {
        [
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.pool_config.clone(),
            accounts.settlement_auth.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.collateral_vault.clone(),
            accounts.collateral_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN]>
for SettleVirtualStablecoinExoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hylo: &arr[0],
            exo_pair: &arr[1],
            pool_config: &arr[2],
            settlement_auth: &arr[3],
            stablecoin_mint_auth: &arr[4],
            pool_auth: &arr[5],
            vault_auth: &arr[6],
            stablecoin_pool: &arr[7],
            collateral_vault: &arr[8],
            collateral_mint: &arr[9],
            stablecoin_mint: &arr[10],
            collateral_usd_pyth_feed: &arr[11],
            token_program: &arr[12],
            earn_pool: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const SETTLE_VIRTUAL_STABLECOIN_EXO_IX_DISCM: [u8; 8usize] = [
    92, 174, 253, 99, 154, 222, 222, 55,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinExoIxData;
impl SettleVirtualStablecoinExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_VIRTUAL_STABLECOIN_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_EXO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_virtual_stablecoin_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleVirtualStablecoinExoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_VIRTUAL_STABLECOIN_EXO_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleVirtualStablecoinExoIxData.try_to_vec()?,
    })
}
pub fn settle_virtual_stablecoin_exo_ix(
    keys: SettleVirtualStablecoinExoKeys,
) -> std::io::Result<Instruction> {
    settle_virtual_stablecoin_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn settle_virtual_stablecoin_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleVirtualStablecoinExoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleVirtualStablecoinExoKeys = accounts.into();
    let ix = settle_virtual_stablecoin_exo_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_virtual_stablecoin_exo_invoke(
    accounts: SettleVirtualStablecoinExoAccounts<'_, '_>,
) -> ProgramResult {
    settle_virtual_stablecoin_exo_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn settle_virtual_stablecoin_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleVirtualStablecoinExoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleVirtualStablecoinExoKeys = accounts.into();
    let ix = settle_virtual_stablecoin_exo_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_virtual_stablecoin_exo_invoke_signed(
    accounts: SettleVirtualStablecoinExoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_virtual_stablecoin_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn settle_virtual_stablecoin_exo_verify_account_keys(
    accounts: SettleVirtualStablecoinExoAccounts<'_, '_>,
    keys: SettleVirtualStablecoinExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_virtual_stablecoin_exo_verify_writable_privileges<'me, 'info>(
    accounts: SettleVirtualStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.exo_pair,
        accounts.stablecoin_pool,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_virtual_stablecoin_exo_verify_account_privileges<'me, 'info>(
    accounts: SettleVirtualStablecoinExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_virtual_stablecoin_exo_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SettleVirtualStablecoinLstAccounts<'me, 'info> {
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinLstKeys {
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub settlement_auth: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SettleVirtualStablecoinLstAccounts<'_, '_>>
for SettleVirtualStablecoinLstKeys {
    fn from(accounts: SettleVirtualStablecoinLstAccounts) -> Self {
        Self {
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            settlement_auth: *accounts.settlement_auth.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            pool_auth: *accounts.pool_auth.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SettleVirtualStablecoinLstKeys>
for [AccountMeta; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleVirtualStablecoinLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN]>
for SettleVirtualStablecoinLstKeys {
    fn from(pubkeys: [Pubkey; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hylo: pubkeys[0],
            pool_config: pubkeys[1],
            settlement_auth: pubkeys[2],
            stablecoin_mint_auth: pubkeys[3],
            pool_auth: pubkeys[4],
            stablecoin_pool: pubkeys[5],
            stablecoin_mint: pubkeys[6],
            sol_usd_pyth_feed: pubkeys[7],
            token_program: pubkeys[8],
            earn_pool: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<SettleVirtualStablecoinLstAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleVirtualStablecoinLstAccounts<'_, 'info>) -> Self {
        [
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.settlement_auth.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN]>
for SettleVirtualStablecoinLstAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hylo: &arr[0],
            pool_config: &arr[1],
            settlement_auth: &arr[2],
            stablecoin_mint_auth: &arr[3],
            pool_auth: &arr[4],
            stablecoin_pool: &arr[5],
            stablecoin_mint: &arr[6],
            sol_usd_pyth_feed: &arr[7],
            token_program: &arr[8],
            earn_pool: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const SETTLE_VIRTUAL_STABLECOIN_LST_IX_DISCM: [u8; 8usize] = [
    78, 145, 213, 121, 130, 237, 100, 139,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinLstIxData;
impl SettleVirtualStablecoinLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_VIRTUAL_STABLECOIN_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_LST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_virtual_stablecoin_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleVirtualStablecoinLstKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_VIRTUAL_STABLECOIN_LST_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleVirtualStablecoinLstIxData.try_to_vec()?,
    })
}
pub fn settle_virtual_stablecoin_lst_ix(
    keys: SettleVirtualStablecoinLstKeys,
) -> std::io::Result<Instruction> {
    settle_virtual_stablecoin_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn settle_virtual_stablecoin_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleVirtualStablecoinLstAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleVirtualStablecoinLstKeys = accounts.into();
    let ix = settle_virtual_stablecoin_lst_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_virtual_stablecoin_lst_invoke(
    accounts: SettleVirtualStablecoinLstAccounts<'_, '_>,
) -> ProgramResult {
    settle_virtual_stablecoin_lst_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn settle_virtual_stablecoin_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleVirtualStablecoinLstAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleVirtualStablecoinLstKeys = accounts.into();
    let ix = settle_virtual_stablecoin_lst_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_virtual_stablecoin_lst_invoke_signed(
    accounts: SettleVirtualStablecoinLstAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_virtual_stablecoin_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn settle_virtual_stablecoin_lst_verify_account_keys(
    accounts: SettleVirtualStablecoinLstAccounts<'_, '_>,
    keys: SettleVirtualStablecoinLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_virtual_stablecoin_lst_verify_writable_privileges<'me, 'info>(
    accounts: SettleVirtualStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.hylo,
        accounts.stablecoin_pool,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_virtual_stablecoin_lst_verify_account_privileges<'me, 'info>(
    accounts: SettleVirtualStablecoinLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_virtual_stablecoin_lst_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SettleVirtualStablecoinUsdcAccounts<'me, 'info> {
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinUsdcKeys {
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub pool_config: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SettleVirtualStablecoinUsdcAccounts<'_, '_>>
for SettleVirtualStablecoinUsdcKeys {
    fn from(accounts: SettleVirtualStablecoinUsdcAccounts) -> Self {
        Self {
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            pool_config: *accounts.pool_config.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            pool_auth: *accounts.pool_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SettleVirtualStablecoinUsdcKeys>
for [AccountMeta; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleVirtualStablecoinUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN]>
for SettleVirtualStablecoinUsdcKeys {
    fn from(pubkeys: [Pubkey; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            hylo: pubkeys[0],
            usdc_pair: pubkeys[1],
            pool_config: pubkeys[2],
            stablecoin_mint_auth: pubkeys[3],
            pool_auth: pubkeys[4],
            usdc_vault_auth: pubkeys[5],
            usdc_collateral_vault: pubkeys[6],
            stablecoin_pool: pubkeys[7],
            usdc_mint: pubkeys[8],
            stablecoin_mint: pubkeys[9],
            usdc_usd_pyth_feed: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SettleVirtualStablecoinUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleVirtualStablecoinUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.pool_config.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN]>
for SettleVirtualStablecoinUsdcAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            hylo: &arr[0],
            usdc_pair: &arr[1],
            pool_config: &arr[2],
            stablecoin_mint_auth: &arr[3],
            pool_auth: &arr[4],
            usdc_vault_auth: &arr[5],
            usdc_collateral_vault: &arr[6],
            stablecoin_pool: &arr[7],
            usdc_mint: &arr[8],
            stablecoin_mint: &arr[9],
            usdc_usd_pyth_feed: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SETTLE_VIRTUAL_STABLECOIN_USDC_IX_DISCM: [u8; 8usize] = [
    199, 72, 3, 127, 17, 169, 203, 49,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SettleVirtualStablecoinUsdcIxData;
impl SettleVirtualStablecoinUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_VIRTUAL_STABLECOIN_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_VIRTUAL_STABLECOIN_USDC_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_virtual_stablecoin_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleVirtualStablecoinUsdcKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_VIRTUAL_STABLECOIN_USDC_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SettleVirtualStablecoinUsdcIxData.try_to_vec()?,
    })
}
pub fn settle_virtual_stablecoin_usdc_ix(
    keys: SettleVirtualStablecoinUsdcKeys,
) -> std::io::Result<Instruction> {
    settle_virtual_stablecoin_usdc_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn settle_virtual_stablecoin_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleVirtualStablecoinUsdcAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SettleVirtualStablecoinUsdcKeys = accounts.into();
    let ix = settle_virtual_stablecoin_usdc_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_virtual_stablecoin_usdc_invoke(
    accounts: SettleVirtualStablecoinUsdcAccounts<'_, '_>,
) -> ProgramResult {
    settle_virtual_stablecoin_usdc_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
    )
}
pub fn settle_virtual_stablecoin_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleVirtualStablecoinUsdcAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleVirtualStablecoinUsdcKeys = accounts.into();
    let ix = settle_virtual_stablecoin_usdc_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_virtual_stablecoin_usdc_invoke_signed(
    accounts: SettleVirtualStablecoinUsdcAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_virtual_stablecoin_usdc_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn settle_virtual_stablecoin_usdc_verify_account_keys(
    accounts: SettleVirtualStablecoinUsdcAccounts<'_, '_>,
    keys: SettleVirtualStablecoinUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn settle_virtual_stablecoin_usdc_verify_writable_privileges<'me, 'info>(
    accounts: SettleVirtualStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.usdc_pair,
        accounts.stablecoin_pool,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_virtual_stablecoin_usdc_verify_account_privileges<'me, 'info>(
    accounts: SettleVirtualStablecoinUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_virtual_stablecoin_usdc_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapExoToUsdcAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExoToUsdcKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub exo_pair: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub settlement_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapExoToUsdcAccounts<'_, '_>> for SwapExoToUsdcKeys {
    fn from(accounts: SwapExoToUsdcAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            exo_pair: *accounts.exo_pair.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            vault_auth: *accounts.vault_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            pool_auth: *accounts.pool_auth.key,
            settlement_auth: *accounts.settlement_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapExoToUsdcKeys> for [AccountMeta; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExoToUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN]> for SwapExoToUsdcKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            exo_pair: pubkeys[3],
            usdc_pair: pubkeys[4],
            stablecoin_mint_auth: pubkeys[5],
            vault_auth: pubkeys[6],
            usdc_vault_auth: pubkeys[7],
            pool_auth: pubkeys[8],
            settlement_auth: pubkeys[9],
            collateral_vault: pubkeys[10],
            usdc_collateral_vault: pubkeys[11],
            stablecoin_pool: pubkeys[12],
            user_collateral_ta: pubkeys[13],
            user_usdc_ta: pubkeys[14],
            collateral_mint: pubkeys[15],
            usdc_mint: pubkeys[16],
            stablecoin_mint: pubkeys[17],
            levercoin_mint: pubkeys[18],
            collateral_usd_pyth_feed: pubkeys[19],
            usdc_usd_pyth_feed: pubkeys[20],
            token_program: pubkeys[21],
            earn_pool: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapExoToUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExoToUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.exo_pair.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.settlement_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN]>
for SwapExoToUsdcAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            exo_pair: &arr[3],
            usdc_pair: &arr[4],
            stablecoin_mint_auth: &arr[5],
            vault_auth: &arr[6],
            usdc_vault_auth: &arr[7],
            pool_auth: &arr[8],
            settlement_auth: &arr[9],
            collateral_vault: &arr[10],
            usdc_collateral_vault: &arr[11],
            stablecoin_pool: &arr[12],
            user_collateral_ta: &arr[13],
            user_usdc_ta: &arr[14],
            collateral_mint: &arr[15],
            usdc_mint: &arr[16],
            stablecoin_mint: &arr[17],
            levercoin_mint: &arr[18],
            collateral_usd_pyth_feed: &arr[19],
            usdc_usd_pyth_feed: &arr[20],
            token_program: &arr[21],
            earn_pool: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const SWAP_EXO_TO_USDC_IX_DISCM: [u8; 8usize] = [234, 79, 247, 244, 61, 125, 18, 51];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExoToUsdcIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExoToUsdcIxData(pub SwapExoToUsdcIxArgs);
impl From<SwapExoToUsdcIxArgs> for SwapExoToUsdcIxData {
    fn from(args: SwapExoToUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExoToUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXO_TO_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapExoToUsdcIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXO_TO_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exo_to_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExoToUsdcKeys,
    args: SwapExoToUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXO_TO_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExoToUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exo_to_usdc_ix(
    keys: SwapExoToUsdcKeys,
    args: SwapExoToUsdcIxArgs,
) -> std::io::Result<Instruction> {
    swap_exo_to_usdc_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_exo_to_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExoToUsdcAccounts<'_, '_>,
    args: SwapExoToUsdcIxArgs,
) -> ProgramResult {
    let keys: SwapExoToUsdcKeys = accounts.into();
    let ix = swap_exo_to_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exo_to_usdc_invoke(
    accounts: SwapExoToUsdcAccounts<'_, '_>,
    args: SwapExoToUsdcIxArgs,
) -> ProgramResult {
    swap_exo_to_usdc_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_exo_to_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExoToUsdcAccounts<'_, '_>,
    args: SwapExoToUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExoToUsdcKeys = accounts.into();
    let ix = swap_exo_to_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exo_to_usdc_invoke_signed(
    accounts: SwapExoToUsdcAccounts<'_, '_>,
    args: SwapExoToUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exo_to_usdc_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exo_to_usdc_verify_account_keys(
    accounts: SwapExoToUsdcAccounts<'_, '_>,
    keys: SwapExoToUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exo_to_usdc_verify_writable_privileges<'me, 'info>(
    accounts: SwapExoToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.exo_pair,
        accounts.usdc_pair,
        accounts.collateral_vault,
        accounts.usdc_collateral_vault,
        accounts.stablecoin_pool,
        accounts.user_collateral_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exo_to_usdc_verify_signer_privileges<'me, 'info>(
    accounts: SwapExoToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exo_to_usdc_verify_account_privileges<'me, 'info>(
    accounts: SwapExoToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exo_to_usdc_verify_writable_privileges(accounts)?;
    swap_exo_to_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapExoToUsdcAllAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExoToUsdcAllKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub exo_pair: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub settlement_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapExoToUsdcAllAccounts<'_, '_>> for SwapExoToUsdcAllKeys {
    fn from(accounts: SwapExoToUsdcAllAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            exo_pair: *accounts.exo_pair.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            vault_auth: *accounts.vault_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            pool_auth: *accounts.pool_auth.key,
            settlement_auth: *accounts.settlement_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapExoToUsdcAllKeys> for [AccountMeta; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExoToUsdcAllKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN]> for SwapExoToUsdcAllKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            exo_pair: pubkeys[3],
            usdc_pair: pubkeys[4],
            stablecoin_mint_auth: pubkeys[5],
            vault_auth: pubkeys[6],
            usdc_vault_auth: pubkeys[7],
            pool_auth: pubkeys[8],
            settlement_auth: pubkeys[9],
            collateral_vault: pubkeys[10],
            usdc_collateral_vault: pubkeys[11],
            stablecoin_pool: pubkeys[12],
            user_collateral_ta: pubkeys[13],
            user_usdc_ta: pubkeys[14],
            collateral_mint: pubkeys[15],
            usdc_mint: pubkeys[16],
            stablecoin_mint: pubkeys[17],
            levercoin_mint: pubkeys[18],
            collateral_usd_pyth_feed: pubkeys[19],
            usdc_usd_pyth_feed: pubkeys[20],
            token_program: pubkeys[21],
            earn_pool: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapExoToUsdcAllAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExoToUsdcAllAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.exo_pair.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.settlement_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN]>
for SwapExoToUsdcAllAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            exo_pair: &arr[3],
            usdc_pair: &arr[4],
            stablecoin_mint_auth: &arr[5],
            vault_auth: &arr[6],
            usdc_vault_auth: &arr[7],
            pool_auth: &arr[8],
            settlement_auth: &arr[9],
            collateral_vault: &arr[10],
            usdc_collateral_vault: &arr[11],
            stablecoin_pool: &arr[12],
            user_collateral_ta: &arr[13],
            user_usdc_ta: &arr[14],
            collateral_mint: &arr[15],
            usdc_mint: &arr[16],
            stablecoin_mint: &arr[17],
            levercoin_mint: &arr[18],
            collateral_usd_pyth_feed: &arr[19],
            usdc_usd_pyth_feed: &arr[20],
            token_program: &arr[21],
            earn_pool: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const SWAP_EXO_TO_USDC_ALL_IX_DISCM: [u8; 8usize] = [
    107, 69, 100, 203, 188, 190, 159, 74,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapExoToUsdcAllIxArgs {
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExoToUsdcAllIxData(pub SwapExoToUsdcAllIxArgs);
impl From<SwapExoToUsdcAllIxArgs> for SwapExoToUsdcAllIxData {
    fn from(args: SwapExoToUsdcAllIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExoToUsdcAllIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXO_TO_USDC_ALL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapExoToUsdcAllIxArgs {
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXO_TO_USDC_ALL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exo_to_usdc_all_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExoToUsdcAllKeys,
    args: SwapExoToUsdcAllIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXO_TO_USDC_ALL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExoToUsdcAllIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exo_to_usdc_all_ix(
    keys: SwapExoToUsdcAllKeys,
    args: SwapExoToUsdcAllIxArgs,
) -> std::io::Result<Instruction> {
    swap_exo_to_usdc_all_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_exo_to_usdc_all_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExoToUsdcAllAccounts<'_, '_>,
    args: SwapExoToUsdcAllIxArgs,
) -> ProgramResult {
    let keys: SwapExoToUsdcAllKeys = accounts.into();
    let ix = swap_exo_to_usdc_all_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exo_to_usdc_all_invoke(
    accounts: SwapExoToUsdcAllAccounts<'_, '_>,
    args: SwapExoToUsdcAllIxArgs,
) -> ProgramResult {
    swap_exo_to_usdc_all_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_exo_to_usdc_all_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExoToUsdcAllAccounts<'_, '_>,
    args: SwapExoToUsdcAllIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExoToUsdcAllKeys = accounts.into();
    let ix = swap_exo_to_usdc_all_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exo_to_usdc_all_invoke_signed(
    accounts: SwapExoToUsdcAllAccounts<'_, '_>,
    args: SwapExoToUsdcAllIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exo_to_usdc_all_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_exo_to_usdc_all_verify_account_keys(
    accounts: SwapExoToUsdcAllAccounts<'_, '_>,
    keys: SwapExoToUsdcAllKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exo_to_usdc_all_verify_writable_privileges<'me, 'info>(
    accounts: SwapExoToUsdcAllAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.exo_pair,
        accounts.usdc_pair,
        accounts.collateral_vault,
        accounts.usdc_collateral_vault,
        accounts.stablecoin_pool,
        accounts.user_collateral_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exo_to_usdc_all_verify_signer_privileges<'me, 'info>(
    accounts: SwapExoToUsdcAllAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exo_to_usdc_all_verify_account_privileges<'me, 'info>(
    accounts: SwapExoToUsdcAllAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exo_to_usdc_all_verify_writable_privileges(accounts)?;
    swap_exo_to_usdc_all_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_LST_TO_LST_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SwapLstToLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub lst_a_mint: &'me AccountInfo<'info>,
    pub lst_a_user_ta: &'me AccountInfo<'info>,
    pub lst_a_vault_auth: &'me AccountInfo<'info>,
    pub lst_a_vault: &'me AccountInfo<'info>,
    pub lst_a_header: &'me AccountInfo<'info>,
    pub lst_b_mint: &'me AccountInfo<'info>,
    pub lst_b_user_ta: &'me AccountInfo<'info>,
    pub lst_b_vault_auth: &'me AccountInfo<'info>,
    pub lst_b_vault: &'me AccountInfo<'info>,
    pub lst_b_header: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapLstToLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub lst_a_mint: Pubkey,
    pub lst_a_user_ta: Pubkey,
    pub lst_a_vault_auth: Pubkey,
    pub lst_a_vault: Pubkey,
    pub lst_a_header: Pubkey,
    pub lst_b_mint: Pubkey,
    pub lst_b_user_ta: Pubkey,
    pub lst_b_vault_auth: Pubkey,
    pub lst_b_vault: Pubkey,
    pub lst_b_header: Pubkey,
    pub fee_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapLstToLstAccounts<'_, '_>> for SwapLstToLstKeys {
    fn from(accounts: SwapLstToLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            lst_a_mint: *accounts.lst_a_mint.key,
            lst_a_user_ta: *accounts.lst_a_user_ta.key,
            lst_a_vault_auth: *accounts.lst_a_vault_auth.key,
            lst_a_vault: *accounts.lst_a_vault.key,
            lst_a_header: *accounts.lst_a_header.key,
            lst_b_mint: *accounts.lst_b_mint.key,
            lst_b_user_ta: *accounts.lst_b_user_ta.key,
            lst_b_vault_auth: *accounts.lst_b_vault_auth.key,
            lst_b_vault: *accounts.lst_b_vault.key,
            lst_b_header: *accounts.lst_b_header.key,
            fee_auth: *accounts.fee_auth.key,
            fee_vault: *accounts.fee_vault.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapLstToLstKeys> for [AccountMeta; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapLstToLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_a_user_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_a_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_a_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_b_user_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_b_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_b_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN]> for SwapLstToLstKeys {
    fn from(pubkeys: [Pubkey; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            lst_a_mint: pubkeys[2],
            lst_a_user_ta: pubkeys[3],
            lst_a_vault_auth: pubkeys[4],
            lst_a_vault: pubkeys[5],
            lst_a_header: pubkeys[6],
            lst_b_mint: pubkeys[7],
            lst_b_user_ta: pubkeys[8],
            lst_b_vault_auth: pubkeys[9],
            lst_b_vault: pubkeys[10],
            lst_b_header: pubkeys[11],
            fee_auth: pubkeys[12],
            fee_vault: pubkeys[13],
            token_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<SwapLstToLstAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapLstToLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.lst_a_mint.clone(),
            accounts.lst_a_user_ta.clone(),
            accounts.lst_a_vault_auth.clone(),
            accounts.lst_a_vault.clone(),
            accounts.lst_a_header.clone(),
            accounts.lst_b_mint.clone(),
            accounts.lst_b_user_ta.clone(),
            accounts.lst_b_vault_auth.clone(),
            accounts.lst_b_vault.clone(),
            accounts.lst_b_header.clone(),
            accounts.fee_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN]>
for SwapLstToLstAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            lst_a_mint: &arr[2],
            lst_a_user_ta: &arr[3],
            lst_a_vault_auth: &arr[4],
            lst_a_vault: &arr[5],
            lst_a_header: &arr[6],
            lst_b_mint: &arr[7],
            lst_b_user_ta: &arr[8],
            lst_b_vault_auth: &arr[9],
            lst_b_vault: &arr[10],
            lst_b_header: &arr[11],
            fee_auth: &arr[12],
            fee_vault: &arr[13],
            token_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const SWAP_LST_TO_LST_IX_DISCM: [u8; 8usize] = [105, 39, 155, 165, 242, 2, 235, 99];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapLstToLstIxArgs {
    pub amount_lst_a: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapLstToLstIxData(pub SwapLstToLstIxArgs);
impl From<SwapLstToLstIxArgs> for SwapLstToLstIxData {
    fn from(args: SwapLstToLstIxArgs) -> Self {
        Self(args)
    }
}
impl SwapLstToLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_LST_TO_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_lst_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapLstToLstIxArgs {
                amount_lst_a,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_LST_TO_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_lst_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_lst_to_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapLstToLstKeys,
    args: SwapLstToLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_LST_TO_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapLstToLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_lst_to_lst_ix(
    keys: SwapLstToLstKeys,
    args: SwapLstToLstIxArgs,
) -> std::io::Result<Instruction> {
    swap_lst_to_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_lst_to_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapLstToLstAccounts<'_, '_>,
    args: SwapLstToLstIxArgs,
) -> ProgramResult {
    let keys: SwapLstToLstKeys = accounts.into();
    let ix = swap_lst_to_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_lst_to_lst_invoke(
    accounts: SwapLstToLstAccounts<'_, '_>,
    args: SwapLstToLstIxArgs,
) -> ProgramResult {
    swap_lst_to_lst_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_lst_to_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapLstToLstAccounts<'_, '_>,
    args: SwapLstToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapLstToLstKeys = accounts.into();
    let ix = swap_lst_to_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_lst_to_lst_invoke_signed(
    accounts: SwapLstToLstAccounts<'_, '_>,
    args: SwapLstToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_lst_to_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_lst_to_lst_verify_account_keys(
    accounts: SwapLstToLstAccounts<'_, '_>,
    keys: SwapLstToLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.lst_a_mint.key, keys.lst_a_mint),
        (*accounts.lst_a_user_ta.key, keys.lst_a_user_ta),
        (*accounts.lst_a_vault_auth.key, keys.lst_a_vault_auth),
        (*accounts.lst_a_vault.key, keys.lst_a_vault),
        (*accounts.lst_a_header.key, keys.lst_a_header),
        (*accounts.lst_b_mint.key, keys.lst_b_mint),
        (*accounts.lst_b_user_ta.key, keys.lst_b_user_ta),
        (*accounts.lst_b_vault_auth.key, keys.lst_b_vault_auth),
        (*accounts.lst_b_vault.key, keys.lst_b_vault),
        (*accounts.lst_b_header.key, keys.lst_b_header),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_lst_to_lst_verify_writable_privileges<'me, 'info>(
    accounts: SwapLstToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.hylo,
        accounts.lst_a_user_ta,
        accounts.lst_a_vault,
        accounts.lst_b_user_ta,
        accounts.lst_b_vault,
        accounts.fee_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_lst_to_lst_verify_signer_privileges<'me, 'info>(
    accounts: SwapLstToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_lst_to_lst_verify_account_privileges<'me, 'info>(
    accounts: SwapLstToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_lst_to_lst_verify_writable_privileges(accounts)?;
    swap_lst_to_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapLstToUsdcAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub lst_vault_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapLstToUsdcKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub lst_header: Pubkey,
    pub pool_state: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub lst_vault_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub settlement_auth: Pubkey,
    pub lst_vault: Pubkey,
    pub usdc_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub user_lst_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub lst_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapLstToUsdcAccounts<'_, '_>> for SwapLstToUsdcKeys {
    fn from(accounts: SwapLstToUsdcAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            lst_header: *accounts.lst_header.key,
            pool_state: *accounts.pool_state.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            lst_vault_auth: *accounts.lst_vault_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            pool_auth: *accounts.pool_auth.key,
            settlement_auth: *accounts.settlement_auth.key,
            lst_vault: *accounts.lst_vault.key,
            usdc_vault: *accounts.usdc_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            lst_mint: *accounts.lst_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapLstToUsdcKeys> for [AccountMeta; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapLstToUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN]> for SwapLstToUsdcKeys {
    fn from(pubkeys: [Pubkey; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            lst_header: pubkeys[3],
            pool_state: pubkeys[4],
            usdc_pair: pubkeys[5],
            stablecoin_mint_auth: pubkeys[6],
            lst_vault_auth: pubkeys[7],
            usdc_vault_auth: pubkeys[8],
            pool_auth: pubkeys[9],
            settlement_auth: pubkeys[10],
            lst_vault: pubkeys[11],
            usdc_vault: pubkeys[12],
            stablecoin_pool: pubkeys[13],
            user_lst_ta: pubkeys[14],
            user_usdc_ta: pubkeys[15],
            lst_mint: pubkeys[16],
            usdc_mint: pubkeys[17],
            stablecoin_mint: pubkeys[18],
            sol_usd_pyth_feed: pubkeys[19],
            usdc_usd_pyth_feed: pubkeys[20],
            token_program: pubkeys[21],
            earn_pool: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapLstToUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapLstToUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.lst_header.clone(),
            accounts.pool_state.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.lst_vault_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.settlement_auth.clone(),
            accounts.lst_vault.clone(),
            accounts.usdc_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.user_lst_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.lst_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN]>
for SwapLstToUsdcAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            lst_header: &arr[3],
            pool_state: &arr[4],
            usdc_pair: &arr[5],
            stablecoin_mint_auth: &arr[6],
            lst_vault_auth: &arr[7],
            usdc_vault_auth: &arr[8],
            pool_auth: &arr[9],
            settlement_auth: &arr[10],
            lst_vault: &arr[11],
            usdc_vault: &arr[12],
            stablecoin_pool: &arr[13],
            user_lst_ta: &arr[14],
            user_usdc_ta: &arr[15],
            lst_mint: &arr[16],
            usdc_mint: &arr[17],
            stablecoin_mint: &arr[18],
            sol_usd_pyth_feed: &arr[19],
            usdc_usd_pyth_feed: &arr[20],
            token_program: &arr[21],
            earn_pool: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const SWAP_LST_TO_USDC_IX_DISCM: [u8; 8usize] = [
    102, 186, 134, 205, 139, 114, 218, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapLstToUsdcIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapLstToUsdcIxData(pub SwapLstToUsdcIxArgs);
impl From<SwapLstToUsdcIxArgs> for SwapLstToUsdcIxData {
    fn from(args: SwapLstToUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl SwapLstToUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_LST_TO_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapLstToUsdcIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_LST_TO_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_lst_to_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapLstToUsdcKeys,
    args: SwapLstToUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_LST_TO_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapLstToUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_lst_to_usdc_ix(
    keys: SwapLstToUsdcKeys,
    args: SwapLstToUsdcIxArgs,
) -> std::io::Result<Instruction> {
    swap_lst_to_usdc_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_lst_to_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapLstToUsdcAccounts<'_, '_>,
    args: SwapLstToUsdcIxArgs,
) -> ProgramResult {
    let keys: SwapLstToUsdcKeys = accounts.into();
    let ix = swap_lst_to_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_lst_to_usdc_invoke(
    accounts: SwapLstToUsdcAccounts<'_, '_>,
    args: SwapLstToUsdcIxArgs,
) -> ProgramResult {
    swap_lst_to_usdc_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_lst_to_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapLstToUsdcAccounts<'_, '_>,
    args: SwapLstToUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapLstToUsdcKeys = accounts.into();
    let ix = swap_lst_to_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_lst_to_usdc_invoke_signed(
    accounts: SwapLstToUsdcAccounts<'_, '_>,
    args: SwapLstToUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_lst_to_usdc_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_lst_to_usdc_verify_account_keys(
    accounts: SwapLstToUsdcAccounts<'_, '_>,
    keys: SwapLstToUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.lst_vault_auth.key, keys.lst_vault_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_lst_to_usdc_verify_writable_privileges<'me, 'info>(
    accounts: SwapLstToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.hylo,
        accounts.usdc_pair,
        accounts.lst_vault,
        accounts.usdc_vault,
        accounts.stablecoin_pool,
        accounts.user_lst_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_lst_to_usdc_verify_signer_privileges<'me, 'info>(
    accounts: SwapLstToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_lst_to_usdc_verify_account_privileges<'me, 'info>(
    accounts: SwapLstToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_lst_to_usdc_verify_writable_privileges(accounts)?;
    swap_lst_to_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapLstToUsdcAllAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub lst_vault_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapLstToUsdcAllKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub lst_header: Pubkey,
    pub pool_state: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub lst_vault_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub settlement_auth: Pubkey,
    pub lst_vault: Pubkey,
    pub usdc_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub user_lst_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub lst_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapLstToUsdcAllAccounts<'_, '_>> for SwapLstToUsdcAllKeys {
    fn from(accounts: SwapLstToUsdcAllAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            lst_header: *accounts.lst_header.key,
            pool_state: *accounts.pool_state.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            lst_vault_auth: *accounts.lst_vault_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            pool_auth: *accounts.pool_auth.key,
            settlement_auth: *accounts.settlement_auth.key,
            lst_vault: *accounts.lst_vault.key,
            usdc_vault: *accounts.usdc_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            lst_mint: *accounts.lst_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapLstToUsdcAllKeys> for [AccountMeta; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapLstToUsdcAllKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN]> for SwapLstToUsdcAllKeys {
    fn from(pubkeys: [Pubkey; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            lst_header: pubkeys[3],
            pool_state: pubkeys[4],
            usdc_pair: pubkeys[5],
            stablecoin_mint_auth: pubkeys[6],
            lst_vault_auth: pubkeys[7],
            usdc_vault_auth: pubkeys[8],
            pool_auth: pubkeys[9],
            settlement_auth: pubkeys[10],
            lst_vault: pubkeys[11],
            usdc_vault: pubkeys[12],
            stablecoin_pool: pubkeys[13],
            user_lst_ta: pubkeys[14],
            user_usdc_ta: pubkeys[15],
            lst_mint: pubkeys[16],
            usdc_mint: pubkeys[17],
            stablecoin_mint: pubkeys[18],
            sol_usd_pyth_feed: pubkeys[19],
            usdc_usd_pyth_feed: pubkeys[20],
            token_program: pubkeys[21],
            earn_pool: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapLstToUsdcAllAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapLstToUsdcAllAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.lst_header.clone(),
            accounts.pool_state.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.lst_vault_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.settlement_auth.clone(),
            accounts.lst_vault.clone(),
            accounts.usdc_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.user_lst_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.lst_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN]>
for SwapLstToUsdcAllAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            lst_header: &arr[3],
            pool_state: &arr[4],
            usdc_pair: &arr[5],
            stablecoin_mint_auth: &arr[6],
            lst_vault_auth: &arr[7],
            usdc_vault_auth: &arr[8],
            pool_auth: &arr[9],
            settlement_auth: &arr[10],
            lst_vault: &arr[11],
            usdc_vault: &arr[12],
            stablecoin_pool: &arr[13],
            user_lst_ta: &arr[14],
            user_usdc_ta: &arr[15],
            lst_mint: &arr[16],
            usdc_mint: &arr[17],
            stablecoin_mint: &arr[18],
            sol_usd_pyth_feed: &arr[19],
            usdc_usd_pyth_feed: &arr[20],
            token_program: &arr[21],
            earn_pool: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const SWAP_LST_TO_USDC_ALL_IX_DISCM: [u8; 8usize] = [
    10, 169, 147, 185, 122, 163, 185, 119,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapLstToUsdcAllIxArgs {
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapLstToUsdcAllIxData(pub SwapLstToUsdcAllIxArgs);
impl From<SwapLstToUsdcAllIxArgs> for SwapLstToUsdcAllIxData {
    fn from(args: SwapLstToUsdcAllIxArgs) -> Self {
        Self(args)
    }
}
impl SwapLstToUsdcAllIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_LST_TO_USDC_ALL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapLstToUsdcAllIxArgs {
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_LST_TO_USDC_ALL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_lst_to_usdc_all_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapLstToUsdcAllKeys,
    args: SwapLstToUsdcAllIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_LST_TO_USDC_ALL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapLstToUsdcAllIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_lst_to_usdc_all_ix(
    keys: SwapLstToUsdcAllKeys,
    args: SwapLstToUsdcAllIxArgs,
) -> std::io::Result<Instruction> {
    swap_lst_to_usdc_all_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_lst_to_usdc_all_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapLstToUsdcAllAccounts<'_, '_>,
    args: SwapLstToUsdcAllIxArgs,
) -> ProgramResult {
    let keys: SwapLstToUsdcAllKeys = accounts.into();
    let ix = swap_lst_to_usdc_all_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_lst_to_usdc_all_invoke(
    accounts: SwapLstToUsdcAllAccounts<'_, '_>,
    args: SwapLstToUsdcAllIxArgs,
) -> ProgramResult {
    swap_lst_to_usdc_all_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_lst_to_usdc_all_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapLstToUsdcAllAccounts<'_, '_>,
    args: SwapLstToUsdcAllIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapLstToUsdcAllKeys = accounts.into();
    let ix = swap_lst_to_usdc_all_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_lst_to_usdc_all_invoke_signed(
    accounts: SwapLstToUsdcAllAccounts<'_, '_>,
    args: SwapLstToUsdcAllIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_lst_to_usdc_all_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_lst_to_usdc_all_verify_account_keys(
    accounts: SwapLstToUsdcAllAccounts<'_, '_>,
    keys: SwapLstToUsdcAllKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.lst_vault_auth.key, keys.lst_vault_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_lst_to_usdc_all_verify_writable_privileges<'me, 'info>(
    accounts: SwapLstToUsdcAllAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.hylo,
        accounts.usdc_pair,
        accounts.lst_vault,
        accounts.usdc_vault,
        accounts.stablecoin_pool,
        accounts.user_lst_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_lst_to_usdc_all_verify_signer_privileges<'me, 'info>(
    accounts: SwapLstToUsdcAllAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_lst_to_usdc_all_verify_account_privileges<'me, 'info>(
    accounts: SwapLstToUsdcAllAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_lst_to_usdc_all_verify_writable_privileges(accounts)?;
    swap_lst_to_usdc_all_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapUsdcToExoAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub vault_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub usdc_collateral_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub user_collateral_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub levercoin_mint: &'me AccountInfo<'info>,
    pub collateral_usd_pyth_feed: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapUsdcToExoKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub exo_pair: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub vault_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub settlement_auth: Pubkey,
    pub collateral_vault: Pubkey,
    pub usdc_collateral_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub user_collateral_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub collateral_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub collateral_usd_pyth_feed: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapUsdcToExoAccounts<'_, '_>> for SwapUsdcToExoKeys {
    fn from(accounts: SwapUsdcToExoAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            exo_pair: *accounts.exo_pair.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            vault_auth: *accounts.vault_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            pool_auth: *accounts.pool_auth.key,
            settlement_auth: *accounts.settlement_auth.key,
            collateral_vault: *accounts.collateral_vault.key,
            usdc_collateral_vault: *accounts.usdc_collateral_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            user_collateral_ta: *accounts.user_collateral_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            collateral_mint: *accounts.collateral_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            levercoin_mint: *accounts.levercoin_mint.key,
            collateral_usd_pyth_feed: *accounts.collateral_usd_pyth_feed.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapUsdcToExoKeys> for [AccountMeta; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapUsdcToExoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.levercoin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN]> for SwapUsdcToExoKeys {
    fn from(pubkeys: [Pubkey; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            exo_pair: pubkeys[3],
            usdc_pair: pubkeys[4],
            stablecoin_mint_auth: pubkeys[5],
            vault_auth: pubkeys[6],
            usdc_vault_auth: pubkeys[7],
            pool_auth: pubkeys[8],
            settlement_auth: pubkeys[9],
            collateral_vault: pubkeys[10],
            usdc_collateral_vault: pubkeys[11],
            stablecoin_pool: pubkeys[12],
            user_collateral_ta: pubkeys[13],
            user_usdc_ta: pubkeys[14],
            collateral_mint: pubkeys[15],
            usdc_mint: pubkeys[16],
            stablecoin_mint: pubkeys[17],
            levercoin_mint: pubkeys[18],
            collateral_usd_pyth_feed: pubkeys[19],
            usdc_usd_pyth_feed: pubkeys[20],
            token_program: pubkeys[21],
            earn_pool: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapUsdcToExoAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapUsdcToExoAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.exo_pair.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.vault_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.settlement_auth.clone(),
            accounts.collateral_vault.clone(),
            accounts.usdc_collateral_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.user_collateral_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.collateral_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.levercoin_mint.clone(),
            accounts.collateral_usd_pyth_feed.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN]>
for SwapUsdcToExoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            exo_pair: &arr[3],
            usdc_pair: &arr[4],
            stablecoin_mint_auth: &arr[5],
            vault_auth: &arr[6],
            usdc_vault_auth: &arr[7],
            pool_auth: &arr[8],
            settlement_auth: &arr[9],
            collateral_vault: &arr[10],
            usdc_collateral_vault: &arr[11],
            stablecoin_pool: &arr[12],
            user_collateral_ta: &arr[13],
            user_usdc_ta: &arr[14],
            collateral_mint: &arr[15],
            usdc_mint: &arr[16],
            stablecoin_mint: &arr[17],
            levercoin_mint: &arr[18],
            collateral_usd_pyth_feed: &arr[19],
            usdc_usd_pyth_feed: &arr[20],
            token_program: &arr[21],
            earn_pool: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const SWAP_USDC_TO_EXO_IX_DISCM: [u8; 8usize] = [
    14, 223, 70, 192, 229, 196, 113, 166,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapUsdcToExoIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapUsdcToExoIxData(pub SwapUsdcToExoIxArgs);
impl From<SwapUsdcToExoIxArgs> for SwapUsdcToExoIxData {
    fn from(args: SwapUsdcToExoIxArgs) -> Self {
        Self(args)
    }
}
impl SwapUsdcToExoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_USDC_TO_EXO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapUsdcToExoIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_USDC_TO_EXO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_usdc_to_exo_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapUsdcToExoKeys,
    args: SwapUsdcToExoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_USDC_TO_EXO_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapUsdcToExoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_usdc_to_exo_ix(
    keys: SwapUsdcToExoKeys,
    args: SwapUsdcToExoIxArgs,
) -> std::io::Result<Instruction> {
    swap_usdc_to_exo_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_usdc_to_exo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapUsdcToExoAccounts<'_, '_>,
    args: SwapUsdcToExoIxArgs,
) -> ProgramResult {
    let keys: SwapUsdcToExoKeys = accounts.into();
    let ix = swap_usdc_to_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_usdc_to_exo_invoke(
    accounts: SwapUsdcToExoAccounts<'_, '_>,
    args: SwapUsdcToExoIxArgs,
) -> ProgramResult {
    swap_usdc_to_exo_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_usdc_to_exo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapUsdcToExoAccounts<'_, '_>,
    args: SwapUsdcToExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapUsdcToExoKeys = accounts.into();
    let ix = swap_usdc_to_exo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_usdc_to_exo_invoke_signed(
    accounts: SwapUsdcToExoAccounts<'_, '_>,
    args: SwapUsdcToExoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_usdc_to_exo_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_usdc_to_exo_verify_account_keys(
    accounts: SwapUsdcToExoAccounts<'_, '_>,
    keys: SwapUsdcToExoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.vault_auth.key, keys.vault_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (*accounts.usdc_collateral_vault.key, keys.usdc_collateral_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.user_collateral_ta.key, keys.user_collateral_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.levercoin_mint.key, keys.levercoin_mint),
        (*accounts.collateral_usd_pyth_feed.key, keys.collateral_usd_pyth_feed),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_usdc_to_exo_verify_writable_privileges<'me, 'info>(
    accounts: SwapUsdcToExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.exo_pair,
        accounts.usdc_pair,
        accounts.collateral_vault,
        accounts.usdc_collateral_vault,
        accounts.stablecoin_pool,
        accounts.user_collateral_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_usdc_to_exo_verify_signer_privileges<'me, 'info>(
    accounts: SwapUsdcToExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_usdc_to_exo_verify_account_privileges<'me, 'info>(
    accounts: SwapUsdcToExoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_usdc_to_exo_verify_writable_privileges(accounts)?;
    swap_usdc_to_exo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapUsdcToLstAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub pool_config: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub stablecoin_mint_auth: &'me AccountInfo<'info>,
    pub lst_vault_auth: &'me AccountInfo<'info>,
    pub usdc_vault_auth: &'me AccountInfo<'info>,
    pub pool_auth: &'me AccountInfo<'info>,
    pub settlement_auth: &'me AccountInfo<'info>,
    pub lst_vault: &'me AccountInfo<'info>,
    pub usdc_vault: &'me AccountInfo<'info>,
    pub stablecoin_pool: &'me AccountInfo<'info>,
    pub user_lst_ta: &'me AccountInfo<'info>,
    pub user_usdc_ta: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub usdc_mint: &'me AccountInfo<'info>,
    pub stablecoin_mint: &'me AccountInfo<'info>,
    pub sol_usd_pyth_feed: &'me AccountInfo<'info>,
    pub usdc_usd_pyth_feed: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub earn_pool: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapUsdcToLstKeys {
    pub user: Pubkey,
    pub hylo: Pubkey,
    pub pool_config: Pubkey,
    pub lst_header: Pubkey,
    pub pool_state: Pubkey,
    pub usdc_pair: Pubkey,
    pub stablecoin_mint_auth: Pubkey,
    pub lst_vault_auth: Pubkey,
    pub usdc_vault_auth: Pubkey,
    pub pool_auth: Pubkey,
    pub settlement_auth: Pubkey,
    pub lst_vault: Pubkey,
    pub usdc_vault: Pubkey,
    pub stablecoin_pool: Pubkey,
    pub user_lst_ta: Pubkey,
    pub user_usdc_ta: Pubkey,
    pub lst_mint: Pubkey,
    pub usdc_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub sol_usd_pyth_feed: Pubkey,
    pub usdc_usd_pyth_feed: Pubkey,
    pub token_program: Pubkey,
    pub earn_pool: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapUsdcToLstAccounts<'_, '_>> for SwapUsdcToLstKeys {
    fn from(accounts: SwapUsdcToLstAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            hylo: *accounts.hylo.key,
            pool_config: *accounts.pool_config.key,
            lst_header: *accounts.lst_header.key,
            pool_state: *accounts.pool_state.key,
            usdc_pair: *accounts.usdc_pair.key,
            stablecoin_mint_auth: *accounts.stablecoin_mint_auth.key,
            lst_vault_auth: *accounts.lst_vault_auth.key,
            usdc_vault_auth: *accounts.usdc_vault_auth.key,
            pool_auth: *accounts.pool_auth.key,
            settlement_auth: *accounts.settlement_auth.key,
            lst_vault: *accounts.lst_vault.key,
            usdc_vault: *accounts.usdc_vault.key,
            stablecoin_pool: *accounts.stablecoin_pool.key,
            user_lst_ta: *accounts.user_lst_ta.key,
            user_usdc_ta: *accounts.user_usdc_ta.key,
            lst_mint: *accounts.lst_mint.key,
            usdc_mint: *accounts.usdc_mint.key,
            stablecoin_mint: *accounts.stablecoin_mint.key,
            sol_usd_pyth_feed: *accounts.sol_usd_pyth_feed.key,
            usdc_usd_pyth_feed: *accounts.usdc_usd_pyth_feed.key,
            token_program: *accounts.token_program.key,
            earn_pool: *accounts.earn_pool.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapUsdcToLstKeys> for [AccountMeta; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapUsdcToLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_vault_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.settlement_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.usdc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stablecoin_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lst_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_usdc_ta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stablecoin_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_usd_pyth_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.earn_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN]> for SwapUsdcToLstKeys {
    fn from(pubkeys: [Pubkey; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            hylo: pubkeys[1],
            pool_config: pubkeys[2],
            lst_header: pubkeys[3],
            pool_state: pubkeys[4],
            usdc_pair: pubkeys[5],
            stablecoin_mint_auth: pubkeys[6],
            lst_vault_auth: pubkeys[7],
            usdc_vault_auth: pubkeys[8],
            pool_auth: pubkeys[9],
            settlement_auth: pubkeys[10],
            lst_vault: pubkeys[11],
            usdc_vault: pubkeys[12],
            stablecoin_pool: pubkeys[13],
            user_lst_ta: pubkeys[14],
            user_usdc_ta: pubkeys[15],
            lst_mint: pubkeys[16],
            usdc_mint: pubkeys[17],
            stablecoin_mint: pubkeys[18],
            sol_usd_pyth_feed: pubkeys[19],
            usdc_usd_pyth_feed: pubkeys[20],
            token_program: pubkeys[21],
            earn_pool: pubkeys[22],
            event_authority: pubkeys[23],
            program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapUsdcToLstAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapUsdcToLstAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.hylo.clone(),
            accounts.pool_config.clone(),
            accounts.lst_header.clone(),
            accounts.pool_state.clone(),
            accounts.usdc_pair.clone(),
            accounts.stablecoin_mint_auth.clone(),
            accounts.lst_vault_auth.clone(),
            accounts.usdc_vault_auth.clone(),
            accounts.pool_auth.clone(),
            accounts.settlement_auth.clone(),
            accounts.lst_vault.clone(),
            accounts.usdc_vault.clone(),
            accounts.stablecoin_pool.clone(),
            accounts.user_lst_ta.clone(),
            accounts.user_usdc_ta.clone(),
            accounts.lst_mint.clone(),
            accounts.usdc_mint.clone(),
            accounts.stablecoin_mint.clone(),
            accounts.sol_usd_pyth_feed.clone(),
            accounts.usdc_usd_pyth_feed.clone(),
            accounts.token_program.clone(),
            accounts.earn_pool.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN]>
for SwapUsdcToLstAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            hylo: &arr[1],
            pool_config: &arr[2],
            lst_header: &arr[3],
            pool_state: &arr[4],
            usdc_pair: &arr[5],
            stablecoin_mint_auth: &arr[6],
            lst_vault_auth: &arr[7],
            usdc_vault_auth: &arr[8],
            pool_auth: &arr[9],
            settlement_auth: &arr[10],
            lst_vault: &arr[11],
            usdc_vault: &arr[12],
            stablecoin_pool: &arr[13],
            user_lst_ta: &arr[14],
            user_usdc_ta: &arr[15],
            lst_mint: &arr[16],
            usdc_mint: &arr[17],
            stablecoin_mint: &arr[18],
            sol_usd_pyth_feed: &arr[19],
            usdc_usd_pyth_feed: &arr[20],
            token_program: &arr[21],
            earn_pool: &arr[22],
            event_authority: &arr[23],
            program: &arr[24],
        }
    }
}
pub const SWAP_USDC_TO_LST_IX_DISCM: [u8; 8usize] = [
    207, 138, 100, 190, 125, 72, 37, 64,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapUsdcToLstIxArgs {
    pub amount: u64,
    pub slippage_config: Option<SlippageConfig>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapUsdcToLstIxData(pub SwapUsdcToLstIxArgs);
impl From<SwapUsdcToLstIxArgs> for SwapUsdcToLstIxData {
    fn from(args: SwapUsdcToLstIxArgs) -> Self {
        Self(args)
    }
}
impl SwapUsdcToLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_USDC_TO_LST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slippage_config: Option<SlippageConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SwapUsdcToLstIxArgs {
                amount,
                slippage_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_USDC_TO_LST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.slippage_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_usdc_to_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapUsdcToLstKeys,
    args: SwapUsdcToLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_USDC_TO_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapUsdcToLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_usdc_to_lst_ix(
    keys: SwapUsdcToLstKeys,
    args: SwapUsdcToLstIxArgs,
) -> std::io::Result<Instruction> {
    swap_usdc_to_lst_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn swap_usdc_to_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapUsdcToLstAccounts<'_, '_>,
    args: SwapUsdcToLstIxArgs,
) -> ProgramResult {
    let keys: SwapUsdcToLstKeys = accounts.into();
    let ix = swap_usdc_to_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_usdc_to_lst_invoke(
    accounts: SwapUsdcToLstAccounts<'_, '_>,
    args: SwapUsdcToLstIxArgs,
) -> ProgramResult {
    swap_usdc_to_lst_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn swap_usdc_to_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapUsdcToLstAccounts<'_, '_>,
    args: SwapUsdcToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapUsdcToLstKeys = accounts.into();
    let ix = swap_usdc_to_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_usdc_to_lst_invoke_signed(
    accounts: SwapUsdcToLstAccounts<'_, '_>,
    args: SwapUsdcToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_usdc_to_lst_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_usdc_to_lst_verify_account_keys(
    accounts: SwapUsdcToLstAccounts<'_, '_>,
    keys: SwapUsdcToLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.pool_config.key, keys.pool_config),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.stablecoin_mint_auth.key, keys.stablecoin_mint_auth),
        (*accounts.lst_vault_auth.key, keys.lst_vault_auth),
        (*accounts.usdc_vault_auth.key, keys.usdc_vault_auth),
        (*accounts.pool_auth.key, keys.pool_auth),
        (*accounts.settlement_auth.key, keys.settlement_auth),
        (*accounts.lst_vault.key, keys.lst_vault),
        (*accounts.usdc_vault.key, keys.usdc_vault),
        (*accounts.stablecoin_pool.key, keys.stablecoin_pool),
        (*accounts.user_lst_ta.key, keys.user_lst_ta),
        (*accounts.user_usdc_ta.key, keys.user_usdc_ta),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.usdc_mint.key, keys.usdc_mint),
        (*accounts.stablecoin_mint.key, keys.stablecoin_mint),
        (*accounts.sol_usd_pyth_feed.key, keys.sol_usd_pyth_feed),
        (*accounts.usdc_usd_pyth_feed.key, keys.usdc_usd_pyth_feed),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.earn_pool.key, keys.earn_pool),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_usdc_to_lst_verify_writable_privileges<'me, 'info>(
    accounts: SwapUsdcToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.hylo,
        accounts.usdc_pair,
        accounts.lst_vault,
        accounts.usdc_vault,
        accounts.stablecoin_pool,
        accounts.user_lst_ta,
        accounts.user_usdc_ta,
        accounts.stablecoin_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_usdc_to_lst_verify_signer_privileges<'me, 'info>(
    accounts: SwapUsdcToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_usdc_to_lst_verify_account_privileges<'me, 'info>(
    accounts: SwapUsdcToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_usdc_to_lst_verify_writable_privileges(accounts)?;
    swap_usdc_to_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseExoPairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseExoPairKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnpauseExoPairAccounts<'_, '_>> for UnpauseExoPairKeys {
    fn from(accounts: UnpauseExoPairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnpauseExoPairKeys> for [AccountMeta; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseExoPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]> for UnpauseExoPairKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UnpauseExoPairAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseExoPairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]>
for UnpauseExoPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UNPAUSE_EXO_PAIR_IX_DISCM: [u8; 8usize] = [
    119, 240, 216, 140, 26, 148, 153, 80,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseExoPairIxData;
impl UnpauseExoPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_EXO_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_EXO_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_exo_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseExoPairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_EXO_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseExoPairIxData.try_to_vec()?,
    })
}
pub fn unpause_exo_pair_ix(keys: UnpauseExoPairKeys) -> std::io::Result<Instruction> {
    unpause_exo_pair_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn unpause_exo_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseExoPairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseExoPairKeys = accounts.into();
    let ix = unpause_exo_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_exo_pair_invoke(
    accounts: UnpauseExoPairAccounts<'_, '_>,
) -> ProgramResult {
    unpause_exo_pair_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn unpause_exo_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseExoPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseExoPairKeys = accounts.into();
    let ix = unpause_exo_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_exo_pair_invoke_signed(
    accounts: UnpauseExoPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_exo_pair_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_exo_pair_verify_account_keys(
    accounts: UnpauseExoPairAccounts<'_, '_>,
    keys: UnpauseExoPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_exo_pair_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseExoPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_exo_pair_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseExoPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_exo_pair_verify_account_privileges<'me, 'info>(
    accounts: UnpauseExoPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_exo_pair_verify_writable_privileges(accounts)?;
    unpause_exo_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseLstPairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseLstPairKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnpauseLstPairAccounts<'_, '_>> for UnpauseLstPairKeys {
    fn from(accounts: UnpauseLstPairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnpauseLstPairKeys> for [AccountMeta; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseLstPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN]> for UnpauseLstPairKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UnpauseLstPairAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseLstPairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN]>
for UnpauseLstPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UNPAUSE_LST_PAIR_IX_DISCM: [u8; 8usize] = [72, 155, 211, 21, 37, 217, 197, 7];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseLstPairIxData;
impl UnpauseLstPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_LST_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_LST_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_lst_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseLstPairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_LST_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseLstPairIxData.try_to_vec()?,
    })
}
pub fn unpause_lst_pair_ix(keys: UnpauseLstPairKeys) -> std::io::Result<Instruction> {
    unpause_lst_pair_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn unpause_lst_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseLstPairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseLstPairKeys = accounts.into();
    let ix = unpause_lst_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_lst_pair_invoke(
    accounts: UnpauseLstPairAccounts<'_, '_>,
) -> ProgramResult {
    unpause_lst_pair_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn unpause_lst_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseLstPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseLstPairKeys = accounts.into();
    let ix = unpause_lst_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_lst_pair_invoke_signed(
    accounts: UnpauseLstPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_lst_pair_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_lst_pair_verify_account_keys(
    accounts: UnpauseLstPairAccounts<'_, '_>,
    keys: UnpauseLstPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_lst_pair_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseLstPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_lst_pair_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseLstPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_lst_pair_verify_account_privileges<'me, 'info>(
    accounts: UnpauseLstPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_lst_pair_verify_writable_privileges(accounts)?;
    unpause_lst_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseProtocolAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseProtocolKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnpauseProtocolAccounts<'_, '_>> for UnpauseProtocolKeys {
    fn from(accounts: UnpauseProtocolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnpauseProtocolKeys> for [AccountMeta; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseProtocolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN]> for UnpauseProtocolKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UnpauseProtocolAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseProtocolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN]>
for UnpauseProtocolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UNPAUSE_PROTOCOL_IX_DISCM: [u8; 8usize] = [183, 154, 5, 183, 105, 76, 87, 18];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseProtocolIxData;
impl UnpauseProtocolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_PROTOCOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_PROTOCOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_protocol_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseProtocolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_PROTOCOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseProtocolIxData.try_to_vec()?,
    })
}
pub fn unpause_protocol_ix(keys: UnpauseProtocolKeys) -> std::io::Result<Instruction> {
    unpause_protocol_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn unpause_protocol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseProtocolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseProtocolKeys = accounts.into();
    let ix = unpause_protocol_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_protocol_invoke(
    accounts: UnpauseProtocolAccounts<'_, '_>,
) -> ProgramResult {
    unpause_protocol_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn unpause_protocol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseProtocolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseProtocolKeys = accounts.into();
    let ix = unpause_protocol_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_protocol_invoke_signed(
    accounts: UnpauseProtocolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_protocol_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_protocol_verify_account_keys(
    accounts: UnpauseProtocolAccounts<'_, '_>,
    keys: UnpauseProtocolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_protocol_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_protocol_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_protocol_verify_account_privileges<'me, 'info>(
    accounts: UnpauseProtocolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_protocol_verify_writable_privileges(accounts)?;
    unpause_protocol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseUsdcPairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseUsdcPairKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnpauseUsdcPairAccounts<'_, '_>> for UnpauseUsdcPairKeys {
    fn from(accounts: UnpauseUsdcPairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnpauseUsdcPairKeys> for [AccountMeta; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseUsdcPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]> for UnpauseUsdcPairKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UnpauseUsdcPairAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseUsdcPairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]>
for UnpauseUsdcPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UNPAUSE_USDC_PAIR_IX_DISCM: [u8; 8usize] = [
    72, 217, 160, 247, 221, 121, 210, 136,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseUsdcPairIxData;
impl UnpauseUsdcPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_USDC_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_USDC_PAIR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_usdc_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseUsdcPairKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_USDC_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseUsdcPairIxData.try_to_vec()?,
    })
}
pub fn unpause_usdc_pair_ix(keys: UnpauseUsdcPairKeys) -> std::io::Result<Instruction> {
    unpause_usdc_pair_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn unpause_usdc_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseUsdcPairAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseUsdcPairKeys = accounts.into();
    let ix = unpause_usdc_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_usdc_pair_invoke(
    accounts: UnpauseUsdcPairAccounts<'_, '_>,
) -> ProgramResult {
    unpause_usdc_pair_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn unpause_usdc_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseUsdcPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseUsdcPairKeys = accounts.into();
    let ix = unpause_usdc_pair_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_usdc_pair_invoke_signed(
    accounts: UnpauseUsdcPairAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_usdc_pair_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_usdc_pair_verify_account_keys(
    accounts: UnpauseUsdcPairAccounts<'_, '_>,
    keys: UnpauseUsdcPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_usdc_pair_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseUsdcPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_usdc_pair_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseUsdcPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_usdc_pair_verify_account_privileges<'me, 'info>(
    accounts: UnpauseUsdcPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_usdc_pair_verify_writable_privileges(accounts)?;
    unpause_usdc_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoBorrowRateCurveAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoBorrowRateCurveKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoBorrowRateCurveAccounts<'_, '_>> for UpdateExoBorrowRateCurveKeys {
    fn from(accounts: UpdateExoBorrowRateCurveAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoBorrowRateCurveKeys>
for [AccountMeta; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoBorrowRateCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN]>
for UpdateExoBorrowRateCurveKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoBorrowRateCurveAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoBorrowRateCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN]>
for UpdateExoBorrowRateCurveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_BORROW_RATE_CURVE_IX_DISCM: [u8; 8usize] = [
    27, 184, 25, 86, 139, 177, 15, 123,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoBorrowRateCurveIxArgs {
    pub new_curve_config: BorrowRateCurveConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoBorrowRateCurveIxData(pub UpdateExoBorrowRateCurveIxArgs);
impl From<UpdateExoBorrowRateCurveIxArgs> for UpdateExoBorrowRateCurveIxData {
    fn from(args: UpdateExoBorrowRateCurveIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoBorrowRateCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_BORROW_RATE_CURVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurveConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoBorrowRateCurveIxArgs {
                new_curve_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_BORROW_RATE_CURVE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_curve_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_borrow_rate_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoBorrowRateCurveKeys,
    args: UpdateExoBorrowRateCurveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_BORROW_RATE_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoBorrowRateCurveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_borrow_rate_curve_ix(
    keys: UpdateExoBorrowRateCurveKeys,
    args: UpdateExoBorrowRateCurveIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_borrow_rate_curve_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_borrow_rate_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoBorrowRateCurveAccounts<'_, '_>,
    args: UpdateExoBorrowRateCurveIxArgs,
) -> ProgramResult {
    let keys: UpdateExoBorrowRateCurveKeys = accounts.into();
    let ix = update_exo_borrow_rate_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_borrow_rate_curve_invoke(
    accounts: UpdateExoBorrowRateCurveAccounts<'_, '_>,
    args: UpdateExoBorrowRateCurveIxArgs,
) -> ProgramResult {
    update_exo_borrow_rate_curve_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_borrow_rate_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoBorrowRateCurveAccounts<'_, '_>,
    args: UpdateExoBorrowRateCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoBorrowRateCurveKeys = accounts.into();
    let ix = update_exo_borrow_rate_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_borrow_rate_curve_invoke_signed(
    accounts: UpdateExoBorrowRateCurveAccounts<'_, '_>,
    args: UpdateExoBorrowRateCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_borrow_rate_curve_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_borrow_rate_curve_verify_account_keys(
    accounts: UpdateExoBorrowRateCurveAccounts<'_, '_>,
    keys: UpdateExoBorrowRateCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_borrow_rate_curve_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoBorrowRateCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_borrow_rate_curve_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoBorrowRateCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_borrow_rate_curve_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoBorrowRateCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_borrow_rate_curve_verify_writable_privileges(accounts)?;
    update_exo_borrow_rate_curve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoBorrowRateFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoBorrowRateFeeKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoBorrowRateFeeAccounts<'_, '_>> for UpdateExoBorrowRateFeeKeys {
    fn from(accounts: UpdateExoBorrowRateFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoBorrowRateFeeKeys>
for [AccountMeta; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoBorrowRateFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN]>
for UpdateExoBorrowRateFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoBorrowRateFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoBorrowRateFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN]>
for UpdateExoBorrowRateFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_BORROW_RATE_FEE_IX_DISCM: [u8; 8usize] = [
    93, 204, 52, 236, 107, 39, 6, 36,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoBorrowRateFeeIxArgs {
    pub new_borrow_rate_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoBorrowRateFeeIxData(pub UpdateExoBorrowRateFeeIxArgs);
impl From<UpdateExoBorrowRateFeeIxArgs> for UpdateExoBorrowRateFeeIxData {
    fn from(args: UpdateExoBorrowRateFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoBorrowRateFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_BORROW_RATE_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_borrow_rate_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoBorrowRateFeeIxArgs {
                new_borrow_rate_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_BORROW_RATE_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_borrow_rate_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_borrow_rate_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoBorrowRateFeeKeys,
    args: UpdateExoBorrowRateFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_BORROW_RATE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoBorrowRateFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_borrow_rate_fee_ix(
    keys: UpdateExoBorrowRateFeeKeys,
    args: UpdateExoBorrowRateFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_borrow_rate_fee_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_borrow_rate_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoBorrowRateFeeAccounts<'_, '_>,
    args: UpdateExoBorrowRateFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateExoBorrowRateFeeKeys = accounts.into();
    let ix = update_exo_borrow_rate_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_borrow_rate_fee_invoke(
    accounts: UpdateExoBorrowRateFeeAccounts<'_, '_>,
    args: UpdateExoBorrowRateFeeIxArgs,
) -> ProgramResult {
    update_exo_borrow_rate_fee_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_borrow_rate_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoBorrowRateFeeAccounts<'_, '_>,
    args: UpdateExoBorrowRateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoBorrowRateFeeKeys = accounts.into();
    let ix = update_exo_borrow_rate_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_borrow_rate_fee_invoke_signed(
    accounts: UpdateExoBorrowRateFeeAccounts<'_, '_>,
    args: UpdateExoBorrowRateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_borrow_rate_fee_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_borrow_rate_fee_verify_account_keys(
    accounts: UpdateExoBorrowRateFeeAccounts<'_, '_>,
    keys: UpdateExoBorrowRateFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_borrow_rate_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoBorrowRateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_borrow_rate_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoBorrowRateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_borrow_rate_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoBorrowRateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_borrow_rate_fee_verify_writable_privileges(accounts)?;
    update_exo_borrow_rate_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoBuyCurveAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoBuyCurveKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoBuyCurveAccounts<'_, '_>> for UpdateExoBuyCurveKeys {
    fn from(accounts: UpdateExoBuyCurveAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoBuyCurveKeys>
for [AccountMeta; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoBuyCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN]> for UpdateExoBuyCurveKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoBuyCurveAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoBuyCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN]>
for UpdateExoBuyCurveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_BUY_CURVE_IX_DISCM: [u8; 8usize] = [
    121, 104, 59, 11, 147, 243, 174, 52,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoBuyCurveIxArgs {
    pub new_buy_curve_config: RebalanceCurveConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoBuyCurveIxData(pub UpdateExoBuyCurveIxArgs);
impl From<UpdateExoBuyCurveIxArgs> for UpdateExoBuyCurveIxData {
    fn from(args: UpdateExoBuyCurveIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoBuyCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_BUY_CURVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_buy_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoBuyCurveIxArgs {
                new_buy_curve_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_BUY_CURVE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_buy_curve_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_buy_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoBuyCurveKeys,
    args: UpdateExoBuyCurveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_BUY_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoBuyCurveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_buy_curve_ix(
    keys: UpdateExoBuyCurveKeys,
    args: UpdateExoBuyCurveIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_buy_curve_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_buy_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoBuyCurveAccounts<'_, '_>,
    args: UpdateExoBuyCurveIxArgs,
) -> ProgramResult {
    let keys: UpdateExoBuyCurveKeys = accounts.into();
    let ix = update_exo_buy_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_buy_curve_invoke(
    accounts: UpdateExoBuyCurveAccounts<'_, '_>,
    args: UpdateExoBuyCurveIxArgs,
) -> ProgramResult {
    update_exo_buy_curve_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn update_exo_buy_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoBuyCurveAccounts<'_, '_>,
    args: UpdateExoBuyCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoBuyCurveKeys = accounts.into();
    let ix = update_exo_buy_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_buy_curve_invoke_signed(
    accounts: UpdateExoBuyCurveAccounts<'_, '_>,
    args: UpdateExoBuyCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_buy_curve_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_buy_curve_verify_account_keys(
    accounts: UpdateExoBuyCurveAccounts<'_, '_>,
    keys: UpdateExoBuyCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_buy_curve_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoBuyCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_buy_curve_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoBuyCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_buy_curve_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoBuyCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_buy_curve_verify_writable_privileges(accounts)?;
    update_exo_buy_curve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoLevercoinFeesAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoLevercoinFeesKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoLevercoinFeesAccounts<'_, '_>> for UpdateExoLevercoinFeesKeys {
    fn from(accounts: UpdateExoLevercoinFeesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoLevercoinFeesKeys>
for [AccountMeta; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoLevercoinFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN]>
for UpdateExoLevercoinFeesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoLevercoinFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoLevercoinFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN]>
for UpdateExoLevercoinFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_LEVERCOIN_FEES_IX_DISCM: [u8; 8usize] = [
    35, 165, 222, 147, 145, 150, 127, 99,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoLevercoinFeesIxArgs {
    pub new_levercoin_fees: LevercoinFees,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoLevercoinFeesIxData(pub UpdateExoLevercoinFeesIxArgs);
impl From<UpdateExoLevercoinFeesIxArgs> for UpdateExoLevercoinFeesIxData {
    fn from(args: UpdateExoLevercoinFeesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoLevercoinFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_LEVERCOIN_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoLevercoinFeesIxArgs {
                new_levercoin_fees,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_LEVERCOIN_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_levercoin_fees, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_levercoin_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoLevercoinFeesKeys,
    args: UpdateExoLevercoinFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_LEVERCOIN_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoLevercoinFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_levercoin_fees_ix(
    keys: UpdateExoLevercoinFeesKeys,
    args: UpdateExoLevercoinFeesIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_levercoin_fees_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_levercoin_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoLevercoinFeesAccounts<'_, '_>,
    args: UpdateExoLevercoinFeesIxArgs,
) -> ProgramResult {
    let keys: UpdateExoLevercoinFeesKeys = accounts.into();
    let ix = update_exo_levercoin_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_levercoin_fees_invoke(
    accounts: UpdateExoLevercoinFeesAccounts<'_, '_>,
    args: UpdateExoLevercoinFeesIxArgs,
) -> ProgramResult {
    update_exo_levercoin_fees_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_levercoin_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoLevercoinFeesAccounts<'_, '_>,
    args: UpdateExoLevercoinFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoLevercoinFeesKeys = accounts.into();
    let ix = update_exo_levercoin_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_levercoin_fees_invoke_signed(
    accounts: UpdateExoLevercoinFeesAccounts<'_, '_>,
    args: UpdateExoLevercoinFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_levercoin_fees_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_levercoin_fees_verify_account_keys(
    accounts: UpdateExoLevercoinFeesAccounts<'_, '_>,
    keys: UpdateExoLevercoinFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_levercoin_fees_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoLevercoinFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_levercoin_fees_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoLevercoinFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_levercoin_fees_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoLevercoinFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_levercoin_fees_verify_writable_privileges(accounts)?;
    update_exo_levercoin_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoLevercoinMarketCapLimitAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoLevercoinMarketCapLimitKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoLevercoinMarketCapLimitAccounts<'_, '_>>
for UpdateExoLevercoinMarketCapLimitKeys {
    fn from(accounts: UpdateExoLevercoinMarketCapLimitAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoLevercoinMarketCapLimitKeys>
for [AccountMeta; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoLevercoinMarketCapLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateExoLevercoinMarketCapLimitKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoLevercoinMarketCapLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoLevercoinMarketCapLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateExoLevercoinMarketCapLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_DISCM: [u8; 8usize] = [
    7, 94, 19, 123, 92, 81, 123, 194,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoLevercoinMarketCapLimitIxArgs {
    pub new_levercoin_market_cap_limit: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoLevercoinMarketCapLimitIxData(
    pub UpdateExoLevercoinMarketCapLimitIxArgs,
);
impl From<UpdateExoLevercoinMarketCapLimitIxArgs>
for UpdateExoLevercoinMarketCapLimitIxData {
    fn from(args: UpdateExoLevercoinMarketCapLimitIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoLevercoinMarketCapLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_levercoin_market_cap_limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoLevercoinMarketCapLimitIxArgs {
                new_levercoin_market_cap_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_levercoin_market_cap_limit,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_levercoin_market_cap_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoLevercoinMarketCapLimitKeys,
    args: UpdateExoLevercoinMarketCapLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_LEVERCOIN_MARKET_CAP_LIMIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateExoLevercoinMarketCapLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_levercoin_market_cap_limit_ix(
    keys: UpdateExoLevercoinMarketCapLimitKeys,
    args: UpdateExoLevercoinMarketCapLimitIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_levercoin_market_cap_limit_ix_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_exo_levercoin_market_cap_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'_, '_>,
    args: UpdateExoLevercoinMarketCapLimitIxArgs,
) -> ProgramResult {
    let keys: UpdateExoLevercoinMarketCapLimitKeys = accounts.into();
    let ix = update_exo_levercoin_market_cap_limit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_levercoin_market_cap_limit_invoke(
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'_, '_>,
    args: UpdateExoLevercoinMarketCapLimitIxArgs,
) -> ProgramResult {
    update_exo_levercoin_market_cap_limit_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_levercoin_market_cap_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'_, '_>,
    args: UpdateExoLevercoinMarketCapLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoLevercoinMarketCapLimitKeys = accounts.into();
    let ix = update_exo_levercoin_market_cap_limit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_levercoin_market_cap_limit_invoke_signed(
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'_, '_>,
    args: UpdateExoLevercoinMarketCapLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_levercoin_market_cap_limit_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_levercoin_market_cap_limit_verify_account_keys(
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'_, '_>,
    keys: UpdateExoLevercoinMarketCapLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_levercoin_market_cap_limit_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_levercoin_market_cap_limit_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_levercoin_market_cap_limit_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoLevercoinMarketCapLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_levercoin_market_cap_limit_verify_writable_privileges(accounts)?;
    update_exo_levercoin_market_cap_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoOracleAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoOracleKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoOracleAccounts<'_, '_>> for UpdateExoOracleKeys {
    fn from(accounts: UpdateExoOracleAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoOracleKeys> for [AccountMeta; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN]> for UpdateExoOracleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN]>
for UpdateExoOracleAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_ORACLE_IX_DISCM: [u8; 8usize] = [
    83, 34, 52, 101, 213, 59, 178, 111,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoOracleIxArgs {
    pub new_oracle: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoOracleIxData(pub UpdateExoOracleIxArgs);
impl From<UpdateExoOracleIxArgs> for UpdateExoOracleIxData {
    fn from(args: UpdateExoOracleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateExoOracleIxArgs {
                new_oracle,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_ORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_oracle, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoOracleKeys,
    args: UpdateExoOracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_ORACLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoOracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_oracle_ix(
    keys: UpdateExoOracleKeys,
    args: UpdateExoOracleIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_oracle_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoOracleAccounts<'_, '_>,
    args: UpdateExoOracleIxArgs,
) -> ProgramResult {
    let keys: UpdateExoOracleKeys = accounts.into();
    let ix = update_exo_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_oracle_invoke(
    accounts: UpdateExoOracleAccounts<'_, '_>,
    args: UpdateExoOracleIxArgs,
) -> ProgramResult {
    update_exo_oracle_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn update_exo_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoOracleAccounts<'_, '_>,
    args: UpdateExoOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoOracleKeys = accounts.into();
    let ix = update_exo_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_oracle_invoke_signed(
    accounts: UpdateExoOracleAccounts<'_, '_>,
    args: UpdateExoOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_oracle_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_oracle_verify_account_keys(
    accounts: UpdateExoOracleAccounts<'_, '_>,
    keys: UpdateExoOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_oracle_verify_writable_privileges(accounts)?;
    update_exo_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoOracleConfToleranceAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoOracleConfToleranceKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoOracleConfToleranceAccounts<'_, '_>>
for UpdateExoOracleConfToleranceKeys {
    fn from(accounts: UpdateExoOracleConfToleranceAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoOracleConfToleranceKeys>
for [AccountMeta; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoOracleConfToleranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateExoOracleConfToleranceKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoOracleConfToleranceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoOracleConfToleranceAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateExoOracleConfToleranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_DISCM: [u8; 8usize] = [
    158, 164, 30, 18, 74, 88, 255, 195,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoOracleConfToleranceIxArgs {
    pub new_oracle_conf_tolerance: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoOracleConfToleranceIxData(pub UpdateExoOracleConfToleranceIxArgs);
impl From<UpdateExoOracleConfToleranceIxArgs> for UpdateExoOracleConfToleranceIxData {
    fn from(args: UpdateExoOracleConfToleranceIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoOracleConfToleranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoOracleConfToleranceIxArgs {
                new_oracle_conf_tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_oracle_conf_tolerance,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_oracle_conf_tolerance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoOracleConfToleranceKeys,
    args: UpdateExoOracleConfToleranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateExoOracleConfToleranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_oracle_conf_tolerance_ix(
    keys: UpdateExoOracleConfToleranceKeys,
    args: UpdateExoOracleConfToleranceIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_oracle_conf_tolerance_ix_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_exo_oracle_conf_tolerance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoOracleConfToleranceAccounts<'_, '_>,
    args: UpdateExoOracleConfToleranceIxArgs,
) -> ProgramResult {
    let keys: UpdateExoOracleConfToleranceKeys = accounts.into();
    let ix = update_exo_oracle_conf_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_oracle_conf_tolerance_invoke(
    accounts: UpdateExoOracleConfToleranceAccounts<'_, '_>,
    args: UpdateExoOracleConfToleranceIxArgs,
) -> ProgramResult {
    update_exo_oracle_conf_tolerance_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_oracle_conf_tolerance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoOracleConfToleranceAccounts<'_, '_>,
    args: UpdateExoOracleConfToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoOracleConfToleranceKeys = accounts.into();
    let ix = update_exo_oracle_conf_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_oracle_conf_tolerance_invoke_signed(
    accounts: UpdateExoOracleConfToleranceAccounts<'_, '_>,
    args: UpdateExoOracleConfToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_oracle_conf_tolerance_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_oracle_conf_tolerance_verify_account_keys(
    accounts: UpdateExoOracleConfToleranceAccounts<'_, '_>,
    keys: UpdateExoOracleConfToleranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_conf_tolerance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_conf_tolerance_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_conf_tolerance_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_oracle_conf_tolerance_verify_writable_privileges(accounts)?;
    update_exo_oracle_conf_tolerance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoOracleIntervalAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoOracleIntervalKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoOracleIntervalAccounts<'_, '_>> for UpdateExoOracleIntervalKeys {
    fn from(accounts: UpdateExoOracleIntervalAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoOracleIntervalKeys>
for [AccountMeta; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoOracleIntervalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]>
for UpdateExoOracleIntervalKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoOracleIntervalAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoOracleIntervalAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]>
for UpdateExoOracleIntervalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_ORACLE_INTERVAL_IX_DISCM: [u8; 8usize] = [
    44, 2, 28, 238, 184, 98, 233, 205,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoOracleIntervalIxArgs {
    pub new_oracle_interval_secs: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoOracleIntervalIxData(pub UpdateExoOracleIntervalIxArgs);
impl From<UpdateExoOracleIntervalIxArgs> for UpdateExoOracleIntervalIxData {
    fn from(args: UpdateExoOracleIntervalIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoOracleIntervalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_ORACLE_INTERVAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateExoOracleIntervalIxArgs {
                new_oracle_interval_secs,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_ORACLE_INTERVAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_oracle_interval_secs, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_oracle_interval_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoOracleIntervalKeys,
    args: UpdateExoOracleIntervalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoOracleIntervalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_oracle_interval_ix(
    keys: UpdateExoOracleIntervalKeys,
    args: UpdateExoOracleIntervalIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_oracle_interval_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_oracle_interval_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoOracleIntervalAccounts<'_, '_>,
    args: UpdateExoOracleIntervalIxArgs,
) -> ProgramResult {
    let keys: UpdateExoOracleIntervalKeys = accounts.into();
    let ix = update_exo_oracle_interval_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_oracle_interval_invoke(
    accounts: UpdateExoOracleIntervalAccounts<'_, '_>,
    args: UpdateExoOracleIntervalIxArgs,
) -> ProgramResult {
    update_exo_oracle_interval_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_oracle_interval_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoOracleIntervalAccounts<'_, '_>,
    args: UpdateExoOracleIntervalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoOracleIntervalKeys = accounts.into();
    let ix = update_exo_oracle_interval_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_oracle_interval_invoke_signed(
    accounts: UpdateExoOracleIntervalAccounts<'_, '_>,
    args: UpdateExoOracleIntervalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_oracle_interval_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_oracle_interval_verify_account_keys(
    accounts: UpdateExoOracleIntervalAccounts<'_, '_>,
    keys: UpdateExoOracleIntervalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_interval_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_interval_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_oracle_interval_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_oracle_interval_verify_writable_privileges(accounts)?;
    update_exo_oracle_interval_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoSellCurveAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoSellCurveKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoSellCurveAccounts<'_, '_>> for UpdateExoSellCurveKeys {
    fn from(accounts: UpdateExoSellCurveAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoSellCurveKeys>
for [AccountMeta; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoSellCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN]> for UpdateExoSellCurveKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoSellCurveAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoSellCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN]>
for UpdateExoSellCurveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_SELL_CURVE_IX_DISCM: [u8; 8usize] = [
    252, 203, 220, 190, 167, 181, 6, 55,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoSellCurveIxArgs {
    pub new_sell_curve_config: RebalanceCurveConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoSellCurveIxData(pub UpdateExoSellCurveIxArgs);
impl From<UpdateExoSellCurveIxArgs> for UpdateExoSellCurveIxData {
    fn from(args: UpdateExoSellCurveIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoSellCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_SELL_CURVE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_sell_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoSellCurveIxArgs {
                new_sell_curve_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_SELL_CURVE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_sell_curve_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_sell_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoSellCurveKeys,
    args: UpdateExoSellCurveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_SELL_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateExoSellCurveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_sell_curve_ix(
    keys: UpdateExoSellCurveKeys,
    args: UpdateExoSellCurveIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_sell_curve_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_exo_sell_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoSellCurveAccounts<'_, '_>,
    args: UpdateExoSellCurveIxArgs,
) -> ProgramResult {
    let keys: UpdateExoSellCurveKeys = accounts.into();
    let ix = update_exo_sell_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_sell_curve_invoke(
    accounts: UpdateExoSellCurveAccounts<'_, '_>,
    args: UpdateExoSellCurveIxArgs,
) -> ProgramResult {
    update_exo_sell_curve_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_sell_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoSellCurveAccounts<'_, '_>,
    args: UpdateExoSellCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoSellCurveKeys = accounts.into();
    let ix = update_exo_sell_curve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_sell_curve_invoke_signed(
    accounts: UpdateExoSellCurveAccounts<'_, '_>,
    args: UpdateExoSellCurveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_sell_curve_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_sell_curve_verify_account_keys(
    accounts: UpdateExoSellCurveAccounts<'_, '_>,
    keys: UpdateExoSellCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_sell_curve_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoSellCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_sell_curve_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoSellCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_sell_curve_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoSellCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_sell_curve_verify_writable_privileges(accounts)?;
    update_exo_sell_curve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExoStablecoinMintThresholdAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub exo_pair: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExoStablecoinMintThresholdKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub exo_pair: Pubkey,
    pub collateral_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateExoStablecoinMintThresholdAccounts<'_, '_>>
for UpdateExoStablecoinMintThresholdKeys {
    fn from(accounts: UpdateExoStablecoinMintThresholdAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            exo_pair: *accounts.exo_pair.key,
            collateral_mint: *accounts.collateral_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateExoStablecoinMintThresholdKeys>
for [AccountMeta; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateExoStablecoinMintThresholdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.exo_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN]>
for UpdateExoStablecoinMintThresholdKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            exo_pair: pubkeys[2],
            collateral_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateExoStablecoinMintThresholdAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateExoStablecoinMintThresholdAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.exo_pair.clone(),
            accounts.collateral_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN]>
for UpdateExoStablecoinMintThresholdAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            exo_pair: &arr[2],
            collateral_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_DISCM: [u8; 8usize] = [
    87, 78, 92, 99, 233, 87, 163, 180,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateExoStablecoinMintThresholdIxArgs {
    pub new_stablecoin_mint_threshold: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExoStablecoinMintThresholdIxData(
    pub UpdateExoStablecoinMintThresholdIxArgs,
);
impl From<UpdateExoStablecoinMintThresholdIxArgs>
for UpdateExoStablecoinMintThresholdIxData {
    fn from(args: UpdateExoStablecoinMintThresholdIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExoStablecoinMintThresholdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateExoStablecoinMintThresholdIxArgs {
                new_stablecoin_mint_threshold,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_stablecoin_mint_threshold,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exo_stablecoin_mint_threshold_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateExoStablecoinMintThresholdKeys,
    args: UpdateExoStablecoinMintThresholdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_EXO_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateExoStablecoinMintThresholdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exo_stablecoin_mint_threshold_ix(
    keys: UpdateExoStablecoinMintThresholdKeys,
    args: UpdateExoStablecoinMintThresholdIxArgs,
) -> std::io::Result<Instruction> {
    update_exo_stablecoin_mint_threshold_ix_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_exo_stablecoin_mint_threshold_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateExoStablecoinMintThresholdIxArgs,
) -> ProgramResult {
    let keys: UpdateExoStablecoinMintThresholdKeys = accounts.into();
    let ix = update_exo_stablecoin_mint_threshold_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_exo_stablecoin_mint_threshold_invoke(
    accounts: UpdateExoStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateExoStablecoinMintThresholdIxArgs,
) -> ProgramResult {
    update_exo_stablecoin_mint_threshold_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_exo_stablecoin_mint_threshold_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateExoStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateExoStablecoinMintThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateExoStablecoinMintThresholdKeys = accounts.into();
    let ix = update_exo_stablecoin_mint_threshold_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_exo_stablecoin_mint_threshold_invoke_signed(
    accounts: UpdateExoStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateExoStablecoinMintThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_exo_stablecoin_mint_threshold_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_exo_stablecoin_mint_threshold_verify_account_keys(
    accounts: UpdateExoStablecoinMintThresholdAccounts<'_, '_>,
    keys: UpdateExoStablecoinMintThresholdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.exo_pair.key, keys.exo_pair),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_exo_stablecoin_mint_threshold_verify_writable_privileges<'me, 'info>(
    accounts: UpdateExoStablecoinMintThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.exo_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_exo_stablecoin_mint_threshold_verify_signer_privileges<'me, 'info>(
    accounts: UpdateExoStablecoinMintThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_exo_stablecoin_mint_threshold_verify_account_privileges<'me, 'info>(
    accounts: UpdateExoStablecoinMintThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_exo_stablecoin_mint_threshold_verify_writable_privileges(accounts)?;
    update_exo_stablecoin_mint_threshold_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLevercoinFeesAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLevercoinFeesKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLevercoinFeesAccounts<'_, '_>> for UpdateLevercoinFeesKeys {
    fn from(accounts: UpdateLevercoinFeesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLevercoinFeesKeys>
for [AccountMeta; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLevercoinFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN]> for UpdateLevercoinFeesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateLevercoinFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLevercoinFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN]>
for UpdateLevercoinFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_LEVERCOIN_FEES_IX_DISCM: [u8; 8usize] = [
    183, 123, 195, 23, 63, 151, 104, 31,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLevercoinFeesIxArgs {
    pub new_levercoin_fees: LevercoinFees,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLevercoinFeesIxData(pub UpdateLevercoinFeesIxArgs);
impl From<UpdateLevercoinFeesIxArgs> for UpdateLevercoinFeesIxData {
    fn from(args: UpdateLevercoinFeesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLevercoinFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LEVERCOIN_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateLevercoinFeesIxArgs {
                new_levercoin_fees,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LEVERCOIN_FEES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_levercoin_fees, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_levercoin_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLevercoinFeesKeys,
    args: UpdateLevercoinFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LEVERCOIN_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateLevercoinFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_levercoin_fees_ix(
    keys: UpdateLevercoinFeesKeys,
    args: UpdateLevercoinFeesIxArgs,
) -> std::io::Result<Instruction> {
    update_levercoin_fees_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_levercoin_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLevercoinFeesAccounts<'_, '_>,
    args: UpdateLevercoinFeesIxArgs,
) -> ProgramResult {
    let keys: UpdateLevercoinFeesKeys = accounts.into();
    let ix = update_levercoin_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_levercoin_fees_invoke(
    accounts: UpdateLevercoinFeesAccounts<'_, '_>,
    args: UpdateLevercoinFeesIxArgs,
) -> ProgramResult {
    update_levercoin_fees_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_levercoin_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLevercoinFeesAccounts<'_, '_>,
    args: UpdateLevercoinFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLevercoinFeesKeys = accounts.into();
    let ix = update_levercoin_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_levercoin_fees_invoke_signed(
    accounts: UpdateLevercoinFeesAccounts<'_, '_>,
    args: UpdateLevercoinFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_levercoin_fees_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_levercoin_fees_verify_account_keys(
    accounts: UpdateLevercoinFeesAccounts<'_, '_>,
    keys: UpdateLevercoinFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_levercoin_fees_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLevercoinFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_levercoin_fees_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLevercoinFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_levercoin_fees_verify_account_privileges<'me, 'info>(
    accounts: UpdateLevercoinFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_levercoin_fees_verify_writable_privileges(accounts)?;
    update_levercoin_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLstBuyCurveConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLstBuyCurveConfigKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLstBuyCurveConfigAccounts<'_, '_>> for UpdateLstBuyCurveConfigKeys {
    fn from(accounts: UpdateLstBuyCurveConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLstBuyCurveConfigKeys>
for [AccountMeta; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLstBuyCurveConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateLstBuyCurveConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateLstBuyCurveConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLstBuyCurveConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateLstBuyCurveConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_LST_BUY_CURVE_CONFIG_IX_DISCM: [u8; 8usize] = [
    240, 121, 232, 195, 65, 250, 25, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstBuyCurveConfigIxArgs {
    pub new_buy_curve_config: RebalanceCurveConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstBuyCurveConfigIxData(pub UpdateLstBuyCurveConfigIxArgs);
impl From<UpdateLstBuyCurveConfigIxArgs> for UpdateLstBuyCurveConfigIxData {
    fn from(args: UpdateLstBuyCurveConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLstBuyCurveConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_BUY_CURVE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_buy_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateLstBuyCurveConfigIxArgs {
                new_buy_curve_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_BUY_CURVE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_buy_curve_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lst_buy_curve_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLstBuyCurveConfigKeys,
    args: UpdateLstBuyCurveConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LST_BUY_CURVE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateLstBuyCurveConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lst_buy_curve_config_ix(
    keys: UpdateLstBuyCurveConfigKeys,
    args: UpdateLstBuyCurveConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_lst_buy_curve_config_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_lst_buy_curve_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstBuyCurveConfigAccounts<'_, '_>,
    args: UpdateLstBuyCurveConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateLstBuyCurveConfigKeys = accounts.into();
    let ix = update_lst_buy_curve_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lst_buy_curve_config_invoke(
    accounts: UpdateLstBuyCurveConfigAccounts<'_, '_>,
    args: UpdateLstBuyCurveConfigIxArgs,
) -> ProgramResult {
    update_lst_buy_curve_config_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_lst_buy_curve_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstBuyCurveConfigAccounts<'_, '_>,
    args: UpdateLstBuyCurveConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLstBuyCurveConfigKeys = accounts.into();
    let ix = update_lst_buy_curve_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lst_buy_curve_config_invoke_signed(
    accounts: UpdateLstBuyCurveConfigAccounts<'_, '_>,
    args: UpdateLstBuyCurveConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lst_buy_curve_config_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_lst_buy_curve_config_verify_account_keys(
    accounts: UpdateLstBuyCurveConfigAccounts<'_, '_>,
    keys: UpdateLstBuyCurveConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lst_buy_curve_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLstBuyCurveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lst_buy_curve_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLstBuyCurveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lst_buy_curve_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateLstBuyCurveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lst_buy_curve_config_verify_writable_privileges(accounts)?;
    update_lst_buy_curve_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LST_PRICES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLstPricesAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub lst_registry: &'me AccountInfo<'info>,
    pub lut_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLstPricesKeys {
    pub payer: Pubkey,
    pub hylo: Pubkey,
    pub lst_registry: Pubkey,
    pub lut_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLstPricesAccounts<'_, '_>> for UpdateLstPricesKeys {
    fn from(accounts: UpdateLstPricesAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            hylo: *accounts.hylo.key,
            lst_registry: *accounts.lst_registry.key,
            lut_program: *accounts.lut_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLstPricesKeys> for [AccountMeta; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLstPricesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lut_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN]> for UpdateLstPricesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            hylo: pubkeys[1],
            lst_registry: pubkeys[2],
            lut_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateLstPricesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLstPricesAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.hylo.clone(),
            accounts.lst_registry.clone(),
            accounts.lut_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN]>
for UpdateLstPricesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            hylo: &arr[1],
            lst_registry: &arr[2],
            lut_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_LST_PRICES_IX_DISCM: [u8; 8usize] = [3, 34, 88, 178, 240, 40, 85, 148];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstPricesIxData;
impl UpdateLstPricesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_PRICES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_PRICES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lst_prices_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLstPricesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LST_PRICES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateLstPricesIxData.try_to_vec()?,
    })
}
pub fn update_lst_prices_ix(keys: UpdateLstPricesKeys) -> std::io::Result<Instruction> {
    update_lst_prices_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn update_lst_prices_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstPricesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateLstPricesKeys = accounts.into();
    let ix = update_lst_prices_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lst_prices_invoke(
    accounts: UpdateLstPricesAccounts<'_, '_>,
) -> ProgramResult {
    update_lst_prices_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn update_lst_prices_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstPricesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLstPricesKeys = accounts.into();
    let ix = update_lst_prices_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lst_prices_invoke_signed(
    accounts: UpdateLstPricesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lst_prices_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_lst_prices_verify_account_keys(
    accounts: UpdateLstPricesAccounts<'_, '_>,
    keys: UpdateLstPricesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.lst_registry.key, keys.lst_registry),
        (*accounts.lut_program.key, keys.lut_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lst_prices_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLstPricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.hylo, accounts.lst_registry] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lst_prices_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLstPricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lst_prices_verify_account_privileges<'me, 'info>(
    accounts: UpdateLstPricesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lst_prices_verify_writable_privileges(accounts)?;
    update_lst_prices_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLstRebalanceFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub lst_header: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLstRebalanceFeeKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub lst_header: Pubkey,
    pub lst_mint: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLstRebalanceFeeAccounts<'_, '_>> for UpdateLstRebalanceFeeKeys {
    fn from(accounts: UpdateLstRebalanceFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            lst_header: *accounts.lst_header.key,
            lst_mint: *accounts.lst_mint.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLstRebalanceFeeKeys>
for [AccountMeta; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLstRebalanceFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lst_header,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN]>
for UpdateLstRebalanceFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            lst_header: pubkeys[2],
            lst_mint: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<UpdateLstRebalanceFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLstRebalanceFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.lst_header.clone(),
            accounts.lst_mint.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN]>
for UpdateLstRebalanceFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            lst_header: &arr[2],
            lst_mint: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const UPDATE_LST_REBALANCE_FEE_IX_DISCM: [u8; 8usize] = [
    27, 3, 7, 125, 154, 26, 49, 27,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstRebalanceFeeIxArgs {
    pub new_rebalance_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstRebalanceFeeIxData(pub UpdateLstRebalanceFeeIxArgs);
impl From<UpdateLstRebalanceFeeIxArgs> for UpdateLstRebalanceFeeIxData {
    fn from(args: UpdateLstRebalanceFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLstRebalanceFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_REBALANCE_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_rebalance_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateLstRebalanceFeeIxArgs {
                new_rebalance_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_REBALANCE_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_rebalance_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lst_rebalance_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLstRebalanceFeeKeys,
    args: UpdateLstRebalanceFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LST_REBALANCE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateLstRebalanceFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lst_rebalance_fee_ix(
    keys: UpdateLstRebalanceFeeKeys,
    args: UpdateLstRebalanceFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_lst_rebalance_fee_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_lst_rebalance_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstRebalanceFeeAccounts<'_, '_>,
    args: UpdateLstRebalanceFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateLstRebalanceFeeKeys = accounts.into();
    let ix = update_lst_rebalance_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lst_rebalance_fee_invoke(
    accounts: UpdateLstRebalanceFeeAccounts<'_, '_>,
    args: UpdateLstRebalanceFeeIxArgs,
) -> ProgramResult {
    update_lst_rebalance_fee_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_lst_rebalance_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstRebalanceFeeAccounts<'_, '_>,
    args: UpdateLstRebalanceFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLstRebalanceFeeKeys = accounts.into();
    let ix = update_lst_rebalance_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lst_rebalance_fee_invoke_signed(
    accounts: UpdateLstRebalanceFeeAccounts<'_, '_>,
    args: UpdateLstRebalanceFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lst_rebalance_fee_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_lst_rebalance_fee_verify_account_keys(
    accounts: UpdateLstRebalanceFeeAccounts<'_, '_>,
    keys: UpdateLstRebalanceFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.lst_header.key, keys.lst_header),
        (*accounts.lst_mint.key, keys.lst_mint),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lst_rebalance_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLstRebalanceFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lst_header] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lst_rebalance_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLstRebalanceFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lst_rebalance_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateLstRebalanceFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lst_rebalance_fee_verify_writable_privileges(accounts)?;
    update_lst_rebalance_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLstSellCurveConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLstSellCurveConfigKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLstSellCurveConfigAccounts<'_, '_>> for UpdateLstSellCurveConfigKeys {
    fn from(accounts: UpdateLstSellCurveConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLstSellCurveConfigKeys>
for [AccountMeta; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLstSellCurveConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateLstSellCurveConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateLstSellCurveConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLstSellCurveConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateLstSellCurveConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_LST_SELL_CURVE_CONFIG_IX_DISCM: [u8; 8usize] = [
    137, 192, 209, 1, 23, 179, 84, 39,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstSellCurveConfigIxArgs {
    pub new_sell_curve_config: RebalanceCurveConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstSellCurveConfigIxData(pub UpdateLstSellCurveConfigIxArgs);
impl From<UpdateLstSellCurveConfigIxArgs> for UpdateLstSellCurveConfigIxData {
    fn from(args: UpdateLstSellCurveConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLstSellCurveConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_SELL_CURVE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_sell_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateLstSellCurveConfigIxArgs {
                new_sell_curve_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_SELL_CURVE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_sell_curve_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lst_sell_curve_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLstSellCurveConfigKeys,
    args: UpdateLstSellCurveConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LST_SELL_CURVE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateLstSellCurveConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lst_sell_curve_config_ix(
    keys: UpdateLstSellCurveConfigKeys,
    args: UpdateLstSellCurveConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_lst_sell_curve_config_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_lst_sell_curve_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstSellCurveConfigAccounts<'_, '_>,
    args: UpdateLstSellCurveConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateLstSellCurveConfigKeys = accounts.into();
    let ix = update_lst_sell_curve_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lst_sell_curve_config_invoke(
    accounts: UpdateLstSellCurveConfigAccounts<'_, '_>,
    args: UpdateLstSellCurveConfigIxArgs,
) -> ProgramResult {
    update_lst_sell_curve_config_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_lst_sell_curve_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstSellCurveConfigAccounts<'_, '_>,
    args: UpdateLstSellCurveConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLstSellCurveConfigKeys = accounts.into();
    let ix = update_lst_sell_curve_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lst_sell_curve_config_invoke_signed(
    accounts: UpdateLstSellCurveConfigAccounts<'_, '_>,
    args: UpdateLstSellCurveConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lst_sell_curve_config_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_lst_sell_curve_config_verify_account_keys(
    accounts: UpdateLstSellCurveConfigAccounts<'_, '_>,
    keys: UpdateLstSellCurveConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lst_sell_curve_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLstSellCurveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lst_sell_curve_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLstSellCurveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lst_sell_curve_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateLstSellCurveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lst_sell_curve_config_verify_writable_privileges(accounts)?;
    update_lst_sell_curve_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLstStablecoinMintThresholdAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLstStablecoinMintThresholdKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLstStablecoinMintThresholdAccounts<'_, '_>>
for UpdateLstStablecoinMintThresholdKeys {
    fn from(accounts: UpdateLstStablecoinMintThresholdAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLstStablecoinMintThresholdKeys>
for [AccountMeta; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLstStablecoinMintThresholdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN]>
for UpdateLstStablecoinMintThresholdKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateLstStablecoinMintThresholdAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLstStablecoinMintThresholdAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN]>
for UpdateLstStablecoinMintThresholdAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_DISCM: [u8; 8usize] = [
    24, 139, 177, 147, 245, 31, 222, 189,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstStablecoinMintThresholdIxArgs {
    pub new_stablecoin_mint_threshold: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstStablecoinMintThresholdIxData(
    pub UpdateLstStablecoinMintThresholdIxArgs,
);
impl From<UpdateLstStablecoinMintThresholdIxArgs>
for UpdateLstStablecoinMintThresholdIxData {
    fn from(args: UpdateLstStablecoinMintThresholdIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLstStablecoinMintThresholdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateLstStablecoinMintThresholdIxArgs {
                new_stablecoin_mint_threshold,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_stablecoin_mint_threshold,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lst_stablecoin_mint_threshold_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLstStablecoinMintThresholdKeys,
    args: UpdateLstStablecoinMintThresholdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LST_STABLECOIN_MINT_THRESHOLD_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateLstStablecoinMintThresholdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lst_stablecoin_mint_threshold_ix(
    keys: UpdateLstStablecoinMintThresholdKeys,
    args: UpdateLstStablecoinMintThresholdIxArgs,
) -> std::io::Result<Instruction> {
    update_lst_stablecoin_mint_threshold_ix_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_lst_stablecoin_mint_threshold_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateLstStablecoinMintThresholdIxArgs,
) -> ProgramResult {
    let keys: UpdateLstStablecoinMintThresholdKeys = accounts.into();
    let ix = update_lst_stablecoin_mint_threshold_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lst_stablecoin_mint_threshold_invoke(
    accounts: UpdateLstStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateLstStablecoinMintThresholdIxArgs,
) -> ProgramResult {
    update_lst_stablecoin_mint_threshold_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_lst_stablecoin_mint_threshold_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateLstStablecoinMintThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLstStablecoinMintThresholdKeys = accounts.into();
    let ix = update_lst_stablecoin_mint_threshold_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lst_stablecoin_mint_threshold_invoke_signed(
    accounts: UpdateLstStablecoinMintThresholdAccounts<'_, '_>,
    args: UpdateLstStablecoinMintThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lst_stablecoin_mint_threshold_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_lst_stablecoin_mint_threshold_verify_account_keys(
    accounts: UpdateLstStablecoinMintThresholdAccounts<'_, '_>,
    keys: UpdateLstStablecoinMintThresholdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lst_stablecoin_mint_threshold_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLstStablecoinMintThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lst_stablecoin_mint_threshold_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLstStablecoinMintThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lst_stablecoin_mint_threshold_verify_account_privileges<'me, 'info>(
    accounts: UpdateLstStablecoinMintThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lst_stablecoin_mint_threshold_verify_writable_privileges(accounts)?;
    update_lst_stablecoin_mint_threshold_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLstSwapFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLstSwapFeeKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateLstSwapFeeAccounts<'_, '_>> for UpdateLstSwapFeeKeys {
    fn from(accounts: UpdateLstSwapFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateLstSwapFeeKeys> for [AccountMeta; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLstSwapFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN]> for UpdateLstSwapFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateLstSwapFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLstSwapFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN]>
for UpdateLstSwapFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_LST_SWAP_FEE_IX_DISCM: [u8; 8usize] = [
    129, 125, 10, 188, 94, 219, 63, 217,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLstSwapFeeIxArgs {
    pub new_lst_swap_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLstSwapFeeIxData(pub UpdateLstSwapFeeIxArgs);
impl From<UpdateLstSwapFeeIxArgs> for UpdateLstSwapFeeIxData {
    fn from(args: UpdateLstSwapFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLstSwapFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LST_SWAP_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_lst_swap_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateLstSwapFeeIxArgs {
                new_lst_swap_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LST_SWAP_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_lst_swap_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lst_swap_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLstSwapFeeKeys,
    args: UpdateLstSwapFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LST_SWAP_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateLstSwapFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lst_swap_fee_ix(
    keys: UpdateLstSwapFeeKeys,
    args: UpdateLstSwapFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_lst_swap_fee_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_lst_swap_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstSwapFeeAccounts<'_, '_>,
    args: UpdateLstSwapFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateLstSwapFeeKeys = accounts.into();
    let ix = update_lst_swap_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lst_swap_fee_invoke(
    accounts: UpdateLstSwapFeeAccounts<'_, '_>,
    args: UpdateLstSwapFeeIxArgs,
) -> ProgramResult {
    update_lst_swap_fee_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn update_lst_swap_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLstSwapFeeAccounts<'_, '_>,
    args: UpdateLstSwapFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLstSwapFeeKeys = accounts.into();
    let ix = update_lst_swap_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lst_swap_fee_invoke_signed(
    accounts: UpdateLstSwapFeeAccounts<'_, '_>,
    args: UpdateLstSwapFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lst_swap_fee_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_lst_swap_fee_verify_account_keys(
    accounts: UpdateLstSwapFeeAccounts<'_, '_>,
    keys: UpdateLstSwapFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lst_swap_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLstSwapFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lst_swap_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLstSwapFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lst_swap_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateLstSwapFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lst_swap_fee_verify_writable_privileges(accounts)?;
    update_lst_swap_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleConfToleranceAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOracleConfToleranceKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateOracleConfToleranceAccounts<'_, '_>> for UpdateOracleConfToleranceKeys {
    fn from(accounts: UpdateOracleConfToleranceAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateOracleConfToleranceKeys>
for [AccountMeta; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOracleConfToleranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateOracleConfToleranceKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateOracleConfToleranceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOracleConfToleranceAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateOracleConfToleranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_ORACLE_CONF_TOLERANCE_IX_DISCM: [u8; 8usize] = [
    59, 81, 210, 112, 245, 81, 187, 151,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleConfToleranceIxArgs {
    pub new_oracle_conf_tolerance: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleConfToleranceIxData(pub UpdateOracleConfToleranceIxArgs);
impl From<UpdateOracleConfToleranceIxArgs> for UpdateOracleConfToleranceIxData {
    fn from(args: UpdateOracleConfToleranceIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOracleConfToleranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_CONF_TOLERANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateOracleConfToleranceIxArgs {
                new_oracle_conf_tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_CONF_TOLERANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_oracle_conf_tolerance,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_oracle_conf_tolerance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOracleConfToleranceKeys,
    args: UpdateOracleConfToleranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOracleConfToleranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracle_conf_tolerance_ix(
    keys: UpdateOracleConfToleranceKeys,
    args: UpdateOracleConfToleranceIxArgs,
) -> std::io::Result<Instruction> {
    update_oracle_conf_tolerance_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_oracle_conf_tolerance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleConfToleranceAccounts<'_, '_>,
    args: UpdateOracleConfToleranceIxArgs,
) -> ProgramResult {
    let keys: UpdateOracleConfToleranceKeys = accounts.into();
    let ix = update_oracle_conf_tolerance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_oracle_conf_tolerance_invoke(
    accounts: UpdateOracleConfToleranceAccounts<'_, '_>,
    args: UpdateOracleConfToleranceIxArgs,
) -> ProgramResult {
    update_oracle_conf_tolerance_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_oracle_conf_tolerance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleConfToleranceAccounts<'_, '_>,
    args: UpdateOracleConfToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOracleConfToleranceKeys = accounts.into();
    let ix = update_oracle_conf_tolerance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_oracle_conf_tolerance_invoke_signed(
    accounts: UpdateOracleConfToleranceAccounts<'_, '_>,
    args: UpdateOracleConfToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_oracle_conf_tolerance_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_oracle_conf_tolerance_verify_account_keys(
    accounts: UpdateOracleConfToleranceAccounts<'_, '_>,
    keys: UpdateOracleConfToleranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_oracle_conf_tolerance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_oracle_conf_tolerance_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_oracle_conf_tolerance_verify_account_privileges<'me, 'info>(
    accounts: UpdateOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_oracle_conf_tolerance_verify_writable_privileges(accounts)?;
    update_oracle_conf_tolerance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleIntervalAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOracleIntervalKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateOracleIntervalAccounts<'_, '_>> for UpdateOracleIntervalKeys {
    fn from(accounts: UpdateOracleIntervalAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateOracleIntervalKeys>
for [AccountMeta; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOracleIntervalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]>
for UpdateOracleIntervalKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateOracleIntervalAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOracleIntervalAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]>
for UpdateOracleIntervalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_ORACLE_INTERVAL_IX_DISCM: [u8; 8usize] = [
    67, 242, 44, 246, 210, 94, 99, 214,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOracleIntervalIxArgs {
    pub new_oracle_interval_secs: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleIntervalIxData(pub UpdateOracleIntervalIxArgs);
impl From<UpdateOracleIntervalIxArgs> for UpdateOracleIntervalIxData {
    fn from(args: UpdateOracleIntervalIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOracleIntervalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_INTERVAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateOracleIntervalIxArgs {
                new_oracle_interval_secs,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_INTERVAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_oracle_interval_secs, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_oracle_interval_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOracleIntervalKeys,
    args: UpdateOracleIntervalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOracleIntervalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracle_interval_ix(
    keys: UpdateOracleIntervalKeys,
    args: UpdateOracleIntervalIxArgs,
) -> std::io::Result<Instruction> {
    update_oracle_interval_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_oracle_interval_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleIntervalAccounts<'_, '_>,
    args: UpdateOracleIntervalIxArgs,
) -> ProgramResult {
    let keys: UpdateOracleIntervalKeys = accounts.into();
    let ix = update_oracle_interval_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_oracle_interval_invoke(
    accounts: UpdateOracleIntervalAccounts<'_, '_>,
    args: UpdateOracleIntervalIxArgs,
) -> ProgramResult {
    update_oracle_interval_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_oracle_interval_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOracleIntervalAccounts<'_, '_>,
    args: UpdateOracleIntervalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOracleIntervalKeys = accounts.into();
    let ix = update_oracle_interval_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_oracle_interval_invoke_signed(
    accounts: UpdateOracleIntervalAccounts<'_, '_>,
    args: UpdateOracleIntervalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_oracle_interval_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_oracle_interval_verify_account_keys(
    accounts: UpdateOracleIntervalAccounts<'_, '_>,
    keys: UpdateOracleIntervalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_oracle_interval_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_oracle_interval_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_oracle_interval_verify_account_privileges<'me, 'info>(
    accounts: UpdateOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_oracle_interval_verify_writable_privileges(accounts)?;
    update_oracle_interval_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateParToleranceAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateParToleranceKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateParToleranceAccounts<'_, '_>> for UpdateParToleranceKeys {
    fn from(accounts: UpdateParToleranceAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateParToleranceKeys>
for [AccountMeta; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateParToleranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN]> for UpdateParToleranceKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateParToleranceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateParToleranceAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateParToleranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_PAR_TOLERANCE_IX_DISCM: [u8; 8usize] = [
    71, 55, 189, 45, 248, 52, 32, 172,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateParToleranceIxArgs {
    pub new_par_tolerance: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateParToleranceIxData(pub UpdateParToleranceIxArgs);
impl From<UpdateParToleranceIxArgs> for UpdateParToleranceIxData {
    fn from(args: UpdateParToleranceIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateParToleranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PAR_TOLERANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_par_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateParToleranceIxArgs {
                new_par_tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PAR_TOLERANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_par_tolerance, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_par_tolerance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateParToleranceKeys,
    args: UpdateParToleranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PAR_TOLERANCE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateParToleranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_par_tolerance_ix(
    keys: UpdateParToleranceKeys,
    args: UpdateParToleranceIxArgs,
) -> std::io::Result<Instruction> {
    update_par_tolerance_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_par_tolerance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateParToleranceAccounts<'_, '_>,
    args: UpdateParToleranceIxArgs,
) -> ProgramResult {
    let keys: UpdateParToleranceKeys = accounts.into();
    let ix = update_par_tolerance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_par_tolerance_invoke(
    accounts: UpdateParToleranceAccounts<'_, '_>,
    args: UpdateParToleranceIxArgs,
) -> ProgramResult {
    update_par_tolerance_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn update_par_tolerance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateParToleranceAccounts<'_, '_>,
    args: UpdateParToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateParToleranceKeys = accounts.into();
    let ix = update_par_tolerance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_par_tolerance_invoke_signed(
    accounts: UpdateParToleranceAccounts<'_, '_>,
    args: UpdateParToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_par_tolerance_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_par_tolerance_verify_account_keys(
    accounts: UpdateParToleranceAccounts<'_, '_>,
    keys: UpdateParToleranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_par_tolerance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateParToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_par_tolerance_verify_signer_privileges<'me, 'info>(
    accounts: UpdateParToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_par_tolerance_verify_account_privileges<'me, 'info>(
    accounts: UpdateParToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_par_tolerance_verify_writable_privileges(accounts)?;
    update_par_tolerance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSolUsdOracleAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSolUsdOracleKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateSolUsdOracleAccounts<'_, '_>> for UpdateSolUsdOracleKeys {
    fn from(accounts: UpdateSolUsdOracleAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateSolUsdOracleKeys>
for [AccountMeta; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSolUsdOracleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN]> for UpdateSolUsdOracleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateSolUsdOracleAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSolUsdOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN]>
for UpdateSolUsdOracleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_SOL_USD_ORACLE_IX_DISCM: [u8; 8usize] = [
    95, 236, 18, 102, 201, 114, 89, 54,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateSolUsdOracleIxArgs {
    pub new_oracle: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSolUsdOracleIxData(pub UpdateSolUsdOracleIxArgs);
impl From<UpdateSolUsdOracleIxArgs> for UpdateSolUsdOracleIxData {
    fn from(args: UpdateSolUsdOracleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSolUsdOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SOL_USD_ORACLE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateSolUsdOracleIxArgs {
                new_oracle,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SOL_USD_ORACLE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_oracle, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_sol_usd_oracle_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSolUsdOracleKeys,
    args: UpdateSolUsdOracleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SOL_USD_ORACLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateSolUsdOracleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_sol_usd_oracle_ix(
    keys: UpdateSolUsdOracleKeys,
    args: UpdateSolUsdOracleIxArgs,
) -> std::io::Result<Instruction> {
    update_sol_usd_oracle_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_sol_usd_oracle_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSolUsdOracleAccounts<'_, '_>,
    args: UpdateSolUsdOracleIxArgs,
) -> ProgramResult {
    let keys: UpdateSolUsdOracleKeys = accounts.into();
    let ix = update_sol_usd_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_sol_usd_oracle_invoke(
    accounts: UpdateSolUsdOracleAccounts<'_, '_>,
    args: UpdateSolUsdOracleIxArgs,
) -> ProgramResult {
    update_sol_usd_oracle_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_sol_usd_oracle_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSolUsdOracleAccounts<'_, '_>,
    args: UpdateSolUsdOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSolUsdOracleKeys = accounts.into();
    let ix = update_sol_usd_oracle_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_sol_usd_oracle_invoke_signed(
    accounts: UpdateSolUsdOracleAccounts<'_, '_>,
    args: UpdateSolUsdOracleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_sol_usd_oracle_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_sol_usd_oracle_verify_account_keys(
    accounts: UpdateSolUsdOracleAccounts<'_, '_>,
    keys: UpdateSolUsdOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_sol_usd_oracle_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSolUsdOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_sol_usd_oracle_verify_signer_privileges<'me, 'info>(
    accounts: UpdateSolUsdOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_sol_usd_oracle_verify_account_privileges<'me, 'info>(
    accounts: UpdateSolUsdOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_sol_usd_oracle_verify_writable_privileges(accounts)?;
    update_sol_usd_oracle_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUsdcMintFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUsdcMintFeeKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateUsdcMintFeeAccounts<'_, '_>> for UpdateUsdcMintFeeKeys {
    fn from(accounts: UpdateUsdcMintFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateUsdcMintFeeKeys>
for [AccountMeta; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUsdcMintFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN]> for UpdateUsdcMintFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateUsdcMintFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUsdcMintFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN]>
for UpdateUsdcMintFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_USDC_MINT_FEE_IX_DISCM: [u8; 8usize] = [
    128, 125, 81, 38, 132, 39, 28, 56,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUsdcMintFeeIxArgs {
    pub new_mint_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUsdcMintFeeIxData(pub UpdateUsdcMintFeeIxArgs);
impl From<UpdateUsdcMintFeeIxArgs> for UpdateUsdcMintFeeIxData {
    fn from(args: UpdateUsdcMintFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUsdcMintFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USDC_MINT_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_mint_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateUsdcMintFeeIxArgs {
                new_mint_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USDC_MINT_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_mint_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_usdc_mint_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUsdcMintFeeKeys,
    args: UpdateUsdcMintFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USDC_MINT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUsdcMintFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_usdc_mint_fee_ix(
    keys: UpdateUsdcMintFeeKeys,
    args: UpdateUsdcMintFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_usdc_mint_fee_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_usdc_mint_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcMintFeeAccounts<'_, '_>,
    args: UpdateUsdcMintFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateUsdcMintFeeKeys = accounts.into();
    let ix = update_usdc_mint_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_usdc_mint_fee_invoke(
    accounts: UpdateUsdcMintFeeAccounts<'_, '_>,
    args: UpdateUsdcMintFeeIxArgs,
) -> ProgramResult {
    update_usdc_mint_fee_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts, args)
}
pub fn update_usdc_mint_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcMintFeeAccounts<'_, '_>,
    args: UpdateUsdcMintFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUsdcMintFeeKeys = accounts.into();
    let ix = update_usdc_mint_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_usdc_mint_fee_invoke_signed(
    accounts: UpdateUsdcMintFeeAccounts<'_, '_>,
    args: UpdateUsdcMintFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_usdc_mint_fee_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_usdc_mint_fee_verify_account_keys(
    accounts: UpdateUsdcMintFeeAccounts<'_, '_>,
    keys: UpdateUsdcMintFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_usdc_mint_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUsdcMintFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_usdc_mint_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUsdcMintFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_usdc_mint_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateUsdcMintFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_usdc_mint_fee_verify_writable_privileges(accounts)?;
    update_usdc_mint_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUsdcOracleConfToleranceAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUsdcOracleConfToleranceKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateUsdcOracleConfToleranceAccounts<'_, '_>>
for UpdateUsdcOracleConfToleranceKeys {
    fn from(accounts: UpdateUsdcOracleConfToleranceAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateUsdcOracleConfToleranceKeys>
for [AccountMeta; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUsdcOracleConfToleranceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateUsdcOracleConfToleranceKeys {
    fn from(
        pubkeys: [Pubkey; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateUsdcOracleConfToleranceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUsdcOracleConfToleranceAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN]>
for UpdateUsdcOracleConfToleranceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_DISCM: [u8; 8usize] = [
    135, 130, 119, 118, 26, 214, 232, 128,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUsdcOracleConfToleranceIxArgs {
    pub new_oracle_conf_tolerance: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUsdcOracleConfToleranceIxData(pub UpdateUsdcOracleConfToleranceIxArgs);
impl From<UpdateUsdcOracleConfToleranceIxArgs> for UpdateUsdcOracleConfToleranceIxData {
    fn from(args: UpdateUsdcOracleConfToleranceIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUsdcOracleConfToleranceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateUsdcOracleConfToleranceIxArgs {
                new_oracle_conf_tolerance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.new_oracle_conf_tolerance,
            &mut writer,
        )?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_usdc_oracle_conf_tolerance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUsdcOracleConfToleranceKeys,
    args: UpdateUsdcOracleConfToleranceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USDC_ORACLE_CONF_TOLERANCE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateUsdcOracleConfToleranceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_usdc_oracle_conf_tolerance_ix(
    keys: UpdateUsdcOracleConfToleranceKeys,
    args: UpdateUsdcOracleConfToleranceIxArgs,
) -> std::io::Result<Instruction> {
    update_usdc_oracle_conf_tolerance_ix_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_usdc_oracle_conf_tolerance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcOracleConfToleranceAccounts<'_, '_>,
    args: UpdateUsdcOracleConfToleranceIxArgs,
) -> ProgramResult {
    let keys: UpdateUsdcOracleConfToleranceKeys = accounts.into();
    let ix = update_usdc_oracle_conf_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn update_usdc_oracle_conf_tolerance_invoke(
    accounts: UpdateUsdcOracleConfToleranceAccounts<'_, '_>,
    args: UpdateUsdcOracleConfToleranceIxArgs,
) -> ProgramResult {
    update_usdc_oracle_conf_tolerance_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_usdc_oracle_conf_tolerance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcOracleConfToleranceAccounts<'_, '_>,
    args: UpdateUsdcOracleConfToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUsdcOracleConfToleranceKeys = accounts.into();
    let ix = update_usdc_oracle_conf_tolerance_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_usdc_oracle_conf_tolerance_invoke_signed(
    accounts: UpdateUsdcOracleConfToleranceAccounts<'_, '_>,
    args: UpdateUsdcOracleConfToleranceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_usdc_oracle_conf_tolerance_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_usdc_oracle_conf_tolerance_verify_account_keys(
    accounts: UpdateUsdcOracleConfToleranceAccounts<'_, '_>,
    keys: UpdateUsdcOracleConfToleranceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_usdc_oracle_conf_tolerance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUsdcOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_usdc_oracle_conf_tolerance_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUsdcOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_usdc_oracle_conf_tolerance_verify_account_privileges<'me, 'info>(
    accounts: UpdateUsdcOracleConfToleranceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_usdc_oracle_conf_tolerance_verify_writable_privileges(accounts)?;
    update_usdc_oracle_conf_tolerance_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUsdcOracleIntervalAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUsdcOracleIntervalKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateUsdcOracleIntervalAccounts<'_, '_>> for UpdateUsdcOracleIntervalKeys {
    fn from(accounts: UpdateUsdcOracleIntervalAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateUsdcOracleIntervalKeys>
for [AccountMeta; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUsdcOracleIntervalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]>
for UpdateUsdcOracleIntervalKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateUsdcOracleIntervalAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUsdcOracleIntervalAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN]>
for UpdateUsdcOracleIntervalAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_USDC_ORACLE_INTERVAL_IX_DISCM: [u8; 8usize] = [
    245, 200, 213, 219, 12, 73, 192, 235,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUsdcOracleIntervalIxArgs {
    pub new_oracle_interval_secs: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUsdcOracleIntervalIxData(pub UpdateUsdcOracleIntervalIxArgs);
impl From<UpdateUsdcOracleIntervalIxArgs> for UpdateUsdcOracleIntervalIxData {
    fn from(args: UpdateUsdcOracleIntervalIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUsdcOracleIntervalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USDC_ORACLE_INTERVAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateUsdcOracleIntervalIxArgs {
                new_oracle_interval_secs,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USDC_ORACLE_INTERVAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_oracle_interval_secs, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_usdc_oracle_interval_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUsdcOracleIntervalKeys,
    args: UpdateUsdcOracleIntervalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USDC_ORACLE_INTERVAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUsdcOracleIntervalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_usdc_oracle_interval_ix(
    keys: UpdateUsdcOracleIntervalKeys,
    args: UpdateUsdcOracleIntervalIxArgs,
) -> std::io::Result<Instruction> {
    update_usdc_oracle_interval_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_usdc_oracle_interval_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcOracleIntervalAccounts<'_, '_>,
    args: UpdateUsdcOracleIntervalIxArgs,
) -> ProgramResult {
    let keys: UpdateUsdcOracleIntervalKeys = accounts.into();
    let ix = update_usdc_oracle_interval_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_usdc_oracle_interval_invoke(
    accounts: UpdateUsdcOracleIntervalAccounts<'_, '_>,
    args: UpdateUsdcOracleIntervalIxArgs,
) -> ProgramResult {
    update_usdc_oracle_interval_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_usdc_oracle_interval_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcOracleIntervalAccounts<'_, '_>,
    args: UpdateUsdcOracleIntervalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUsdcOracleIntervalKeys = accounts.into();
    let ix = update_usdc_oracle_interval_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_usdc_oracle_interval_invoke_signed(
    accounts: UpdateUsdcOracleIntervalAccounts<'_, '_>,
    args: UpdateUsdcOracleIntervalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_usdc_oracle_interval_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_usdc_oracle_interval_verify_account_keys(
    accounts: UpdateUsdcOracleIntervalAccounts<'_, '_>,
    keys: UpdateUsdcOracleIntervalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_usdc_oracle_interval_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUsdcOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_usdc_oracle_interval_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUsdcOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_usdc_oracle_interval_verify_account_privileges<'me, 'info>(
    accounts: UpdateUsdcOracleIntervalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_usdc_oracle_interval_verify_writable_privileges(accounts)?;
    update_usdc_oracle_interval_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUsdcRedeemFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub usdc_pair: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUsdcRedeemFeeKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub usdc_pair: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateUsdcRedeemFeeAccounts<'_, '_>> for UpdateUsdcRedeemFeeKeys {
    fn from(accounts: UpdateUsdcRedeemFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            usdc_pair: *accounts.usdc_pair.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateUsdcRedeemFeeKeys>
for [AccountMeta; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUsdcRedeemFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.usdc_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN]> for UpdateUsdcRedeemFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            usdc_pair: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateUsdcRedeemFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUsdcRedeemFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.usdc_pair.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN]>
for UpdateUsdcRedeemFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            usdc_pair: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_USDC_REDEEM_FEE_IX_DISCM: [u8; 8usize] = [
    88, 181, 23, 199, 213, 123, 102, 112,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUsdcRedeemFeeIxArgs {
    pub new_redeem_fee: UFixValue64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUsdcRedeemFeeIxData(pub UpdateUsdcRedeemFeeIxArgs);
impl From<UpdateUsdcRedeemFeeIxArgs> for UpdateUsdcRedeemFeeIxData {
    fn from(args: UpdateUsdcRedeemFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUsdcRedeemFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USDC_REDEEM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_redeem_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateUsdcRedeemFeeIxArgs {
                new_redeem_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USDC_REDEEM_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_redeem_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_usdc_redeem_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUsdcRedeemFeeKeys,
    args: UpdateUsdcRedeemFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USDC_REDEEM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUsdcRedeemFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_usdc_redeem_fee_ix(
    keys: UpdateUsdcRedeemFeeKeys,
    args: UpdateUsdcRedeemFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_usdc_redeem_fee_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_usdc_redeem_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcRedeemFeeAccounts<'_, '_>,
    args: UpdateUsdcRedeemFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateUsdcRedeemFeeKeys = accounts.into();
    let ix = update_usdc_redeem_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_usdc_redeem_fee_invoke(
    accounts: UpdateUsdcRedeemFeeAccounts<'_, '_>,
    args: UpdateUsdcRedeemFeeIxArgs,
) -> ProgramResult {
    update_usdc_redeem_fee_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_usdc_redeem_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUsdcRedeemFeeAccounts<'_, '_>,
    args: UpdateUsdcRedeemFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUsdcRedeemFeeKeys = accounts.into();
    let ix = update_usdc_redeem_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_usdc_redeem_fee_invoke_signed(
    accounts: UpdateUsdcRedeemFeeAccounts<'_, '_>,
    args: UpdateUsdcRedeemFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_usdc_redeem_fee_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_usdc_redeem_fee_verify_account_keys(
    accounts: UpdateUsdcRedeemFeeAccounts<'_, '_>,
    keys: UpdateUsdcRedeemFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.usdc_pair.key, keys.usdc_pair),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_usdc_redeem_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUsdcRedeemFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.usdc_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_usdc_redeem_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUsdcRedeemFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_usdc_redeem_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateUsdcRedeemFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_usdc_redeem_fee_verify_writable_privileges(accounts)?;
    update_usdc_redeem_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateYieldHarvestConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateYieldHarvestConfigKeys {
    pub admin: Pubkey,
    pub hylo: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateYieldHarvestConfigAccounts<'_, '_>> for UpdateYieldHarvestConfigKeys {
    fn from(accounts: UpdateYieldHarvestConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            hylo: *accounts.hylo.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateYieldHarvestConfigKeys>
for [AccountMeta; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateYieldHarvestConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateYieldHarvestConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            hylo: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateYieldHarvestConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateYieldHarvestConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.hylo.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateYieldHarvestConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            hylo: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_YIELD_HARVEST_CONFIG_IX_DISCM: [u8; 8usize] = [
    140, 91, 46, 213, 44, 179, 201, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateYieldHarvestConfigIxArgs {
    pub new_yield_harvest_config: YieldHarvestConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateYieldHarvestConfigIxData(pub UpdateYieldHarvestConfigIxArgs);
impl From<UpdateYieldHarvestConfigIxArgs> for UpdateYieldHarvestConfigIxData {
    fn from(args: UpdateYieldHarvestConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateYieldHarvestConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_YIELD_HARVEST_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_yield_harvest_config = if reader.is_empty() {
            Default::default()
        } else {
            <YieldHarvestConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateYieldHarvestConfigIxArgs {
                new_yield_harvest_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_YIELD_HARVEST_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_yield_harvest_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_yield_harvest_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateYieldHarvestConfigKeys,
    args: UpdateYieldHarvestConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_YIELD_HARVEST_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateYieldHarvestConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_yield_harvest_config_ix(
    keys: UpdateYieldHarvestConfigKeys,
    args: UpdateYieldHarvestConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_yield_harvest_config_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys, args)
}
pub fn update_yield_harvest_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldHarvestConfigAccounts<'_, '_>,
    args: UpdateYieldHarvestConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateYieldHarvestConfigKeys = accounts.into();
    let ix = update_yield_harvest_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_yield_harvest_config_invoke(
    accounts: UpdateYieldHarvestConfigAccounts<'_, '_>,
    args: UpdateYieldHarvestConfigIxArgs,
) -> ProgramResult {
    update_yield_harvest_config_invoke_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_yield_harvest_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateYieldHarvestConfigAccounts<'_, '_>,
    args: UpdateYieldHarvestConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateYieldHarvestConfigKeys = accounts.into();
    let ix = update_yield_harvest_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_yield_harvest_config_invoke_signed(
    accounts: UpdateYieldHarvestConfigAccounts<'_, '_>,
    args: UpdateYieldHarvestConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_yield_harvest_config_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_yield_harvest_config_verify_account_keys(
    accounts: UpdateYieldHarvestConfigAccounts<'_, '_>,
    keys: UpdateYieldHarvestConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_yield_harvest_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateYieldHarvestConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.hylo] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_yield_harvest_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateYieldHarvestConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_yield_harvest_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateYieldHarvestConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_yield_harvest_config_verify_writable_privileges(accounts)?;
    update_yield_harvest_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FEES_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFeesAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub treasury: &'me AccountInfo<'info>,
    pub hylo: &'me AccountInfo<'info>,
    pub fee_auth: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub treasury_ata: &'me AccountInfo<'info>,
    pub fee_token_mint: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFeesKeys {
    pub payer: Pubkey,
    pub treasury: Pubkey,
    pub hylo: Pubkey,
    pub fee_auth: Pubkey,
    pub fee_vault: Pubkey,
    pub treasury_ata: Pubkey,
    pub fee_token_mint: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawFeesAccounts<'_, '_>> for WithdrawFeesKeys {
    fn from(accounts: WithdrawFeesAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            treasury: *accounts.treasury.key,
            hylo: *accounts.hylo.key,
            fee_auth: *accounts.fee_auth.key,
            fee_vault: *accounts.fee_vault.key,
            treasury_ata: *accounts.treasury_ata.key,
            fee_token_mint: *accounts.fee_token_mint.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawFeesKeys> for [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.hylo,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_FEES_IX_ACCOUNTS_LEN]> for WithdrawFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            treasury: pubkeys[1],
            hylo: pubkeys[2],
            fee_auth: pubkeys[3],
            fee_vault: pubkeys[4],
            treasury_ata: pubkeys[5],
            fee_token_mint: pubkeys[6],
            associated_token_program: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.treasury.clone(),
            accounts.hylo.clone(),
            accounts.fee_auth.clone(),
            accounts.fee_vault.clone(),
            accounts.treasury_ata.clone(),
            accounts.fee_token_mint.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]>
for WithdrawFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            treasury: &arr[1],
            hylo: &arr[2],
            fee_auth: &arr[3],
            fee_vault: &arr[4],
            treasury_ata: &arr[5],
            fee_token_mint: &arr[6],
            associated_token_program: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const WITHDRAW_FEES_IX_DISCM: [u8; 8usize] = [198, 212, 171, 109, 144, 215, 174, 89];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFeesIxData;
impl WithdrawFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_fees_ix(keys: WithdrawFeesKeys) -> std::io::Result<Instruction> {
    withdraw_fees_ix_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, keys)
}
pub fn withdraw_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawFeesKeys = accounts.into();
    let ix = withdraw_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_fees_invoke(accounts: WithdrawFeesAccounts<'_, '_>) -> ProgramResult {
    withdraw_fees_invoke_with_program_id(HYLO_EXCHANGE_PROGRAM_ID, accounts)
}
pub fn withdraw_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFeesKeys = accounts.into();
    let ix = withdraw_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_fees_invoke_signed(
    accounts: WithdrawFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_fees_invoke_signed_with_program_id(
        HYLO_EXCHANGE_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn withdraw_fees_verify_account_keys(
    accounts: WithdrawFeesAccounts<'_, '_>,
    keys: WithdrawFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.treasury.key, keys.treasury),
        (*accounts.hylo.key, keys.hylo),
        (*accounts.fee_auth.key, keys.fee_auth),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.treasury_ata.key, keys.treasury_ata),
        (*accounts.fee_token_mint.key, keys.fee_token_mint),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.treasury,
        accounts.fee_vault,
        accounts.treasury_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_fees_verify_writable_privileges(accounts)?;
    withdraw_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
