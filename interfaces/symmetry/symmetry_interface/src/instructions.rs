use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum SymmetryProgramIx {
    CreateFund(CreateFundIxArgs),
    CloseFund,
    SetRules(SetRulesIxArgs),
    FundEdit(FundEditIxArgs),
    SimpleFundEdit(SimpleFundEditIxArgs),
    LiquidityProvision(LiquidityProvisionIxArgs),
    BuyFund(BuyFundIxArgs),
    MintFund,
    InstantMint(InstantMintIxArgs),
    SingleTokenDeposit(SingleTokenDepositIxArgs),
    SellFund(SellFundIxArgs),
    InstantBurn(InstantBurnIxArgs),
    ClaimToken(ClaimTokenIxArgs),
    ClaimTokenFromBuyState(ClaimTokenFromBuyStateIxArgs),
    UpdateCurrentWeights,
    BuyStateRebalance(BuyStateRebalanceIxArgs),
    RebalanceToUsdc(RebalanceToUsdcIxArgs),
    RebalanceFromUsdc(RebalanceFromUsdcIxArgs),
    RebalanceBuyState(RebalanceBuyStateIxArgs),
    RebalanceSell(RebalanceSellIxArgs),
    RebalanceBuy(RebalanceBuyIxArgs),
    Reweight,
    Refilter,
    UpdateTokenList(UpdateTokenListIxArgs),
    ModifyFeeStructure(ModifyFeeStructureIxArgs),
    AddToken(AddTokenIxArgs),
    UpdateDatabase(UpdateDatabaseIxArgs),
    ClearDatabase(ClearDatabaseIxArgs),
    UpdateTokenStats(UpdateTokenStatsIxArgs),
    UpdateTokenStatsV2(UpdateTokenStatsV2IxArgs),
    UpdateCurveData(UpdateCurveDataIxArgs),
    InitializeTokenList,
    InitializeDatabase,
    InitializeTokenStats,
    InitializeCurveData,
    CloseDatabase,
    CloseTokenStats,
    CloseTokenInfo,
    CloseTokenList,
    CloseToken,
    CloseOpenOrders,
    CreateFundTokenMintMetadata(CreateFundTokenMintMetadataIxArgs),
    UpdateFundTokenMintMetadata(UpdateFundTokenMintMetadataIxArgs),
}
impl SymmetryProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_FUND_IX_DISCM) {
            let mut reader = &buf[CREATE_FUND_IX_DISCM.len()..];
            let message_digest_five: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let host_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let host_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let actively_managed: u64 = crate::borsh_de_or_default(&mut reader)?;
            let asset_pool = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            let refilter_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reweight_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
            let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_and_lp: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateFund(CreateFundIxArgs {
                    message_digest_five,
                    manager_fee,
                    host_pubkey,
                    host_fee,
                    actively_managed,
                    asset_pool,
                    refilter_interval,
                    reweight_interval,
                    rebalance_interval,
                    rebalance_threshold,
                    rebalance_slippage,
                    lp_offset_threshold,
                    rebalance_and_lp,
                }),
            );
        }
        if buf.starts_with(&CLOSE_FUND_IX_DISCM) {
            return Ok(Self::CloseFund);
        }
        if buf.starts_with(&SET_RULES_IX_DISCM) {
            let mut reader = &buf[SET_RULES_IX_DISCM.len()..];
            let num_of_rules: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rules_data = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            let rule_weights: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
            let rule_expos: [i64; 20] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetRules(SetRulesIxArgs {
                    num_of_rules,
                    rules_data,
                    rule_weights,
                    rule_expos,
                }),
            );
        }
        if buf.starts_with(&FUND_EDIT_IX_DISCM) {
            let mut reader = &buf[FUND_EDIT_IX_DISCM.len()..];
            let message_digest_five: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
            let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let actively_managed: u64 = crate::borsh_de_or_default(&mut reader)?;
            let asset_pool = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            let refilter_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reweight_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
            let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_and_lp: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FundEdit(FundEditIxArgs {
                    message_digest_five,
                    manager_fee,
                    actively_managed,
                    asset_pool,
                    refilter_interval,
                    reweight_interval,
                    rebalance_interval,
                    rebalance_threshold,
                    rebalance_slippage,
                    lp_offset_threshold,
                    rebalance_and_lp,
                }),
            );
        }
        if buf.starts_with(&SIMPLE_FUND_EDIT_IX_DISCM) {
            let mut reader = &buf[SIMPLE_FUND_EDIT_IX_DISCM.len()..];
            let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
            let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance_and_lp: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
            let num_of_tokens: u8 = crate::borsh_de_or_default(&mut reader)?;
            let target_composition: [u8; 20] = crate::borsh_de_or_default(&mut reader)?;
            let target_weights: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SimpleFundEdit(SimpleFundEditIxArgs {
                    manager_fee,
                    rebalance_interval,
                    rebalance_threshold,
                    rebalance_slippage,
                    lp_offset_threshold,
                    rebalance_and_lp,
                    num_of_tokens,
                    target_composition,
                    target_weights,
                }),
            );
        }
        if buf.starts_with(&LIQUIDITY_PROVISION_IX_DISCM) {
            let mut reader = &buf[LIQUIDITY_PROVISION_IX_DISCM.len()..];
            let from_token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let to_token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let from_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_to_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::LiquidityProvision(LiquidityProvisionIxArgs {
                    from_token_id,
                    to_token_id,
                    from_amount,
                    minimum_to_amount,
                }),
            );
        }
        if buf.starts_with(&BUY_FUND_IX_DISCM) {
            let mut reader = &buf[BUY_FUND_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::BuyFund(BuyFundIxArgs { amount }));
        }
        if buf.starts_with(&MINT_FUND_IX_DISCM) {
            return Ok(Self::MintFund);
        }
        if buf.starts_with(&INSTANT_MINT_IX_DISCM) {
            let mut reader = &buf[INSTANT_MINT_IX_DISCM.len()..];
            let amounts: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InstantMint(InstantMintIxArgs { amounts }));
        }
        if buf.starts_with(&SINGLE_TOKEN_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[SINGLE_TOKEN_DEPOSIT_IX_DISCM.len()..];
            let token: u8 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SingleTokenDeposit(SingleTokenDepositIxArgs {
                    token,
                    amount,
                }),
            );
        }
        if buf.starts_with(&SELL_FUND_IX_DISCM) {
            let mut reader = &buf[SELL_FUND_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let rebalance: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellFund(SellFundIxArgs {
                    amount,
                    rebalance,
                }),
            );
        }
        if buf.starts_with(&INSTANT_BURN_IX_DISCM) {
            let mut reader = &buf[INSTANT_BURN_IX_DISCM.len()..];
            let burn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_token: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InstantBurn(InstantBurnIxArgs {
                    burn_amount,
                    withdraw_token,
                }),
            );
        }
        if buf.starts_with(&CLAIM_TOKEN_IX_DISCM) {
            let mut reader = &buf[CLAIM_TOKEN_IX_DISCM.len()..];
            let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ClaimToken(ClaimTokenIxArgs { token_id }));
        }
        if buf.starts_with(&CLAIM_TOKEN_FROM_BUY_STATE_IX_DISCM) {
            let mut reader = &buf[CLAIM_TOKEN_FROM_BUY_STATE_IX_DISCM.len()..];
            let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ClaimTokenFromBuyState(ClaimTokenFromBuyStateIxArgs {
                    token_id,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CURRENT_WEIGHTS_IX_DISCM) {
            return Ok(Self::UpdateCurrentWeights);
        }
        if buf.starts_with(&BUY_STATE_REBALANCE_IX_DISCM) {
            let mut reader = &buf[BUY_STATE_REBALANCE_IX_DISCM.len()..];
            let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_id: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
            let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_data: [u8; 28] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyStateRebalance(BuyStateRebalanceIxArgs {
                    token_id,
                    instruction_id,
                    instruction_size,
                    instruction_data,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_TO_USDC_IX_DISCM) {
            let mut reader = &buf[REBALANCE_TO_USDC_IX_DISCM.len()..];
            let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let max_amount_to_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_id: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
            let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_data: [u8; 28] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RebalanceToUsdc(RebalanceToUsdcIxArgs {
                    token_id,
                    max_amount_to_sell,
                    instruction_id,
                    instruction_size,
                    instruction_data,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_FROM_USDC_IX_DISCM) {
            let mut reader = &buf[REBALANCE_FROM_USDC_IX_DISCM.len()..];
            let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let max_amount_to_spend: u64 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_id: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
            let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_data: [u8; 28] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RebalanceFromUsdc(RebalanceFromUsdcIxArgs {
                    token_id,
                    max_amount_to_spend,
                    instruction_id,
                    instruction_size,
                    instruction_data,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_BUY_STATE_IX_DISCM) {
            let mut reader = &buf[REBALANCE_BUY_STATE_IX_DISCM.len()..];
            let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_data = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::RebalanceBuyState(RebalanceBuyStateIxArgs {
                    token_id,
                    instruction_size,
                    instruction_data,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_SELL_IX_DISCM) {
            let mut reader = &buf[REBALANCE_SELL_IX_DISCM.len()..];
            let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let max_amount_to_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_data = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::RebalanceSell(RebalanceSellIxArgs {
                    token_id,
                    max_amount_to_sell,
                    instruction_size,
                    instruction_data,
                }),
            );
        }
        if buf.starts_with(&REBALANCE_BUY_IX_DISCM) {
            let mut reader = &buf[REBALANCE_BUY_IX_DISCM.len()..];
            let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
            let max_amount_to_spend: u64 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
            let instruction_data = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::RebalanceBuy(RebalanceBuyIxArgs {
                    token_id,
                    max_amount_to_spend,
                    instruction_size,
                    instruction_data,
                }),
            );
        }
        if buf.starts_with(&REWEIGHT_IX_DISCM) {
            return Ok(Self::Reweight);
        }
        if buf.starts_with(&REFILTER_IX_DISCM) {
            return Ok(Self::Refilter);
        }
        if buf.starts_with(&UPDATE_TOKEN_LIST_IX_DISCM) {
            let mut reader = &buf[UPDATE_TOKEN_LIST_IX_DISCM.len()..];
            let index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
            let coingecko_id: [u8; 30] = crate::borsh_de_or_default(&mut reader)?;
            let pda_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
            let oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let oracle_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let oracle_confidence_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
            let fixed_confidence_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
            let token_swap_fee_before_tw_bps: u8 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_swap_fee_after_tw_bps: u8 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let is_live: u8 = crate::borsh_de_or_default(&mut reader)?;
            let lp_on: u8 = crate::borsh_de_or_default(&mut reader)?;
            let use_curve_data: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateTokenList(UpdateTokenListIxArgs {
                    index,
                    token_mint,
                    decimals,
                    coingecko_id,
                    pda_token_account,
                    oracle_type,
                    oracle_account,
                    oracle_index,
                    oracle_confidence_pct,
                    fixed_confidence_bps,
                    token_swap_fee_before_tw_bps,
                    token_swap_fee_after_tw_bps,
                    is_live,
                    lp_on,
                    use_curve_data,
                }),
            );
        }
        if buf.starts_with(&MODIFY_FEE_STRUCTURE_IX_DISCM) {
            let mut reader = &buf[MODIFY_FEE_STRUCTURE_IX_DISCM.len()..];
            let symmetry_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
            let host_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
            let manager_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ModifyFeeStructure(ModifyFeeStructureIxArgs {
                    symmetry_fee,
                    host_fee,
                    manager_fee,
                }),
            );
        }
        if buf.starts_with(&ADD_TOKEN_IX_DISCM) {
            let mut reader = &buf[ADD_TOKEN_IX_DISCM.len()..];
            let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let pda_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let coingecko_id: [u8; 30] = crate::borsh_de_or_default(&mut reader)?;
            let pyth: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
            let index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddToken(AddTokenIxArgs {
                    token_mint,
                    pda_token_account,
                    coingecko_id,
                    pyth,
                    decimals,
                    index,
                }),
            );
        }
        if buf.starts_with(&UPDATE_DATABASE_IX_DISCM) {
            let mut reader = &buf[UPDATE_DATABASE_IX_DISCM.len()..];
            let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let circulating_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
            let volume: u64 = crate::borsh_de_or_default(&mut reader)?;
            let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateDatabase(UpdateDatabaseIxArgs {
                    token_id,
                    price,
                    circulating_supply,
                    volume,
                    timestamp,
                }),
            );
        }
        if buf.starts_with(&CLEAR_DATABASE_IX_DISCM) {
            let mut reader = &buf[CLEAR_DATABASE_IX_DISCM.len()..];
            let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ClearDatabase(ClearDatabaseIxArgs { token_id }));
        }
        if buf.starts_with(&UPDATE_TOKEN_STATS_IX_DISCM) {
            let mut reader = &buf[UPDATE_TOKEN_STATS_IX_DISCM.len()..];
            let start_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let end_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateTokenStats(UpdateTokenStatsIxArgs {
                    start_index,
                    end_index,
                }),
            );
        }
        if buf.starts_with(&UPDATE_TOKEN_STATS_V2_IX_DISCM) {
            let mut reader = &buf[UPDATE_TOKEN_STATS_V2_IX_DISCM.len()..];
            let token: u8 = crate::borsh_de_or_default(&mut reader)?;
            let data: [[u64; 3]; 6] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateTokenStatsV2(UpdateTokenStatsV2IxArgs {
                    token,
                    data,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CURVE_DATA_IX_DISCM) {
            let mut reader = &buf[UPDATE_CURVE_DATA_IX_DISCM.len()..];
            let start_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let end_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let price_data: [[[[u64; 2]; 10]; 2]; 3] = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateCurveData(UpdateCurveDataIxArgs {
                    start_index,
                    end_index,
                    price_data,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_TOKEN_LIST_IX_DISCM) {
            return Ok(Self::InitializeTokenList);
        }
        if buf.starts_with(&INITIALIZE_DATABASE_IX_DISCM) {
            return Ok(Self::InitializeDatabase);
        }
        if buf.starts_with(&INITIALIZE_TOKEN_STATS_IX_DISCM) {
            return Ok(Self::InitializeTokenStats);
        }
        if buf.starts_with(&INITIALIZE_CURVE_DATA_IX_DISCM) {
            return Ok(Self::InitializeCurveData);
        }
        if buf.starts_with(&CLOSE_DATABASE_IX_DISCM) {
            return Ok(Self::CloseDatabase);
        }
        if buf.starts_with(&CLOSE_TOKEN_STATS_IX_DISCM) {
            return Ok(Self::CloseTokenStats);
        }
        if buf.starts_with(&CLOSE_TOKEN_INFO_IX_DISCM) {
            return Ok(Self::CloseTokenInfo);
        }
        if buf.starts_with(&CLOSE_TOKEN_LIST_IX_DISCM) {
            return Ok(Self::CloseTokenList);
        }
        if buf.starts_with(&CLOSE_TOKEN_IX_DISCM) {
            return Ok(Self::CloseToken);
        }
        if buf.starts_with(&CLOSE_OPEN_ORDERS_IX_DISCM) {
            return Ok(Self::CloseOpenOrders);
        }
        if buf.starts_with(&CREATE_FUND_TOKEN_MINT_METADATA_IX_DISCM) {
            let mut reader = &buf[CREATE_FUND_TOKEN_MINT_METADATA_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateMetadataParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateFundTokenMintMetadata(CreateFundTokenMintMetadataIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FUND_TOKEN_MINT_METADATA_IX_DISCM) {
            let mut reader = &buf[UPDATE_FUND_TOKEN_MINT_METADATA_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateMetadataParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateFundTokenMintMetadata(UpdateFundTokenMintMetadataIxArgs {
                    params,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateFund(args) => {
                writer.write_all(&CREATE_FUND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.message_digest_five,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.manager_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.host_pubkey, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.host_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.actively_managed, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.asset_pool, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.refilter_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reweight_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rebalance_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.rebalance_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.rebalance_slippage, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.lp_offset_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.rebalance_and_lp, &mut writer)?;
                Ok(())
            }
            Self::CloseFund => writer.write_all(&CLOSE_FUND_IX_DISCM),
            Self::SetRules(args) => {
                writer.write_all(&SET_RULES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.num_of_rules, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rules_data, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rule_weights, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rule_expos, &mut writer)?;
                Ok(())
            }
            Self::FundEdit(args) => {
                writer.write_all(&FUND_EDIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.message_digest_five,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.manager_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.actively_managed, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.asset_pool, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.refilter_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reweight_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rebalance_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.rebalance_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.rebalance_slippage, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.lp_offset_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.rebalance_and_lp, &mut writer)?;
                Ok(())
            }
            Self::SimpleFundEdit(args) => {
                writer.write_all(&SIMPLE_FUND_EDIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.manager_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rebalance_interval, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.rebalance_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.rebalance_slippage, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.lp_offset_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.rebalance_and_lp, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.num_of_tokens, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.target_composition, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.target_weights, &mut writer)?;
                Ok(())
            }
            Self::LiquidityProvision(args) => {
                writer.write_all(&LIQUIDITY_PROVISION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.from_token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.to_token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.from_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_to_amount, &mut writer)?;
                Ok(())
            }
            Self::BuyFund(args) => {
                writer.write_all(&BUY_FUND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::MintFund => writer.write_all(&MINT_FUND_IX_DISCM),
            Self::InstantMint(args) => {
                writer.write_all(&INSTANT_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amounts, &mut writer)?;
                Ok(())
            }
            Self::SingleTokenDeposit(args) => {
                writer.write_all(&SINGLE_TOKEN_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::SellFund(args) => {
                writer.write_all(&SELL_FUND_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.rebalance, &mut writer)?;
                Ok(())
            }
            Self::InstantBurn(args) => {
                writer.write_all(&INSTANT_BURN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.burn_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_token, &mut writer)?;
                Ok(())
            }
            Self::ClaimToken(args) => {
                writer.write_all(&CLAIM_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                Ok(())
            }
            Self::ClaimTokenFromBuyState(args) => {
                writer.write_all(&CLAIM_TOKEN_FROM_BUY_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                Ok(())
            }
            Self::UpdateCurrentWeights => {
                writer.write_all(&UPDATE_CURRENT_WEIGHTS_IX_DISCM)
            }
            Self::BuyStateRebalance(args) => {
                writer.write_all(&BUY_STATE_REBALANCE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_data, &mut writer)?;
                Ok(())
            }
            Self::RebalanceToUsdc(args) => {
                writer.write_all(&REBALANCE_TO_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_amount_to_sell, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_data, &mut writer)?;
                Ok(())
            }
            Self::RebalanceFromUsdc(args) => {
                writer.write_all(&REBALANCE_FROM_USDC_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_amount_to_spend,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.instruction_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_data, &mut writer)?;
                Ok(())
            }
            Self::RebalanceBuyState(args) => {
                writer.write_all(&REBALANCE_BUY_STATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_data, &mut writer)?;
                Ok(())
            }
            Self::RebalanceSell(args) => {
                writer.write_all(&REBALANCE_SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_amount_to_sell, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_data, &mut writer)?;
                Ok(())
            }
            Self::RebalanceBuy(args) => {
                writer.write_all(&REBALANCE_BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_amount_to_spend,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.instruction_size, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.instruction_data, &mut writer)?;
                Ok(())
            }
            Self::Reweight => writer.write_all(&REWEIGHT_IX_DISCM),
            Self::Refilter => writer.write_all(&REFILTER_IX_DISCM),
            Self::UpdateTokenList(args) => {
                writer.write_all(&UPDATE_TOKEN_LIST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_mint, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.decimals, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.coingecko_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pda_token_account, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle_account, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.oracle_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.oracle_confidence_pct,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.fixed_confidence_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_swap_fee_before_tw_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_swap_fee_after_tw_bps,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.is_live, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lp_on, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.use_curve_data, &mut writer)?;
                Ok(())
            }
            Self::ModifyFeeStructure(args) => {
                writer.write_all(&MODIFY_FEE_STRUCTURE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.symmetry_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.host_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.manager_fee, &mut writer)?;
                Ok(())
            }
            Self::AddToken(args) => {
                writer.write_all(&ADD_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_mint, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pda_token_account, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.coingecko_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pyth, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.decimals, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::UpdateDatabase(args) => {
                writer.write_all(&UPDATE_DATABASE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.circulating_supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.volume, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.timestamp, &mut writer)?;
                Ok(())
            }
            Self::ClearDatabase(args) => {
                writer.write_all(&CLEAR_DATABASE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_id, &mut writer)?;
                Ok(())
            }
            Self::UpdateTokenStats(args) => {
                writer.write_all(&UPDATE_TOKEN_STATS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_index, &mut writer)?;
                Ok(())
            }
            Self::UpdateTokenStatsV2(args) => {
                writer.write_all(&UPDATE_TOKEN_STATS_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.data, &mut writer)?;
                Ok(())
            }
            Self::UpdateCurveData(args) => {
                writer.write_all(&UPDATE_CURVE_DATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.price_data, &mut writer)?;
                Ok(())
            }
            Self::InitializeTokenList => {
                writer.write_all(&INITIALIZE_TOKEN_LIST_IX_DISCM)
            }
            Self::InitializeDatabase => writer.write_all(&INITIALIZE_DATABASE_IX_DISCM),
            Self::InitializeTokenStats => {
                writer.write_all(&INITIALIZE_TOKEN_STATS_IX_DISCM)
            }
            Self::InitializeCurveData => {
                writer.write_all(&INITIALIZE_CURVE_DATA_IX_DISCM)
            }
            Self::CloseDatabase => writer.write_all(&CLOSE_DATABASE_IX_DISCM),
            Self::CloseTokenStats => writer.write_all(&CLOSE_TOKEN_STATS_IX_DISCM),
            Self::CloseTokenInfo => writer.write_all(&CLOSE_TOKEN_INFO_IX_DISCM),
            Self::CloseTokenList => writer.write_all(&CLOSE_TOKEN_LIST_IX_DISCM),
            Self::CloseToken => writer.write_all(&CLOSE_TOKEN_IX_DISCM),
            Self::CloseOpenOrders => writer.write_all(&CLOSE_OPEN_ORDERS_IX_DISCM),
            Self::CreateFundTokenMintMetadata(args) => {
                writer.write_all(&CREATE_FUND_TOKEN_MINT_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UpdateFundTokenMintMetadata(args) => {
                writer.write_all(&UPDATE_FUND_TOKEN_MINT_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
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
pub const CREATE_FUND_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateFundAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub create_fee_sweeper: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFundKeys {
    pub manager: Pubkey,
    pub token_list: Pubkey,
    pub fund_state: Pubkey,
    pub pda_account: Pubkey,
    pub fund_token: Pubkey,
    pub create_fee_sweeper: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateFundAccounts<'_, '_>> for CreateFundKeys {
    fn from(accounts: CreateFundAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            token_list: *accounts.token_list.key,
            fund_state: *accounts.fund_state.key,
            pda_account: *accounts.pda_account.key,
            fund_token: *accounts.fund_token.key,
            create_fee_sweeper: *accounts.create_fee_sweeper.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateFundKeys> for [AccountMeta; CREATE_FUND_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.create_fee_sweeper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_FUND_IX_ACCOUNTS_LEN]> for CreateFundKeys {
    fn from(pubkeys: [Pubkey; CREATE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            token_list: pubkeys[1],
            fund_state: pubkeys[2],
            pda_account: pubkeys[3],
            fund_token: pubkeys[4],
            create_fee_sweeper: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<CreateFundAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFundAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.token_list.clone(),
            accounts.fund_state.clone(),
            accounts.pda_account.clone(),
            accounts.fund_token.clone(),
            accounts.create_fee_sweeper.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FUND_IX_ACCOUNTS_LEN]>
for CreateFundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            token_list: &arr[1],
            fund_state: &arr[2],
            pda_account: &arr[3],
            fund_token: &arr[4],
            create_fee_sweeper: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            rent: &arr[8],
        }
    }
}
pub const CREATE_FUND_IX_DISCM: [u8; 8usize] = [38, 128, 18, 11, 203, 0, 153, 21];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFundIxArgs {
    pub message_digest_five: [u8; 16],
    pub manager_fee: u64,
    pub host_pubkey: Pubkey,
    pub host_fee: u64,
    pub actively_managed: u64,
    #[serde(with = "crate::big_array_serde")]
    pub asset_pool: [u8; 256],
    pub refilter_interval: u64,
    pub reweight_interval: u64,
    pub rebalance_interval: u64,
    pub rebalance_threshold: u64,
    pub rebalance_slippage: u64,
    pub lp_offset_threshold: u64,
    pub rebalance_and_lp: [u8; 2],
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFundIxData(pub CreateFundIxArgs);
impl From<CreateFundIxArgs> for CreateFundIxData {
    fn from(args: CreateFundIxArgs) -> Self {
        Self(args)
    }
}
impl CreateFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let message_digest_five: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let host_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let host_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let actively_managed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_pool = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let refilter_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reweight_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_and_lp: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateFundIxArgs {
                message_digest_five,
                manager_fee,
                host_pubkey,
                host_fee,
                actively_managed,
                asset_pool,
                refilter_interval,
                reweight_interval,
                rebalance_interval,
                rebalance_threshold,
                rebalance_slippage,
                lp_offset_threshold,
                rebalance_and_lp,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FUND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.message_digest_five, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.manager_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.host_pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.host_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.actively_managed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.asset_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.refilter_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reweight_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lp_offset_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_and_lp, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_fund_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFundKeys,
    args: CreateFundIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FUND_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateFundIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_fund_ix(
    keys: CreateFundKeys,
    args: CreateFundIxArgs,
) -> std::io::Result<Instruction> {
    create_fund_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn create_fund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFundAccounts<'_, '_>,
    args: CreateFundIxArgs,
) -> ProgramResult {
    let keys: CreateFundKeys = accounts.into();
    let ix = create_fund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_fund_invoke(
    accounts: CreateFundAccounts<'_, '_>,
    args: CreateFundIxArgs,
) -> ProgramResult {
    create_fund_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn create_fund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFundAccounts<'_, '_>,
    args: CreateFundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFundKeys = accounts.into();
    let ix = create_fund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_fund_invoke_signed(
    accounts: CreateFundAccounts<'_, '_>,
    args: CreateFundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_fund_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_fund_verify_account_keys(
    accounts: CreateFundAccounts<'_, '_>,
    keys: CreateFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.create_fee_sweeper.key, keys.create_fee_sweeper),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_fund_verify_writable_privileges<'me, 'info>(
    accounts: CreateFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.fund_state,
        accounts.fund_token,
        accounts.create_fee_sweeper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_fund_verify_signer_privileges<'me, 'info>(
    accounts: CreateFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager, accounts.fund_token] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_fund_verify_account_privileges<'me, 'info>(
    accounts: CreateFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_fund_verify_writable_privileges(accounts)?;
    create_fund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_FUND_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CloseFundAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseFundKeys {
    pub manager: Pubkey,
    pub fund_token: Pubkey,
    pub fund_state: Pubkey,
    pub pda_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<CloseFundAccounts<'_, '_>> for CloseFundKeys {
    fn from(accounts: CloseFundAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fund_token: *accounts.fund_token.key,
            fund_state: *accounts.fund_state.key,
            pda_account: *accounts.pda_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CloseFundKeys> for [AccountMeta; CLOSE_FUND_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseFundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_FUND_IX_ACCOUNTS_LEN]> for CloseFundKeys {
    fn from(pubkeys: [Pubkey; CLOSE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fund_token: pubkeys[1],
            fund_state: pubkeys[2],
            pda_account: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<CloseFundAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_FUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseFundAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fund_token.clone(),
            accounts.fund_state.clone(),
            accounts.pda_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_FUND_IX_ACCOUNTS_LEN]>
for CloseFundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fund_token: &arr[1],
            fund_state: &arr[2],
            pda_account: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CLOSE_FUND_IX_DISCM: [u8; 8usize] = [230, 183, 3, 112, 236, 252, 5, 185];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseFundIxData;
impl CloseFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_FUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_FUND_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_fund_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseFundKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_FUND_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseFundIxData.try_to_vec()?,
    })
}
pub fn close_fund_ix(keys: CloseFundKeys) -> std::io::Result<Instruction> {
    close_fund_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_fund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseFundAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseFundKeys = accounts.into();
    let ix = close_fund_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_fund_invoke(accounts: CloseFundAccounts<'_, '_>) -> ProgramResult {
    close_fund_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_fund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseFundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseFundKeys = accounts.into();
    let ix = close_fund_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_fund_invoke_signed(
    accounts: CloseFundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_fund_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_fund_verify_account_keys(
    accounts: CloseFundAccounts<'_, '_>,
    keys: CloseFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_fund_verify_writable_privileges<'me, 'info>(
    accounts: CloseFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.fund_token,
        accounts.fund_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_fund_verify_signer_privileges<'me, 'info>(
    accounts: CloseFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_fund_verify_account_privileges<'me, 'info>(
    accounts: CloseFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_fund_verify_writable_privileges(accounts)?;
    close_fund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_RULES_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetRulesAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRulesKeys {
    pub manager: Pubkey,
    pub fund_state: Pubkey,
}
impl From<SetRulesAccounts<'_, '_>> for SetRulesKeys {
    fn from(accounts: SetRulesAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fund_state: *accounts.fund_state.key,
        }
    }
}
impl From<SetRulesKeys> for [AccountMeta; SET_RULES_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRulesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_RULES_IX_ACCOUNTS_LEN]> for SetRulesKeys {
    fn from(pubkeys: [Pubkey; SET_RULES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fund_state: pubkeys[1],
        }
    }
}
impl<'info> From<SetRulesAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_RULES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRulesAccounts<'_, 'info>) -> Self {
        [accounts.manager.clone(), accounts.fund_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_RULES_IX_ACCOUNTS_LEN]>
for SetRulesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_RULES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fund_state: &arr[1],
        }
    }
}
pub const SET_RULES_IX_DISCM: [u8; 8usize] = [66, 148, 196, 43, 232, 210, 174, 169];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetRulesIxArgs {
    pub num_of_rules: u64,
    #[serde(with = "crate::big_array_serde")]
    pub rules_data: [u8; 512],
    pub rule_weights: [u64; 20],
    pub rule_expos: [i64; 20],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRulesIxData(pub SetRulesIxArgs);
impl From<SetRulesIxArgs> for SetRulesIxData {
    fn from(args: SetRulesIxArgs) -> Self {
        Self(args)
    }
}
impl SetRulesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_RULES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let num_of_rules: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rules_data = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let rule_weights: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        let rule_expos: [i64; 20] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetRulesIxArgs {
                num_of_rules,
                rules_data,
                rule_weights,
                rule_expos,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_RULES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.num_of_rules, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rules_data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rule_weights, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rule_expos, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_rules_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRulesKeys,
    args: SetRulesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_RULES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRulesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_rules_ix(
    keys: SetRulesKeys,
    args: SetRulesIxArgs,
) -> std::io::Result<Instruction> {
    set_rules_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn set_rules_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRulesAccounts<'_, '_>,
    args: SetRulesIxArgs,
) -> ProgramResult {
    let keys: SetRulesKeys = accounts.into();
    let ix = set_rules_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_rules_invoke(
    accounts: SetRulesAccounts<'_, '_>,
    args: SetRulesIxArgs,
) -> ProgramResult {
    set_rules_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn set_rules_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRulesAccounts<'_, '_>,
    args: SetRulesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRulesKeys = accounts.into();
    let ix = set_rules_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_rules_invoke_signed(
    accounts: SetRulesAccounts<'_, '_>,
    args: SetRulesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_rules_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_rules_verify_account_keys(
    accounts: SetRulesAccounts<'_, '_>,
    keys: SetRulesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.fund_state.key, keys.fund_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_rules_verify_writable_privileges<'me, 'info>(
    accounts: SetRulesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.manager, accounts.fund_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_rules_verify_signer_privileges<'me, 'info>(
    accounts: SetRulesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_rules_verify_account_privileges<'me, 'info>(
    accounts: SetRulesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_rules_verify_writable_privileges(accounts)?;
    set_rules_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FUND_EDIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct FundEditAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundEditKeys {
    pub manager: Pubkey,
    pub token_list: Pubkey,
    pub fund_state: Pubkey,
}
impl From<FundEditAccounts<'_, '_>> for FundEditKeys {
    fn from(accounts: FundEditAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            token_list: *accounts.token_list.key,
            fund_state: *accounts.fund_state.key,
        }
    }
}
impl From<FundEditKeys> for [AccountMeta; FUND_EDIT_IX_ACCOUNTS_LEN] {
    fn from(keys: FundEditKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; FUND_EDIT_IX_ACCOUNTS_LEN]> for FundEditKeys {
    fn from(pubkeys: [Pubkey; FUND_EDIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            token_list: pubkeys[1],
            fund_state: pubkeys[2],
        }
    }
}
impl<'info> From<FundEditAccounts<'_, 'info>>
for [AccountInfo<'info>; FUND_EDIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundEditAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.token_list.clone(),
            accounts.fund_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_EDIT_IX_ACCOUNTS_LEN]>
for FundEditAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FUND_EDIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            token_list: &arr[1],
            fund_state: &arr[2],
        }
    }
}
pub const FUND_EDIT_IX_DISCM: [u8; 8usize] = [110, 43, 45, 159, 248, 226, 79, 91];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundEditIxArgs {
    pub message_digest_five: [u8; 16],
    pub manager_fee: u64,
    pub actively_managed: u64,
    #[serde(with = "crate::big_array_serde")]
    pub asset_pool: [u8; 256],
    pub refilter_interval: u64,
    pub reweight_interval: u64,
    pub rebalance_interval: u64,
    pub rebalance_threshold: u64,
    pub rebalance_slippage: u64,
    pub lp_offset_threshold: u64,
    pub rebalance_and_lp: [u8; 2],
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundEditIxData(pub FundEditIxArgs);
impl From<FundEditIxArgs> for FundEditIxData {
    fn from(args: FundEditIxArgs) -> Self {
        Self(args)
    }
}
impl FundEditIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_EDIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let message_digest_five: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let actively_managed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_pool = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let refilter_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reweight_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_and_lp: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FundEditIxArgs {
                message_digest_five,
                manager_fee,
                actively_managed,
                asset_pool,
                refilter_interval,
                reweight_interval,
                rebalance_interval,
                rebalance_threshold,
                rebalance_slippage,
                lp_offset_threshold,
                rebalance_and_lp,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_EDIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.message_digest_five, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.manager_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.actively_managed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.asset_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.refilter_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reweight_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lp_offset_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_and_lp, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fund_edit_ix_with_program_id(
    program_id: Pubkey,
    keys: FundEditKeys,
    args: FundEditIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FUND_EDIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: FundEditIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fund_edit_ix(
    keys: FundEditKeys,
    args: FundEditIxArgs,
) -> std::io::Result<Instruction> {
    fund_edit_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn fund_edit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FundEditAccounts<'_, '_>,
    args: FundEditIxArgs,
) -> ProgramResult {
    let keys: FundEditKeys = accounts.into();
    let ix = fund_edit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fund_edit_invoke(
    accounts: FundEditAccounts<'_, '_>,
    args: FundEditIxArgs,
) -> ProgramResult {
    fund_edit_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn fund_edit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FundEditAccounts<'_, '_>,
    args: FundEditIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FundEditKeys = accounts.into();
    let ix = fund_edit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fund_edit_invoke_signed(
    accounts: FundEditAccounts<'_, '_>,
    args: FundEditIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fund_edit_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn fund_edit_verify_account_keys(
    accounts: FundEditAccounts<'_, '_>,
    keys: FundEditKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.fund_state.key, keys.fund_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fund_edit_verify_writable_privileges<'me, 'info>(
    accounts: FundEditAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.manager, accounts.fund_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fund_edit_verify_signer_privileges<'me, 'info>(
    accounts: FundEditAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fund_edit_verify_account_privileges<'me, 'info>(
    accounts: FundEditAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fund_edit_verify_writable_privileges(accounts)?;
    fund_edit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SimpleFundEditAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SimpleFundEditKeys {
    pub manager: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
}
impl From<SimpleFundEditAccounts<'_, '_>> for SimpleFundEditKeys {
    fn from(accounts: SimpleFundEditAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
        }
    }
}
impl From<SimpleFundEditKeys> for [AccountMeta; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SimpleFundEditKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN]> for SimpleFundEditKeys {
    fn from(pubkeys: [Pubkey; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
        }
    }
}
impl<'info> From<SimpleFundEditAccounts<'_, 'info>>
for [AccountInfo<'info>; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SimpleFundEditAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN]>
for SimpleFundEditAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
        }
    }
}
pub const SIMPLE_FUND_EDIT_IX_DISCM: [u8; 8usize] = [170, 87, 151, 213, 64, 41, 213, 83];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimpleFundEditIxArgs {
    pub manager_fee: u64,
    pub rebalance_interval: u64,
    pub rebalance_threshold: u64,
    pub rebalance_slippage: u64,
    pub lp_offset_threshold: u64,
    pub rebalance_and_lp: [u8; 2],
    pub num_of_tokens: u8,
    pub target_composition: [u8; 20],
    pub target_weights: [u64; 20],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleFundEditIxData(pub SimpleFundEditIxArgs);
impl From<SimpleFundEditIxArgs> for SimpleFundEditIxData {
    fn from(args: SimpleFundEditIxArgs) -> Self {
        Self(args)
    }
}
impl SimpleFundEditIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SIMPLE_FUND_EDIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let manager_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_interval: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_offset_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_and_lp: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let num_of_tokens: u8 = crate::borsh_de_or_default(&mut reader)?;
        let target_composition: [u8; 20] = crate::borsh_de_or_default(&mut reader)?;
        let target_weights: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SimpleFundEditIxArgs {
                manager_fee,
                rebalance_interval,
                rebalance_threshold,
                rebalance_slippage,
                lp_offset_threshold,
                rebalance_and_lp,
                num_of_tokens,
                target_composition,
                target_weights,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SIMPLE_FUND_EDIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.manager_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_interval, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_slippage, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lp_offset_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance_and_lp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.num_of_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.target_composition, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.target_weights, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn simple_fund_edit_ix_with_program_id(
    program_id: Pubkey,
    keys: SimpleFundEditKeys,
    args: SimpleFundEditIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SIMPLE_FUND_EDIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SimpleFundEditIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn simple_fund_edit_ix(
    keys: SimpleFundEditKeys,
    args: SimpleFundEditIxArgs,
) -> std::io::Result<Instruction> {
    simple_fund_edit_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn simple_fund_edit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SimpleFundEditAccounts<'_, '_>,
    args: SimpleFundEditIxArgs,
) -> ProgramResult {
    let keys: SimpleFundEditKeys = accounts.into();
    let ix = simple_fund_edit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn simple_fund_edit_invoke(
    accounts: SimpleFundEditAccounts<'_, '_>,
    args: SimpleFundEditIxArgs,
) -> ProgramResult {
    simple_fund_edit_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn simple_fund_edit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SimpleFundEditAccounts<'_, '_>,
    args: SimpleFundEditIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SimpleFundEditKeys = accounts.into();
    let ix = simple_fund_edit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn simple_fund_edit_invoke_signed(
    accounts: SimpleFundEditAccounts<'_, '_>,
    args: SimpleFundEditIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    simple_fund_edit_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn simple_fund_edit_verify_account_keys(
    accounts: SimpleFundEditAccounts<'_, '_>,
    keys: SimpleFundEditKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn simple_fund_edit_verify_writable_privileges<'me, 'info>(
    accounts: SimpleFundEditAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.manager, accounts.fund_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn simple_fund_edit_verify_signer_privileges<'me, 'info>(
    accounts: SimpleFundEditAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn simple_fund_edit_verify_account_privileges<'me, 'info>(
    accounts: SimpleFundEditAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    simple_fund_edit_verify_writable_privileges(accounts)?;
    simple_fund_edit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct LiquidityProvisionAccounts<'me, 'info> {
    pub buyer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_from_token_account: &'me AccountInfo<'info>,
    pub buyer_from_token_account: &'me AccountInfo<'info>,
    pub pda_to_token_account: &'me AccountInfo<'info>,
    pub buyer_to_token_account: &'me AccountInfo<'info>,
    pub swap_fee_account: &'me AccountInfo<'info>,
    pub host_fee_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub curve_data: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidityProvisionKeys {
    pub buyer: Pubkey,
    pub fund_state: Pubkey,
    pub pda_account: Pubkey,
    pub pda_from_token_account: Pubkey,
    pub buyer_from_token_account: Pubkey,
    pub pda_to_token_account: Pubkey,
    pub buyer_to_token_account: Pubkey,
    pub swap_fee_account: Pubkey,
    pub host_fee_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub token_list: Pubkey,
    pub curve_data: Pubkey,
    pub token_program: Pubkey,
}
impl From<LiquidityProvisionAccounts<'_, '_>> for LiquidityProvisionKeys {
    fn from(accounts: LiquidityProvisionAccounts) -> Self {
        Self {
            buyer: *accounts.buyer.key,
            fund_state: *accounts.fund_state.key,
            pda_account: *accounts.pda_account.key,
            pda_from_token_account: *accounts.pda_from_token_account.key,
            buyer_from_token_account: *accounts.buyer_from_token_account.key,
            pda_to_token_account: *accounts.pda_to_token_account.key,
            buyer_to_token_account: *accounts.buyer_to_token_account.key,
            swap_fee_account: *accounts.swap_fee_account.key,
            host_fee_account: *accounts.host_fee_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            token_list: *accounts.token_list.key,
            curve_data: *accounts.curve_data.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<LiquidityProvisionKeys>
for [AccountMeta; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidityProvisionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_from_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_from_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.curve_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN]> for LiquidityProvisionKeys {
    fn from(pubkeys: [Pubkey; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: pubkeys[0],
            fund_state: pubkeys[1],
            pda_account: pubkeys[2],
            pda_from_token_account: pubkeys[3],
            buyer_from_token_account: pubkeys[4],
            pda_to_token_account: pubkeys[5],
            buyer_to_token_account: pubkeys[6],
            swap_fee_account: pubkeys[7],
            host_fee_account: pubkeys[8],
            manager_fee_account: pubkeys[9],
            token_list: pubkeys[10],
            curve_data: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<LiquidityProvisionAccounts<'_, 'info>>
for [AccountInfo<'info>; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN] {
    fn from(accounts: LiquidityProvisionAccounts<'_, 'info>) -> Self {
        [
            accounts.buyer.clone(),
            accounts.fund_state.clone(),
            accounts.pda_account.clone(),
            accounts.pda_from_token_account.clone(),
            accounts.buyer_from_token_account.clone(),
            accounts.pda_to_token_account.clone(),
            accounts.buyer_to_token_account.clone(),
            accounts.swap_fee_account.clone(),
            accounts.host_fee_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.token_list.clone(),
            accounts.curve_data.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN]>
for LiquidityProvisionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            buyer: &arr[0],
            fund_state: &arr[1],
            pda_account: &arr[2],
            pda_from_token_account: &arr[3],
            buyer_from_token_account: &arr[4],
            pda_to_token_account: &arr[5],
            buyer_to_token_account: &arr[6],
            swap_fee_account: &arr[7],
            host_fee_account: &arr[8],
            manager_fee_account: &arr[9],
            token_list: &arr[10],
            curve_data: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const LIQUIDITY_PROVISION_IX_DISCM: [u8; 8usize] = [
    130, 113, 21, 240, 202, 190, 11, 3,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityProvisionIxArgs {
    pub from_token_id: u64,
    pub to_token_id: u64,
    pub from_amount: u64,
    pub minimum_to_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityProvisionIxData(pub LiquidityProvisionIxArgs);
impl From<LiquidityProvisionIxArgs> for LiquidityProvisionIxData {
    fn from(args: LiquidityProvisionIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidityProvisionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_PROVISION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let from_token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let to_token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let from_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_to_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(LiquidityProvisionIxArgs {
                from_token_id,
                to_token_id,
                from_amount,
                minimum_to_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_PROVISION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.from_token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.to_token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.from_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_to_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidity_provision_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidityProvisionKeys,
    args: LiquidityProvisionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDITY_PROVISION_IX_ACCOUNTS_LEN] = keys.into();
    let data: LiquidityProvisionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidity_provision_ix(
    keys: LiquidityProvisionKeys,
    args: LiquidityProvisionIxArgs,
) -> std::io::Result<Instruction> {
    liquidity_provision_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn liquidity_provision_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidityProvisionAccounts<'_, '_>,
    args: LiquidityProvisionIxArgs,
) -> ProgramResult {
    let keys: LiquidityProvisionKeys = accounts.into();
    let ix = liquidity_provision_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidity_provision_invoke(
    accounts: LiquidityProvisionAccounts<'_, '_>,
    args: LiquidityProvisionIxArgs,
) -> ProgramResult {
    liquidity_provision_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn liquidity_provision_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidityProvisionAccounts<'_, '_>,
    args: LiquidityProvisionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidityProvisionKeys = accounts.into();
    let ix = liquidity_provision_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidity_provision_invoke_signed(
    accounts: LiquidityProvisionAccounts<'_, '_>,
    args: LiquidityProvisionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidity_provision_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidity_provision_verify_account_keys(
    accounts: LiquidityProvisionAccounts<'_, '_>,
    keys: LiquidityProvisionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.buyer.key, keys.buyer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_from_token_account.key, keys.pda_from_token_account),
        (*accounts.buyer_from_token_account.key, keys.buyer_from_token_account),
        (*accounts.pda_to_token_account.key, keys.pda_to_token_account),
        (*accounts.buyer_to_token_account.key, keys.buyer_to_token_account),
        (*accounts.swap_fee_account.key, keys.swap_fee_account),
        (*accounts.host_fee_account.key, keys.host_fee_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.curve_data.key, keys.curve_data),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidity_provision_verify_writable_privileges<'me, 'info>(
    accounts: LiquidityProvisionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fund_state,
        accounts.pda_from_token_account,
        accounts.buyer_from_token_account,
        accounts.pda_to_token_account,
        accounts.buyer_to_token_account,
        accounts.swap_fee_account,
        accounts.host_fee_account,
        accounts.manager_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidity_provision_verify_signer_privileges<'me, 'info>(
    accounts: LiquidityProvisionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.buyer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidity_provision_verify_account_privileges<'me, 'info>(
    accounts: LiquidityProvisionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidity_provision_verify_writable_privileges(accounts)?;
    liquidity_provision_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_FUND_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct BuyFundAccounts<'me, 'info> {
    pub buyer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub buyer_usdc_account: &'me AccountInfo<'info>,
    pub manager_usdc_account: &'me AccountInfo<'info>,
    pub smf_fee_account: &'me AccountInfo<'info>,
    pub host_usdc_account: &'me AccountInfo<'info>,
    pub buyer_fund_token_account: &'me AccountInfo<'info>,
    pub buy_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyFundKeys {
    pub buyer: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub pda_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub buyer_usdc_account: Pubkey,
    pub manager_usdc_account: Pubkey,
    pub smf_fee_account: Pubkey,
    pub host_usdc_account: Pubkey,
    pub buyer_fund_token_account: Pubkey,
    pub buy_state: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<BuyFundAccounts<'_, '_>> for BuyFundKeys {
    fn from(accounts: BuyFundAccounts) -> Self {
        Self {
            buyer: *accounts.buyer.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            pda_account: *accounts.pda_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            buyer_usdc_account: *accounts.buyer_usdc_account.key,
            manager_usdc_account: *accounts.manager_usdc_account.key,
            smf_fee_account: *accounts.smf_fee_account.key,
            host_usdc_account: *accounts.host_usdc_account.key,
            buyer_fund_token_account: *accounts.buyer_fund_token_account.key,
            buy_state: *accounts.buy_state.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<BuyFundKeys> for [AccountMeta; BUY_FUND_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyFundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smf_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_fund_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buy_state,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_FUND_IX_ACCOUNTS_LEN]> for BuyFundKeys {
    fn from(pubkeys: [Pubkey; BUY_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
            pda_account: pubkeys[3],
            pda_usdc_account: pubkeys[4],
            buyer_usdc_account: pubkeys[5],
            manager_usdc_account: pubkeys[6],
            smf_fee_account: pubkeys[7],
            host_usdc_account: pubkeys[8],
            buyer_fund_token_account: pubkeys[9],
            buy_state: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<BuyFundAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_FUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyFundAccounts<'_, 'info>) -> Self {
        [
            accounts.buyer.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.pda_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.buyer_usdc_account.clone(),
            accounts.manager_usdc_account.clone(),
            accounts.smf_fee_account.clone(),
            accounts.host_usdc_account.clone(),
            accounts.buyer_fund_token_account.clone(),
            accounts.buy_state.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_FUND_IX_ACCOUNTS_LEN]>
for BuyFundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
            pda_account: &arr[3],
            pda_usdc_account: &arr[4],
            buyer_usdc_account: &arr[5],
            manager_usdc_account: &arr[6],
            smf_fee_account: &arr[7],
            host_usdc_account: &arr[8],
            buyer_fund_token_account: &arr[9],
            buy_state: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const BUY_FUND_IX_DISCM: [u8; 8usize] = [251, 50, 158, 62, 174, 248, 165, 197];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyFundIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyFundIxData(pub BuyFundIxArgs);
impl From<BuyFundIxArgs> for BuyFundIxData {
    fn from(args: BuyFundIxArgs) -> Self {
        Self(args)
    }
}
impl BuyFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_FUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(BuyFundIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_FUND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_fund_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyFundKeys,
    args: BuyFundIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_FUND_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyFundIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_fund_ix(
    keys: BuyFundKeys,
    args: BuyFundIxArgs,
) -> std::io::Result<Instruction> {
    buy_fund_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn buy_fund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyFundAccounts<'_, '_>,
    args: BuyFundIxArgs,
) -> ProgramResult {
    let keys: BuyFundKeys = accounts.into();
    let ix = buy_fund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_fund_invoke(
    accounts: BuyFundAccounts<'_, '_>,
    args: BuyFundIxArgs,
) -> ProgramResult {
    buy_fund_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn buy_fund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyFundAccounts<'_, '_>,
    args: BuyFundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyFundKeys = accounts.into();
    let ix = buy_fund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_fund_invoke_signed(
    accounts: BuyFundAccounts<'_, '_>,
    args: BuyFundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_fund_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_fund_verify_account_keys(
    accounts: BuyFundAccounts<'_, '_>,
    keys: BuyFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.buyer.key, keys.buyer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.buyer_usdc_account.key, keys.buyer_usdc_account),
        (*accounts.manager_usdc_account.key, keys.manager_usdc_account),
        (*accounts.smf_fee_account.key, keys.smf_fee_account),
        (*accounts.host_usdc_account.key, keys.host_usdc_account),
        (*accounts.buyer_fund_token_account.key, keys.buyer_fund_token_account),
        (*accounts.buy_state.key, keys.buy_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_fund_verify_writable_privileges<'me, 'info>(
    accounts: BuyFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.buyer,
        accounts.fund_state,
        accounts.pda_usdc_account,
        accounts.buyer_usdc_account,
        accounts.manager_usdc_account,
        accounts.smf_fee_account,
        accounts.host_usdc_account,
        accounts.buyer_fund_token_account,
        accounts.buy_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_fund_verify_signer_privileges<'me, 'info>(
    accounts: BuyFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.buyer, accounts.buy_state] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_fund_verify_account_privileges<'me, 'info>(
    accounts: BuyFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_fund_verify_writable_privileges(accounts)?;
    buy_fund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_FUND_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct MintFundAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub buyer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub buy_state: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub buyer_fund_token_account: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MintFundKeys {
    pub signer: Pubkey,
    pub buyer: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub buy_state: Pubkey,
    pub pda_account: Pubkey,
    pub buyer_fund_token_account: Pubkey,
    pub fund_token: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<MintFundAccounts<'_, '_>> for MintFundKeys {
    fn from(accounts: MintFundAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            buyer: *accounts.buyer.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            buy_state: *accounts.buy_state.key,
            pda_account: *accounts.pda_account.key,
            buyer_fund_token_account: *accounts.buyer_fund_token_account.key,
            fund_token: *accounts.fund_token.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<MintFundKeys> for [AccountMeta; MINT_FUND_IX_ACCOUNTS_LEN] {
    fn from(keys: MintFundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buy_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buyer_fund_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_FUND_IX_ACCOUNTS_LEN]> for MintFundKeys {
    fn from(pubkeys: [Pubkey; MINT_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            buyer: pubkeys[1],
            fund_state: pubkeys[2],
            token_list: pubkeys[3],
            buy_state: pubkeys[4],
            pda_account: pubkeys[5],
            buyer_fund_token_account: pubkeys[6],
            fund_token: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<MintFundAccounts<'_, 'info>>
for [AccountInfo<'info>; MINT_FUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintFundAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.buyer.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.buy_state.clone(),
            accounts.pda_account.clone(),
            accounts.buyer_fund_token_account.clone(),
            accounts.fund_token.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_FUND_IX_ACCOUNTS_LEN]>
for MintFundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MINT_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            buyer: &arr[1],
            fund_state: &arr[2],
            token_list: &arr[3],
            buy_state: &arr[4],
            pda_account: &arr[5],
            buyer_fund_token_account: &arr[6],
            fund_token: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const MINT_FUND_IX_DISCM: [u8; 8usize] = [239, 150, 109, 73, 55, 160, 38, 162];
#[derive(Clone, Debug, PartialEq)]
pub struct MintFundIxData;
impl MintFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_FUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_FUND_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_fund_ix_with_program_id(
    program_id: Pubkey,
    keys: MintFundKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_FUND_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MintFundIxData.try_to_vec()?,
    })
}
pub fn mint_fund_ix(keys: MintFundKeys) -> std::io::Result<Instruction> {
    mint_fund_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn mint_fund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MintFundAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MintFundKeys = accounts.into();
    let ix = mint_fund_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn mint_fund_invoke(accounts: MintFundAccounts<'_, '_>) -> ProgramResult {
    mint_fund_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn mint_fund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MintFundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintFundKeys = accounts.into();
    let ix = mint_fund_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mint_fund_invoke_signed(
    accounts: MintFundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mint_fund_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn mint_fund_verify_account_keys(
    accounts: MintFundAccounts<'_, '_>,
    keys: MintFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.buyer.key, keys.buyer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.buy_state.key, keys.buy_state),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.buyer_fund_token_account.key, keys.buyer_fund_token_account),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mint_fund_verify_writable_privileges<'me, 'info>(
    accounts: MintFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.buyer,
        accounts.fund_state,
        accounts.buy_state,
        accounts.buyer_fund_token_account,
        accounts.fund_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_fund_verify_signer_privileges<'me, 'info>(
    accounts: MintFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_fund_verify_account_privileges<'me, 'info>(
    accounts: MintFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_fund_verify_writable_privileges(accounts)?;
    mint_fund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_MINT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InstantMintAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub buyer_fund_token_account: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantMintKeys {
    pub authority: Pubkey,
    pub buyer_fund_token_account: Pubkey,
    pub fund_token: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub pda_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<InstantMintAccounts<'_, '_>> for InstantMintKeys {
    fn from(accounts: InstantMintAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            buyer_fund_token_account: *accounts.buyer_fund_token_account.key,
            fund_token: *accounts.fund_token.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            pda_account: *accounts.pda_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InstantMintKeys> for [AccountMeta; INSTANT_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_fund_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_MINT_IX_ACCOUNTS_LEN]> for InstantMintKeys {
    fn from(pubkeys: [Pubkey; INSTANT_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            buyer_fund_token_account: pubkeys[1],
            fund_token: pubkeys[2],
            fund_state: pubkeys[3],
            token_list: pubkeys[4],
            pda_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<InstantMintAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantMintAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.buyer_fund_token_account.clone(),
            accounts.fund_token.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.pda_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_MINT_IX_ACCOUNTS_LEN]>
for InstantMintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INSTANT_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            buyer_fund_token_account: &arr[1],
            fund_token: &arr[2],
            fund_state: &arr[3],
            token_list: &arr[4],
            pda_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const INSTANT_MINT_IX_DISCM: [u8; 8usize] = [70, 132, 88, 215, 74, 8, 209, 236];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantMintIxArgs {
    pub amounts: [u64; 20],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantMintIxData(pub InstantMintIxArgs);
impl From<InstantMintIxArgs> for InstantMintIxData {
    fn from(args: InstantMintIxArgs) -> Self {
        Self(args)
    }
}
impl InstantMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amounts: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InstantMintIxArgs { amounts }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amounts, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantMintKeys,
    args: InstantMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_mint_ix(
    keys: InstantMintKeys,
    args: InstantMintIxArgs,
) -> std::io::Result<Instruction> {
    instant_mint_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn instant_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantMintAccounts<'_, '_>,
    args: InstantMintIxArgs,
) -> ProgramResult {
    let keys: InstantMintKeys = accounts.into();
    let ix = instant_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_mint_invoke(
    accounts: InstantMintAccounts<'_, '_>,
    args: InstantMintIxArgs,
) -> ProgramResult {
    instant_mint_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn instant_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantMintAccounts<'_, '_>,
    args: InstantMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantMintKeys = accounts.into();
    let ix = instant_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_mint_invoke_signed(
    accounts: InstantMintAccounts<'_, '_>,
    args: InstantMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_mint_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_mint_verify_account_keys(
    accounts: InstantMintAccounts<'_, '_>,
    keys: InstantMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.buyer_fund_token_account.key, keys.buyer_fund_token_account),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_mint_verify_writable_privileges<'me, 'info>(
    accounts: InstantMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.buyer_fund_token_account,
        accounts.fund_token,
        accounts.fund_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_mint_verify_signer_privileges<'me, 'info>(
    accounts: InstantMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_mint_verify_account_privileges<'me, 'info>(
    accounts: InstantMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_mint_verify_writable_privileges(accounts)?;
    instant_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SingleTokenDepositAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub buyer_token_account: &'me AccountInfo<'info>,
    pub buyer_fund_token_account: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub symmetry_fee_account: &'me AccountInfo<'info>,
    pub host_fee_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub oracle_account: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SingleTokenDepositKeys {
    pub authority: Pubkey,
    pub fund_state: Pubkey,
    pub fund_token: Pubkey,
    pub buyer_token_account: Pubkey,
    pub buyer_fund_token_account: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub symmetry_fee_account: Pubkey,
    pub host_fee_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub oracle_account: Pubkey,
    pub token_list: Pubkey,
    pub token_program: Pubkey,
}
impl From<SingleTokenDepositAccounts<'_, '_>> for SingleTokenDepositKeys {
    fn from(accounts: SingleTokenDepositAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            fund_state: *accounts.fund_state.key,
            fund_token: *accounts.fund_token.key,
            buyer_token_account: *accounts.buyer_token_account.key,
            buyer_fund_token_account: *accounts.buyer_fund_token_account.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            symmetry_fee_account: *accounts.symmetry_fee_account.key,
            host_fee_account: *accounts.host_fee_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            oracle_account: *accounts.oracle_account.key,
            token_list: *accounts.token_list.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SingleTokenDepositKeys>
for [AccountMeta; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SingleTokenDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_fund_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.symmetry_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN]> for SingleTokenDepositKeys {
    fn from(pubkeys: [Pubkey; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            fund_state: pubkeys[1],
            fund_token: pubkeys[2],
            buyer_token_account: pubkeys[3],
            buyer_fund_token_account: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            symmetry_fee_account: pubkeys[7],
            host_fee_account: pubkeys[8],
            manager_fee_account: pubkeys[9],
            oracle_account: pubkeys[10],
            token_list: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<SingleTokenDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SingleTokenDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.fund_state.clone(),
            accounts.fund_token.clone(),
            accounts.buyer_token_account.clone(),
            accounts.buyer_fund_token_account.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.symmetry_fee_account.clone(),
            accounts.host_fee_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.oracle_account.clone(),
            accounts.token_list.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN]>
for SingleTokenDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            fund_state: &arr[1],
            fund_token: &arr[2],
            buyer_token_account: &arr[3],
            buyer_fund_token_account: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            symmetry_fee_account: &arr[7],
            host_fee_account: &arr[8],
            manager_fee_account: &arr[9],
            oracle_account: &arr[10],
            token_list: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const SINGLE_TOKEN_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    197, 12, 239, 83, 28, 4, 37, 97,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SingleTokenDepositIxArgs {
    pub token: u8,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SingleTokenDepositIxData(pub SingleTokenDepositIxArgs);
impl From<SingleTokenDepositIxArgs> for SingleTokenDepositIxData {
    fn from(args: SingleTokenDepositIxArgs) -> Self {
        Self(args)
    }
}
impl SingleTokenDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SINGLE_TOKEN_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SingleTokenDepositIxArgs {
                token,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SINGLE_TOKEN_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn single_token_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: SingleTokenDepositKeys,
    args: SingleTokenDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SINGLE_TOKEN_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SingleTokenDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn single_token_deposit_ix(
    keys: SingleTokenDepositKeys,
    args: SingleTokenDepositIxArgs,
) -> std::io::Result<Instruction> {
    single_token_deposit_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn single_token_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SingleTokenDepositAccounts<'_, '_>,
    args: SingleTokenDepositIxArgs,
) -> ProgramResult {
    let keys: SingleTokenDepositKeys = accounts.into();
    let ix = single_token_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn single_token_deposit_invoke(
    accounts: SingleTokenDepositAccounts<'_, '_>,
    args: SingleTokenDepositIxArgs,
) -> ProgramResult {
    single_token_deposit_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn single_token_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SingleTokenDepositAccounts<'_, '_>,
    args: SingleTokenDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SingleTokenDepositKeys = accounts.into();
    let ix = single_token_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn single_token_deposit_invoke_signed(
    accounts: SingleTokenDepositAccounts<'_, '_>,
    args: SingleTokenDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    single_token_deposit_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn single_token_deposit_verify_account_keys(
    accounts: SingleTokenDepositAccounts<'_, '_>,
    keys: SingleTokenDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.buyer_token_account.key, keys.buyer_token_account),
        (*accounts.buyer_fund_token_account.key, keys.buyer_fund_token_account),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.symmetry_fee_account.key, keys.symmetry_fee_account),
        (*accounts.host_fee_account.key, keys.host_fee_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.oracle_account.key, keys.oracle_account),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn single_token_deposit_verify_writable_privileges<'me, 'info>(
    accounts: SingleTokenDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.fund_state,
        accounts.fund_token,
        accounts.buyer_token_account,
        accounts.buyer_fund_token_account,
        accounts.pda_token_account,
        accounts.symmetry_fee_account,
        accounts.host_fee_account,
        accounts.manager_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn single_token_deposit_verify_signer_privileges<'me, 'info>(
    accounts: SingleTokenDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn single_token_deposit_verify_account_privileges<'me, 'info>(
    accounts: SingleTokenDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    single_token_deposit_verify_writable_privileges(accounts)?;
    single_token_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_FUND_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SellFundAccounts<'me, 'info> {
    pub seller: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub new_fund_state: &'me AccountInfo<'info>,
    pub seller_fund_token_account: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellFundKeys {
    pub seller: Pubkey,
    pub fund_state: Pubkey,
    pub pda_account: Pubkey,
    pub new_fund_state: Pubkey,
    pub seller_fund_token_account: Pubkey,
    pub fund_token: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<SellFundAccounts<'_, '_>> for SellFundKeys {
    fn from(accounts: SellFundAccounts) -> Self {
        Self {
            seller: *accounts.seller.key,
            fund_state: *accounts.fund_state.key,
            pda_account: *accounts.pda_account.key,
            new_fund_state: *accounts.new_fund_state.key,
            seller_fund_token_account: *accounts.seller_fund_token_account.key,
            fund_token: *accounts.fund_token.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<SellFundKeys> for [AccountMeta; SELL_FUND_IX_ACCOUNTS_LEN] {
    fn from(keys: SellFundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.seller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seller_fund_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SELL_FUND_IX_ACCOUNTS_LEN]> for SellFundKeys {
    fn from(pubkeys: [Pubkey; SELL_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seller: pubkeys[0],
            fund_state: pubkeys[1],
            pda_account: pubkeys[2],
            new_fund_state: pubkeys[3],
            seller_fund_token_account: pubkeys[4],
            fund_token: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<SellFundAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_FUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellFundAccounts<'_, 'info>) -> Self {
        [
            accounts.seller.clone(),
            accounts.fund_state.clone(),
            accounts.pda_account.clone(),
            accounts.new_fund_state.clone(),
            accounts.seller_fund_token_account.clone(),
            accounts.fund_token.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_FUND_IX_ACCOUNTS_LEN]>
for SellFundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seller: &arr[0],
            fund_state: &arr[1],
            pda_account: &arr[2],
            new_fund_state: &arr[3],
            seller_fund_token_account: &arr[4],
            fund_token: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            rent: &arr[8],
        }
    }
}
pub const SELL_FUND_IX_DISCM: [u8; 8usize] = [78, 253, 22, 133, 38, 176, 110, 5];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellFundIxArgs {
    pub amount: u64,
    pub rebalance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellFundIxData(pub SellFundIxArgs);
impl From<SellFundIxArgs> for SellFundIxData {
    fn from(args: SellFundIxArgs) -> Self {
        Self(args)
    }
}
impl SellFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_FUND_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rebalance: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellFundIxArgs {
                amount,
                rebalance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_FUND_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.rebalance, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_fund_ix_with_program_id(
    program_id: Pubkey,
    keys: SellFundKeys,
    args: SellFundIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_FUND_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellFundIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_fund_ix(
    keys: SellFundKeys,
    args: SellFundIxArgs,
) -> std::io::Result<Instruction> {
    sell_fund_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn sell_fund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellFundAccounts<'_, '_>,
    args: SellFundIxArgs,
) -> ProgramResult {
    let keys: SellFundKeys = accounts.into();
    let ix = sell_fund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_fund_invoke(
    accounts: SellFundAccounts<'_, '_>,
    args: SellFundIxArgs,
) -> ProgramResult {
    sell_fund_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn sell_fund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellFundAccounts<'_, '_>,
    args: SellFundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellFundKeys = accounts.into();
    let ix = sell_fund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_fund_invoke_signed(
    accounts: SellFundAccounts<'_, '_>,
    args: SellFundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_fund_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_fund_verify_account_keys(
    accounts: SellFundAccounts<'_, '_>,
    keys: SellFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.seller.key, keys.seller),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.new_fund_state.key, keys.new_fund_state),
        (*accounts.seller_fund_token_account.key, keys.seller_fund_token_account),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_fund_verify_writable_privileges<'me, 'info>(
    accounts: SellFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.seller,
        accounts.fund_state,
        accounts.new_fund_state,
        accounts.seller_fund_token_account,
        accounts.fund_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_fund_verify_signer_privileges<'me, 'info>(
    accounts: SellFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.seller] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_fund_verify_account_privileges<'me, 'info>(
    accounts: SellFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_fund_verify_writable_privileges(accounts)?;
    sell_fund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSTANT_BURN_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InstantBurnAccounts<'me, 'info> {
    pub seller: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub seller_fund_token_account: &'me AccountInfo<'info>,
    pub withdraw_token_mint: &'me AccountInfo<'info>,
    pub seller_token_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstantBurnKeys {
    pub seller: Pubkey,
    pub pda_account: Pubkey,
    pub fund_state: Pubkey,
    pub fund_token: Pubkey,
    pub seller_fund_token_account: Pubkey,
    pub withdraw_token_mint: Pubkey,
    pub seller_token_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub token_list: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<InstantBurnAccounts<'_, '_>> for InstantBurnKeys {
    fn from(accounts: InstantBurnAccounts) -> Self {
        Self {
            seller: *accounts.seller.key,
            pda_account: *accounts.pda_account.key,
            fund_state: *accounts.fund_state.key,
            fund_token: *accounts.fund_token.key,
            seller_fund_token_account: *accounts.seller_fund_token_account.key,
            withdraw_token_mint: *accounts.withdraw_token_mint.key,
            seller_token_account: *accounts.seller_token_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            token_list: *accounts.token_list.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InstantBurnKeys> for [AccountMeta; INSTANT_BURN_IX_ACCOUNTS_LEN] {
    fn from(keys: InstantBurnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.seller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seller_fund_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seller_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INSTANT_BURN_IX_ACCOUNTS_LEN]> for InstantBurnKeys {
    fn from(pubkeys: [Pubkey; INSTANT_BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seller: pubkeys[0],
            pda_account: pubkeys[1],
            fund_state: pubkeys[2],
            fund_token: pubkeys[3],
            seller_fund_token_account: pubkeys[4],
            withdraw_token_mint: pubkeys[5],
            seller_token_account: pubkeys[6],
            pda_token_account: pubkeys[7],
            token_list: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<InstantBurnAccounts<'_, 'info>>
for [AccountInfo<'info>; INSTANT_BURN_IX_ACCOUNTS_LEN] {
    fn from(accounts: InstantBurnAccounts<'_, 'info>) -> Self {
        [
            accounts.seller.clone(),
            accounts.pda_account.clone(),
            accounts.fund_state.clone(),
            accounts.fund_token.clone(),
            accounts.seller_fund_token_account.clone(),
            accounts.withdraw_token_mint.clone(),
            accounts.seller_token_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.token_list.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSTANT_BURN_IX_ACCOUNTS_LEN]>
for InstantBurnAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INSTANT_BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seller: &arr[0],
            pda_account: &arr[1],
            fund_state: &arr[2],
            fund_token: &arr[3],
            seller_fund_token_account: &arr[4],
            withdraw_token_mint: &arr[5],
            seller_token_account: &arr[6],
            pda_token_account: &arr[7],
            token_list: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const INSTANT_BURN_IX_DISCM: [u8; 8usize] = [34, 231, 151, 248, 32, 1, 32, 125];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstantBurnIxArgs {
    pub burn_amount: u64,
    pub withdraw_token: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstantBurnIxData(pub InstantBurnIxArgs);
impl From<InstantBurnIxArgs> for InstantBurnIxData {
    fn from(args: InstantBurnIxArgs) -> Self {
        Self(args)
    }
}
impl InstantBurnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSTANT_BURN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let burn_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_token: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InstantBurnIxArgs {
                burn_amount,
                withdraw_token,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSTANT_BURN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.burn_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_token, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn instant_burn_ix_with_program_id(
    program_id: Pubkey,
    keys: InstantBurnKeys,
    args: InstantBurnIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSTANT_BURN_IX_ACCOUNTS_LEN] = keys.into();
    let data: InstantBurnIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn instant_burn_ix(
    keys: InstantBurnKeys,
    args: InstantBurnIxArgs,
) -> std::io::Result<Instruction> {
    instant_burn_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn instant_burn_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InstantBurnAccounts<'_, '_>,
    args: InstantBurnIxArgs,
) -> ProgramResult {
    let keys: InstantBurnKeys = accounts.into();
    let ix = instant_burn_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn instant_burn_invoke(
    accounts: InstantBurnAccounts<'_, '_>,
    args: InstantBurnIxArgs,
) -> ProgramResult {
    instant_burn_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn instant_burn_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InstantBurnAccounts<'_, '_>,
    args: InstantBurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InstantBurnKeys = accounts.into();
    let ix = instant_burn_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn instant_burn_invoke_signed(
    accounts: InstantBurnAccounts<'_, '_>,
    args: InstantBurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    instant_burn_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn instant_burn_verify_account_keys(
    accounts: InstantBurnAccounts<'_, '_>,
    keys: InstantBurnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.seller.key, keys.seller),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.seller_fund_token_account.key, keys.seller_fund_token_account),
        (*accounts.withdraw_token_mint.key, keys.withdraw_token_mint),
        (*accounts.seller_token_account.key, keys.seller_token_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn instant_burn_verify_writable_privileges<'me, 'info>(
    accounts: InstantBurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.seller,
        accounts.fund_state,
        accounts.fund_token,
        accounts.seller_fund_token_account,
        accounts.withdraw_token_mint,
        accounts.seller_token_account,
        accounts.pda_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn instant_burn_verify_signer_privileges<'me, 'info>(
    accounts: InstantBurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.seller] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn instant_burn_verify_account_privileges<'me, 'info>(
    accounts: InstantBurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    instant_burn_verify_writable_privileges(accounts)?;
    instant_burn_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_TOKEN_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub seller_token_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenKeys {
    pub manager: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub seller_token_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimTokenAccounts<'_, '_>> for ClaimTokenKeys {
    fn from(accounts: ClaimTokenAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            seller_token_account: *accounts.seller_token_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_account: *accounts.pda_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimTokenKeys> for [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seller_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]> for ClaimTokenKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
            seller_token_account: pubkeys[3],
            pda_token_account: pubkeys[4],
            pda_account: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<ClaimTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.seller_token_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]>
for ClaimTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
            seller_token_account: &arr[3],
            pda_token_account: &arr[4],
            pda_account: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const CLAIM_TOKEN_IX_DISCM: [u8; 8usize] = [116, 206, 27, 191, 166, 19, 0, 73];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimTokenIxArgs {
    pub token_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIxData(pub ClaimTokenIxArgs);
impl From<ClaimTokenIxArgs> for ClaimTokenIxData {
    fn from(args: ClaimTokenIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ClaimTokenIxArgs { token_id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenKeys,
    args: ClaimTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_token_ix(
    keys: ClaimTokenKeys,
    args: ClaimTokenIxArgs,
) -> std::io::Result<Instruction> {
    claim_token_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn claim_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_token_invoke(
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
) -> ProgramResult {
    claim_token_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn claim_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_token_invoke_signed(
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn claim_token_verify_account_keys(
    accounts: ClaimTokenAccounts<'_, '_>,
    keys: ClaimTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.seller_token_account.key, keys.seller_token_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_token_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.fund_state,
        accounts.seller_token_account,
        accounts.pda_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_token_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_token_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_verify_writable_privileges(accounts)?;
    claim_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenFromBuyStateAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub buyer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub buy_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub buyer_token_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenFromBuyStateKeys {
    pub signer: Pubkey,
    pub buyer: Pubkey,
    pub fund_state: Pubkey,
    pub buy_state: Pubkey,
    pub token_list: Pubkey,
    pub buyer_token_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimTokenFromBuyStateAccounts<'_, '_>> for ClaimTokenFromBuyStateKeys {
    fn from(accounts: ClaimTokenFromBuyStateAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            buyer: *accounts.buyer.key,
            fund_state: *accounts.fund_state.key,
            buy_state: *accounts.buy_state.key,
            token_list: *accounts.token_list.key,
            buyer_token_account: *accounts.buyer_token_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_account: *accounts.pda_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimTokenFromBuyStateKeys>
for [AccountMeta; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenFromBuyStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buy_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buyer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN]>
for ClaimTokenFromBuyStateKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            buyer: pubkeys[1],
            fund_state: pubkeys[2],
            buy_state: pubkeys[3],
            token_list: pubkeys[4],
            buyer_token_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_account: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimTokenFromBuyStateAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTokenFromBuyStateAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.buyer.clone(),
            accounts.fund_state.clone(),
            accounts.buy_state.clone(),
            accounts.token_list.clone(),
            accounts.buyer_token_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN]>
for ClaimTokenFromBuyStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            buyer: &arr[1],
            fund_state: &arr[2],
            buy_state: &arr[3],
            token_list: &arr[4],
            buyer_token_account: &arr[5],
            pda_token_account: &arr[6],
            pda_account: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const CLAIM_TOKEN_FROM_BUY_STATE_IX_DISCM: [u8; 8usize] = [
    132, 137, 239, 21, 204, 222, 213, 220,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClaimTokenFromBuyStateIxArgs {
    pub token_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenFromBuyStateIxData(pub ClaimTokenFromBuyStateIxArgs);
impl From<ClaimTokenFromBuyStateIxArgs> for ClaimTokenFromBuyStateIxData {
    fn from(args: ClaimTokenFromBuyStateIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimTokenFromBuyStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_FROM_BUY_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ClaimTokenFromBuyStateIxArgs {
                token_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_FROM_BUY_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_token_from_buy_state_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenFromBuyStateKeys,
    args: ClaimTokenFromBuyStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_FROM_BUY_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimTokenFromBuyStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_token_from_buy_state_ix(
    keys: ClaimTokenFromBuyStateKeys,
    args: ClaimTokenFromBuyStateIxArgs,
) -> std::io::Result<Instruction> {
    claim_token_from_buy_state_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn claim_token_from_buy_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenFromBuyStateAccounts<'_, '_>,
    args: ClaimTokenFromBuyStateIxArgs,
) -> ProgramResult {
    let keys: ClaimTokenFromBuyStateKeys = accounts.into();
    let ix = claim_token_from_buy_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_token_from_buy_state_invoke(
    accounts: ClaimTokenFromBuyStateAccounts<'_, '_>,
    args: ClaimTokenFromBuyStateIxArgs,
) -> ProgramResult {
    claim_token_from_buy_state_invoke_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn claim_token_from_buy_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenFromBuyStateAccounts<'_, '_>,
    args: ClaimTokenFromBuyStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenFromBuyStateKeys = accounts.into();
    let ix = claim_token_from_buy_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_token_from_buy_state_invoke_signed(
    accounts: ClaimTokenFromBuyStateAccounts<'_, '_>,
    args: ClaimTokenFromBuyStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_from_buy_state_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn claim_token_from_buy_state_verify_account_keys(
    accounts: ClaimTokenFromBuyStateAccounts<'_, '_>,
    keys: ClaimTokenFromBuyStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.buyer.key, keys.buyer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.buy_state.key, keys.buy_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.buyer_token_account.key, keys.buyer_token_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_token_from_buy_state_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenFromBuyStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.buyer,
        accounts.fund_state,
        accounts.buy_state,
        accounts.buyer_token_account,
        accounts.pda_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_token_from_buy_state_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenFromBuyStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_token_from_buy_state_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenFromBuyStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_from_buy_state_verify_writable_privileges(accounts)?;
    claim_token_from_buy_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCurrentWeightsAccounts<'me, 'info> {
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCurrentWeightsKeys {
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
}
impl From<UpdateCurrentWeightsAccounts<'_, '_>> for UpdateCurrentWeightsKeys {
    fn from(accounts: UpdateCurrentWeightsAccounts) -> Self {
        Self {
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
        }
    }
}
impl From<UpdateCurrentWeightsKeys>
for [AccountMeta; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCurrentWeightsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN]>
for UpdateCurrentWeightsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fund_state: pubkeys[0],
            token_list: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateCurrentWeightsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCurrentWeightsAccounts<'_, 'info>) -> Self {
        [accounts.fund_state.clone(), accounts.token_list.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN]>
for UpdateCurrentWeightsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fund_state: &arr[0],
            token_list: &arr[1],
        }
    }
}
pub const UPDATE_CURRENT_WEIGHTS_IX_DISCM: [u8; 8usize] = [
    237, 108, 97, 157, 0, 221, 11, 81,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCurrentWeightsIxData;
impl UpdateCurrentWeightsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CURRENT_WEIGHTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CURRENT_WEIGHTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_current_weights_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCurrentWeightsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CURRENT_WEIGHTS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateCurrentWeightsIxData.try_to_vec()?,
    })
}
pub fn update_current_weights_ix(
    keys: UpdateCurrentWeightsKeys,
) -> std::io::Result<Instruction> {
    update_current_weights_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn update_current_weights_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCurrentWeightsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateCurrentWeightsKeys = accounts.into();
    let ix = update_current_weights_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_current_weights_invoke(
    accounts: UpdateCurrentWeightsAccounts<'_, '_>,
) -> ProgramResult {
    update_current_weights_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn update_current_weights_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCurrentWeightsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCurrentWeightsKeys = accounts.into();
    let ix = update_current_weights_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_current_weights_invoke_signed(
    accounts: UpdateCurrentWeightsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_current_weights_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_current_weights_verify_account_keys(
    accounts: UpdateCurrentWeightsAccounts<'_, '_>,
    keys: UpdateCurrentWeightsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_current_weights_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCurrentWeightsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fund_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_current_weights_verify_account_privileges<'me, 'info>(
    accounts: UpdateCurrentWeightsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_current_weights_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct BuyStateRebalanceAccounts<'me, 'info> {
    pub fund_state: &'me AccountInfo<'info>,
    pub buy_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub oracle_token: &'me AccountInfo<'info>,
    pub oracle_usdc: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub rebalance_fee_account: &'me AccountInfo<'info>,
    pub prism_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyStateRebalanceKeys {
    pub fund_state: Pubkey,
    pub buy_state: Pubkey,
    pub token_list: Pubkey,
    pub oracle_token: Pubkey,
    pub oracle_usdc: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub rebalance_fee_account: Pubkey,
    pub prism_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<BuyStateRebalanceAccounts<'_, '_>> for BuyStateRebalanceKeys {
    fn from(accounts: BuyStateRebalanceAccounts) -> Self {
        Self {
            fund_state: *accounts.fund_state.key,
            buy_state: *accounts.buy_state.key,
            token_list: *accounts.token_list.key,
            oracle_token: *accounts.oracle_token.key,
            oracle_usdc: *accounts.oracle_usdc.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            rebalance_fee_account: *accounts.rebalance_fee_account.key,
            prism_program: *accounts.prism_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<BuyStateRebalanceKeys> for [AccountMeta; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyStateRebalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buy_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_usdc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebalance_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prism_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN]> for BuyStateRebalanceKeys {
    fn from(pubkeys: [Pubkey; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fund_state: pubkeys[0],
            buy_state: pubkeys[1],
            token_list: pubkeys[2],
            oracle_token: pubkeys[3],
            oracle_usdc: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_usdc_account: pubkeys[7],
            rebalance_fee_account: pubkeys[8],
            prism_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<BuyStateRebalanceAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyStateRebalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.fund_state.clone(),
            accounts.buy_state.clone(),
            accounts.token_list.clone(),
            accounts.oracle_token.clone(),
            accounts.oracle_usdc.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.rebalance_fee_account.clone(),
            accounts.prism_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN]>
for BuyStateRebalanceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fund_state: &arr[0],
            buy_state: &arr[1],
            token_list: &arr[2],
            oracle_token: &arr[3],
            oracle_usdc: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            pda_usdc_account: &arr[7],
            rebalance_fee_account: &arr[8],
            prism_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const BUY_STATE_REBALANCE_IX_DISCM: [u8; 8usize] = [
    35, 97, 248, 48, 213, 224, 68, 2,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyStateRebalanceIxArgs {
    pub token_id: u8,
    pub instruction_id: [u8; 8],
    pub instruction_size: u8,
    pub instruction_data: [u8; 28],
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyStateRebalanceIxData(pub BuyStateRebalanceIxArgs);
impl From<BuyStateRebalanceIxArgs> for BuyStateRebalanceIxData {
    fn from(args: BuyStateRebalanceIxArgs) -> Self {
        Self(args)
    }
}
impl BuyStateRebalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_STATE_REBALANCE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_id: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_data: [u8; 28] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyStateRebalanceIxArgs {
                token_id,
                instruction_id,
                instruction_size,
                instruction_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_STATE_REBALANCE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_state_rebalance_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyStateRebalanceKeys,
    args: BuyStateRebalanceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_STATE_REBALANCE_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyStateRebalanceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_state_rebalance_ix(
    keys: BuyStateRebalanceKeys,
    args: BuyStateRebalanceIxArgs,
) -> std::io::Result<Instruction> {
    buy_state_rebalance_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn buy_state_rebalance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyStateRebalanceAccounts<'_, '_>,
    args: BuyStateRebalanceIxArgs,
) -> ProgramResult {
    let keys: BuyStateRebalanceKeys = accounts.into();
    let ix = buy_state_rebalance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_state_rebalance_invoke(
    accounts: BuyStateRebalanceAccounts<'_, '_>,
    args: BuyStateRebalanceIxArgs,
) -> ProgramResult {
    buy_state_rebalance_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn buy_state_rebalance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyStateRebalanceAccounts<'_, '_>,
    args: BuyStateRebalanceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyStateRebalanceKeys = accounts.into();
    let ix = buy_state_rebalance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_state_rebalance_invoke_signed(
    accounts: BuyStateRebalanceAccounts<'_, '_>,
    args: BuyStateRebalanceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_state_rebalance_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_state_rebalance_verify_account_keys(
    accounts: BuyStateRebalanceAccounts<'_, '_>,
    keys: BuyStateRebalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.buy_state.key, keys.buy_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.oracle_token.key, keys.oracle_token),
        (*accounts.oracle_usdc.key, keys.oracle_usdc),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.rebalance_fee_account.key, keys.rebalance_fee_account),
        (*accounts.prism_program.key, keys.prism_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_state_rebalance_verify_writable_privileges<'me, 'info>(
    accounts: BuyStateRebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fund_state,
        accounts.buy_state,
        accounts.pda_token_account,
        accounts.pda_usdc_account,
        accounts.rebalance_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_state_rebalance_verify_account_privileges<'me, 'info>(
    accounts: BuyStateRebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_state_rebalance_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_TO_USDC_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceToUsdcAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub oracle_token: &'me AccountInfo<'info>,
    pub oracle_usdc: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub rebalance_fee_account: &'me AccountInfo<'info>,
    pub prism_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceToUsdcKeys {
    pub signer: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub oracle_token: Pubkey,
    pub oracle_usdc: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub rebalance_fee_account: Pubkey,
    pub prism_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RebalanceToUsdcAccounts<'_, '_>> for RebalanceToUsdcKeys {
    fn from(accounts: RebalanceToUsdcAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            oracle_token: *accounts.oracle_token.key,
            oracle_usdc: *accounts.oracle_usdc.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            rebalance_fee_account: *accounts.rebalance_fee_account.key,
            prism_program: *accounts.prism_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RebalanceToUsdcKeys> for [AccountMeta; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceToUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_usdc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebalance_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prism_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN]> for RebalanceToUsdcKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
            oracle_token: pubkeys[3],
            oracle_usdc: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_usdc_account: pubkeys[7],
            rebalance_fee_account: pubkeys[8],
            prism_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<RebalanceToUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceToUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.oracle_token.clone(),
            accounts.oracle_usdc.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.rebalance_fee_account.clone(),
            accounts.prism_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN]>
for RebalanceToUsdcAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
            oracle_token: &arr[3],
            oracle_usdc: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            pda_usdc_account: &arr[7],
            rebalance_fee_account: &arr[8],
            prism_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const REBALANCE_TO_USDC_IX_DISCM: [u8; 8usize] = [27, 44, 71, 100, 238, 190, 86, 15];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RebalanceToUsdcIxArgs {
    pub token_id: u8,
    pub max_amount_to_sell: u64,
    pub instruction_id: [u8; 8],
    pub instruction_size: u8,
    pub instruction_data: [u8; 28],
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceToUsdcIxData(pub RebalanceToUsdcIxArgs);
impl From<RebalanceToUsdcIxArgs> for RebalanceToUsdcIxData {
    fn from(args: RebalanceToUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl RebalanceToUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_TO_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_to_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_id: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_data: [u8; 28] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RebalanceToUsdcIxArgs {
                token_id,
                max_amount_to_sell,
                instruction_id,
                instruction_size,
                instruction_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_TO_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_to_sell, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_to_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceToUsdcKeys,
    args: RebalanceToUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_TO_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: RebalanceToUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn rebalance_to_usdc_ix(
    keys: RebalanceToUsdcKeys,
    args: RebalanceToUsdcIxArgs,
) -> std::io::Result<Instruction> {
    rebalance_to_usdc_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn rebalance_to_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceToUsdcAccounts<'_, '_>,
    args: RebalanceToUsdcIxArgs,
) -> ProgramResult {
    let keys: RebalanceToUsdcKeys = accounts.into();
    let ix = rebalance_to_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_to_usdc_invoke(
    accounts: RebalanceToUsdcAccounts<'_, '_>,
    args: RebalanceToUsdcIxArgs,
) -> ProgramResult {
    rebalance_to_usdc_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn rebalance_to_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceToUsdcAccounts<'_, '_>,
    args: RebalanceToUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceToUsdcKeys = accounts.into();
    let ix = rebalance_to_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_to_usdc_invoke_signed(
    accounts: RebalanceToUsdcAccounts<'_, '_>,
    args: RebalanceToUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_to_usdc_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn rebalance_to_usdc_verify_account_keys(
    accounts: RebalanceToUsdcAccounts<'_, '_>,
    keys: RebalanceToUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.oracle_token.key, keys.oracle_token),
        (*accounts.oracle_usdc.key, keys.oracle_usdc),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.rebalance_fee_account.key, keys.rebalance_fee_account),
        (*accounts.prism_program.key, keys.prism_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_to_usdc_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.fund_state,
        accounts.pda_token_account,
        accounts.pda_usdc_account,
        accounts.rebalance_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_to_usdc_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_to_usdc_verify_account_privileges<'me, 'info>(
    accounts: RebalanceToUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_to_usdc_verify_writable_privileges(accounts)?;
    rebalance_to_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceFromUsdcAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub oracle_token: &'me AccountInfo<'info>,
    pub oracle_usdc: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub rebalance_fee_account: &'me AccountInfo<'info>,
    pub prism_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceFromUsdcKeys {
    pub signer: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub oracle_token: Pubkey,
    pub oracle_usdc: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub rebalance_fee_account: Pubkey,
    pub prism_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RebalanceFromUsdcAccounts<'_, '_>> for RebalanceFromUsdcKeys {
    fn from(accounts: RebalanceFromUsdcAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            oracle_token: *accounts.oracle_token.key,
            oracle_usdc: *accounts.oracle_usdc.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            rebalance_fee_account: *accounts.rebalance_fee_account.key,
            prism_program: *accounts.prism_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RebalanceFromUsdcKeys> for [AccountMeta; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceFromUsdcKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_usdc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebalance_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.prism_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN]> for RebalanceFromUsdcKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
            oracle_token: pubkeys[3],
            oracle_usdc: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_usdc_account: pubkeys[7],
            rebalance_fee_account: pubkeys[8],
            prism_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<RebalanceFromUsdcAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceFromUsdcAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.oracle_token.clone(),
            accounts.oracle_usdc.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.rebalance_fee_account.clone(),
            accounts.prism_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN]>
for RebalanceFromUsdcAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
            oracle_token: &arr[3],
            oracle_usdc: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            pda_usdc_account: &arr[7],
            rebalance_fee_account: &arr[8],
            prism_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const REBALANCE_FROM_USDC_IX_DISCM: [u8; 8usize] = [
    217, 191, 217, 222, 153, 152, 134, 182,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RebalanceFromUsdcIxArgs {
    pub token_id: u8,
    pub max_amount_to_spend: u64,
    pub instruction_id: [u8; 8],
    pub instruction_size: u8,
    pub instruction_data: [u8; 28],
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceFromUsdcIxData(pub RebalanceFromUsdcIxArgs);
impl From<RebalanceFromUsdcIxArgs> for RebalanceFromUsdcIxData {
    fn from(args: RebalanceFromUsdcIxArgs) -> Self {
        Self(args)
    }
}
impl RebalanceFromUsdcIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_FROM_USDC_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_to_spend: u64 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_id: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_data: [u8; 28] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RebalanceFromUsdcIxArgs {
                token_id,
                max_amount_to_spend,
                instruction_id,
                instruction_size,
                instruction_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_FROM_USDC_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_to_spend, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_from_usdc_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceFromUsdcKeys,
    args: RebalanceFromUsdcIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_FROM_USDC_IX_ACCOUNTS_LEN] = keys.into();
    let data: RebalanceFromUsdcIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn rebalance_from_usdc_ix(
    keys: RebalanceFromUsdcKeys,
    args: RebalanceFromUsdcIxArgs,
) -> std::io::Result<Instruction> {
    rebalance_from_usdc_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn rebalance_from_usdc_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceFromUsdcAccounts<'_, '_>,
    args: RebalanceFromUsdcIxArgs,
) -> ProgramResult {
    let keys: RebalanceFromUsdcKeys = accounts.into();
    let ix = rebalance_from_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_from_usdc_invoke(
    accounts: RebalanceFromUsdcAccounts<'_, '_>,
    args: RebalanceFromUsdcIxArgs,
) -> ProgramResult {
    rebalance_from_usdc_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn rebalance_from_usdc_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceFromUsdcAccounts<'_, '_>,
    args: RebalanceFromUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceFromUsdcKeys = accounts.into();
    let ix = rebalance_from_usdc_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_from_usdc_invoke_signed(
    accounts: RebalanceFromUsdcAccounts<'_, '_>,
    args: RebalanceFromUsdcIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_from_usdc_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn rebalance_from_usdc_verify_account_keys(
    accounts: RebalanceFromUsdcAccounts<'_, '_>,
    keys: RebalanceFromUsdcKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.oracle_token.key, keys.oracle_token),
        (*accounts.oracle_usdc.key, keys.oracle_usdc),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.rebalance_fee_account.key, keys.rebalance_fee_account),
        (*accounts.prism_program.key, keys.prism_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_from_usdc_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceFromUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.fund_state,
        accounts.pda_token_account,
        accounts.pda_usdc_account,
        accounts.rebalance_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_from_usdc_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceFromUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_from_usdc_verify_account_privileges<'me, 'info>(
    accounts: RebalanceFromUsdcAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_from_usdc_verify_writable_privileges(accounts)?;
    rebalance_from_usdc_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceBuyStateAccounts<'me, 'info> {
    pub fund_state: &'me AccountInfo<'info>,
    pub buy_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub oracle_token: &'me AccountInfo<'info>,
    pub oracle_usdc: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub rebalance_fee_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceBuyStateKeys {
    pub fund_state: Pubkey,
    pub buy_state: Pubkey,
    pub token_list: Pubkey,
    pub oracle_token: Pubkey,
    pub oracle_usdc: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub rebalance_fee_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<RebalanceBuyStateAccounts<'_, '_>> for RebalanceBuyStateKeys {
    fn from(accounts: RebalanceBuyStateAccounts) -> Self {
        Self {
            fund_state: *accounts.fund_state.key,
            buy_state: *accounts.buy_state.key,
            token_list: *accounts.token_list.key,
            oracle_token: *accounts.oracle_token.key,
            oracle_usdc: *accounts.oracle_usdc.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            rebalance_fee_account: *accounts.rebalance_fee_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RebalanceBuyStateKeys> for [AccountMeta; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceBuyStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buy_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_usdc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebalance_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN]> for RebalanceBuyStateKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fund_state: pubkeys[0],
            buy_state: pubkeys[1],
            token_list: pubkeys[2],
            oracle_token: pubkeys[3],
            oracle_usdc: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_usdc_account: pubkeys[7],
            rebalance_fee_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<RebalanceBuyStateAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceBuyStateAccounts<'_, 'info>) -> Self {
        [
            accounts.fund_state.clone(),
            accounts.buy_state.clone(),
            accounts.token_list.clone(),
            accounts.oracle_token.clone(),
            accounts.oracle_usdc.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.rebalance_fee_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN]>
for RebalanceBuyStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            fund_state: &arr[0],
            buy_state: &arr[1],
            token_list: &arr[2],
            oracle_token: &arr[3],
            oracle_usdc: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            pda_usdc_account: &arr[7],
            rebalance_fee_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const REBALANCE_BUY_STATE_IX_DISCM: [u8; 8usize] = [166, 154, 51, 71, 63, 4, 7, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RebalanceBuyStateIxArgs {
    pub token_id: u8,
    pub instruction_size: u8,
    #[serde(with = "crate::big_array_serde")]
    pub instruction_data: [u8; 128],
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceBuyStateIxData(pub RebalanceBuyStateIxArgs);
impl From<RebalanceBuyStateIxArgs> for RebalanceBuyStateIxData {
    fn from(args: RebalanceBuyStateIxArgs) -> Self {
        Self(args)
    }
}
impl RebalanceBuyStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_BUY_STATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_data = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(RebalanceBuyStateIxArgs {
                token_id,
                instruction_size,
                instruction_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_BUY_STATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_buy_state_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceBuyStateKeys,
    args: RebalanceBuyStateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_BUY_STATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RebalanceBuyStateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn rebalance_buy_state_ix(
    keys: RebalanceBuyStateKeys,
    args: RebalanceBuyStateIxArgs,
) -> std::io::Result<Instruction> {
    rebalance_buy_state_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn rebalance_buy_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceBuyStateAccounts<'_, '_>,
    args: RebalanceBuyStateIxArgs,
) -> ProgramResult {
    let keys: RebalanceBuyStateKeys = accounts.into();
    let ix = rebalance_buy_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_buy_state_invoke(
    accounts: RebalanceBuyStateAccounts<'_, '_>,
    args: RebalanceBuyStateIxArgs,
) -> ProgramResult {
    rebalance_buy_state_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn rebalance_buy_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceBuyStateAccounts<'_, '_>,
    args: RebalanceBuyStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceBuyStateKeys = accounts.into();
    let ix = rebalance_buy_state_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_buy_state_invoke_signed(
    accounts: RebalanceBuyStateAccounts<'_, '_>,
    args: RebalanceBuyStateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_buy_state_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn rebalance_buy_state_verify_account_keys(
    accounts: RebalanceBuyStateAccounts<'_, '_>,
    keys: RebalanceBuyStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.buy_state.key, keys.buy_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.oracle_token.key, keys.oracle_token),
        (*accounts.oracle_usdc.key, keys.oracle_usdc),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.rebalance_fee_account.key, keys.rebalance_fee_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_buy_state_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceBuyStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fund_state,
        accounts.buy_state,
        accounts.pda_token_account,
        accounts.pda_usdc_account,
        accounts.rebalance_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_buy_state_verify_account_privileges<'me, 'info>(
    accounts: RebalanceBuyStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_buy_state_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_SELL_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceSellAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub oracle_token: &'me AccountInfo<'info>,
    pub oracle_usdc: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub rebalance_fee_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceSellKeys {
    pub signer: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub oracle_token: Pubkey,
    pub oracle_usdc: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub rebalance_fee_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<RebalanceSellAccounts<'_, '_>> for RebalanceSellKeys {
    fn from(accounts: RebalanceSellAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            oracle_token: *accounts.oracle_token.key,
            oracle_usdc: *accounts.oracle_usdc.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            rebalance_fee_account: *accounts.rebalance_fee_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RebalanceSellKeys> for [AccountMeta; REBALANCE_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_usdc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebalance_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REBALANCE_SELL_IX_ACCOUNTS_LEN]> for RebalanceSellKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
            oracle_token: pubkeys[3],
            oracle_usdc: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_usdc_account: pubkeys[7],
            rebalance_fee_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<RebalanceSellAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceSellAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.oracle_token.clone(),
            accounts.oracle_usdc.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.rebalance_fee_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_SELL_IX_ACCOUNTS_LEN]>
for RebalanceSellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REBALANCE_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
            oracle_token: &arr[3],
            oracle_usdc: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            pda_usdc_account: &arr[7],
            rebalance_fee_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const REBALANCE_SELL_IX_DISCM: [u8; 8usize] = [53, 35, 128, 189, 197, 148, 81, 28];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RebalanceSellIxArgs {
    pub token_id: u8,
    pub max_amount_to_sell: u64,
    pub instruction_size: u8,
    #[serde(with = "crate::big_array_serde")]
    pub instruction_data: [u8; 128],
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceSellIxData(pub RebalanceSellIxArgs);
impl From<RebalanceSellIxArgs> for RebalanceSellIxData {
    fn from(args: RebalanceSellIxArgs) -> Self {
        Self(args)
    }
}
impl RebalanceSellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_SELL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_to_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_data = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(RebalanceSellIxArgs {
                token_id,
                max_amount_to_sell,
                instruction_size,
                instruction_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_to_sell, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_sell_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceSellKeys,
    args: RebalanceSellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: RebalanceSellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn rebalance_sell_ix(
    keys: RebalanceSellKeys,
    args: RebalanceSellIxArgs,
) -> std::io::Result<Instruction> {
    rebalance_sell_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn rebalance_sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceSellAccounts<'_, '_>,
    args: RebalanceSellIxArgs,
) -> ProgramResult {
    let keys: RebalanceSellKeys = accounts.into();
    let ix = rebalance_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_sell_invoke(
    accounts: RebalanceSellAccounts<'_, '_>,
    args: RebalanceSellIxArgs,
) -> ProgramResult {
    rebalance_sell_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn rebalance_sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceSellAccounts<'_, '_>,
    args: RebalanceSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceSellKeys = accounts.into();
    let ix = rebalance_sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_sell_invoke_signed(
    accounts: RebalanceSellAccounts<'_, '_>,
    args: RebalanceSellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_sell_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn rebalance_sell_verify_account_keys(
    accounts: RebalanceSellAccounts<'_, '_>,
    keys: RebalanceSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.oracle_token.key, keys.oracle_token),
        (*accounts.oracle_usdc.key, keys.oracle_usdc),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.rebalance_fee_account.key, keys.rebalance_fee_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_sell_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.fund_state,
        accounts.pda_token_account,
        accounts.pda_usdc_account,
        accounts.rebalance_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_sell_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_sell_verify_account_privileges<'me, 'info>(
    accounts: RebalanceSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_sell_verify_writable_privileges(accounts)?;
    rebalance_sell_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REBALANCE_BUY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RebalanceBuyAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub oracle_token: &'me AccountInfo<'info>,
    pub oracle_usdc: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_token_account: &'me AccountInfo<'info>,
    pub pda_usdc_account: &'me AccountInfo<'info>,
    pub rebalance_fee_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RebalanceBuyKeys {
    pub signer: Pubkey,
    pub fund_state: Pubkey,
    pub token_list: Pubkey,
    pub oracle_token: Pubkey,
    pub oracle_usdc: Pubkey,
    pub pda_account: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda_usdc_account: Pubkey,
    pub rebalance_fee_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<RebalanceBuyAccounts<'_, '_>> for RebalanceBuyKeys {
    fn from(accounts: RebalanceBuyAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            fund_state: *accounts.fund_state.key,
            token_list: *accounts.token_list.key,
            oracle_token: *accounts.oracle_token.key,
            oracle_usdc: *accounts.oracle_usdc.key,
            pda_account: *accounts.pda_account.key,
            pda_token_account: *accounts.pda_token_account.key,
            pda_usdc_account: *accounts.pda_usdc_account.key,
            rebalance_fee_account: *accounts.rebalance_fee_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RebalanceBuyKeys> for [AccountMeta; REBALANCE_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: RebalanceBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_usdc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_usdc_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebalance_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REBALANCE_BUY_IX_ACCOUNTS_LEN]> for RebalanceBuyKeys {
    fn from(pubkeys: [Pubkey; REBALANCE_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            fund_state: pubkeys[1],
            token_list: pubkeys[2],
            oracle_token: pubkeys[3],
            oracle_usdc: pubkeys[4],
            pda_account: pubkeys[5],
            pda_token_account: pubkeys[6],
            pda_usdc_account: pubkeys[7],
            rebalance_fee_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<RebalanceBuyAccounts<'_, 'info>>
for [AccountInfo<'info>; REBALANCE_BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RebalanceBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.fund_state.clone(),
            accounts.token_list.clone(),
            accounts.oracle_token.clone(),
            accounts.oracle_usdc.clone(),
            accounts.pda_account.clone(),
            accounts.pda_token_account.clone(),
            accounts.pda_usdc_account.clone(),
            accounts.rebalance_fee_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REBALANCE_BUY_IX_ACCOUNTS_LEN]>
for RebalanceBuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REBALANCE_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            fund_state: &arr[1],
            token_list: &arr[2],
            oracle_token: &arr[3],
            oracle_usdc: &arr[4],
            pda_account: &arr[5],
            pda_token_account: &arr[6],
            pda_usdc_account: &arr[7],
            rebalance_fee_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const REBALANCE_BUY_IX_DISCM: [u8; 8usize] = [205, 136, 203, 130, 86, 125, 162, 20];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RebalanceBuyIxArgs {
    pub token_id: u8,
    pub max_amount_to_spend: u64,
    pub instruction_size: u8,
    #[serde(with = "crate::big_array_serde")]
    pub instruction_data: [u8; 128],
}
#[derive(Clone, Debug, PartialEq)]
pub struct RebalanceBuyIxData(pub RebalanceBuyIxArgs);
impl From<RebalanceBuyIxArgs> for RebalanceBuyIxData {
    fn from(args: RebalanceBuyIxArgs) -> Self {
        Self(args)
    }
}
impl RebalanceBuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REBALANCE_BUY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount_to_spend: u64 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_size: u8 = crate::borsh_de_or_default(&mut reader)?;
        let instruction_data = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(RebalanceBuyIxArgs {
                token_id,
                max_amount_to_spend,
                instruction_size,
                instruction_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REBALANCE_BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_amount_to_spend, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.instruction_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn rebalance_buy_ix_with_program_id(
    program_id: Pubkey,
    keys: RebalanceBuyKeys,
    args: RebalanceBuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REBALANCE_BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RebalanceBuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn rebalance_buy_ix(
    keys: RebalanceBuyKeys,
    args: RebalanceBuyIxArgs,
) -> std::io::Result<Instruction> {
    rebalance_buy_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn rebalance_buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceBuyAccounts<'_, '_>,
    args: RebalanceBuyIxArgs,
) -> ProgramResult {
    let keys: RebalanceBuyKeys = accounts.into();
    let ix = rebalance_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn rebalance_buy_invoke(
    accounts: RebalanceBuyAccounts<'_, '_>,
    args: RebalanceBuyIxArgs,
) -> ProgramResult {
    rebalance_buy_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn rebalance_buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RebalanceBuyAccounts<'_, '_>,
    args: RebalanceBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RebalanceBuyKeys = accounts.into();
    let ix = rebalance_buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn rebalance_buy_invoke_signed(
    accounts: RebalanceBuyAccounts<'_, '_>,
    args: RebalanceBuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    rebalance_buy_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn rebalance_buy_verify_account_keys(
    accounts: RebalanceBuyAccounts<'_, '_>,
    keys: RebalanceBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.oracle_token.key, keys.oracle_token),
        (*accounts.oracle_usdc.key, keys.oracle_usdc),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.pda_token_account.key, keys.pda_token_account),
        (*accounts.pda_usdc_account.key, keys.pda_usdc_account),
        (*accounts.rebalance_fee_account.key, keys.rebalance_fee_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn rebalance_buy_verify_writable_privileges<'me, 'info>(
    accounts: RebalanceBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.fund_state,
        accounts.pda_token_account,
        accounts.pda_usdc_account,
        accounts.rebalance_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn rebalance_buy_verify_signer_privileges<'me, 'info>(
    accounts: RebalanceBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn rebalance_buy_verify_account_privileges<'me, 'info>(
    accounts: RebalanceBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    rebalance_buy_verify_writable_privileges(accounts)?;
    rebalance_buy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REWEIGHT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ReweightAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReweightKeys {
    pub signer: Pubkey,
    pub fund_state: Pubkey,
    pub token_stats: Pubkey,
}
impl From<ReweightAccounts<'_, '_>> for ReweightKeys {
    fn from(accounts: ReweightAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            fund_state: *accounts.fund_state.key,
            token_stats: *accounts.token_stats.key,
        }
    }
}
impl From<ReweightKeys> for [AccountMeta; REWEIGHT_IX_ACCOUNTS_LEN] {
    fn from(keys: ReweightKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_stats,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REWEIGHT_IX_ACCOUNTS_LEN]> for ReweightKeys {
    fn from(pubkeys: [Pubkey; REWEIGHT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            fund_state: pubkeys[1],
            token_stats: pubkeys[2],
        }
    }
}
impl<'info> From<ReweightAccounts<'_, 'info>>
for [AccountInfo<'info>; REWEIGHT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ReweightAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.fund_state.clone(),
            accounts.token_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REWEIGHT_IX_ACCOUNTS_LEN]>
for ReweightAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REWEIGHT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            fund_state: &arr[1],
            token_stats: &arr[2],
        }
    }
}
pub const REWEIGHT_IX_DISCM: [u8; 8usize] = [0, 197, 236, 165, 101, 52, 241, 75];
#[derive(Clone, Debug, PartialEq)]
pub struct ReweightIxData;
impl ReweightIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REWEIGHT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REWEIGHT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reweight_ix_with_program_id(
    program_id: Pubkey,
    keys: ReweightKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REWEIGHT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ReweightIxData.try_to_vec()?,
    })
}
pub fn reweight_ix(keys: ReweightKeys) -> std::io::Result<Instruction> {
    reweight_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn reweight_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ReweightAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ReweightKeys = accounts.into();
    let ix = reweight_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn reweight_invoke(accounts: ReweightAccounts<'_, '_>) -> ProgramResult {
    reweight_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn reweight_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ReweightAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ReweightKeys = accounts.into();
    let ix = reweight_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reweight_invoke_signed(
    accounts: ReweightAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reweight_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn reweight_verify_account_keys(
    accounts: ReweightAccounts<'_, '_>,
    keys: ReweightKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_stats.key, keys.token_stats),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn reweight_verify_writable_privileges<'me, 'info>(
    accounts: ReweightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.fund_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reweight_verify_signer_privileges<'me, 'info>(
    accounts: ReweightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reweight_verify_account_privileges<'me, 'info>(
    accounts: ReweightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reweight_verify_writable_privileges(accounts)?;
    reweight_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFILTER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RefilterAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub token_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefilterKeys {
    pub signer: Pubkey,
    pub fund_state: Pubkey,
    pub token_stats: Pubkey,
}
impl From<RefilterAccounts<'_, '_>> for RefilterKeys {
    fn from(accounts: RefilterAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            fund_state: *accounts.fund_state.key,
            token_stats: *accounts.token_stats.key,
        }
    }
}
impl From<RefilterKeys> for [AccountMeta; REFILTER_IX_ACCOUNTS_LEN] {
    fn from(keys: RefilterKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_stats,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFILTER_IX_ACCOUNTS_LEN]> for RefilterKeys {
    fn from(pubkeys: [Pubkey; REFILTER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            fund_state: pubkeys[1],
            token_stats: pubkeys[2],
        }
    }
}
impl<'info> From<RefilterAccounts<'_, 'info>>
for [AccountInfo<'info>; REFILTER_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefilterAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.fund_state.clone(),
            accounts.token_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFILTER_IX_ACCOUNTS_LEN]>
for RefilterAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFILTER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            fund_state: &arr[1],
            token_stats: &arr[2],
        }
    }
}
pub const REFILTER_IX_DISCM: [u8; 8usize] = [89, 202, 198, 50, 226, 121, 115, 157];
#[derive(Clone, Debug, PartialEq)]
pub struct RefilterIxData;
impl RefilterIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFILTER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFILTER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refilter_ix_with_program_id(
    program_id: Pubkey,
    keys: RefilterKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFILTER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefilterIxData.try_to_vec()?,
    })
}
pub fn refilter_ix(keys: RefilterKeys) -> std::io::Result<Instruction> {
    refilter_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn refilter_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefilterAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefilterKeys = accounts.into();
    let ix = refilter_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refilter_invoke(accounts: RefilterAccounts<'_, '_>) -> ProgramResult {
    refilter_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn refilter_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefilterAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefilterKeys = accounts.into();
    let ix = refilter_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refilter_invoke_signed(
    accounts: RefilterAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refilter_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn refilter_verify_account_keys(
    accounts: RefilterAccounts<'_, '_>,
    keys: RefilterKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.token_stats.key, keys.token_stats),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refilter_verify_writable_privileges<'me, 'info>(
    accounts: RefilterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.fund_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refilter_verify_signer_privileges<'me, 'info>(
    accounts: RefilterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refilter_verify_account_privileges<'me, 'info>(
    accounts: RefilterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refilter_verify_writable_privileges(accounts)?;
    refilter_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenListAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTokenListKeys {
    pub owner: Pubkey,
    pub token_list: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateTokenListAccounts<'_, '_>> for UpdateTokenListKeys {
    fn from(accounts: UpdateTokenListAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_list: *accounts.token_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateTokenListKeys> for [AccountMeta; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTokenListKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN]> for UpdateTokenListKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_list: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateTokenListAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTokenListAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN]>
for UpdateTokenListAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            token_list: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_TOKEN_LIST_IX_DISCM: [u8; 8usize] = [
    58, 208, 96, 231, 40, 122, 12, 190,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTokenListIxArgs {
    pub index: u8,
    pub token_mint: Pubkey,
    pub decimals: u8,
    pub coingecko_id: [u8; 30],
    pub pda_token_account: Pubkey,
    pub oracle_type: u8,
    pub oracle_account: Pubkey,
    pub oracle_index: u8,
    pub oracle_confidence_pct: u8,
    pub fixed_confidence_bps: u8,
    pub token_swap_fee_before_tw_bps: u8,
    pub token_swap_fee_after_tw_bps: u8,
    pub is_live: u8,
    pub lp_on: u8,
    pub use_curve_data: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenListIxData(pub UpdateTokenListIxArgs);
impl From<UpdateTokenListIxArgs> for UpdateTokenListIxData {
    fn from(args: UpdateTokenListIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTokenListIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TOKEN_LIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let coingecko_id: [u8; 30] = crate::borsh_de_or_default(&mut reader)?;
        let pda_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_confidence_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_confidence_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_swap_fee_before_tw_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_swap_fee_after_tw_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_live: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_on: u8 = crate::borsh_de_or_default(&mut reader)?;
        let use_curve_data: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateTokenListIxArgs {
                index,
                token_mint,
                decimals,
                coingecko_id,
                pda_token_account,
                oracle_type,
                oracle_account,
                oracle_index,
                oracle_confidence_pct,
                fixed_confidence_bps,
                token_swap_fee_before_tw_bps,
                token_swap_fee_after_tw_bps,
                is_live,
                lp_on,
                use_curve_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TOKEN_LIST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.coingecko_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pda_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.oracle_confidence_pct, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fixed_confidence_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.token_swap_fee_before_tw_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.token_swap_fee_after_tw_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.is_live, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lp_on, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.use_curve_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_token_list_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTokenListKeys,
    args: UpdateTokenListIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TOKEN_LIST_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTokenListIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_token_list_ix(
    keys: UpdateTokenListKeys,
    args: UpdateTokenListIxArgs,
) -> std::io::Result<Instruction> {
    update_token_list_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn update_token_list_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenListAccounts<'_, '_>,
    args: UpdateTokenListIxArgs,
) -> ProgramResult {
    let keys: UpdateTokenListKeys = accounts.into();
    let ix = update_token_list_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_token_list_invoke(
    accounts: UpdateTokenListAccounts<'_, '_>,
    args: UpdateTokenListIxArgs,
) -> ProgramResult {
    update_token_list_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn update_token_list_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenListAccounts<'_, '_>,
    args: UpdateTokenListIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTokenListKeys = accounts.into();
    let ix = update_token_list_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_token_list_invoke_signed(
    accounts: UpdateTokenListAccounts<'_, '_>,
    args: UpdateTokenListIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_token_list_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_token_list_verify_account_keys(
    accounts: UpdateTokenListAccounts<'_, '_>,
    keys: UpdateTokenListKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_token_list_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_token_list_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_token_list_verify_account_privileges<'me, 'info>(
    accounts: UpdateTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_token_list_verify_writable_privileges(accounts)?;
    update_token_list_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ModifyFeeStructureAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ModifyFeeStructureKeys {
    pub owner: Pubkey,
    pub token_list: Pubkey,
    pub system_program: Pubkey,
}
impl From<ModifyFeeStructureAccounts<'_, '_>> for ModifyFeeStructureKeys {
    fn from(accounts: ModifyFeeStructureAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_list: *accounts.token_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ModifyFeeStructureKeys>
for [AccountMeta; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN] {
    fn from(keys: ModifyFeeStructureKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN]> for ModifyFeeStructureKeys {
    fn from(pubkeys: [Pubkey; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_list: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ModifyFeeStructureAccounts<'_, 'info>>
for [AccountInfo<'info>; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ModifyFeeStructureAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN]>
for ModifyFeeStructureAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            token_list: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const MODIFY_FEE_STRUCTURE_IX_DISCM: [u8; 8usize] = [
    176, 201, 85, 183, 157, 197, 162, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModifyFeeStructureIxArgs {
    pub symmetry_fee: u8,
    pub host_fee: u8,
    pub manager_fee: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModifyFeeStructureIxData(pub ModifyFeeStructureIxArgs);
impl From<ModifyFeeStructureIxArgs> for ModifyFeeStructureIxData {
    fn from(args: ModifyFeeStructureIxArgs) -> Self {
        Self(args)
    }
}
impl ModifyFeeStructureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODIFY_FEE_STRUCTURE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let symmetry_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
        let host_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
        let manager_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ModifyFeeStructureIxArgs {
                symmetry_fee,
                host_fee,
                manager_fee,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODIFY_FEE_STRUCTURE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.symmetry_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.host_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.manager_fee, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn modify_fee_structure_ix_with_program_id(
    program_id: Pubkey,
    keys: ModifyFeeStructureKeys,
    args: ModifyFeeStructureIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MODIFY_FEE_STRUCTURE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ModifyFeeStructureIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn modify_fee_structure_ix(
    keys: ModifyFeeStructureKeys,
    args: ModifyFeeStructureIxArgs,
) -> std::io::Result<Instruction> {
    modify_fee_structure_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn modify_fee_structure_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ModifyFeeStructureAccounts<'_, '_>,
    args: ModifyFeeStructureIxArgs,
) -> ProgramResult {
    let keys: ModifyFeeStructureKeys = accounts.into();
    let ix = modify_fee_structure_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn modify_fee_structure_invoke(
    accounts: ModifyFeeStructureAccounts<'_, '_>,
    args: ModifyFeeStructureIxArgs,
) -> ProgramResult {
    modify_fee_structure_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn modify_fee_structure_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ModifyFeeStructureAccounts<'_, '_>,
    args: ModifyFeeStructureIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ModifyFeeStructureKeys = accounts.into();
    let ix = modify_fee_structure_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn modify_fee_structure_invoke_signed(
    accounts: ModifyFeeStructureAccounts<'_, '_>,
    args: ModifyFeeStructureIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    modify_fee_structure_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn modify_fee_structure_verify_account_keys(
    accounts: ModifyFeeStructureAccounts<'_, '_>,
    keys: ModifyFeeStructureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn modify_fee_structure_verify_writable_privileges<'me, 'info>(
    accounts: ModifyFeeStructureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn modify_fee_structure_verify_signer_privileges<'me, 'info>(
    accounts: ModifyFeeStructureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn modify_fee_structure_verify_account_privileges<'me, 'info>(
    accounts: ModifyFeeStructureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    modify_fee_structure_verify_writable_privileges(accounts)?;
    modify_fee_structure_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_TOKEN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AddTokenAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_info: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddTokenKeys {
    pub owner: Pubkey,
    pub token_info: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddTokenAccounts<'_, '_>> for AddTokenKeys {
    fn from(accounts: AddTokenAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_info: *accounts.token_info.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddTokenKeys> for [AccountMeta; ADD_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: AddTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_TOKEN_IX_ACCOUNTS_LEN]> for AddTokenKeys {
    fn from(pubkeys: [Pubkey; ADD_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_info: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<AddTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_info.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_TOKEN_IX_ACCOUNTS_LEN]>
for AddTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            token_info: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const ADD_TOKEN_IX_DISCM: [u8; 8usize] = [237, 255, 26, 54, 56, 48, 68, 52];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddTokenIxArgs {
    pub token_mint: Pubkey,
    pub pda_token_account: Pubkey,
    pub coingecko_id: [u8; 30],
    pub pyth: Pubkey,
    pub decimals: u8,
    pub index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddTokenIxData(pub AddTokenIxArgs);
impl From<AddTokenIxArgs> for AddTokenIxData {
    fn from(args: AddTokenIxArgs) -> Self {
        Self(args)
    }
}
impl AddTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pda_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let coingecko_id: [u8; 30] = crate::borsh_de_or_default(&mut reader)?;
        let pyth: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddTokenIxArgs {
                token_mint,
                pda_token_account,
                coingecko_id,
                pyth,
                decimals,
                index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pda_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.coingecko_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pyth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_token_ix_with_program_id(
    program_id: Pubkey,
    keys: AddTokenKeys,
    args: AddTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_token_ix(
    keys: AddTokenKeys,
    args: AddTokenIxArgs,
) -> std::io::Result<Instruction> {
    add_token_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn add_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddTokenAccounts<'_, '_>,
    args: AddTokenIxArgs,
) -> ProgramResult {
    let keys: AddTokenKeys = accounts.into();
    let ix = add_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_token_invoke(
    accounts: AddTokenAccounts<'_, '_>,
    args: AddTokenIxArgs,
) -> ProgramResult {
    add_token_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn add_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddTokenAccounts<'_, '_>,
    args: AddTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddTokenKeys = accounts.into();
    let ix = add_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_token_invoke_signed(
    accounts: AddTokenAccounts<'_, '_>,
    args: AddTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_token_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_token_verify_account_keys(
    accounts: AddTokenAccounts<'_, '_>,
    keys: AddTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_info.key, keys.token_info),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_token_verify_writable_privileges<'me, 'info>(
    accounts: AddTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_info] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_token_verify_signer_privileges<'me, 'info>(
    accounts: AddTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_token_verify_account_privileges<'me, 'info>(
    accounts: AddTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_token_verify_writable_privileges(accounts)?;
    add_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_DATABASE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDatabaseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub database: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDatabaseKeys {
    pub owner: Pubkey,
    pub database: Pubkey,
}
impl From<UpdateDatabaseAccounts<'_, '_>> for UpdateDatabaseKeys {
    fn from(accounts: UpdateDatabaseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            database: *accounts.database.key,
        }
    }
}
impl From<UpdateDatabaseKeys> for [AccountMeta; UPDATE_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateDatabaseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.database,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_DATABASE_IX_ACCOUNTS_LEN]> for UpdateDatabaseKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            database: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateDatabaseAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateDatabaseAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.database.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_DATABASE_IX_ACCOUNTS_LEN]>
for UpdateDatabaseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            database: &arr[1],
        }
    }
}
pub const UPDATE_DATABASE_IX_DISCM: [u8; 8usize] = [85, 153, 200, 159, 33, 197, 46, 77];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateDatabaseIxArgs {
    pub token_id: u64,
    pub price: u64,
    pub circulating_supply: u64,
    pub volume: u64,
    pub timestamp: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDatabaseIxData(pub UpdateDatabaseIxArgs);
impl From<UpdateDatabaseIxArgs> for UpdateDatabaseIxData {
    fn from(args: UpdateDatabaseIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDatabaseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DATABASE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let circulating_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateDatabaseIxArgs {
                token_id,
                price,
                circulating_supply,
                volume,
                timestamp,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DATABASE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.circulating_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.timestamp, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_database_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateDatabaseKeys,
    args: UpdateDatabaseIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_DATABASE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateDatabaseIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_database_ix(
    keys: UpdateDatabaseKeys,
    args: UpdateDatabaseIxArgs,
) -> std::io::Result<Instruction> {
    update_database_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn update_database_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDatabaseAccounts<'_, '_>,
    args: UpdateDatabaseIxArgs,
) -> ProgramResult {
    let keys: UpdateDatabaseKeys = accounts.into();
    let ix = update_database_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_database_invoke(
    accounts: UpdateDatabaseAccounts<'_, '_>,
    args: UpdateDatabaseIxArgs,
) -> ProgramResult {
    update_database_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn update_database_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateDatabaseAccounts<'_, '_>,
    args: UpdateDatabaseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateDatabaseKeys = accounts.into();
    let ix = update_database_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_database_invoke_signed(
    accounts: UpdateDatabaseAccounts<'_, '_>,
    args: UpdateDatabaseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_database_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_database_verify_account_keys(
    accounts: UpdateDatabaseAccounts<'_, '_>,
    keys: UpdateDatabaseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.database.key, keys.database),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_database_verify_writable_privileges<'me, 'info>(
    accounts: UpdateDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.database] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_database_verify_signer_privileges<'me, 'info>(
    accounts: UpdateDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_database_verify_account_privileges<'me, 'info>(
    accounts: UpdateDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_database_verify_writable_privileges(accounts)?;
    update_database_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLEAR_DATABASE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ClearDatabaseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub database: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClearDatabaseKeys {
    pub owner: Pubkey,
    pub database: Pubkey,
}
impl From<ClearDatabaseAccounts<'_, '_>> for ClearDatabaseKeys {
    fn from(accounts: ClearDatabaseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            database: *accounts.database.key,
        }
    }
}
impl From<ClearDatabaseKeys> for [AccountMeta; CLEAR_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClearDatabaseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.database,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLEAR_DATABASE_IX_ACCOUNTS_LEN]> for ClearDatabaseKeys {
    fn from(pubkeys: [Pubkey; CLEAR_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            database: pubkeys[1],
        }
    }
}
impl<'info> From<ClearDatabaseAccounts<'_, 'info>>
for [AccountInfo<'info>; CLEAR_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClearDatabaseAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.database.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLEAR_DATABASE_IX_ACCOUNTS_LEN]>
for ClearDatabaseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLEAR_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            database: &arr[1],
        }
    }
}
pub const CLEAR_DATABASE_IX_DISCM: [u8; 8usize] = [3, 215, 185, 177, 43, 253, 204, 10];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClearDatabaseIxArgs {
    pub token_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClearDatabaseIxData(pub ClearDatabaseIxArgs);
impl From<ClearDatabaseIxArgs> for ClearDatabaseIxData {
    fn from(args: ClearDatabaseIxArgs) -> Self {
        Self(args)
    }
}
impl ClearDatabaseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLEAR_DATABASE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ClearDatabaseIxArgs { token_id }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLEAR_DATABASE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clear_database_ix_with_program_id(
    program_id: Pubkey,
    keys: ClearDatabaseKeys,
    args: ClearDatabaseIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEAR_DATABASE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClearDatabaseIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn clear_database_ix(
    keys: ClearDatabaseKeys,
    args: ClearDatabaseIxArgs,
) -> std::io::Result<Instruction> {
    clear_database_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn clear_database_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClearDatabaseAccounts<'_, '_>,
    args: ClearDatabaseIxArgs,
) -> ProgramResult {
    let keys: ClearDatabaseKeys = accounts.into();
    let ix = clear_database_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn clear_database_invoke(
    accounts: ClearDatabaseAccounts<'_, '_>,
    args: ClearDatabaseIxArgs,
) -> ProgramResult {
    clear_database_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn clear_database_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClearDatabaseAccounts<'_, '_>,
    args: ClearDatabaseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClearDatabaseKeys = accounts.into();
    let ix = clear_database_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clear_database_invoke_signed(
    accounts: ClearDatabaseAccounts<'_, '_>,
    args: ClearDatabaseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clear_database_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn clear_database_verify_account_keys(
    accounts: ClearDatabaseAccounts<'_, '_>,
    keys: ClearDatabaseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.database.key, keys.database),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clear_database_verify_writable_privileges<'me, 'info>(
    accounts: ClearDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.database] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clear_database_verify_signer_privileges<'me, 'info>(
    accounts: ClearDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn clear_database_verify_account_privileges<'me, 'info>(
    accounts: ClearDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clear_database_verify_writable_privileges(accounts)?;
    clear_database_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenStatsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_stats: &'me AccountInfo<'info>,
    pub database: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTokenStatsKeys {
    pub owner: Pubkey,
    pub token_stats: Pubkey,
    pub database: Pubkey,
}
impl From<UpdateTokenStatsAccounts<'_, '_>> for UpdateTokenStatsKeys {
    fn from(accounts: UpdateTokenStatsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_stats: *accounts.token_stats.key,
            database: *accounts.database.key,
        }
    }
}
impl From<UpdateTokenStatsKeys> for [AccountMeta; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTokenStatsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.database,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN]> for UpdateTokenStatsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_stats: pubkeys[1],
            database: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateTokenStatsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTokenStatsAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.token_stats.clone(), accounts.database.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN]>
for UpdateTokenStatsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            token_stats: &arr[1],
            database: &arr[2],
        }
    }
}
pub const UPDATE_TOKEN_STATS_IX_DISCM: [u8; 8usize] = [
    204, 141, 43, 8, 99, 142, 205, 101,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTokenStatsIxArgs {
    pub start_index: u8,
    pub end_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenStatsIxData(pub UpdateTokenStatsIxArgs);
impl From<UpdateTokenStatsIxArgs> for UpdateTokenStatsIxData {
    fn from(args: UpdateTokenStatsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTokenStatsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TOKEN_STATS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let end_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateTokenStatsIxArgs {
                start_index,
                end_index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TOKEN_STATS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_token_stats_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTokenStatsKeys,
    args: UpdateTokenStatsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TOKEN_STATS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTokenStatsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_token_stats_ix(
    keys: UpdateTokenStatsKeys,
    args: UpdateTokenStatsIxArgs,
) -> std::io::Result<Instruction> {
    update_token_stats_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn update_token_stats_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenStatsAccounts<'_, '_>,
    args: UpdateTokenStatsIxArgs,
) -> ProgramResult {
    let keys: UpdateTokenStatsKeys = accounts.into();
    let ix = update_token_stats_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_token_stats_invoke(
    accounts: UpdateTokenStatsAccounts<'_, '_>,
    args: UpdateTokenStatsIxArgs,
) -> ProgramResult {
    update_token_stats_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn update_token_stats_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenStatsAccounts<'_, '_>,
    args: UpdateTokenStatsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTokenStatsKeys = accounts.into();
    let ix = update_token_stats_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_token_stats_invoke_signed(
    accounts: UpdateTokenStatsAccounts<'_, '_>,
    args: UpdateTokenStatsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_token_stats_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_token_stats_verify_account_keys(
    accounts: UpdateTokenStatsAccounts<'_, '_>,
    keys: UpdateTokenStatsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_stats.key, keys.token_stats),
        (*accounts.database.key, keys.database),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_token_stats_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_token_stats_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_token_stats_verify_account_privileges<'me, 'info>(
    accounts: UpdateTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_token_stats_verify_writable_privileges(accounts)?;
    update_token_stats_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTokenStatsV2Accounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_stats: &'me AccountInfo<'info>,
    pub database: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTokenStatsV2Keys {
    pub owner: Pubkey,
    pub token_stats: Pubkey,
    pub database: Pubkey,
}
impl From<UpdateTokenStatsV2Accounts<'_, '_>> for UpdateTokenStatsV2Keys {
    fn from(accounts: UpdateTokenStatsV2Accounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_stats: *accounts.token_stats.key,
            database: *accounts.database.key,
        }
    }
}
impl From<UpdateTokenStatsV2Keys>
for [AccountMeta; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTokenStatsV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.database,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN]> for UpdateTokenStatsV2Keys {
    fn from(pubkeys: [Pubkey; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_stats: pubkeys[1],
            database: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateTokenStatsV2Accounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTokenStatsV2Accounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.token_stats.clone(), accounts.database.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN]>
for UpdateTokenStatsV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            token_stats: &arr[1],
            database: &arr[2],
        }
    }
}
pub const UPDATE_TOKEN_STATS_V2_IX_DISCM: [u8; 8usize] = [
    176, 137, 40, 199, 138, 237, 183, 238,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTokenStatsV2IxArgs {
    pub token: u8,
    pub data: [[u64; 3]; 6],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenStatsV2IxData(pub UpdateTokenStatsV2IxArgs);
impl From<UpdateTokenStatsV2IxArgs> for UpdateTokenStatsV2IxData {
    fn from(args: UpdateTokenStatsV2IxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTokenStatsV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TOKEN_STATS_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token: u8 = crate::borsh_de_or_default(&mut reader)?;
        let data: [[u64; 3]; 6] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateTokenStatsV2IxArgs {
                token,
                data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TOKEN_STATS_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_token_stats_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTokenStatsV2Keys,
    args: UpdateTokenStatsV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TOKEN_STATS_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTokenStatsV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_token_stats_v2_ix(
    keys: UpdateTokenStatsV2Keys,
    args: UpdateTokenStatsV2IxArgs,
) -> std::io::Result<Instruction> {
    update_token_stats_v2_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn update_token_stats_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenStatsV2Accounts<'_, '_>,
    args: UpdateTokenStatsV2IxArgs,
) -> ProgramResult {
    let keys: UpdateTokenStatsV2Keys = accounts.into();
    let ix = update_token_stats_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_token_stats_v2_invoke(
    accounts: UpdateTokenStatsV2Accounts<'_, '_>,
    args: UpdateTokenStatsV2IxArgs,
) -> ProgramResult {
    update_token_stats_v2_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn update_token_stats_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTokenStatsV2Accounts<'_, '_>,
    args: UpdateTokenStatsV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTokenStatsV2Keys = accounts.into();
    let ix = update_token_stats_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_token_stats_v2_invoke_signed(
    accounts: UpdateTokenStatsV2Accounts<'_, '_>,
    args: UpdateTokenStatsV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_token_stats_v2_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_token_stats_v2_verify_account_keys(
    accounts: UpdateTokenStatsV2Accounts<'_, '_>,
    keys: UpdateTokenStatsV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_stats.key, keys.token_stats),
        (*accounts.database.key, keys.database),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_token_stats_v2_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTokenStatsV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_token_stats_v2_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTokenStatsV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_token_stats_v2_verify_account_privileges<'me, 'info>(
    accounts: UpdateTokenStatsV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_token_stats_v2_verify_writable_privileges(accounts)?;
    update_token_stats_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCurveDataAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub curve_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCurveDataKeys {
    pub owner: Pubkey,
    pub curve_data: Pubkey,
}
impl From<UpdateCurveDataAccounts<'_, '_>> for UpdateCurveDataKeys {
    fn from(accounts: UpdateCurveDataAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            curve_data: *accounts.curve_data.key,
        }
    }
}
impl From<UpdateCurveDataKeys> for [AccountMeta; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCurveDataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_data,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN]> for UpdateCurveDataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            curve_data: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateCurveDataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCurveDataAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.curve_data.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN]>
for UpdateCurveDataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            curve_data: &arr[1],
        }
    }
}
pub const UPDATE_CURVE_DATA_IX_DISCM: [u8; 8usize] = [
    159, 128, 18, 180, 18, 190, 166, 46,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCurveDataIxArgs {
    pub start_index: u8,
    pub end_index: u8,
    pub price_data: [[[[u64; 2]; 10]; 2]; 3],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCurveDataIxData(pub UpdateCurveDataIxArgs);
impl From<UpdateCurveDataIxArgs> for UpdateCurveDataIxData {
    fn from(args: UpdateCurveDataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCurveDataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CURVE_DATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let end_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let price_data: [[[[u64; 2]; 10]; 2]; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateCurveDataIxArgs {
                start_index,
                end_index,
                price_data,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CURVE_DATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.price_data, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_curve_data_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCurveDataKeys,
    args: UpdateCurveDataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CURVE_DATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCurveDataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_curve_data_ix(
    keys: UpdateCurveDataKeys,
    args: UpdateCurveDataIxArgs,
) -> std::io::Result<Instruction> {
    update_curve_data_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn update_curve_data_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCurveDataAccounts<'_, '_>,
    args: UpdateCurveDataIxArgs,
) -> ProgramResult {
    let keys: UpdateCurveDataKeys = accounts.into();
    let ix = update_curve_data_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_curve_data_invoke(
    accounts: UpdateCurveDataAccounts<'_, '_>,
    args: UpdateCurveDataIxArgs,
) -> ProgramResult {
    update_curve_data_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts, args)
}
pub fn update_curve_data_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCurveDataAccounts<'_, '_>,
    args: UpdateCurveDataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCurveDataKeys = accounts.into();
    let ix = update_curve_data_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_curve_data_invoke_signed(
    accounts: UpdateCurveDataAccounts<'_, '_>,
    args: UpdateCurveDataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_curve_data_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_curve_data_verify_account_keys(
    accounts: UpdateCurveDataAccounts<'_, '_>,
    keys: UpdateCurveDataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.curve_data.key, keys.curve_data),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_curve_data_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCurveDataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.curve_data] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_curve_data_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCurveDataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_curve_data_verify_account_privileges<'me, 'info>(
    accounts: UpdateCurveDataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_curve_data_verify_writable_privileges(accounts)?;
    update_curve_data_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTokenListAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTokenListKeys {
    pub owner: Pubkey,
    pub token_list: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTokenListAccounts<'_, '_>> for InitializeTokenListKeys {
    fn from(accounts: InitializeTokenListAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_list: *accounts.token_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTokenListKeys>
for [AccountMeta; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTokenListKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN]> for InitializeTokenListKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_list: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeTokenListAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTokenListAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN]>
for InitializeTokenListAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            token_list: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_TOKEN_LIST_IX_DISCM: [u8; 8usize] = [
    177, 254, 236, 199, 227, 201, 142, 179,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenListIxData;
impl InitializeTokenListIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TOKEN_LIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TOKEN_LIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_token_list_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTokenListKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TOKEN_LIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeTokenListIxData.try_to_vec()?,
    })
}
pub fn initialize_token_list_ix(
    keys: InitializeTokenListKeys,
) -> std::io::Result<Instruction> {
    initialize_token_list_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn initialize_token_list_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenListAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeTokenListKeys = accounts.into();
    let ix = initialize_token_list_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_token_list_invoke(
    accounts: InitializeTokenListAccounts<'_, '_>,
) -> ProgramResult {
    initialize_token_list_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn initialize_token_list_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenListAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTokenListKeys = accounts.into();
    let ix = initialize_token_list_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_token_list_invoke_signed(
    accounts: InitializeTokenListAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_token_list_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_token_list_verify_account_keys(
    accounts: InitializeTokenListAccounts<'_, '_>,
    keys: InitializeTokenListKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_token_list_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_token_list_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_token_list_verify_account_privileges<'me, 'info>(
    accounts: InitializeTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_token_list_verify_writable_privileges(accounts)?;
    initialize_token_list_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_DATABASE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeDatabaseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub database: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeDatabaseKeys {
    pub owner: Pubkey,
    pub database: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeDatabaseAccounts<'_, '_>> for InitializeDatabaseKeys {
    fn from(accounts: InitializeDatabaseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            database: *accounts.database.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeDatabaseKeys>
for [AccountMeta; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeDatabaseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.database,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN]> for InitializeDatabaseKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            database: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeDatabaseAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeDatabaseAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.database.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN]>
for InitializeDatabaseAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            database: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_DATABASE_IX_DISCM: [u8; 8usize] = [
    210, 169, 3, 198, 98, 238, 23, 40,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeDatabaseIxData;
impl InitializeDatabaseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_DATABASE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_DATABASE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_database_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeDatabaseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_DATABASE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeDatabaseIxData.try_to_vec()?,
    })
}
pub fn initialize_database_ix(
    keys: InitializeDatabaseKeys,
) -> std::io::Result<Instruction> {
    initialize_database_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn initialize_database_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeDatabaseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeDatabaseKeys = accounts.into();
    let ix = initialize_database_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_database_invoke(
    accounts: InitializeDatabaseAccounts<'_, '_>,
) -> ProgramResult {
    initialize_database_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn initialize_database_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeDatabaseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeDatabaseKeys = accounts.into();
    let ix = initialize_database_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_database_invoke_signed(
    accounts: InitializeDatabaseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_database_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_database_verify_account_keys(
    accounts: InitializeDatabaseAccounts<'_, '_>,
    keys: InitializeDatabaseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.database.key, keys.database),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_database_verify_writable_privileges<'me, 'info>(
    accounts: InitializeDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.database] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_database_verify_signer_privileges<'me, 'info>(
    accounts: InitializeDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_database_verify_account_privileges<'me, 'info>(
    accounts: InitializeDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_database_verify_writable_privileges(accounts)?;
    initialize_database_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTokenStatsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_stats: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTokenStatsKeys {
    pub owner: Pubkey,
    pub token_stats: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeTokenStatsAccounts<'_, '_>> for InitializeTokenStatsKeys {
    fn from(accounts: InitializeTokenStatsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_stats: *accounts.token_stats.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeTokenStatsKeys>
for [AccountMeta; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTokenStatsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN]>
for InitializeTokenStatsKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_stats: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeTokenStatsAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTokenStatsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_stats.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN]>
for InitializeTokenStatsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            token_stats: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_TOKEN_STATS_IX_DISCM: [u8; 8usize] = [
    234, 129, 212, 97, 174, 172, 212, 102,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenStatsIxData;
impl InitializeTokenStatsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TOKEN_STATS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TOKEN_STATS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_token_stats_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTokenStatsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TOKEN_STATS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeTokenStatsIxData.try_to_vec()?,
    })
}
pub fn initialize_token_stats_ix(
    keys: InitializeTokenStatsKeys,
) -> std::io::Result<Instruction> {
    initialize_token_stats_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn initialize_token_stats_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenStatsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeTokenStatsKeys = accounts.into();
    let ix = initialize_token_stats_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_token_stats_invoke(
    accounts: InitializeTokenStatsAccounts<'_, '_>,
) -> ProgramResult {
    initialize_token_stats_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn initialize_token_stats_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTokenStatsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTokenStatsKeys = accounts.into();
    let ix = initialize_token_stats_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_token_stats_invoke_signed(
    accounts: InitializeTokenStatsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_token_stats_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_token_stats_verify_account_keys(
    accounts: InitializeTokenStatsAccounts<'_, '_>,
    keys: InitializeTokenStatsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_stats.key, keys.token_stats),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_token_stats_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_token_stats_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_token_stats_verify_account_privileges<'me, 'info>(
    accounts: InitializeTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_token_stats_verify_writable_privileges(accounts)?;
    initialize_token_stats_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCurveDataAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub curve_data: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCurveDataKeys {
    pub owner: Pubkey,
    pub curve_data: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeCurveDataAccounts<'_, '_>> for InitializeCurveDataKeys {
    fn from(accounts: InitializeCurveDataAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            curve_data: *accounts.curve_data.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeCurveDataKeys>
for [AccountMeta; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCurveDataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_data,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN]> for InitializeCurveDataKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            curve_data: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeCurveDataAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeCurveDataAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.curve_data.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN]>
for InitializeCurveDataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            curve_data: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_CURVE_DATA_IX_DISCM: [u8; 8usize] = [
    123, 246, 28, 169, 14, 102, 111, 189,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCurveDataIxData;
impl InitializeCurveDataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CURVE_DATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CURVE_DATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_curve_data_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCurveDataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CURVE_DATA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeCurveDataIxData.try_to_vec()?,
    })
}
pub fn initialize_curve_data_ix(
    keys: InitializeCurveDataKeys,
) -> std::io::Result<Instruction> {
    initialize_curve_data_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn initialize_curve_data_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCurveDataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeCurveDataKeys = accounts.into();
    let ix = initialize_curve_data_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_curve_data_invoke(
    accounts: InitializeCurveDataAccounts<'_, '_>,
) -> ProgramResult {
    initialize_curve_data_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn initialize_curve_data_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCurveDataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCurveDataKeys = accounts.into();
    let ix = initialize_curve_data_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_curve_data_invoke_signed(
    accounts: InitializeCurveDataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_curve_data_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_curve_data_verify_account_keys(
    accounts: InitializeCurveDataAccounts<'_, '_>,
    keys: InitializeCurveDataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.curve_data.key, keys.curve_data),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_curve_data_verify_writable_privileges<'me, 'info>(
    accounts: InitializeCurveDataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.curve_data] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_curve_data_verify_signer_privileges<'me, 'info>(
    accounts: InitializeCurveDataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_curve_data_verify_account_privileges<'me, 'info>(
    accounts: InitializeCurveDataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_curve_data_verify_writable_privileges(accounts)?;
    initialize_curve_data_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_DATABASE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseDatabaseAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub database: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseDatabaseKeys {
    pub owner: Pubkey,
    pub database: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseDatabaseAccounts<'_, '_>> for CloseDatabaseKeys {
    fn from(accounts: CloseDatabaseAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            database: *accounts.database.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseDatabaseKeys> for [AccountMeta; CLOSE_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseDatabaseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.database,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_DATABASE_IX_ACCOUNTS_LEN]> for CloseDatabaseKeys {
    fn from(pubkeys: [Pubkey; CLOSE_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            database: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CloseDatabaseAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_DATABASE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseDatabaseAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.database.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_DATABASE_IX_ACCOUNTS_LEN]>
for CloseDatabaseAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_DATABASE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            database: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLOSE_DATABASE_IX_DISCM: [u8; 8usize] = [103, 169, 4, 1, 13, 77, 6, 153];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseDatabaseIxData;
impl CloseDatabaseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_DATABASE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_DATABASE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_database_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseDatabaseKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_DATABASE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseDatabaseIxData.try_to_vec()?,
    })
}
pub fn close_database_ix(keys: CloseDatabaseKeys) -> std::io::Result<Instruction> {
    close_database_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_database_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseDatabaseAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseDatabaseKeys = accounts.into();
    let ix = close_database_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_database_invoke(accounts: CloseDatabaseAccounts<'_, '_>) -> ProgramResult {
    close_database_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_database_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseDatabaseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseDatabaseKeys = accounts.into();
    let ix = close_database_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_database_invoke_signed(
    accounts: CloseDatabaseAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_database_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_database_verify_account_keys(
    accounts: CloseDatabaseAccounts<'_, '_>,
    keys: CloseDatabaseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.database.key, keys.database),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_database_verify_writable_privileges<'me, 'info>(
    accounts: CloseDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.database] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_database_verify_signer_privileges<'me, 'info>(
    accounts: CloseDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_database_verify_account_privileges<'me, 'info>(
    accounts: CloseDatabaseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_database_verify_writable_privileges(accounts)?;
    close_database_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseTokenStatsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_stats: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTokenStatsKeys {
    pub owner: Pubkey,
    pub token_stats: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseTokenStatsAccounts<'_, '_>> for CloseTokenStatsKeys {
    fn from(accounts: CloseTokenStatsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_stats: *accounts.token_stats.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseTokenStatsKeys> for [AccountMeta; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTokenStatsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN]> for CloseTokenStatsKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_stats: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CloseTokenStatsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTokenStatsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_stats.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN]>
for CloseTokenStatsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            token_stats: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLOSE_TOKEN_STATS_IX_DISCM: [u8; 8usize] = [
    59, 235, 178, 110, 116, 224, 231, 75,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTokenStatsIxData;
impl CloseTokenStatsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TOKEN_STATS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TOKEN_STATS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_token_stats_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTokenStatsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TOKEN_STATS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseTokenStatsIxData.try_to_vec()?,
    })
}
pub fn close_token_stats_ix(keys: CloseTokenStatsKeys) -> std::io::Result<Instruction> {
    close_token_stats_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_token_stats_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenStatsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseTokenStatsKeys = accounts.into();
    let ix = close_token_stats_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_token_stats_invoke(
    accounts: CloseTokenStatsAccounts<'_, '_>,
) -> ProgramResult {
    close_token_stats_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_token_stats_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenStatsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTokenStatsKeys = accounts.into();
    let ix = close_token_stats_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_token_stats_invoke_signed(
    accounts: CloseTokenStatsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_token_stats_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_token_stats_verify_account_keys(
    accounts: CloseTokenStatsAccounts<'_, '_>,
    keys: CloseTokenStatsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_stats.key, keys.token_stats),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_token_stats_verify_writable_privileges<'me, 'info>(
    accounts: CloseTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_token_stats_verify_signer_privileges<'me, 'info>(
    accounts: CloseTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_token_stats_verify_account_privileges<'me, 'info>(
    accounts: CloseTokenStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_token_stats_verify_writable_privileges(accounts)?;
    close_token_stats_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseTokenInfoAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_info: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTokenInfoKeys {
    pub owner: Pubkey,
    pub token_info: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseTokenInfoAccounts<'_, '_>> for CloseTokenInfoKeys {
    fn from(accounts: CloseTokenInfoAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_info: *accounts.token_info.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseTokenInfoKeys> for [AccountMeta; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTokenInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN]> for CloseTokenInfoKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_info: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CloseTokenInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTokenInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_info.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN]>
for CloseTokenInfoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            token_info: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLOSE_TOKEN_INFO_IX_DISCM: [u8; 8usize] = [104, 64, 9, 62, 102, 170, 61, 109];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTokenInfoIxData;
impl CloseTokenInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TOKEN_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TOKEN_INFO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_token_info_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTokenInfoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TOKEN_INFO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseTokenInfoIxData.try_to_vec()?,
    })
}
pub fn close_token_info_ix(keys: CloseTokenInfoKeys) -> std::io::Result<Instruction> {
    close_token_info_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_token_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenInfoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseTokenInfoKeys = accounts.into();
    let ix = close_token_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_token_info_invoke(
    accounts: CloseTokenInfoAccounts<'_, '_>,
) -> ProgramResult {
    close_token_info_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_token_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTokenInfoKeys = accounts.into();
    let ix = close_token_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_token_info_invoke_signed(
    accounts: CloseTokenInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_token_info_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_token_info_verify_account_keys(
    accounts: CloseTokenInfoAccounts<'_, '_>,
    keys: CloseTokenInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_info.key, keys.token_info),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_token_info_verify_writable_privileges<'me, 'info>(
    accounts: CloseTokenInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_info] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_token_info_verify_signer_privileges<'me, 'info>(
    accounts: CloseTokenInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_token_info_verify_account_privileges<'me, 'info>(
    accounts: CloseTokenInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_token_info_verify_writable_privileges(accounts)?;
    close_token_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseTokenListAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTokenListKeys {
    pub owner: Pubkey,
    pub token_list: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseTokenListAccounts<'_, '_>> for CloseTokenListKeys {
    fn from(accounts: CloseTokenListAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            token_list: *accounts.token_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseTokenListKeys> for [AccountMeta; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTokenListKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN]> for CloseTokenListKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            token_list: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CloseTokenListAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTokenListAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.token_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN]>
for CloseTokenListAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            token_list: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLOSE_TOKEN_LIST_IX_DISCM: [u8; 8usize] = [57, 90, 77, 95, 117, 161, 100, 222];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTokenListIxData;
impl CloseTokenListIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TOKEN_LIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TOKEN_LIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_token_list_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTokenListKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TOKEN_LIST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseTokenListIxData.try_to_vec()?,
    })
}
pub fn close_token_list_ix(keys: CloseTokenListKeys) -> std::io::Result<Instruction> {
    close_token_list_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_token_list_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenListAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseTokenListKeys = accounts.into();
    let ix = close_token_list_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_token_list_invoke(
    accounts: CloseTokenListAccounts<'_, '_>,
) -> ProgramResult {
    close_token_list_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_token_list_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenListAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTokenListKeys = accounts.into();
    let ix = close_token_list_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_token_list_invoke_signed(
    accounts: CloseTokenListAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_token_list_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_token_list_verify_account_keys(
    accounts: CloseTokenListAccounts<'_, '_>,
    keys: CloseTokenListKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_token_list_verify_writable_privileges<'me, 'info>(
    accounts: CloseTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_token_list_verify_signer_privileges<'me, 'info>(
    accounts: CloseTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_token_list_verify_account_privileges<'me, 'info>(
    accounts: CloseTokenListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_token_list_verify_writable_privileges(accounts)?;
    close_token_list_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_TOKEN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseTokenAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseTokenKeys {
    pub owner: Pubkey,
    pub pda_account: Pubkey,
    pub token_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<CloseTokenAccounts<'_, '_>> for CloseTokenKeys {
    fn from(accounts: CloseTokenAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pda_account: *accounts.pda_account.key,
            token_account: *accounts.token_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CloseTokenKeys> for [AccountMeta; CLOSE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_TOKEN_IX_ACCOUNTS_LEN]> for CloseTokenKeys {
    fn from(pubkeys: [Pubkey; CLOSE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pda_account: pubkeys[1],
            token_account: pubkeys[2],
            system_program: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pda_account.clone(),
            accounts.token_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_TOKEN_IX_ACCOUNTS_LEN]>
for CloseTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pda_account: &arr[1],
            token_account: &arr[2],
            system_program: &arr[3],
            token_program: &arr[4],
        }
    }
}
pub const CLOSE_TOKEN_IX_DISCM: [u8; 8usize] = [26, 74, 236, 151, 104, 64, 183, 249];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseTokenIxData;
impl CloseTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_token_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseTokenIxData.try_to_vec()?,
    })
}
pub fn close_token_ix(keys: CloseTokenKeys) -> std::io::Result<Instruction> {
    close_token_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseTokenKeys = accounts.into();
    let ix = close_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_token_invoke(accounts: CloseTokenAccounts<'_, '_>) -> ProgramResult {
    close_token_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseTokenKeys = accounts.into();
    let ix = close_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_token_invoke_signed(
    accounts: CloseTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_token_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_token_verify_account_keys(
    accounts: CloseTokenAccounts<'_, '_>,
    keys: CloseTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.token_account.key, keys.token_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_token_verify_writable_privileges<'me, 'info>(
    accounts: CloseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_token_verify_signer_privileges<'me, 'info>(
    accounts: CloseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_token_verify_account_privileges<'me, 'info>(
    accounts: CloseTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_token_verify_writable_privileges(accounts)?;
    close_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CloseOpenOrdersAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub open_orders: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub serum_dex: &'me AccountInfo<'info>,
    pub serum_swap_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersKeys {
    pub owner: Pubkey,
    pub pda_account: Pubkey,
    pub open_orders: Pubkey,
    pub market: Pubkey,
    pub serum_dex: Pubkey,
    pub serum_swap_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseOpenOrdersAccounts<'_, '_>> for CloseOpenOrdersKeys {
    fn from(accounts: CloseOpenOrdersAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pda_account: *accounts.pda_account.key,
            open_orders: *accounts.open_orders.key,
            market: *accounts.market.key,
            serum_dex: *accounts.serum_dex.key,
            serum_swap_program: *accounts.serum_swap_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseOpenOrdersKeys> for [AccountMeta; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseOpenOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_swap_program,
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
impl From<[Pubkey; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]> for CloseOpenOrdersKeys {
    fn from(pubkeys: [Pubkey; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pda_account: pubkeys[1],
            open_orders: pubkeys[2],
            market: pubkeys[3],
            serum_dex: pubkeys[4],
            serum_swap_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<CloseOpenOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseOpenOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pda_account.clone(),
            accounts.open_orders.clone(),
            accounts.market.clone(),
            accounts.serum_dex.clone(),
            accounts.serum_swap_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]>
for CloseOpenOrdersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pda_account: &arr[1],
            open_orders: &arr[2],
            market: &arr[3],
            serum_dex: &arr[4],
            serum_swap_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const CLOSE_OPEN_ORDERS_IX_DISCM: [u8; 8usize] = [
    200, 216, 63, 239, 7, 230, 255, 20,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseOpenOrdersIxData;
impl CloseOpenOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_OPEN_ORDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_OPEN_ORDERS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_open_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseOpenOrdersKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_OPEN_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseOpenOrdersIxData.try_to_vec()?,
    })
}
pub fn close_open_orders_ix(keys: CloseOpenOrdersKeys) -> std::io::Result<Instruction> {
    close_open_orders_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys)
}
pub fn close_open_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseOpenOrdersKeys = accounts.into();
    let ix = close_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_open_orders_invoke(
    accounts: CloseOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    close_open_orders_invoke_with_program_id(SYMMETRY_PROGRAM_ID, accounts)
}
pub fn close_open_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseOpenOrdersKeys = accounts.into();
    let ix = close_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_open_orders_invoke_signed(
    accounts: CloseOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_open_orders_invoke_signed_with_program_id(SYMMETRY_PROGRAM_ID, accounts, seeds)
}
pub fn close_open_orders_verify_account_keys(
    accounts: CloseOpenOrdersAccounts<'_, '_>,
    keys: CloseOpenOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pda_account.key, keys.pda_account),
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.market.key, keys.market),
        (*accounts.serum_dex.key, keys.serum_dex),
        (*accounts.serum_swap_program.key, keys.serum_swap_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_open_orders_verify_writable_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.pda_account,
        accounts.open_orders,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_open_orders_verify_signer_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_open_orders_verify_account_privileges<'me, 'info>(
    accounts: CloseOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_open_orders_verify_writable_privileges(accounts)?;
    close_open_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateFundTokenMintMetadataAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub update_authority: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFundTokenMintMetadataKeys {
    pub manager: Pubkey,
    pub fund_token: Pubkey,
    pub fund_state: Pubkey,
    pub update_authority: Pubkey,
    pub metadata_account: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateFundTokenMintMetadataAccounts<'_, '_>>
for CreateFundTokenMintMetadataKeys {
    fn from(accounts: CreateFundTokenMintMetadataAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fund_token: *accounts.fund_token.key,
            fund_state: *accounts.fund_state.key,
            update_authority: *accounts.update_authority.key,
            metadata_account: *accounts.metadata_account.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateFundTokenMintMetadataKeys>
for [AccountMeta; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFundTokenMintMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.update_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN]>
for CreateFundTokenMintMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fund_token: pubkeys[1],
            fund_state: pubkeys[2],
            update_authority: pubkeys[3],
            metadata_account: pubkeys[4],
            metadata_program: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<CreateFundTokenMintMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFundTokenMintMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fund_token.clone(),
            accounts.fund_state.clone(),
            accounts.update_authority.clone(),
            accounts.metadata_account.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN]>
for CreateFundTokenMintMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            fund_token: &arr[1],
            fund_state: &arr[2],
            update_authority: &arr[3],
            metadata_account: &arr[4],
            metadata_program: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const CREATE_FUND_TOKEN_MINT_METADATA_IX_DISCM: [u8; 8usize] = [
    3, 229, 115, 126, 144, 130, 135, 7,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFundTokenMintMetadataIxArgs {
    pub params: UpdateMetadataParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFundTokenMintMetadataIxData(pub CreateFundTokenMintMetadataIxArgs);
impl From<CreateFundTokenMintMetadataIxArgs> for CreateFundTokenMintMetadataIxData {
    fn from(args: CreateFundTokenMintMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl CreateFundTokenMintMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FUND_TOKEN_MINT_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateMetadataParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateFundTokenMintMetadataIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FUND_TOKEN_MINT_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_fund_token_mint_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFundTokenMintMetadataKeys,
    args: CreateFundTokenMintMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateFundTokenMintMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_fund_token_mint_metadata_ix(
    keys: CreateFundTokenMintMetadataKeys,
    args: CreateFundTokenMintMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_fund_token_mint_metadata_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn create_fund_token_mint_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFundTokenMintMetadataAccounts<'_, '_>,
    args: CreateFundTokenMintMetadataIxArgs,
) -> ProgramResult {
    let keys: CreateFundTokenMintMetadataKeys = accounts.into();
    let ix = create_fund_token_mint_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_fund_token_mint_metadata_invoke(
    accounts: CreateFundTokenMintMetadataAccounts<'_, '_>,
    args: CreateFundTokenMintMetadataIxArgs,
) -> ProgramResult {
    create_fund_token_mint_metadata_invoke_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_fund_token_mint_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFundTokenMintMetadataAccounts<'_, '_>,
    args: CreateFundTokenMintMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFundTokenMintMetadataKeys = accounts.into();
    let ix = create_fund_token_mint_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_fund_token_mint_metadata_invoke_signed(
    accounts: CreateFundTokenMintMetadataAccounts<'_, '_>,
    args: CreateFundTokenMintMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_fund_token_mint_metadata_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_fund_token_mint_metadata_verify_account_keys(
    accounts: CreateFundTokenMintMetadataAccounts<'_, '_>,
    keys: CreateFundTokenMintMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.update_authority.key, keys.update_authority),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_fund_token_mint_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateFundTokenMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.fund_state,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_fund_token_mint_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateFundTokenMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_fund_token_mint_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateFundTokenMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_fund_token_mint_metadata_verify_writable_privileges(accounts)?;
    create_fund_token_mint_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFundTokenMintMetadataAccounts<'me, 'info> {
    pub manager: &'me AccountInfo<'info>,
    pub fund_token: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub update_authority: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFundTokenMintMetadataKeys {
    pub manager: Pubkey,
    pub fund_token: Pubkey,
    pub fund_state: Pubkey,
    pub update_authority: Pubkey,
    pub metadata_account: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<UpdateFundTokenMintMetadataAccounts<'_, '_>>
for UpdateFundTokenMintMetadataKeys {
    fn from(accounts: UpdateFundTokenMintMetadataAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fund_token: *accounts.fund_token.key,
            fund_state: *accounts.fund_state.key,
            update_authority: *accounts.update_authority.key,
            metadata_account: *accounts.metadata_account.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<UpdateFundTokenMintMetadataKeys>
for [AccountMeta; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFundTokenMintMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fund_token,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.update_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN]>
for UpdateFundTokenMintMetadataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fund_token: pubkeys[1],
            fund_state: pubkeys[2],
            update_authority: pubkeys[3],
            metadata_account: pubkeys[4],
            metadata_program: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<UpdateFundTokenMintMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFundTokenMintMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fund_token.clone(),
            accounts.fund_state.clone(),
            accounts.update_authority.clone(),
            accounts.metadata_account.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN]>
for UpdateFundTokenMintMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            manager: &arr[0],
            fund_token: &arr[1],
            fund_state: &arr[2],
            update_authority: &arr[3],
            metadata_account: &arr[4],
            metadata_program: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
pub const UPDATE_FUND_TOKEN_MINT_METADATA_IX_DISCM: [u8; 8usize] = [
    209, 33, 201, 11, 12, 124, 34, 54,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFundTokenMintMetadataIxArgs {
    pub params: UpdateMetadataParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFundTokenMintMetadataIxData(pub UpdateFundTokenMintMetadataIxArgs);
impl From<UpdateFundTokenMintMetadataIxArgs> for UpdateFundTokenMintMetadataIxData {
    fn from(args: UpdateFundTokenMintMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFundTokenMintMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FUND_TOKEN_MINT_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateMetadataParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateFundTokenMintMetadataIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FUND_TOKEN_MINT_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fund_token_mint_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFundTokenMintMetadataKeys,
    args: UpdateFundTokenMintMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FUND_TOKEN_MINT_METADATA_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateFundTokenMintMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fund_token_mint_metadata_ix(
    keys: UpdateFundTokenMintMetadataKeys,
    args: UpdateFundTokenMintMetadataIxArgs,
) -> std::io::Result<Instruction> {
    update_fund_token_mint_metadata_ix_with_program_id(SYMMETRY_PROGRAM_ID, keys, args)
}
pub fn update_fund_token_mint_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFundTokenMintMetadataAccounts<'_, '_>,
    args: UpdateFundTokenMintMetadataIxArgs,
) -> ProgramResult {
    let keys: UpdateFundTokenMintMetadataKeys = accounts.into();
    let ix = update_fund_token_mint_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fund_token_mint_metadata_invoke(
    accounts: UpdateFundTokenMintMetadataAccounts<'_, '_>,
    args: UpdateFundTokenMintMetadataIxArgs,
) -> ProgramResult {
    update_fund_token_mint_metadata_invoke_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_fund_token_mint_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFundTokenMintMetadataAccounts<'_, '_>,
    args: UpdateFundTokenMintMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFundTokenMintMetadataKeys = accounts.into();
    let ix = update_fund_token_mint_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fund_token_mint_metadata_invoke_signed(
    accounts: UpdateFundTokenMintMetadataAccounts<'_, '_>,
    args: UpdateFundTokenMintMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fund_token_mint_metadata_invoke_signed_with_program_id(
        SYMMETRY_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_fund_token_mint_metadata_verify_account_keys(
    accounts: UpdateFundTokenMintMetadataAccounts<'_, '_>,
    keys: UpdateFundTokenMintMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.manager.key, keys.manager),
        (*accounts.fund_token.key, keys.fund_token),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.update_authority.key, keys.update_authority),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fund_token_mint_metadata_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFundTokenMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.manager,
        accounts.fund_state,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fund_token_mint_metadata_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFundTokenMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fund_token_mint_metadata_verify_account_privileges<'me, 'info>(
    accounts: UpdateFundTokenMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fund_token_mint_metadata_verify_writable_privileges(accounts)?;
    update_fund_token_mint_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
