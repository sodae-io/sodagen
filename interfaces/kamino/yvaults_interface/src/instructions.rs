use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum YvaultsProgramIx {
    InitializeStrategy(InitializeStrategyIxArgs),
    InitializeKaminoReward(InitializeKaminoRewardIxArgs),
    AddKaminoRewards(AddKaminoRewardsIxArgs),
    InitializeGlobalConfig,
    InitializeCollateralInfo,
    UpdateCollateralInfo(UpdateCollateralInfoIxArgs),
    InsertCollateralInfo(InsertCollateralInfoIxArgs),
    InitializeSharesMetadata(InitializeSharesMetadataIxArgs),
    UpdateSharesMetadata(UpdateSharesMetadataIxArgs),
    UpdateGlobalConfig(UpdateGlobalConfigIxArgs),
    AcceptGlobalConfigOwnership,
    UpdateTreasuryFeeVault(UpdateTreasuryFeeVaultIxArgs),
    UpdateStrategyConfig(UpdateStrategyConfigIxArgs),
    SetRefTickIndexPrice(SetRefTickIndexPriceIxArgs),
    UpdateRewardMapping(UpdateRewardMappingIxArgs),
    OpenLiquidityPosition(OpenLiquidityPositionIxArgs),
    CloseStrategy,
    Deposit(DepositIxArgs),
    Invest(InvestIxArgs),
    DepositAndInvest(DepositAndInvestIxArgs),
    Withdraw(WithdrawIxArgs),
    ExecutiveWithdraw(ExecutiveWithdrawIxArgs),
    CollectFeesAndRewards,
    SwapRewards(SwapRewardsIxArgs),
    CheckExpectedVaultsBalances(CheckExpectedVaultsBalancesIxArgs),
    SingleTokenDepositAndInvestWithMin(SingleTokenDepositAndInvestWithMinIxArgs),
    SingleTokenDepositWithMin(SingleTokenDepositWithMinIxArgs),
    FlashSwapUnevenVaultsStart(FlashSwapUnevenVaultsStartIxArgs),
    FlashSwapUnevenVaultsEnd(FlashSwapUnevenVaultsEndIxArgs),
    EmergencySwap(EmergencySwapIxArgs),
    WithdrawFromTreasury(WithdrawFromTreasuryIxArgs),
    PermisionlessWithdrawFromTreasury,
    WithdrawFromTopup(WithdrawFromTopupIxArgs),
    ChangePool,
    CloseProgramAccount,
    OrcaSwap(OrcaSwapIxArgs),
    SignTerms(SignTermsIxArgs),
    UpdateStrategyAdmin,
    ResizeTokenInfos,
    DeprecateCollateralInfo(DeprecateCollateralInfoIxArgs),
    DeprecateStrategy,
    ResetStrategyPadding,
    GetKtokenPrice,
}
impl YvaultsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INITIALIZE_STRATEGY_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_STRATEGY_IX_DISCM.len()..];
            let strategy_type: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_a_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_b_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeStrategy(InitializeStrategyIxArgs {
                    strategy_type,
                    token_a_collateral_id,
                    token_b_collateral_id,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_KAMINO_REWARD_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_KAMINO_REWARD_IX_DISCM.len()..];
            let kamino_reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let collateral_token: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeKaminoReward(InitializeKaminoRewardIxArgs {
                    kamino_reward_index,
                    collateral_token,
                }),
            );
        }
        if buf.starts_with(&ADD_KAMINO_REWARDS_IX_DISCM) {
            let mut reader = &buf[ADD_KAMINO_REWARDS_IX_DISCM.len()..];
            let kamino_reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AddKaminoRewards(AddKaminoRewardsIxArgs {
                    kamino_reward_index,
                    amount,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM) {
            return Ok(Self::InitializeGlobalConfig);
        }
        if buf.starts_with(&INITIALIZE_COLLATERAL_INFO_IX_DISCM) {
            return Ok(Self::InitializeCollateralInfo);
        }
        if buf.starts_with(&UPDATE_COLLATERAL_INFO_IX_DISCM) {
            let mut reader = &buf[UPDATE_COLLATERAL_INFO_IX_DISCM.len()..];
            let index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let mode: u64 = crate::borsh_de_or_default(&mut reader)?;
            let value: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateCollateralInfo(UpdateCollateralInfoIxArgs {
                    index,
                    mode,
                    value,
                }),
            );
        }
        if buf.starts_with(&INSERT_COLLATERAL_INFO_IX_DISCM) {
            let mut reader = &buf[INSERT_COLLATERAL_INFO_IX_DISCM.len()..];
            let index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CollateralInfoParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InsertCollateralInfo(InsertCollateralInfoIxArgs {
                    index,
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_SHARES_METADATA_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_SHARES_METADATA_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeSharesMetadata(InitializeSharesMetadataIxArgs {
                    name,
                    symbol,
                    uri,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SHARES_METADATA_IX_DISCM) {
            let mut reader = &buf[UPDATE_SHARES_METADATA_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateSharesMetadata(UpdateSharesMetadataIxArgs {
                    name,
                    symbol,
                    uri,
                }),
            );
        }
        if buf.starts_with(&UPDATE_GLOBAL_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_GLOBAL_CONFIG_IX_DISCM.len()..];
            let key: u16 = crate::borsh_de_or_default(&mut reader)?;
            let index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let value: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateGlobalConfig(UpdateGlobalConfigIxArgs {
                    key,
                    index,
                    value,
                }),
            );
        }
        if buf.starts_with(&ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_DISCM) {
            return Ok(Self::AcceptGlobalConfigOwnership);
        }
        if buf.starts_with(&UPDATE_TREASURY_FEE_VAULT_IX_DISCM) {
            let mut reader = &buf[UPDATE_TREASURY_FEE_VAULT_IX_DISCM.len()..];
            let collateral_id: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateTreasuryFeeVault(UpdateTreasuryFeeVaultIxArgs {
                    collateral_id,
                }),
            );
        }
        if buf.starts_with(&UPDATE_STRATEGY_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_STRATEGY_CONFIG_IX_DISCM.len()..];
            let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
            let value = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateStrategyConfig(UpdateStrategyConfigIxArgs {
                    mode,
                    value,
                }),
            );
        }
        if buf.starts_with(&SET_REF_TICK_INDEX_PRICE_IX_DISCM) {
            let mut reader = &buf[SET_REF_TICK_INDEX_PRICE_IX_DISCM.len()..];
            let ref_tick_index_price: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetRefTickIndexPrice(SetRefTickIndexPriceIxArgs {
                    ref_tick_index_price,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REWARD_MAPPING_IX_DISCM) {
            let mut reader = &buf[UPDATE_REWARD_MAPPING_IX_DISCM.len()..];
            let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
            let collateral_token: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRewardMapping(UpdateRewardMappingIxArgs {
                    reward_index,
                    collateral_token,
                }),
            );
        }
        if buf.starts_with(&OPEN_LIQUIDITY_POSITION_IX_DISCM) {
            let mut reader = &buf[OPEN_LIQUIDITY_POSITION_IX_DISCM.len()..];
            let tick_lower_index: i64 = crate::borsh_de_or_default(&mut reader)?;
            let tick_upper_index: i64 = crate::borsh_de_or_default(&mut reader)?;
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OpenLiquidityPosition(OpenLiquidityPositionIxArgs {
                    tick_lower_index,
                    tick_upper_index,
                    bump,
                }),
            );
        }
        if buf.starts_with(&CLOSE_STRATEGY_IX_DISCM) {
            return Ok(Self::CloseStrategy);
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let token_max_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_max_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    token_max_a,
                    token_max_b,
                }),
            );
        }
        if buf.starts_with(&INVEST_IX_DISCM) {
            let mut reader = &buf[INVEST_IX_DISCM.len()..];
            let reference_price_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Invest(InvestIxArgs {
                    reference_price_tick,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_AND_INVEST_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_AND_INVEST_IX_DISCM.len()..];
            let token_max_a: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_max_b: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositAndInvest(DepositAndInvestIxArgs {
                    token_max_a,
                    token_max_b,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let shares_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::Withdraw(WithdrawIxArgs { shares_amount }));
        }
        if buf.starts_with(&EXECUTIVE_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[EXECUTIVE_WITHDRAW_IX_DISCM.len()..];
            let action: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ExecutiveWithdraw(ExecutiveWithdrawIxArgs { action }));
        }
        if buf.starts_with(&COLLECT_FEES_AND_REWARDS_IX_DISCM) {
            return Ok(Self::CollectFeesAndRewards);
        }
        if buf.starts_with(&SWAP_REWARDS_IX_DISCM) {
            let mut reader = &buf[SWAP_REWARDS_IX_DISCM.len()..];
            let token_a_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_b_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
            let reward_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_collateral_token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapRewards(SwapRewardsIxArgs {
                    token_a_in,
                    token_b_in,
                    reward_index,
                    reward_collateral_id,
                    min_collateral_token_out,
                }),
            );
        }
        if buf.starts_with(&CHECK_EXPECTED_VAULTS_BALANCES_IX_DISCM) {
            let mut reader = &buf[CHECK_EXPECTED_VAULTS_BALANCES_IX_DISCM.len()..];
            let token_a_ata_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token_b_ata_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CheckExpectedVaultsBalances(CheckExpectedVaultsBalancesIxArgs {
                    token_a_ata_balance,
                    token_b_ata_balance,
                }),
            );
        }
        if buf.starts_with(&SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_DISCM) {
            let mut reader = &buf[SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_DISCM
                .len()..];
            let token_a_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_b_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SingleTokenDepositAndInvestWithMin(SingleTokenDepositAndInvestWithMinIxArgs {
                    token_a_min_post_deposit_balance,
                    token_b_min_post_deposit_balance,
                }),
            );
        }
        if buf.starts_with(&SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_DISCM) {
            let mut reader = &buf[SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_DISCM.len()..];
            let token_a_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_b_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SingleTokenDepositWithMin(SingleTokenDepositWithMinIxArgs {
                    token_a_min_post_deposit_balance,
                    token_b_min_post_deposit_balance,
                }),
            );
        }
        if buf.starts_with(&FLASH_SWAP_UNEVEN_VAULTS_START_IX_DISCM) {
            let mut reader = &buf[FLASH_SWAP_UNEVEN_VAULTS_START_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FlashSwapUnevenVaultsStart(FlashSwapUnevenVaultsStartIxArgs {
                    amount,
                    a_to_b,
                }),
            );
        }
        if buf.starts_with(&FLASH_SWAP_UNEVEN_VAULTS_END_IX_DISCM) {
            let mut reader = &buf[FLASH_SWAP_UNEVEN_VAULTS_END_IX_DISCM.len()..];
            let min_repay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_to_leave_to_user: u64 = crate::borsh_de_or_default(&mut reader)?;
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::FlashSwapUnevenVaultsEnd(FlashSwapUnevenVaultsEndIxArgs {
                    min_repay_amount,
                    amount_to_leave_to_user,
                    a_to_b,
                }),
            );
        }
        if buf.starts_with(&EMERGENCY_SWAP_IX_DISCM) {
            let mut reader = &buf[EMERGENCY_SWAP_IX_DISCM.len()..];
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            let target_limit_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::EmergencySwap(EmergencySwapIxArgs {
                    a_to_b,
                    target_limit_bps,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FROM_TREASURY_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_TREASURY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawFromTreasury(WithdrawFromTreasuryIxArgs {
                    amount,
                }),
            );
        }
        if buf.starts_with(&PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_DISCM) {
            return Ok(Self::PermisionlessWithdrawFromTreasury);
        }
        if buf.starts_with(&WITHDRAW_FROM_TOPUP_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_TOPUP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::WithdrawFromTopup(WithdrawFromTopupIxArgs { amount }));
        }
        if buf.starts_with(&CHANGE_POOL_IX_DISCM) {
            return Ok(Self::ChangePool);
        }
        if buf.starts_with(&CLOSE_PROGRAM_ACCOUNT_IX_DISCM) {
            return Ok(Self::CloseProgramAccount);
        }
        if buf.starts_with(&ORCA_SWAP_IX_DISCM) {
            let mut reader = &buf[ORCA_SWAP_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
            let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
            let amount_specified_is_input: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::OrcaSwap(OrcaSwapIxArgs {
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit,
                    amount_specified_is_input,
                    a_to_b,
                }),
            );
        }
        if buf.starts_with(&SIGN_TERMS_IX_DISCM) {
            let mut reader = &buf[SIGN_TERMS_IX_DISCM.len()..];
            let signature = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(Self::SignTerms(SignTermsIxArgs { signature }));
        }
        if buf.starts_with(&UPDATE_STRATEGY_ADMIN_IX_DISCM) {
            return Ok(Self::UpdateStrategyAdmin);
        }
        if buf.starts_with(&RESIZE_TOKEN_INFOS_IX_DISCM) {
            return Ok(Self::ResizeTokenInfos);
        }
        if buf.starts_with(&DEPRECATE_COLLATERAL_INFO_IX_DISCM) {
            let mut reader = &buf[DEPRECATE_COLLATERAL_INFO_IX_DISCM.len()..];
            let index: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DeprecateCollateralInfo(DeprecateCollateralInfoIxArgs {
                    index,
                }),
            );
        }
        if buf.starts_with(&DEPRECATE_STRATEGY_IX_DISCM) {
            return Ok(Self::DeprecateStrategy);
        }
        if buf.starts_with(&RESET_STRATEGY_PADDING_IX_DISCM) {
            return Ok(Self::ResetStrategyPadding);
        }
        if buf.starts_with(&GET_KTOKEN_PRICE_IX_DISCM) {
            return Ok(Self::GetKtokenPrice);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeStrategy(args) => {
                writer.write_all(&INITIALIZE_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.strategy_type, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.token_a_collateral_id,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_b_collateral_id,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeKaminoReward(args) => {
                writer.write_all(&INITIALIZE_KAMINO_REWARD_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.kamino_reward_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.collateral_token, &mut writer)?;
                Ok(())
            }
            Self::AddKaminoRewards(args) => {
                writer.write_all(&ADD_KAMINO_REWARDS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.kamino_reward_index,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::InitializeGlobalConfig => {
                writer.write_all(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM)
            }
            Self::InitializeCollateralInfo => {
                writer.write_all(&INITIALIZE_COLLATERAL_INFO_IX_DISCM)
            }
            Self::UpdateCollateralInfo(args) => {
                writer.write_all(&UPDATE_COLLATERAL_INFO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::InsertCollateralInfo(args) => {
                writer.write_all(&INSERT_COLLATERAL_INFO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeSharesMetadata(args) => {
                writer.write_all(&INITIALIZE_SHARES_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::UpdateSharesMetadata(args) => {
                writer.write_all(&UPDATE_SHARES_METADATA_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                Ok(())
            }
            Self::UpdateGlobalConfig(args) => {
                writer.write_all(&UPDATE_GLOBAL_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.key, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::AcceptGlobalConfigOwnership => {
                writer.write_all(&ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_DISCM)
            }
            Self::UpdateTreasuryFeeVault(args) => {
                writer.write_all(&UPDATE_TREASURY_FEE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.collateral_id, &mut writer)?;
                Ok(())
            }
            Self::UpdateStrategyConfig(args) => {
                writer.write_all(&UPDATE_STRATEGY_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.mode, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::SetRefTickIndexPrice(args) => {
                writer.write_all(&SET_REF_TICK_INDEX_PRICE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.ref_tick_index_price,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateRewardMapping(args) => {
                writer.write_all(&UPDATE_REWARD_MAPPING_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.collateral_token, &mut writer)?;
                Ok(())
            }
            Self::OpenLiquidityPosition(args) => {
                writer.write_all(&OPEN_LIQUIDITY_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.tick_lower_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.tick_upper_index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CloseStrategy => writer.write_all(&CLOSE_STRATEGY_IX_DISCM),
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_max_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_max_b, &mut writer)?;
                Ok(())
            }
            Self::Invest(args) => {
                writer.write_all(&INVEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.reference_price_tick,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::DepositAndInvest(args) => {
                writer.write_all(&DEPOSIT_AND_INVEST_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_max_a, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_max_b, &mut writer)?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares_amount, &mut writer)?;
                Ok(())
            }
            Self::ExecutiveWithdraw(args) => {
                writer.write_all(&EXECUTIVE_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.action, &mut writer)?;
                Ok(())
            }
            Self::CollectFeesAndRewards => {
                writer.write_all(&COLLECT_FEES_AND_REWARDS_IX_DISCM)
            }
            Self::SwapRewards(args) => {
                writer.write_all(&SWAP_REWARDS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_a_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token_b_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.reward_index, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.reward_collateral_id,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.min_collateral_token_out,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::CheckExpectedVaultsBalances(args) => {
                writer.write_all(&CHECK_EXPECTED_VAULTS_BALANCES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.token_a_ata_balance,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_b_ata_balance,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SingleTokenDepositAndInvestWithMin(args) => {
                writer.write_all(&SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.token_a_min_post_deposit_balance,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_b_min_post_deposit_balance,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SingleTokenDepositWithMin(args) => {
                writer.write_all(&SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.token_a_min_post_deposit_balance,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_b_min_post_deposit_balance,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::FlashSwapUnevenVaultsStart(args) => {
                writer.write_all(&FLASH_SWAP_UNEVEN_VAULTS_START_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                Ok(())
            }
            Self::FlashSwapUnevenVaultsEnd(args) => {
                writer.write_all(&FLASH_SWAP_UNEVEN_VAULTS_END_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.min_repay_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.amount_to_leave_to_user,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                Ok(())
            }
            Self::EmergencySwap(args) => {
                writer.write_all(&EMERGENCY_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.target_limit_bps, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromTreasury(args) => {
                writer.write_all(&WITHDRAW_FROM_TREASURY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::PermisionlessWithdrawFromTreasury => {
                writer.write_all(&PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_DISCM)
            }
            Self::WithdrawFromTopup(args) => {
                writer.write_all(&WITHDRAW_FROM_TOPUP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                Ok(())
            }
            Self::ChangePool => writer.write_all(&CHANGE_POOL_IX_DISCM),
            Self::CloseProgramAccount => {
                writer.write_all(&CLOSE_PROGRAM_ACCOUNT_IX_DISCM)
            }
            Self::OrcaSwap(args) => {
                writer.write_all(&ORCA_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.other_amount_threshold,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.sqrt_price_limit, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.amount_specified_is_input,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.a_to_b, &mut writer)?;
                Ok(())
            }
            Self::SignTerms(args) => {
                writer.write_all(&SIGN_TERMS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.signature, &mut writer)?;
                Ok(())
            }
            Self::UpdateStrategyAdmin => {
                writer.write_all(&UPDATE_STRATEGY_ADMIN_IX_DISCM)
            }
            Self::ResizeTokenInfos => writer.write_all(&RESIZE_TOKEN_INFOS_IX_DISCM),
            Self::DeprecateCollateralInfo(args) => {
                writer.write_all(&DEPRECATE_COLLATERAL_INFO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                Ok(())
            }
            Self::DeprecateStrategy => writer.write_all(&DEPRECATE_STRATEGY_IX_DISCM),
            Self::ResetStrategyPadding => {
                writer.write_all(&RESET_STRATEGY_PADDING_IX_DISCM)
            }
            Self::GetKtokenPrice => writer.write_all(&GET_KTOKEN_PRICE_IX_DISCM),
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
pub const INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct InitializeStrategyAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeStrategyKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub token_infos: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub strategy: Pubkey,
}
impl From<InitializeStrategyAccounts<'_, '_>> for InitializeStrategyKeys {
    fn from(accounts: InitializeStrategyAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            shares_mint: *accounts.shares_mint.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            token_infos: *accounts.token_infos.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            strategy: *accounts.strategy.key,
        }
    }
}
impl From<InitializeStrategyKeys>
for [AccountMeta; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_infos,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]> for InitializeStrategyKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            pool: pubkeys[2],
            token_a_mint: pubkeys[3],
            token_b_mint: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            base_vault_authority: pubkeys[7],
            shares_mint: pubkeys[8],
            shares_mint_authority: pubkeys[9],
            token_infos: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
            token_program: pubkeys[13],
            token_a_token_program: pubkeys[14],
            token_b_token_program: pubkeys[15],
            strategy: pubkeys[16],
        }
    }
}
impl<'info> From<InitializeStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.token_infos.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.strategy.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN]>
for InitializeStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            pool: &arr[2],
            token_a_mint: &arr[3],
            token_b_mint: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            base_vault_authority: &arr[7],
            shares_mint: &arr[8],
            shares_mint_authority: &arr[9],
            token_infos: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
            token_program: &arr[13],
            token_a_token_program: &arr[14],
            token_b_token_program: &arr[15],
            strategy: &arr[16],
        }
    }
}
pub const INITIALIZE_STRATEGY_IX_DISCM: [u8; 8usize] = [
    208, 119, 144, 145, 178, 57, 105, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeStrategyIxArgs {
    pub strategy_type: u64,
    pub token_a_collateral_id: u64,
    pub token_b_collateral_id: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeStrategyIxData(pub InitializeStrategyIxArgs);
impl From<InitializeStrategyIxArgs> for InitializeStrategyIxData {
    fn from(args: InitializeStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let strategy_type: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeStrategyIxArgs {
                strategy_type,
                token_a_collateral_id,
                token_b_collateral_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.strategy_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_a_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_b_collateral_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeStrategyKeys,
    args: InitializeStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_strategy_ix(
    keys: InitializeStrategyKeys,
    args: InitializeStrategyIxArgs,
) -> std::io::Result<Instruction> {
    initialize_strategy_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn initialize_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
) -> ProgramResult {
    let keys: InitializeStrategyKeys = accounts.into();
    let ix = initialize_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_strategy_invoke(
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
) -> ProgramResult {
    initialize_strategy_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn initialize_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeStrategyKeys = accounts.into();
    let ix = initialize_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_strategy_invoke_signed(
    accounts: InitializeStrategyAccounts<'_, '_>,
    args: InitializeStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_strategy_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_strategy_verify_account_keys(
    accounts: InitializeStrategyAccounts<'_, '_>,
    keys: InitializeStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.strategy.key, keys.strategy),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_strategy_verify_writable_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.base_vault_authority,
        accounts.shares_mint,
        accounts.shares_mint_authority,
        accounts.strategy,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_strategy_verify_signer_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_strategy_verify_account_privileges<'me, 'info>(
    accounts: InitializeStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_strategy_verify_writable_privileges(accounts)?;
    initialize_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeKaminoRewardAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKaminoRewardKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub token_infos: Pubkey,
    pub base_vault_authority: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitializeKaminoRewardAccounts<'_, '_>> for InitializeKaminoRewardKeys {
    fn from(accounts: InitializeKaminoRewardAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            token_infos: *accounts.token_infos.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializeKaminoRewardKeys>
for [AccountMeta; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKaminoRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN]>
for InitializeKaminoRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            reward_mint: pubkeys[3],
            reward_vault: pubkeys[4],
            token_infos: pubkeys[5],
            base_vault_authority: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitializeKaminoRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeKaminoRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.token_infos.clone(),
            accounts.base_vault_authority.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN]>
for InitializeKaminoRewardAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            reward_mint: &arr[3],
            reward_vault: &arr[4],
            token_infos: &arr[5],
            base_vault_authority: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const INITIALIZE_KAMINO_REWARD_IX_DISCM: [u8; 8usize] = [
    203, 212, 8, 90, 91, 118, 111, 50,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeKaminoRewardIxArgs {
    pub kamino_reward_index: u64,
    pub collateral_token: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeKaminoRewardIxData(pub InitializeKaminoRewardIxArgs);
impl From<InitializeKaminoRewardIxArgs> for InitializeKaminoRewardIxData {
    fn from(args: InitializeKaminoRewardIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeKaminoRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_KAMINO_REWARD_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kamino_reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeKaminoRewardIxArgs {
                kamino_reward_index,
                collateral_token,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_KAMINO_REWARD_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kamino_reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_token, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_kamino_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKaminoRewardKeys,
    args: InitializeKaminoRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_KAMINO_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeKaminoRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_kamino_reward_ix(
    keys: InitializeKaminoRewardKeys,
    args: InitializeKaminoRewardIxArgs,
) -> std::io::Result<Instruction> {
    initialize_kamino_reward_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn initialize_kamino_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeKaminoRewardAccounts<'_, '_>,
    args: InitializeKaminoRewardIxArgs,
) -> ProgramResult {
    let keys: InitializeKaminoRewardKeys = accounts.into();
    let ix = initialize_kamino_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_kamino_reward_invoke(
    accounts: InitializeKaminoRewardAccounts<'_, '_>,
    args: InitializeKaminoRewardIxArgs,
) -> ProgramResult {
    initialize_kamino_reward_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn initialize_kamino_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeKaminoRewardAccounts<'_, '_>,
    args: InitializeKaminoRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKaminoRewardKeys = accounts.into();
    let ix = initialize_kamino_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_kamino_reward_invoke_signed(
    accounts: InitializeKaminoRewardAccounts<'_, '_>,
    args: InitializeKaminoRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_kamino_reward_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_kamino_reward_verify_account_keys(
    accounts: InitializeKaminoRewardAccounts<'_, '_>,
    keys: InitializeKaminoRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_kamino_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeKaminoRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.strategy,
        accounts.reward_vault,
        accounts.base_vault_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_kamino_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeKaminoRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority, accounts.reward_vault] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_kamino_reward_verify_account_privileges<'me, 'info>(
    accounts: InitializeKaminoRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_kamino_reward_verify_writable_privileges(accounts)?;
    initialize_kamino_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct AddKaminoRewardsAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub reward_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddKaminoRewardsKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub reward_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddKaminoRewardsAccounts<'_, '_>> for AddKaminoRewardsKeys {
    fn from(accounts: AddKaminoRewardsAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            reward_ata: *accounts.reward_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddKaminoRewardsKeys> for [AccountMeta; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: AddKaminoRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_ata,
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
impl From<[Pubkey; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN]> for AddKaminoRewardsKeys {
    fn from(pubkeys: [Pubkey; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            reward_mint: pubkeys[3],
            reward_vault: pubkeys[4],
            base_vault_authority: pubkeys[5],
            reward_ata: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<AddKaminoRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddKaminoRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.reward_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN]>
for AddKaminoRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            reward_mint: &arr[3],
            reward_vault: &arr[4],
            base_vault_authority: &arr[5],
            reward_ata: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const ADD_KAMINO_REWARDS_IX_DISCM: [u8; 8usize] = [
    174, 174, 142, 193, 47, 77, 235, 65,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddKaminoRewardsIxArgs {
    pub kamino_reward_index: u64,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddKaminoRewardsIxData(pub AddKaminoRewardsIxArgs);
impl From<AddKaminoRewardsIxArgs> for AddKaminoRewardsIxData {
    fn from(args: AddKaminoRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl AddKaminoRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_KAMINO_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let kamino_reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AddKaminoRewardsIxArgs {
                kamino_reward_index,
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_KAMINO_REWARDS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.kamino_reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_kamino_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: AddKaminoRewardsKeys,
    args: AddKaminoRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_KAMINO_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddKaminoRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_kamino_rewards_ix(
    keys: AddKaminoRewardsKeys,
    args: AddKaminoRewardsIxArgs,
) -> std::io::Result<Instruction> {
    add_kamino_rewards_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn add_kamino_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddKaminoRewardsAccounts<'_, '_>,
    args: AddKaminoRewardsIxArgs,
) -> ProgramResult {
    let keys: AddKaminoRewardsKeys = accounts.into();
    let ix = add_kamino_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_kamino_rewards_invoke(
    accounts: AddKaminoRewardsAccounts<'_, '_>,
    args: AddKaminoRewardsIxArgs,
) -> ProgramResult {
    add_kamino_rewards_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn add_kamino_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddKaminoRewardsAccounts<'_, '_>,
    args: AddKaminoRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddKaminoRewardsKeys = accounts.into();
    let ix = add_kamino_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_kamino_rewards_invoke_signed(
    accounts: AddKaminoRewardsAccounts<'_, '_>,
    args: AddKaminoRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_kamino_rewards_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_kamino_rewards_verify_account_keys(
    accounts: AddKaminoRewardsAccounts<'_, '_>,
    keys: AddKaminoRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.reward_ata.key, keys.reward_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_kamino_rewards_verify_writable_privileges<'me, 'info>(
    accounts: AddKaminoRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.strategy,
        accounts.reward_vault,
        accounts.base_vault_authority,
        accounts.reward_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_kamino_rewards_verify_signer_privileges<'me, 'info>(
    accounts: AddKaminoRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_kamino_rewards_verify_account_privileges<'me, 'info>(
    accounts: AddKaminoRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_kamino_rewards_verify_writable_privileges(accounts)?;
    add_kamino_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeGlobalConfigAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeGlobalConfigKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeGlobalConfigAccounts<'_, '_>> for InitializeGlobalConfigKeys {
    fn from(accounts: InitializeGlobalConfigAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeGlobalConfigKeys>
for [AccountMeta; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeGlobalConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
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
impl From<[Pubkey; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeGlobalConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeGlobalConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeGlobalConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeGlobalConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_GLOBAL_CONFIG_IX_DISCM: [u8; 8usize] = [
    113, 216, 122, 131, 225, 209, 22, 55,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGlobalConfigIxData;
impl InitializeGlobalConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_GLOBAL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_GLOBAL_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_global_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeGlobalConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeGlobalConfigIxData.try_to_vec()?,
    })
}
pub fn initialize_global_config_ix(
    keys: InitializeGlobalConfigKeys,
) -> std::io::Result<Instruction> {
    initialize_global_config_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn initialize_global_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeGlobalConfigKeys = accounts.into();
    let ix = initialize_global_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_global_config_invoke(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
) -> ProgramResult {
    initialize_global_config_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn initialize_global_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeGlobalConfigKeys = accounts.into();
    let ix = initialize_global_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_global_config_invoke_signed(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_global_config_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_global_config_verify_account_keys(
    accounts: InitializeGlobalConfigAccounts<'_, '_>,
    keys: InitializeGlobalConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_global_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_global_config_verify_writable_privileges(accounts)?;
    initialize_global_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCollateralInfoAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub coll_info: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCollateralInfoKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub coll_info: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeCollateralInfoAccounts<'_, '_>> for InitializeCollateralInfoKeys {
    fn from(accounts: InitializeCollateralInfoAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            coll_info: *accounts.coll_info.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeCollateralInfoKeys>
for [AccountMeta; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCollateralInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coll_info,
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
impl From<[Pubkey; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for InitializeCollateralInfoKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            coll_info: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeCollateralInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeCollateralInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.coll_info.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for InitializeCollateralInfoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            coll_info: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_COLLATERAL_INFO_IX_DISCM: [u8; 8usize] = [
    74, 61, 216, 76, 244, 91, 18, 119,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCollateralInfoIxData;
impl InitializeCollateralInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_COLLATERAL_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_COLLATERAL_INFO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_collateral_info_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCollateralInfoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeCollateralInfoIxData.try_to_vec()?,
    })
}
pub fn initialize_collateral_info_ix(
    keys: InitializeCollateralInfoKeys,
) -> std::io::Result<Instruction> {
    initialize_collateral_info_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn initialize_collateral_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCollateralInfoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeCollateralInfoKeys = accounts.into();
    let ix = initialize_collateral_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_collateral_info_invoke(
    accounts: InitializeCollateralInfoAccounts<'_, '_>,
) -> ProgramResult {
    initialize_collateral_info_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn initialize_collateral_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCollateralInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCollateralInfoKeys = accounts.into();
    let ix = initialize_collateral_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_collateral_info_invoke_signed(
    accounts: InitializeCollateralInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_collateral_info_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_collateral_info_verify_account_keys(
    accounts: InitializeCollateralInfoAccounts<'_, '_>,
    keys: InitializeCollateralInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.coll_info.key, keys.coll_info),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_collateral_info_verify_writable_privileges<'me, 'info>(
    accounts: InitializeCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.global_config,
        accounts.coll_info,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_collateral_info_verify_signer_privileges<'me, 'info>(
    accounts: InitializeCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_collateral_info_verify_account_privileges<'me, 'info>(
    accounts: InitializeCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_collateral_info_verify_writable_privileges(accounts)?;
    initialize_collateral_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCollateralInfoAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCollateralInfoKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub token_infos: Pubkey,
}
impl From<UpdateCollateralInfoAccounts<'_, '_>> for UpdateCollateralInfoKeys {
    fn from(accounts: UpdateCollateralInfoAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            token_infos: *accounts.token_infos.key,
        }
    }
}
impl From<UpdateCollateralInfoKeys>
for [AccountMeta; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCollateralInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for UpdateCollateralInfoKeys {
    fn from(pubkeys: [Pubkey; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            token_infos: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateCollateralInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCollateralInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.token_infos.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for UpdateCollateralInfoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            token_infos: &arr[2],
        }
    }
}
pub const UPDATE_COLLATERAL_INFO_IX_DISCM: [u8; 8usize] = [
    76, 94, 131, 44, 137, 61, 161, 110,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCollateralInfoIxArgs {
    pub index: u64,
    pub mode: u64,
    pub value: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCollateralInfoIxData(pub UpdateCollateralInfoIxArgs);
impl From<UpdateCollateralInfoIxArgs> for UpdateCollateralInfoIxData {
    fn from(args: UpdateCollateralInfoIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCollateralInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_COLLATERAL_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mode: u64 = crate::borsh_de_or_default(&mut reader)?;
        let value: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateCollateralInfoIxArgs {
                index,
                mode,
                value,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_COLLATERAL_INFO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_collateral_info_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCollateralInfoKeys,
    args: UpdateCollateralInfoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCollateralInfoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_collateral_info_ix(
    keys: UpdateCollateralInfoKeys,
    args: UpdateCollateralInfoIxArgs,
) -> std::io::Result<Instruction> {
    update_collateral_info_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn update_collateral_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCollateralInfoAccounts<'_, '_>,
    args: UpdateCollateralInfoIxArgs,
) -> ProgramResult {
    let keys: UpdateCollateralInfoKeys = accounts.into();
    let ix = update_collateral_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_collateral_info_invoke(
    accounts: UpdateCollateralInfoAccounts<'_, '_>,
    args: UpdateCollateralInfoIxArgs,
) -> ProgramResult {
    update_collateral_info_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn update_collateral_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCollateralInfoAccounts<'_, '_>,
    args: UpdateCollateralInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCollateralInfoKeys = accounts.into();
    let ix = update_collateral_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_collateral_info_invoke_signed(
    accounts: UpdateCollateralInfoAccounts<'_, '_>,
    args: UpdateCollateralInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_collateral_info_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_collateral_info_verify_account_keys(
    accounts: UpdateCollateralInfoAccounts<'_, '_>,
    keys: UpdateCollateralInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_infos.key, keys.token_infos),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_collateral_info_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.token_infos] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_collateral_info_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_collateral_info_verify_account_privileges<'me, 'info>(
    accounts: UpdateCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_collateral_info_verify_writable_privileges(accounts)?;
    update_collateral_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InsertCollateralInfoAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InsertCollateralInfoKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub token_infos: Pubkey,
}
impl From<InsertCollateralInfoAccounts<'_, '_>> for InsertCollateralInfoKeys {
    fn from(accounts: InsertCollateralInfoAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            token_infos: *accounts.token_infos.key,
        }
    }
}
impl From<InsertCollateralInfoKeys>
for [AccountMeta; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: InsertCollateralInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for InsertCollateralInfoKeys {
    fn from(pubkeys: [Pubkey; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            token_infos: pubkeys[2],
        }
    }
}
impl<'info> From<InsertCollateralInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: InsertCollateralInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.token_infos.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for InsertCollateralInfoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            token_infos: &arr[2],
        }
    }
}
pub const INSERT_COLLATERAL_INFO_IX_DISCM: [u8; 8usize] = [
    22, 97, 4, 78, 166, 188, 51, 190,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertCollateralInfoIxArgs {
    pub index: u64,
    pub params: CollateralInfoParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsertCollateralInfoIxData(pub InsertCollateralInfoIxArgs);
impl From<InsertCollateralInfoIxArgs> for InsertCollateralInfoIxData {
    fn from(args: InsertCollateralInfoIxArgs) -> Self {
        Self(args)
    }
}
impl InsertCollateralInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSERT_COLLATERAL_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CollateralInfoParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InsertCollateralInfoIxArgs {
                index,
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSERT_COLLATERAL_INFO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn insert_collateral_info_ix_with_program_id(
    program_id: Pubkey,
    keys: InsertCollateralInfoKeys,
    args: InsertCollateralInfoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INSERT_COLLATERAL_INFO_IX_ACCOUNTS_LEN] = keys.into();
    let data: InsertCollateralInfoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn insert_collateral_info_ix(
    keys: InsertCollateralInfoKeys,
    args: InsertCollateralInfoIxArgs,
) -> std::io::Result<Instruction> {
    insert_collateral_info_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn insert_collateral_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InsertCollateralInfoAccounts<'_, '_>,
    args: InsertCollateralInfoIxArgs,
) -> ProgramResult {
    let keys: InsertCollateralInfoKeys = accounts.into();
    let ix = insert_collateral_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn insert_collateral_info_invoke(
    accounts: InsertCollateralInfoAccounts<'_, '_>,
    args: InsertCollateralInfoIxArgs,
) -> ProgramResult {
    insert_collateral_info_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn insert_collateral_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InsertCollateralInfoAccounts<'_, '_>,
    args: InsertCollateralInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InsertCollateralInfoKeys = accounts.into();
    let ix = insert_collateral_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn insert_collateral_info_invoke_signed(
    accounts: InsertCollateralInfoAccounts<'_, '_>,
    args: InsertCollateralInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    insert_collateral_info_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn insert_collateral_info_verify_account_keys(
    accounts: InsertCollateralInfoAccounts<'_, '_>,
    keys: InsertCollateralInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_infos.key, keys.token_infos),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn insert_collateral_info_verify_writable_privileges<'me, 'info>(
    accounts: InsertCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.token_infos] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn insert_collateral_info_verify_signer_privileges<'me, 'info>(
    accounts: InsertCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn insert_collateral_info_verify_account_privileges<'me, 'info>(
    accounts: InsertCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    insert_collateral_info_verify_writable_privileges(accounts)?;
    insert_collateral_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeSharesMetadataAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_metadata: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeSharesMetadataKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_metadata: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<InitializeSharesMetadataAccounts<'_, '_>> for InitializeSharesMetadataKeys {
    fn from(accounts: InitializeSharesMetadataAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            shares_mint: *accounts.shares_mint.key,
            shares_metadata: *accounts.shares_metadata.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<InitializeSharesMetadataKeys>
for [AccountMeta; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeSharesMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.shares_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
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
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN]>
for InitializeSharesMetadataKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            shares_mint: pubkeys[3],
            shares_metadata: pubkeys[4],
            shares_mint_authority: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
            metadata_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeSharesMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeSharesMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_metadata.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN]>
for InitializeSharesMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            shares_mint: &arr[3],
            shares_metadata: &arr[4],
            shares_mint_authority: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
            metadata_program: &arr[8],
        }
    }
}
pub const INITIALIZE_SHARES_METADATA_IX_DISCM: [u8; 8usize] = [
    3, 15, 172, 114, 200, 0, 131, 32,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeSharesMetadataIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeSharesMetadataIxData(pub InitializeSharesMetadataIxArgs);
impl From<InitializeSharesMetadataIxArgs> for InitializeSharesMetadataIxData {
    fn from(args: InitializeSharesMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeSharesMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_SHARES_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeSharesMetadataIxArgs {
                name,
                symbol,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_SHARES_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_shares_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeSharesMetadataKeys,
    args: InitializeSharesMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_SHARES_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeSharesMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_shares_metadata_ix(
    keys: InitializeSharesMetadataKeys,
    args: InitializeSharesMetadataIxArgs,
) -> std::io::Result<Instruction> {
    initialize_shares_metadata_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn initialize_shares_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeSharesMetadataAccounts<'_, '_>,
    args: InitializeSharesMetadataIxArgs,
) -> ProgramResult {
    let keys: InitializeSharesMetadataKeys = accounts.into();
    let ix = initialize_shares_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_shares_metadata_invoke(
    accounts: InitializeSharesMetadataAccounts<'_, '_>,
    args: InitializeSharesMetadataIxArgs,
) -> ProgramResult {
    initialize_shares_metadata_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn initialize_shares_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeSharesMetadataAccounts<'_, '_>,
    args: InitializeSharesMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeSharesMetadataKeys = accounts.into();
    let ix = initialize_shares_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_shares_metadata_invoke_signed(
    accounts: InitializeSharesMetadataAccounts<'_, '_>,
    args: InitializeSharesMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_shares_metadata_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_shares_metadata_verify_account_keys(
    accounts: InitializeSharesMetadataAccounts<'_, '_>,
    keys: InitializeSharesMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_metadata.key, keys.shares_metadata),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_shares_metadata_verify_writable_privileges<'me, 'info>(
    accounts: InitializeSharesMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.shares_metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_shares_metadata_verify_signer_privileges<'me, 'info>(
    accounts: InitializeSharesMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_shares_metadata_verify_account_privileges<'me, 'info>(
    accounts: InitializeSharesMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_shares_metadata_verify_writable_privileges(accounts)?;
    initialize_shares_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSharesMetadataAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_metadata: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSharesMetadataKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_metadata: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<UpdateSharesMetadataAccounts<'_, '_>> for UpdateSharesMetadataKeys {
    fn from(accounts: UpdateSharesMetadataAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            shares_mint: *accounts.shares_mint.key,
            shares_metadata: *accounts.shares_metadata.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<UpdateSharesMetadataKeys>
for [AccountMeta; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSharesMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.shares_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN]>
for UpdateSharesMetadataKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            shares_mint: pubkeys[3],
            shares_metadata: pubkeys[4],
            shares_mint_authority: pubkeys[5],
            metadata_program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateSharesMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSharesMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_metadata.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN]>
for UpdateSharesMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            shares_mint: &arr[3],
            shares_metadata: &arr[4],
            shares_mint_authority: &arr[5],
            metadata_program: &arr[6],
        }
    }
}
pub const UPDATE_SHARES_METADATA_IX_DISCM: [u8; 8usize] = [
    155, 34, 122, 165, 245, 137, 147, 107,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateSharesMetadataIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSharesMetadataIxData(pub UpdateSharesMetadataIxArgs);
impl From<UpdateSharesMetadataIxArgs> for UpdateSharesMetadataIxData {
    fn from(args: UpdateSharesMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSharesMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SHARES_METADATA_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateSharesMetadataIxArgs {
                name,
                symbol,
                uri,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SHARES_METADATA_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_shares_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSharesMetadataKeys,
    args: UpdateSharesMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SHARES_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateSharesMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_shares_metadata_ix(
    keys: UpdateSharesMetadataKeys,
    args: UpdateSharesMetadataIxArgs,
) -> std::io::Result<Instruction> {
    update_shares_metadata_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn update_shares_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSharesMetadataAccounts<'_, '_>,
    args: UpdateSharesMetadataIxArgs,
) -> ProgramResult {
    let keys: UpdateSharesMetadataKeys = accounts.into();
    let ix = update_shares_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_shares_metadata_invoke(
    accounts: UpdateSharesMetadataAccounts<'_, '_>,
    args: UpdateSharesMetadataIxArgs,
) -> ProgramResult {
    update_shares_metadata_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn update_shares_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSharesMetadataAccounts<'_, '_>,
    args: UpdateSharesMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSharesMetadataKeys = accounts.into();
    let ix = update_shares_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_shares_metadata_invoke_signed(
    accounts: UpdateSharesMetadataAccounts<'_, '_>,
    args: UpdateSharesMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_shares_metadata_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_shares_metadata_verify_account_keys(
    accounts: UpdateSharesMetadataAccounts<'_, '_>,
    keys: UpdateSharesMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_metadata.key, keys.shares_metadata),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_shares_metadata_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSharesMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.shares_metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_shares_metadata_verify_signer_privileges<'me, 'info>(
    accounts: UpdateSharesMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_shares_metadata_verify_account_privileges<'me, 'info>(
    accounts: UpdateSharesMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_shares_metadata_verify_writable_privileges(accounts)?;
    update_shares_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalConfigAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateGlobalConfigAccounts<'_, '_>> for UpdateGlobalConfigKeys {
    fn from(accounts: UpdateGlobalConfigAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateGlobalConfigKeys>
for [AccountMeta; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
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
impl From<[Pubkey; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]> for UpdateGlobalConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateGlobalConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateGlobalConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const UPDATE_GLOBAL_CONFIG_IX_DISCM: [u8; 8usize] = [
    164, 84, 130, 189, 111, 58, 250, 200,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateGlobalConfigIxArgs {
    pub key: u16,
    pub index: u16,
    pub value: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalConfigIxData(pub UpdateGlobalConfigIxArgs);
impl From<UpdateGlobalConfigIxArgs> for UpdateGlobalConfigIxData {
    fn from(args: UpdateGlobalConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateGlobalConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GLOBAL_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let key: u16 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let value: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateGlobalConfigIxArgs {
                key,
                index,
                value,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_global_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalConfigKeys,
    args: UpdateGlobalConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateGlobalConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_global_config_ix(
    keys: UpdateGlobalConfigKeys,
    args: UpdateGlobalConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_global_config_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn update_global_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateGlobalConfigKeys = accounts.into();
    let ix = update_global_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_global_config_invoke(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
) -> ProgramResult {
    update_global_config_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn update_global_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGlobalConfigKeys = accounts.into();
    let ix = update_global_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_global_config_invoke_signed(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    args: UpdateGlobalConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_config_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_global_config_verify_account_keys(
    accounts: UpdateGlobalConfigAccounts<'_, '_>,
    keys: UpdateGlobalConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_global_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_config_verify_writable_privileges(accounts)?;
    update_global_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AcceptGlobalConfigOwnershipAccounts<'me, 'info> {
    pub pending_admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AcceptGlobalConfigOwnershipKeys {
    pub pending_admin: Pubkey,
    pub global_config: Pubkey,
}
impl From<AcceptGlobalConfigOwnershipAccounts<'_, '_>>
for AcceptGlobalConfigOwnershipKeys {
    fn from(accounts: AcceptGlobalConfigOwnershipAccounts) -> Self {
        Self {
            pending_admin: *accounts.pending_admin.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<AcceptGlobalConfigOwnershipKeys>
for [AccountMeta; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(keys: AcceptGlobalConfigOwnershipKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for AcceptGlobalConfigOwnershipKeys {
    fn from(pubkeys: [Pubkey; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_admin: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<AcceptGlobalConfigOwnershipAccounts<'_, 'info>>
for [AccountInfo<'info>; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] {
    fn from(accounts: AcceptGlobalConfigOwnershipAccounts<'_, 'info>) -> Self {
        [accounts.pending_admin.clone(), accounts.global_config.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN]>
for AcceptGlobalConfigOwnershipAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pending_admin: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_DISCM: [u8; 8usize] = [
    209, 63, 231, 151, 188, 204, 0, 151,
];
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptGlobalConfigOwnershipIxData;
impl AcceptGlobalConfigOwnershipIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn accept_global_config_ownership_ix_with_program_id(
    program_id: Pubkey,
    keys: AcceptGlobalConfigOwnershipKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACCEPT_GLOBAL_CONFIG_OWNERSHIP_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AcceptGlobalConfigOwnershipIxData.try_to_vec()?,
    })
}
pub fn accept_global_config_ownership_ix(
    keys: AcceptGlobalConfigOwnershipKeys,
) -> std::io::Result<Instruction> {
    accept_global_config_ownership_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn accept_global_config_ownership_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AcceptGlobalConfigOwnershipAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AcceptGlobalConfigOwnershipKeys = accounts.into();
    let ix = accept_global_config_ownership_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn accept_global_config_ownership_invoke(
    accounts: AcceptGlobalConfigOwnershipAccounts<'_, '_>,
) -> ProgramResult {
    accept_global_config_ownership_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn accept_global_config_ownership_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AcceptGlobalConfigOwnershipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AcceptGlobalConfigOwnershipKeys = accounts.into();
    let ix = accept_global_config_ownership_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn accept_global_config_ownership_invoke_signed(
    accounts: AcceptGlobalConfigOwnershipAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    accept_global_config_ownership_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn accept_global_config_ownership_verify_account_keys(
    accounts: AcceptGlobalConfigOwnershipAccounts<'_, '_>,
    keys: AcceptGlobalConfigOwnershipKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_admin.key, keys.pending_admin),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn accept_global_config_ownership_verify_writable_privileges<'me, 'info>(
    accounts: AcceptGlobalConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pending_admin, accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn accept_global_config_ownership_verify_signer_privileges<'me, 'info>(
    accounts: AcceptGlobalConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn accept_global_config_ownership_verify_account_privileges<'me, 'info>(
    accounts: AcceptGlobalConfigOwnershipAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    accept_global_config_ownership_verify_writable_privileges(accounts)?;
    accept_global_config_ownership_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTreasuryFeeVaultAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub fee_mint: &'me AccountInfo<'info>,
    pub treasury_fee_vault: &'me AccountInfo<'info>,
    pub treasury_fee_vault_authority: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTreasuryFeeVaultKeys {
    pub signer: Pubkey,
    pub global_config: Pubkey,
    pub fee_mint: Pubkey,
    pub treasury_fee_vault: Pubkey,
    pub treasury_fee_vault_authority: Pubkey,
    pub token_infos: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<UpdateTreasuryFeeVaultAccounts<'_, '_>> for UpdateTreasuryFeeVaultKeys {
    fn from(accounts: UpdateTreasuryFeeVaultAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            global_config: *accounts.global_config.key,
            fee_mint: *accounts.fee_mint.key,
            treasury_fee_vault: *accounts.treasury_fee_vault.key,
            treasury_fee_vault_authority: *accounts.treasury_fee_vault_authority.key,
            token_infos: *accounts.token_infos.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateTreasuryFeeVaultKeys>
for [AccountMeta; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTreasuryFeeVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN]>
for UpdateTreasuryFeeVaultKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            global_config: pubkeys[1],
            fee_mint: pubkeys[2],
            treasury_fee_vault: pubkeys[3],
            treasury_fee_vault_authority: pubkeys[4],
            token_infos: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<UpdateTreasuryFeeVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTreasuryFeeVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.global_config.clone(),
            accounts.fee_mint.clone(),
            accounts.treasury_fee_vault.clone(),
            accounts.treasury_fee_vault_authority.clone(),
            accounts.token_infos.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN]>
for UpdateTreasuryFeeVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            global_config: &arr[1],
            fee_mint: &arr[2],
            treasury_fee_vault: &arr[3],
            treasury_fee_vault_authority: &arr[4],
            token_infos: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const UPDATE_TREASURY_FEE_VAULT_IX_DISCM: [u8; 8usize] = [
    9, 241, 94, 91, 173, 74, 166, 119,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateTreasuryFeeVaultIxArgs {
    pub collateral_id: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTreasuryFeeVaultIxData(pub UpdateTreasuryFeeVaultIxArgs);
impl From<UpdateTreasuryFeeVaultIxArgs> for UpdateTreasuryFeeVaultIxData {
    fn from(args: UpdateTreasuryFeeVaultIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTreasuryFeeVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TREASURY_FEE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let collateral_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateTreasuryFeeVaultIxArgs {
                collateral_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TREASURY_FEE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_treasury_fee_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTreasuryFeeVaultKeys,
    args: UpdateTreasuryFeeVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TREASURY_FEE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTreasuryFeeVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_treasury_fee_vault_ix(
    keys: UpdateTreasuryFeeVaultKeys,
    args: UpdateTreasuryFeeVaultIxArgs,
) -> std::io::Result<Instruction> {
    update_treasury_fee_vault_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn update_treasury_fee_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTreasuryFeeVaultAccounts<'_, '_>,
    args: UpdateTreasuryFeeVaultIxArgs,
) -> ProgramResult {
    let keys: UpdateTreasuryFeeVaultKeys = accounts.into();
    let ix = update_treasury_fee_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_treasury_fee_vault_invoke(
    accounts: UpdateTreasuryFeeVaultAccounts<'_, '_>,
    args: UpdateTreasuryFeeVaultIxArgs,
) -> ProgramResult {
    update_treasury_fee_vault_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn update_treasury_fee_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTreasuryFeeVaultAccounts<'_, '_>,
    args: UpdateTreasuryFeeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTreasuryFeeVaultKeys = accounts.into();
    let ix = update_treasury_fee_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_treasury_fee_vault_invoke_signed(
    accounts: UpdateTreasuryFeeVaultAccounts<'_, '_>,
    args: UpdateTreasuryFeeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_treasury_fee_vault_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_treasury_fee_vault_verify_account_keys(
    accounts: UpdateTreasuryFeeVaultAccounts<'_, '_>,
    keys: UpdateTreasuryFeeVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.fee_mint.key, keys.fee_mint),
        (*accounts.treasury_fee_vault.key, keys.treasury_fee_vault),
        (*accounts.treasury_fee_vault_authority.key, keys.treasury_fee_vault_authority),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_treasury_fee_vault_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTreasuryFeeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.treasury_fee_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_treasury_fee_vault_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTreasuryFeeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_treasury_fee_vault_verify_account_privileges<'me, 'info>(
    accounts: UpdateTreasuryFeeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_treasury_fee_vault_verify_writable_privileges(accounts)?;
    update_treasury_fee_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateStrategyConfigAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub new_account: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateStrategyConfigKeys {
    pub admin_authority: Pubkey,
    pub new_account: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateStrategyConfigAccounts<'_, '_>> for UpdateStrategyConfigKeys {
    fn from(accounts: UpdateStrategyConfigAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            new_account: *accounts.new_account.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateStrategyConfigKeys>
for [AccountMeta; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateStrategyConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
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
impl From<[Pubkey; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateStrategyConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            new_account: pubkeys[1],
            strategy: pubkeys[2],
            global_config: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateStrategyConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateStrategyConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.new_account.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateStrategyConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            new_account: &arr[1],
            strategy: &arr[2],
            global_config: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const UPDATE_STRATEGY_CONFIG_IX_DISCM: [u8; 8usize] = [
    81, 217, 177, 65, 40, 227, 8, 165,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateStrategyConfigIxArgs {
    pub mode: u16,
    #[serde(with = "crate::big_array_serde")]
    pub value: [u8; 128],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStrategyConfigIxData(pub UpdateStrategyConfigIxArgs);
impl From<UpdateStrategyConfigIxArgs> for UpdateStrategyConfigIxData {
    fn from(args: UpdateStrategyConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateStrategyConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_STRATEGY_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let mode: u16 = crate::borsh_de_or_default(&mut reader)?;
        let value = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(UpdateStrategyConfigIxArgs {
                mode,
                value,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_STRATEGY_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_strategy_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateStrategyConfigKeys,
    args: UpdateStrategyConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_STRATEGY_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateStrategyConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_strategy_config_ix(
    keys: UpdateStrategyConfigKeys,
    args: UpdateStrategyConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_strategy_config_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn update_strategy_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStrategyConfigAccounts<'_, '_>,
    args: UpdateStrategyConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateStrategyConfigKeys = accounts.into();
    let ix = update_strategy_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_strategy_config_invoke(
    accounts: UpdateStrategyConfigAccounts<'_, '_>,
    args: UpdateStrategyConfigIxArgs,
) -> ProgramResult {
    update_strategy_config_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn update_strategy_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStrategyConfigAccounts<'_, '_>,
    args: UpdateStrategyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateStrategyConfigKeys = accounts.into();
    let ix = update_strategy_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_strategy_config_invoke_signed(
    accounts: UpdateStrategyConfigAccounts<'_, '_>,
    args: UpdateStrategyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_strategy_config_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_strategy_config_verify_account_keys(
    accounts: UpdateStrategyConfigAccounts<'_, '_>,
    keys: UpdateStrategyConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.new_account.key, keys.new_account),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_strategy_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateStrategyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.strategy] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_strategy_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateStrategyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_strategy_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateStrategyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_strategy_config_verify_writable_privileges(accounts)?;
    update_strategy_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetRefTickIndexPriceAccounts<'me, 'info> {
    pub actions_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRefTickIndexPriceKeys {
    pub actions_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<SetRefTickIndexPriceAccounts<'_, '_>> for SetRefTickIndexPriceKeys {
    fn from(accounts: SetRefTickIndexPriceAccounts) -> Self {
        Self {
            actions_authority: *accounts.actions_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<SetRefTickIndexPriceKeys>
for [AccountMeta; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRefTickIndexPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.actions_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN]>
for SetRefTickIndexPriceKeys {
    fn from(pubkeys: [Pubkey; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            actions_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            instruction_sysvar_account: pubkeys[3],
        }
    }
}
impl<'info> From<SetRefTickIndexPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRefTickIndexPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.actions_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN]>
for SetRefTickIndexPriceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            actions_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            instruction_sysvar_account: &arr[3],
        }
    }
}
pub const SET_REF_TICK_INDEX_PRICE_IX_DISCM: [u8; 8usize] = [
    18, 222, 143, 102, 211, 81, 246, 152,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetRefTickIndexPriceIxArgs {
    pub ref_tick_index_price: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRefTickIndexPriceIxData(pub SetRefTickIndexPriceIxArgs);
impl From<SetRefTickIndexPriceIxArgs> for SetRefTickIndexPriceIxData {
    fn from(args: SetRefTickIndexPriceIxArgs) -> Self {
        Self(args)
    }
}
impl SetRefTickIndexPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_REF_TICK_INDEX_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let ref_tick_index_price: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetRefTickIndexPriceIxArgs {
                ref_tick_index_price,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_REF_TICK_INDEX_PRICE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.ref_tick_index_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_ref_tick_index_price_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRefTickIndexPriceKeys,
    args: SetRefTickIndexPriceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REF_TICK_INDEX_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRefTickIndexPriceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_ref_tick_index_price_ix(
    keys: SetRefTickIndexPriceKeys,
    args: SetRefTickIndexPriceIxArgs,
) -> std::io::Result<Instruction> {
    set_ref_tick_index_price_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn set_ref_tick_index_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRefTickIndexPriceAccounts<'_, '_>,
    args: SetRefTickIndexPriceIxArgs,
) -> ProgramResult {
    let keys: SetRefTickIndexPriceKeys = accounts.into();
    let ix = set_ref_tick_index_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_ref_tick_index_price_invoke(
    accounts: SetRefTickIndexPriceAccounts<'_, '_>,
    args: SetRefTickIndexPriceIxArgs,
) -> ProgramResult {
    set_ref_tick_index_price_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn set_ref_tick_index_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRefTickIndexPriceAccounts<'_, '_>,
    args: SetRefTickIndexPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRefTickIndexPriceKeys = accounts.into();
    let ix = set_ref_tick_index_price_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_ref_tick_index_price_invoke_signed(
    accounts: SetRefTickIndexPriceAccounts<'_, '_>,
    args: SetRefTickIndexPriceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_ref_tick_index_price_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_ref_tick_index_price_verify_account_keys(
    accounts: SetRefTickIndexPriceAccounts<'_, '_>,
    keys: SetRefTickIndexPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.actions_authority.key, keys.actions_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_ref_tick_index_price_verify_writable_privileges<'me, 'info>(
    accounts: SetRefTickIndexPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.strategy] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_ref_tick_index_price_verify_signer_privileges<'me, 'info>(
    accounts: SetRefTickIndexPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.actions_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_ref_tick_index_price_verify_account_privileges<'me, 'info>(
    accounts: SetRefTickIndexPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_ref_tick_index_price_verify_writable_privileges(accounts)?;
    set_ref_tick_index_price_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardMappingAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardMappingKeys {
    pub payer: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub token_infos: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<UpdateRewardMappingAccounts<'_, '_>> for UpdateRewardMappingKeys {
    fn from(accounts: UpdateRewardMappingAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            reward_mint: *accounts.reward_mint.key,
            reward_vault: *accounts.reward_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            token_infos: *accounts.token_infos.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateRewardMappingKeys>
for [AccountMeta; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardMappingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_infos,
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
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN]> for UpdateRewardMappingKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            reward_mint: pubkeys[4],
            reward_vault: pubkeys[5],
            base_vault_authority: pubkeys[6],
            token_infos: pubkeys[7],
            system_program: pubkeys[8],
            rent: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<UpdateRewardMappingAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardMappingAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.reward_mint.clone(),
            accounts.reward_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.token_infos.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN]>
for UpdateRewardMappingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            reward_mint: &arr[4],
            reward_vault: &arr[5],
            base_vault_authority: &arr[6],
            token_infos: &arr[7],
            system_program: &arr[8],
            rent: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const UPDATE_REWARD_MAPPING_IX_DISCM: [u8; 8usize] = [
    203, 37, 37, 96, 23, 85, 233, 42,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardMappingIxArgs {
    pub reward_index: u8,
    pub collateral_token: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardMappingIxData(pub UpdateRewardMappingIxArgs);
impl From<UpdateRewardMappingIxArgs> for UpdateRewardMappingIxData {
    fn from(args: UpdateRewardMappingIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewardMappingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_MAPPING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reward_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRewardMappingIxArgs {
                reward_index,
                collateral_token,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_MAPPING_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.collateral_token, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_mapping_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardMappingKeys,
    args: UpdateRewardMappingIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_MAPPING_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewardMappingIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reward_mapping_ix(
    keys: UpdateRewardMappingKeys,
    args: UpdateRewardMappingIxArgs,
) -> std::io::Result<Instruction> {
    update_reward_mapping_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn update_reward_mapping_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardMappingAccounts<'_, '_>,
    args: UpdateRewardMappingIxArgs,
) -> ProgramResult {
    let keys: UpdateRewardMappingKeys = accounts.into();
    let ix = update_reward_mapping_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_mapping_invoke(
    accounts: UpdateRewardMappingAccounts<'_, '_>,
    args: UpdateRewardMappingIxArgs,
) -> ProgramResult {
    update_reward_mapping_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn update_reward_mapping_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardMappingAccounts<'_, '_>,
    args: UpdateRewardMappingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardMappingKeys = accounts.into();
    let ix = update_reward_mapping_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_mapping_invoke_signed(
    accounts: UpdateRewardMappingAccounts<'_, '_>,
    args: UpdateRewardMappingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_mapping_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_reward_mapping_verify_account_keys(
    accounts: UpdateRewardMappingAccounts<'_, '_>,
    keys: UpdateRewardMappingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_mapping_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardMappingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.strategy,
        accounts.reward_vault,
        accounts.base_vault_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_mapping_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardMappingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.reward_vault] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reward_mapping_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardMappingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_mapping_verify_writable_privileges(accounts)?;
    update_reward_mapping_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN: usize = 36;
#[derive(Copy, Clone, Debug)]
pub struct OpenLiquidityPositionAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_mint: &'me AccountInfo<'info>,
    pub position_metadata_account: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub old_tick_array_lower_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_tick_array_upper_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_position_mint_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_position_token_account_or_base_vault_authority: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub consensus_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenLiquidityPositionKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub base_vault_authority: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_metadata_account: Pubkey,
    pub position_token_account: Pubkey,
    pub rent: Pubkey,
    pub system: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub pool_program: Pubkey,
    pub old_tick_array_lower_or_base_vault_authority: Pubkey,
    pub old_tick_array_upper_or_base_vault_authority: Pubkey,
    pub old_position_or_base_vault_authority: Pubkey,
    pub old_position_mint_or_base_vault_authority: Pubkey,
    pub old_position_token_account_or_base_vault_authority: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub event_authority: Pubkey,
    pub consensus_account: Pubkey,
}
impl From<OpenLiquidityPositionAccounts<'_, '_>> for OpenLiquidityPositionKeys {
    fn from(accounts: OpenLiquidityPositionAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            position: *accounts.position.key,
            position_mint: *accounts.position_mint.key,
            position_metadata_account: *accounts.position_metadata_account.key,
            position_token_account: *accounts.position_token_account.key,
            rent: *accounts.rent.key,
            system: *accounts.system.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            pool_program: *accounts.pool_program.key,
            old_tick_array_lower_or_base_vault_authority: *accounts
                .old_tick_array_lower_or_base_vault_authority
                .key,
            old_tick_array_upper_or_base_vault_authority: *accounts
                .old_tick_array_upper_or_base_vault_authority
                .key,
            old_position_or_base_vault_authority: *accounts
                .old_position_or_base_vault_authority
                .key,
            old_position_mint_or_base_vault_authority: *accounts
                .old_position_mint_or_base_vault_authority
                .key,
            old_position_token_account_or_base_vault_authority: *accounts
                .old_position_token_account_or_base_vault_authority
                .key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            event_authority: *accounts.event_authority.key,
            consensus_account: *accounts.consensus_account.key,
        }
    }
}
impl From<OpenLiquidityPositionKeys>
for [AccountMeta; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenLiquidityPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_tick_array_lower_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_tick_array_upper_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_position_mint_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_position_token_account_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consensus_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN]>
for OpenLiquidityPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            tick_array_lower: pubkeys[4],
            tick_array_upper: pubkeys[5],
            base_vault_authority: pubkeys[6],
            position: pubkeys[7],
            position_mint: pubkeys[8],
            position_metadata_account: pubkeys[9],
            position_token_account: pubkeys[10],
            rent: pubkeys[11],
            system: pubkeys[12],
            token_program: pubkeys[13],
            token_program2022: pubkeys[14],
            token_a_token_program: pubkeys[15],
            token_b_token_program: pubkeys[16],
            memo_program: pubkeys[17],
            associated_token_program: pubkeys[18],
            pool_program: pubkeys[19],
            old_tick_array_lower_or_base_vault_authority: pubkeys[20],
            old_tick_array_upper_or_base_vault_authority: pubkeys[21],
            old_position_or_base_vault_authority: pubkeys[22],
            old_position_mint_or_base_vault_authority: pubkeys[23],
            old_position_token_account_or_base_vault_authority: pubkeys[24],
            token_a_vault: pubkeys[25],
            token_b_vault: pubkeys[26],
            token_a_mint: pubkeys[27],
            token_b_mint: pubkeys[28],
            pool_token_vault_a: pubkeys[29],
            pool_token_vault_b: pubkeys[30],
            scope_prices_a: pubkeys[31],
            scope_prices_b: pubkeys[32],
            token_infos: pubkeys[33],
            event_authority: pubkeys[34],
            consensus_account: pubkeys[35],
        }
    }
}
impl<'info> From<OpenLiquidityPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenLiquidityPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.base_vault_authority.clone(),
            accounts.position.clone(),
            accounts.position_mint.clone(),
            accounts.position_metadata_account.clone(),
            accounts.position_token_account.clone(),
            accounts.rent.clone(),
            accounts.system.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.pool_program.clone(),
            accounts.old_tick_array_lower_or_base_vault_authority.clone(),
            accounts.old_tick_array_upper_or_base_vault_authority.clone(),
            accounts.old_position_or_base_vault_authority.clone(),
            accounts.old_position_mint_or_base_vault_authority.clone(),
            accounts.old_position_token_account_or_base_vault_authority.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.event_authority.clone(),
            accounts.consensus_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN]>
for OpenLiquidityPositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            tick_array_lower: &arr[4],
            tick_array_upper: &arr[5],
            base_vault_authority: &arr[6],
            position: &arr[7],
            position_mint: &arr[8],
            position_metadata_account: &arr[9],
            position_token_account: &arr[10],
            rent: &arr[11],
            system: &arr[12],
            token_program: &arr[13],
            token_program2022: &arr[14],
            token_a_token_program: &arr[15],
            token_b_token_program: &arr[16],
            memo_program: &arr[17],
            associated_token_program: &arr[18],
            pool_program: &arr[19],
            old_tick_array_lower_or_base_vault_authority: &arr[20],
            old_tick_array_upper_or_base_vault_authority: &arr[21],
            old_position_or_base_vault_authority: &arr[22],
            old_position_mint_or_base_vault_authority: &arr[23],
            old_position_token_account_or_base_vault_authority: &arr[24],
            token_a_vault: &arr[25],
            token_b_vault: &arr[26],
            token_a_mint: &arr[27],
            token_b_mint: &arr[28],
            pool_token_vault_a: &arr[29],
            pool_token_vault_b: &arr[30],
            scope_prices_a: &arr[31],
            scope_prices_b: &arr[32],
            token_infos: &arr[33],
            event_authority: &arr[34],
            consensus_account: &arr[35],
        }
    }
}
pub const OPEN_LIQUIDITY_POSITION_IX_DISCM: [u8; 8usize] = [
    204, 234, 204, 219, 6, 91, 96, 241,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpenLiquidityPositionIxArgs {
    pub tick_lower_index: i64,
    pub tick_upper_index: i64,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenLiquidityPositionIxData(pub OpenLiquidityPositionIxArgs);
impl From<OpenLiquidityPositionIxArgs> for OpenLiquidityPositionIxData {
    fn from(args: OpenLiquidityPositionIxArgs) -> Self {
        Self(args)
    }
}
impl OpenLiquidityPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_LIQUIDITY_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let tick_lower_index: i64 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OpenLiquidityPositionIxArgs {
                tick_lower_index,
                tick_upper_index,
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_LIQUIDITY_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_liquidity_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenLiquidityPositionKeys,
    args: OpenLiquidityPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_LIQUIDITY_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenLiquidityPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_liquidity_position_ix(
    keys: OpenLiquidityPositionKeys,
    args: OpenLiquidityPositionIxArgs,
) -> std::io::Result<Instruction> {
    open_liquidity_position_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn open_liquidity_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenLiquidityPositionAccounts<'_, '_>,
    args: OpenLiquidityPositionIxArgs,
) -> ProgramResult {
    let keys: OpenLiquidityPositionKeys = accounts.into();
    let ix = open_liquidity_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_liquidity_position_invoke(
    accounts: OpenLiquidityPositionAccounts<'_, '_>,
    args: OpenLiquidityPositionIxArgs,
) -> ProgramResult {
    open_liquidity_position_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn open_liquidity_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenLiquidityPositionAccounts<'_, '_>,
    args: OpenLiquidityPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenLiquidityPositionKeys = accounts.into();
    let ix = open_liquidity_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_liquidity_position_invoke_signed(
    accounts: OpenLiquidityPositionAccounts<'_, '_>,
    args: OpenLiquidityPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_liquidity_position_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_liquidity_position_verify_account_keys(
    accounts: OpenLiquidityPositionAccounts<'_, '_>,
    keys: OpenLiquidityPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.position.key, keys.position),
        (*accounts.position_mint.key, keys.position_mint),
        (*accounts.position_metadata_account.key, keys.position_metadata_account),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.rent.key, keys.rent),
        (*accounts.system.key, keys.system),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.pool_program.key, keys.pool_program),
        (
            *accounts.old_tick_array_lower_or_base_vault_authority.key,
            keys.old_tick_array_lower_or_base_vault_authority,
        ),
        (
            *accounts.old_tick_array_upper_or_base_vault_authority.key,
            keys.old_tick_array_upper_or_base_vault_authority,
        ),
        (
            *accounts.old_position_or_base_vault_authority.key,
            keys.old_position_or_base_vault_authority,
        ),
        (
            *accounts.old_position_mint_or_base_vault_authority.key,
            keys.old_position_mint_or_base_vault_authority,
        ),
        (
            *accounts.old_position_token_account_or_base_vault_authority.key,
            keys.old_position_token_account_or_base_vault_authority,
        ),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.consensus_account.key, keys.consensus_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_liquidity_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenLiquidityPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.strategy,
        accounts.pool,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.base_vault_authority,
        accounts.position,
        accounts.position_mint,
        accounts.position_metadata_account,
        accounts.position_token_account,
        accounts.old_tick_array_lower_or_base_vault_authority,
        accounts.old_tick_array_upper_or_base_vault_authority,
        accounts.old_position_or_base_vault_authority,
        accounts.old_position_mint_or_base_vault_authority,
        accounts.old_position_token_account_or_base_vault_authority,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_liquidity_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenLiquidityPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_liquidity_position_verify_account_privileges<'me, 'info>(
    accounts: OpenLiquidityPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_liquidity_position_verify_writable_privileges(accounts)?;
    open_liquidity_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_STRATEGY_IX_ACCOUNTS_LEN: usize = 34;
#[derive(Copy, Clone, Debug)]
pub struct CloseStrategyAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub old_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_position_mint_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_position_token_account_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_tick_array_lower_or_base_vault_authority: &'me AccountInfo<'info>,
    pub old_tick_array_upper_or_base_vault_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub user_token_a_ata: &'me AccountInfo<'info>,
    pub user_token_b_ata: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub reward0_vault: &'me AccountInfo<'info>,
    pub reward1_vault: &'me AccountInfo<'info>,
    pub reward2_vault: &'me AccountInfo<'info>,
    pub kamino_reward0_vault: &'me AccountInfo<'info>,
    pub kamino_reward1_vault: &'me AccountInfo<'info>,
    pub kamino_reward2_vault: &'me AccountInfo<'info>,
    pub user_reward0_ata: &'me AccountInfo<'info>,
    pub user_reward1_ata: &'me AccountInfo<'info>,
    pub user_reward2_ata: &'me AccountInfo<'info>,
    pub user_kamino_reward0_ata: &'me AccountInfo<'info>,
    pub user_kamino_reward1_ata: &'me AccountInfo<'info>,
    pub user_kamino_reward2_ata: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub system: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseStrategyKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub old_position_or_base_vault_authority: Pubkey,
    pub old_position_mint_or_base_vault_authority: Pubkey,
    pub old_position_token_account_or_base_vault_authority: Pubkey,
    pub old_tick_array_lower_or_base_vault_authority: Pubkey,
    pub old_tick_array_upper_or_base_vault_authority: Pubkey,
    pub pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub user_token_a_ata: Pubkey,
    pub user_token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub reward0_vault: Pubkey,
    pub reward1_vault: Pubkey,
    pub reward2_vault: Pubkey,
    pub kamino_reward0_vault: Pubkey,
    pub kamino_reward1_vault: Pubkey,
    pub kamino_reward2_vault: Pubkey,
    pub user_reward0_ata: Pubkey,
    pub user_reward1_ata: Pubkey,
    pub user_reward2_ata: Pubkey,
    pub user_kamino_reward0_ata: Pubkey,
    pub user_kamino_reward1_ata: Pubkey,
    pub user_kamino_reward2_ata: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool_program: Pubkey,
    pub token_program: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub system: Pubkey,
    pub event_authority: Pubkey,
}
impl From<CloseStrategyAccounts<'_, '_>> for CloseStrategyKeys {
    fn from(accounts: CloseStrategyAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            old_position_or_base_vault_authority: *accounts
                .old_position_or_base_vault_authority
                .key,
            old_position_mint_or_base_vault_authority: *accounts
                .old_position_mint_or_base_vault_authority
                .key,
            old_position_token_account_or_base_vault_authority: *accounts
                .old_position_token_account_or_base_vault_authority
                .key,
            old_tick_array_lower_or_base_vault_authority: *accounts
                .old_tick_array_lower_or_base_vault_authority
                .key,
            old_tick_array_upper_or_base_vault_authority: *accounts
                .old_tick_array_upper_or_base_vault_authority
                .key,
            pool: *accounts.pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            user_token_a_ata: *accounts.user_token_a_ata.key,
            user_token_b_ata: *accounts.user_token_b_ata.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            reward0_vault: *accounts.reward0_vault.key,
            reward1_vault: *accounts.reward1_vault.key,
            reward2_vault: *accounts.reward2_vault.key,
            kamino_reward0_vault: *accounts.kamino_reward0_vault.key,
            kamino_reward1_vault: *accounts.kamino_reward1_vault.key,
            kamino_reward2_vault: *accounts.kamino_reward2_vault.key,
            user_reward0_ata: *accounts.user_reward0_ata.key,
            user_reward1_ata: *accounts.user_reward1_ata.key,
            user_reward2_ata: *accounts.user_reward2_ata.key,
            user_kamino_reward0_ata: *accounts.user_kamino_reward0_ata.key,
            user_kamino_reward1_ata: *accounts.user_kamino_reward1_ata.key,
            user_kamino_reward2_ata: *accounts.user_kamino_reward2_ata.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool_program: *accounts.pool_program.key,
            token_program: *accounts.token_program.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            system: *accounts.system.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<CloseStrategyKeys> for [AccountMeta; CLOSE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_position_mint_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_position_token_account_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_tick_array_lower_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.old_tick_array_upper_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward2_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_reward0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_reward1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kamino_reward2_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reward0_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reward1_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reward2_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_kamino_reward0_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_kamino_reward1_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_kamino_reward2_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]> for CloseStrategyKeys {
    fn from(pubkeys: [Pubkey; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            old_position_or_base_vault_authority: pubkeys[3],
            old_position_mint_or_base_vault_authority: pubkeys[4],
            old_position_token_account_or_base_vault_authority: pubkeys[5],
            old_tick_array_lower_or_base_vault_authority: pubkeys[6],
            old_tick_array_upper_or_base_vault_authority: pubkeys[7],
            pool: pubkeys[8],
            token_a_vault: pubkeys[9],
            token_b_vault: pubkeys[10],
            user_token_a_ata: pubkeys[11],
            user_token_b_ata: pubkeys[12],
            token_a_mint: pubkeys[13],
            token_b_mint: pubkeys[14],
            reward0_vault: pubkeys[15],
            reward1_vault: pubkeys[16],
            reward2_vault: pubkeys[17],
            kamino_reward0_vault: pubkeys[18],
            kamino_reward1_vault: pubkeys[19],
            kamino_reward2_vault: pubkeys[20],
            user_reward0_ata: pubkeys[21],
            user_reward1_ata: pubkeys[22],
            user_reward2_ata: pubkeys[23],
            user_kamino_reward0_ata: pubkeys[24],
            user_kamino_reward1_ata: pubkeys[25],
            user_kamino_reward2_ata: pubkeys[26],
            base_vault_authority: pubkeys[27],
            pool_program: pubkeys[28],
            token_program: pubkeys[29],
            token_a_token_program: pubkeys[30],
            token_b_token_program: pubkeys[31],
            system: pubkeys[32],
            event_authority: pubkeys[33],
        }
    }
}
impl<'info> From<CloseStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.old_position_or_base_vault_authority.clone(),
            accounts.old_position_mint_or_base_vault_authority.clone(),
            accounts.old_position_token_account_or_base_vault_authority.clone(),
            accounts.old_tick_array_lower_or_base_vault_authority.clone(),
            accounts.old_tick_array_upper_or_base_vault_authority.clone(),
            accounts.pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.user_token_a_ata.clone(),
            accounts.user_token_b_ata.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.reward0_vault.clone(),
            accounts.reward1_vault.clone(),
            accounts.reward2_vault.clone(),
            accounts.kamino_reward0_vault.clone(),
            accounts.kamino_reward1_vault.clone(),
            accounts.kamino_reward2_vault.clone(),
            accounts.user_reward0_ata.clone(),
            accounts.user_reward1_ata.clone(),
            accounts.user_reward2_ata.clone(),
            accounts.user_kamino_reward0_ata.clone(),
            accounts.user_kamino_reward1_ata.clone(),
            accounts.user_kamino_reward2_ata.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool_program.clone(),
            accounts.token_program.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.system.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]>
for CloseStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            old_position_or_base_vault_authority: &arr[3],
            old_position_mint_or_base_vault_authority: &arr[4],
            old_position_token_account_or_base_vault_authority: &arr[5],
            old_tick_array_lower_or_base_vault_authority: &arr[6],
            old_tick_array_upper_or_base_vault_authority: &arr[7],
            pool: &arr[8],
            token_a_vault: &arr[9],
            token_b_vault: &arr[10],
            user_token_a_ata: &arr[11],
            user_token_b_ata: &arr[12],
            token_a_mint: &arr[13],
            token_b_mint: &arr[14],
            reward0_vault: &arr[15],
            reward1_vault: &arr[16],
            reward2_vault: &arr[17],
            kamino_reward0_vault: &arr[18],
            kamino_reward1_vault: &arr[19],
            kamino_reward2_vault: &arr[20],
            user_reward0_ata: &arr[21],
            user_reward1_ata: &arr[22],
            user_reward2_ata: &arr[23],
            user_kamino_reward0_ata: &arr[24],
            user_kamino_reward1_ata: &arr[25],
            user_kamino_reward2_ata: &arr[26],
            base_vault_authority: &arr[27],
            pool_program: &arr[28],
            token_program: &arr[29],
            token_a_token_program: &arr[30],
            token_b_token_program: &arr[31],
            system: &arr[32],
            event_authority: &arr[33],
        }
    }
}
pub const CLOSE_STRATEGY_IX_DISCM: [u8; 8usize] = [56, 247, 170, 246, 89, 221, 134, 200];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseStrategyIxData;
impl CloseStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_STRATEGY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseStrategyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseStrategyIxData.try_to_vec()?,
    })
}
pub fn close_strategy_ix(keys: CloseStrategyKeys) -> std::io::Result<Instruction> {
    close_strategy_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn close_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseStrategyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseStrategyKeys = accounts.into();
    let ix = close_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_strategy_invoke(accounts: CloseStrategyAccounts<'_, '_>) -> ProgramResult {
    close_strategy_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn close_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseStrategyKeys = accounts.into();
    let ix = close_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_strategy_invoke_signed(
    accounts: CloseStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_strategy_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, seeds)
}
pub fn close_strategy_verify_account_keys(
    accounts: CloseStrategyAccounts<'_, '_>,
    keys: CloseStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (
            *accounts.old_position_or_base_vault_authority.key,
            keys.old_position_or_base_vault_authority,
        ),
        (
            *accounts.old_position_mint_or_base_vault_authority.key,
            keys.old_position_mint_or_base_vault_authority,
        ),
        (
            *accounts.old_position_token_account_or_base_vault_authority.key,
            keys.old_position_token_account_or_base_vault_authority,
        ),
        (
            *accounts.old_tick_array_lower_or_base_vault_authority.key,
            keys.old_tick_array_lower_or_base_vault_authority,
        ),
        (
            *accounts.old_tick_array_upper_or_base_vault_authority.key,
            keys.old_tick_array_upper_or_base_vault_authority,
        ),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.user_token_a_ata.key, keys.user_token_a_ata),
        (*accounts.user_token_b_ata.key, keys.user_token_b_ata),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.reward0_vault.key, keys.reward0_vault),
        (*accounts.reward1_vault.key, keys.reward1_vault),
        (*accounts.reward2_vault.key, keys.reward2_vault),
        (*accounts.kamino_reward0_vault.key, keys.kamino_reward0_vault),
        (*accounts.kamino_reward1_vault.key, keys.kamino_reward1_vault),
        (*accounts.kamino_reward2_vault.key, keys.kamino_reward2_vault),
        (*accounts.user_reward0_ata.key, keys.user_reward0_ata),
        (*accounts.user_reward1_ata.key, keys.user_reward1_ata),
        (*accounts.user_reward2_ata.key, keys.user_reward2_ata),
        (*accounts.user_kamino_reward0_ata.key, keys.user_kamino_reward0_ata),
        (*accounts.user_kamino_reward1_ata.key, keys.user_kamino_reward1_ata),
        (*accounts.user_kamino_reward2_ata.key, keys.user_kamino_reward2_ata),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.system.key, keys.system),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_strategy_verify_writable_privileges<'me, 'info>(
    accounts: CloseStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.strategy,
        accounts.old_position_or_base_vault_authority,
        accounts.old_position_mint_or_base_vault_authority,
        accounts.old_position_token_account_or_base_vault_authority,
        accounts.old_tick_array_lower_or_base_vault_authority,
        accounts.old_tick_array_upper_or_base_vault_authority,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.user_token_a_ata,
        accounts.user_token_b_ata,
        accounts.token_a_mint,
        accounts.token_b_mint,
        accounts.reward0_vault,
        accounts.reward1_vault,
        accounts.reward2_vault,
        accounts.kamino_reward0_vault,
        accounts.kamino_reward1_vault,
        accounts.kamino_reward2_vault,
        accounts.user_reward0_ata,
        accounts.user_reward1_ata,
        accounts.user_reward2_ata,
        accounts.user_kamino_reward0_ata,
        accounts.user_kamino_reward1_ata,
        accounts.user_kamino_reward2_ata,
        accounts.base_vault_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_strategy_verify_signer_privileges<'me, 'info>(
    accounts: CloseStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_strategy_verify_account_privileges<'me, 'info>(
    accounts: CloseStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_strategy_verify_writable_privileges(accounts)?;
    close_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub user_shares_ata: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub token_program: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            shares_mint: *accounts.shares_mint.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            token_program: *accounts.token_program.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            base_vault_authority: pubkeys[9],
            token_a_ata: pubkeys[10],
            token_b_ata: pubkeys[11],
            token_a_mint: pubkeys[12],
            token_b_mint: pubkeys[13],
            user_shares_ata: pubkeys[14],
            shares_mint: pubkeys[15],
            shares_mint_authority: pubkeys[16],
            scope_prices_a: pubkeys[17],
            scope_prices_b: pubkeys[18],
            token_infos: pubkeys[19],
            token_program: pubkeys[20],
            token_a_token_program: pubkeys[21],
            token_b_token_program: pubkeys[22],
            instruction_sysvar_account: pubkeys[23],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.user_shares_ata.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.token_program.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            base_vault_authority: &arr[9],
            token_a_ata: &arr[10],
            token_b_ata: &arr[11],
            token_a_mint: &arr[12],
            token_b_mint: &arr[13],
            user_shares_ata: &arr[14],
            shares_mint: &arr[15],
            shares_mint_authority: &arr[16],
            scope_prices_a: &arr[17],
            scope_prices_b: &arr[18],
            token_infos: &arr[19],
            token_program: &arr[20],
            token_a_token_program: &arr[21],
            token_b_token_program: &arr[22],
            instruction_sysvar_account: &arr[23],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub token_max_a: u64,
    pub token_max_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIxData(pub DepositIxArgs);
impl From<DepositIxArgs> for DepositIxData {
    fn from(args: DepositIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_max_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_max_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                token_max_a,
                token_max_b,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_max_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_max_b, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_ix(
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    deposit_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_invoke(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    deposit_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_invoke_signed(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_writable_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.user_shares_ata,
        accounts.shares_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_verify_signer_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_verify_account_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_verify_writable_privileges(accounts)?;
    deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INVEST_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct InvestAccounts<'me, 'info> {
    pub actions_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub raydium_protocol_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InvestKeys {
    pub actions_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub position: Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub position_token_account: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub pool_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub event_authority: Pubkey,
}
impl From<InvestAccounts<'_, '_>> for InvestKeys {
    fn from(accounts: InvestAccounts) -> Self {
        Self {
            actions_authority: *accounts.actions_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool: *accounts.pool.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            position: *accounts.position.key,
            raydium_protocol_position_or_base_vault_authority: *accounts
                .raydium_protocol_position_or_base_vault_authority
                .key,
            position_token_account: *accounts.position_token_account.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            pool_program: *accounts.pool_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<InvestKeys> for [AccountMeta; INVEST_IX_ACCOUNTS_LEN] {
    fn from(keys: InvestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.actions_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_protocol_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INVEST_IX_ACCOUNTS_LEN]> for InvestKeys {
    fn from(pubkeys: [Pubkey; INVEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            actions_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            token_a_vault: pubkeys[3],
            token_b_vault: pubkeys[4],
            token_a_mint: pubkeys[5],
            token_b_mint: pubkeys[6],
            base_vault_authority: pubkeys[7],
            pool: pubkeys[8],
            token_a_token_program: pubkeys[9],
            token_b_token_program: pubkeys[10],
            memo_program: pubkeys[11],
            token_program: pubkeys[12],
            token_program2022: pubkeys[13],
            position: pubkeys[14],
            raydium_protocol_position_or_base_vault_authority: pubkeys[15],
            position_token_account: pubkeys[16],
            pool_token_vault_a: pubkeys[17],
            pool_token_vault_b: pubkeys[18],
            tick_array_lower: pubkeys[19],
            tick_array_upper: pubkeys[20],
            scope_prices_a: pubkeys[21],
            scope_prices_b: pubkeys[22],
            token_infos: pubkeys[23],
            pool_program: pubkeys[24],
            instruction_sysvar_account: pubkeys[25],
            event_authority: pubkeys[26],
        }
    }
}
impl<'info> From<InvestAccounts<'_, 'info>>
for [AccountInfo<'info>; INVEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: InvestAccounts<'_, 'info>) -> Self {
        [
            accounts.actions_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.position.clone(),
            accounts.raydium_protocol_position_or_base_vault_authority.clone(),
            accounts.position_token_account.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.pool_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INVEST_IX_ACCOUNTS_LEN]>
for InvestAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INVEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            actions_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            token_a_vault: &arr[3],
            token_b_vault: &arr[4],
            token_a_mint: &arr[5],
            token_b_mint: &arr[6],
            base_vault_authority: &arr[7],
            pool: &arr[8],
            token_a_token_program: &arr[9],
            token_b_token_program: &arr[10],
            memo_program: &arr[11],
            token_program: &arr[12],
            token_program2022: &arr[13],
            position: &arr[14],
            raydium_protocol_position_or_base_vault_authority: &arr[15],
            position_token_account: &arr[16],
            pool_token_vault_a: &arr[17],
            pool_token_vault_b: &arr[18],
            tick_array_lower: &arr[19],
            tick_array_upper: &arr[20],
            scope_prices_a: &arr[21],
            scope_prices_b: &arr[22],
            token_infos: &arr[23],
            pool_program: &arr[24],
            instruction_sysvar_account: &arr[25],
            event_authority: &arr[26],
        }
    }
}
pub const INVEST_IX_DISCM: [u8; 8usize] = [13, 245, 180, 103, 254, 182, 121, 4];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvestIxArgs {
    pub reference_price_tick: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InvestIxData(pub InvestIxArgs);
impl From<InvestIxArgs> for InvestIxData {
    fn from(args: InvestIxArgs) -> Self {
        Self(args)
    }
}
impl InvestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INVEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let reference_price_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InvestIxArgs {
                reference_price_tick,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INVEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.reference_price_tick, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn invest_ix_with_program_id(
    program_id: Pubkey,
    keys: InvestKeys,
    args: InvestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INVEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: InvestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn invest_ix(keys: InvestKeys, args: InvestIxArgs) -> std::io::Result<Instruction> {
    invest_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn invest_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InvestAccounts<'_, '_>,
    args: InvestIxArgs,
) -> ProgramResult {
    let keys: InvestKeys = accounts.into();
    let ix = invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn invest_invoke(
    accounts: InvestAccounts<'_, '_>,
    args: InvestIxArgs,
) -> ProgramResult {
    invest_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn invest_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InvestAccounts<'_, '_>,
    args: InvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InvestKeys = accounts.into();
    let ix = invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn invest_invoke_signed(
    accounts: InvestAccounts<'_, '_>,
    args: InvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    invest_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, args, seeds)
}
pub fn invest_verify_account_keys(
    accounts: InvestAccounts<'_, '_>,
    keys: InvestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.actions_authority.key, keys.actions_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.position.key, keys.position),
        (
            *accounts.raydium_protocol_position_or_base_vault_authority.key,
            keys.raydium_protocol_position_or_base_vault_authority,
        ),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn invest_verify_writable_privileges<'me, 'info>(
    accounts: InvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.actions_authority,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.base_vault_authority,
        accounts.pool,
        accounts.position,
        accounts.raydium_protocol_position_or_base_vault_authority,
        accounts.position_token_account,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn invest_verify_signer_privileges<'me, 'info>(
    accounts: InvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.actions_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn invest_verify_account_privileges<'me, 'info>(
    accounts: InvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    invest_verify_writable_privileges(accounts)?;
    invest_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN: usize = 32;
#[derive(Copy, Clone, Debug)]
pub struct DepositAndInvestAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub raydium_protocol_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositAndInvestKeys {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub position_token_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub base_vault_authority: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub user_shares_ata: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub pool_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub event_authority: Pubkey,
}
impl From<DepositAndInvestAccounts<'_, '_>> for DepositAndInvestKeys {
    fn from(accounts: DepositAndInvestAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            raydium_protocol_position_or_base_vault_authority: *accounts
                .raydium_protocol_position_or_base_vault_authority
                .key,
            position_token_account: *accounts.position_token_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            shares_mint: *accounts.shares_mint.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            pool_program: *accounts.pool_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<DepositAndInvestKeys> for [AccountMeta; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositAndInvestKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_protocol_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN]> for DepositAndInvestKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            raydium_protocol_position_or_base_vault_authority: pubkeys[5],
            position_token_account: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            pool_token_vault_a: pubkeys[9],
            pool_token_vault_b: pubkeys[10],
            tick_array_lower: pubkeys[11],
            tick_array_upper: pubkeys[12],
            base_vault_authority: pubkeys[13],
            token_a_ata: pubkeys[14],
            token_b_ata: pubkeys[15],
            token_a_mint: pubkeys[16],
            token_b_mint: pubkeys[17],
            user_shares_ata: pubkeys[18],
            shares_mint: pubkeys[19],
            shares_mint_authority: pubkeys[20],
            scope_prices_a: pubkeys[21],
            scope_prices_b: pubkeys[22],
            token_infos: pubkeys[23],
            token_program: pubkeys[24],
            token_program2022: pubkeys[25],
            token_a_token_program: pubkeys[26],
            token_b_token_program: pubkeys[27],
            memo_program: pubkeys[28],
            pool_program: pubkeys[29],
            instruction_sysvar_account: pubkeys[30],
            event_authority: pubkeys[31],
        }
    }
}
impl<'info> From<DepositAndInvestAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAndInvestAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.raydium_protocol_position_or_base_vault_authority.clone(),
            accounts.position_token_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.base_vault_authority.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.user_shares_ata.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.pool_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN]>
for DepositAndInvestAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            raydium_protocol_position_or_base_vault_authority: &arr[5],
            position_token_account: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            pool_token_vault_a: &arr[9],
            pool_token_vault_b: &arr[10],
            tick_array_lower: &arr[11],
            tick_array_upper: &arr[12],
            base_vault_authority: &arr[13],
            token_a_ata: &arr[14],
            token_b_ata: &arr[15],
            token_a_mint: &arr[16],
            token_b_mint: &arr[17],
            user_shares_ata: &arr[18],
            shares_mint: &arr[19],
            shares_mint_authority: &arr[20],
            scope_prices_a: &arr[21],
            scope_prices_b: &arr[22],
            token_infos: &arr[23],
            token_program: &arr[24],
            token_program2022: &arr[25],
            token_a_token_program: &arr[26],
            token_b_token_program: &arr[27],
            memo_program: &arr[28],
            pool_program: &arr[29],
            instruction_sysvar_account: &arr[30],
            event_authority: &arr[31],
        }
    }
}
pub const DEPOSIT_AND_INVEST_IX_DISCM: [u8; 8usize] = [
    22, 157, 173, 6, 187, 25, 86, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositAndInvestIxArgs {
    pub token_max_a: u64,
    pub token_max_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositAndInvestIxData(pub DepositAndInvestIxArgs);
impl From<DepositAndInvestIxArgs> for DepositAndInvestIxData {
    fn from(args: DepositAndInvestIxArgs) -> Self {
        Self(args)
    }
}
impl DepositAndInvestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_AND_INVEST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_max_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_max_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositAndInvestIxArgs {
                token_max_a,
                token_max_b,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_AND_INVEST_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_max_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_max_b, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_and_invest_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositAndInvestKeys,
    args: DepositAndInvestIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_AND_INVEST_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositAndInvestIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_and_invest_ix(
    keys: DepositAndInvestKeys,
    args: DepositAndInvestIxArgs,
) -> std::io::Result<Instruction> {
    deposit_and_invest_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn deposit_and_invest_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAndInvestAccounts<'_, '_>,
    args: DepositAndInvestIxArgs,
) -> ProgramResult {
    let keys: DepositAndInvestKeys = accounts.into();
    let ix = deposit_and_invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_and_invest_invoke(
    accounts: DepositAndInvestAccounts<'_, '_>,
    args: DepositAndInvestIxArgs,
) -> ProgramResult {
    deposit_and_invest_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn deposit_and_invest_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAndInvestAccounts<'_, '_>,
    args: DepositAndInvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositAndInvestKeys = accounts.into();
    let ix = deposit_and_invest_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_and_invest_invoke_signed(
    accounts: DepositAndInvestAccounts<'_, '_>,
    args: DepositAndInvestIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_and_invest_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_and_invest_verify_account_keys(
    accounts: DepositAndInvestAccounts<'_, '_>,
    keys: DepositAndInvestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (
            *accounts.raydium_protocol_position_or_base_vault_authority.key,
            keys.raydium_protocol_position_or_base_vault_authority,
        ),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_and_invest_verify_writable_privileges<'me, 'info>(
    accounts: DepositAndInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.strategy,
        accounts.pool,
        accounts.position,
        accounts.raydium_protocol_position_or_base_vault_authority,
        accounts.position_token_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.base_vault_authority,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.user_shares_ata,
        accounts.shares_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_and_invest_verify_signer_privileges<'me, 'info>(
    accounts: DepositAndInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_and_invest_verify_account_privileges<'me, 'info>(
    accounts: DepositAndInvestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_and_invest_verify_writable_privileges(accounts)?;
    deposit_and_invest_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 29;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub treasury_fee_token_a_vault: &'me AccountInfo<'info>,
    pub treasury_fee_token_b_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub user_shares_ata: Pubkey,
    pub shares_mint: Pubkey,
    pub treasury_fee_token_a_vault: Pubkey,
    pub treasury_fee_token_b_vault: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub position_token_account: Pubkey,
    pub pool_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub event_authority: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            shares_mint: *accounts.shares_mint.key,
            treasury_fee_token_a_vault: *accounts.treasury_fee_token_a_vault.key,
            treasury_fee_token_b_vault: *accounts.treasury_fee_token_b_vault.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            position_token_account: *accounts.position_token_account.key,
            pool_program: *accounts.pool_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            base_vault_authority: pubkeys[9],
            pool_token_vault_a: pubkeys[10],
            pool_token_vault_b: pubkeys[11],
            token_a_ata: pubkeys[12],
            token_b_ata: pubkeys[13],
            token_a_mint: pubkeys[14],
            token_b_mint: pubkeys[15],
            user_shares_ata: pubkeys[16],
            shares_mint: pubkeys[17],
            treasury_fee_token_a_vault: pubkeys[18],
            treasury_fee_token_b_vault: pubkeys[19],
            token_program: pubkeys[20],
            token_program2022: pubkeys[21],
            token_a_token_program: pubkeys[22],
            token_b_token_program: pubkeys[23],
            memo_program: pubkeys[24],
            position_token_account: pubkeys[25],
            pool_program: pubkeys[26],
            instruction_sysvar_account: pubkeys[27],
            event_authority: pubkeys[28],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.user_shares_ata.clone(),
            accounts.shares_mint.clone(),
            accounts.treasury_fee_token_a_vault.clone(),
            accounts.treasury_fee_token_b_vault.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.position_token_account.clone(),
            accounts.pool_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            base_vault_authority: &arr[9],
            pool_token_vault_a: &arr[10],
            pool_token_vault_b: &arr[11],
            token_a_ata: &arr[12],
            token_b_ata: &arr[13],
            token_a_mint: &arr[14],
            token_b_mint: &arr[15],
            user_shares_ata: &arr[16],
            shares_mint: &arr[17],
            treasury_fee_token_a_vault: &arr[18],
            treasury_fee_token_b_vault: &arr[19],
            token_program: &arr[20],
            token_program2022: &arr[21],
            token_a_token_program: &arr[22],
            token_b_token_program: &arr[23],
            memo_program: &arr[24],
            position_token_account: &arr[25],
            pool_program: &arr[26],
            instruction_sysvar_account: &arr[27],
            event_authority: &arr[28],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub shares_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIxData(pub WithdrawIxArgs);
impl From<WithdrawIxArgs> for WithdrawIxData {
    fn from(args: WithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawIxArgs { shares_amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_ix(
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_invoke(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    withdraw_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_invoke_signed(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.treasury_fee_token_a_vault.key, keys.treasury_fee_token_a_vault),
        (*accounts.treasury_fee_token_b_vault.key, keys.treasury_fee_token_b_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.strategy,
        accounts.pool,
        accounts.position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.user_shares_ata,
        accounts.shares_mint,
        accounts.treasury_fee_token_a_vault,
        accounts.treasury_fee_token_b_vault,
        accounts.position_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_writable_privileges(accounts)?;
    withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct ExecutiveWithdrawAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub raydium_protocol_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecutiveWithdrawKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub position_token_account: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub pool_program: Pubkey,
    pub event_authority: Pubkey,
}
impl From<ExecutiveWithdrawAccounts<'_, '_>> for ExecutiveWithdrawKeys {
    fn from(accounts: ExecutiveWithdrawAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            raydium_protocol_position_or_base_vault_authority: *accounts
                .raydium_protocol_position_or_base_vault_authority
                .key,
            position_token_account: *accounts.position_token_account.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            pool_program: *accounts.pool_program.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<ExecutiveWithdrawKeys> for [AccountMeta; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecutiveWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_protocol_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN]> for ExecutiveWithdrawKeys {
    fn from(pubkeys: [Pubkey; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            raydium_protocol_position_or_base_vault_authority: pubkeys[5],
            position_token_account: pubkeys[6],
            tick_array_lower: pubkeys[7],
            tick_array_upper: pubkeys[8],
            token_a_vault: pubkeys[9],
            token_b_vault: pubkeys[10],
            base_vault_authority: pubkeys[11],
            pool_token_vault_a: pubkeys[12],
            pool_token_vault_b: pubkeys[13],
            token_a_mint: pubkeys[14],
            token_b_mint: pubkeys[15],
            scope_prices_a: pubkeys[16],
            scope_prices_b: pubkeys[17],
            token_infos: pubkeys[18],
            token_a_token_program: pubkeys[19],
            token_b_token_program: pubkeys[20],
            memo_program: pubkeys[21],
            token_program: pubkeys[22],
            token_program2022: pubkeys[23],
            pool_program: pubkeys[24],
            event_authority: pubkeys[25],
        }
    }
}
impl<'info> From<ExecutiveWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecutiveWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.raydium_protocol_position_or_base_vault_authority.clone(),
            accounts.position_token_account.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.pool_program.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN]>
for ExecutiveWithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            raydium_protocol_position_or_base_vault_authority: &arr[5],
            position_token_account: &arr[6],
            tick_array_lower: &arr[7],
            tick_array_upper: &arr[8],
            token_a_vault: &arr[9],
            token_b_vault: &arr[10],
            base_vault_authority: &arr[11],
            pool_token_vault_a: &arr[12],
            pool_token_vault_b: &arr[13],
            token_a_mint: &arr[14],
            token_b_mint: &arr[15],
            scope_prices_a: &arr[16],
            scope_prices_b: &arr[17],
            token_infos: &arr[18],
            token_a_token_program: &arr[19],
            token_b_token_program: &arr[20],
            memo_program: &arr[21],
            token_program: &arr[22],
            token_program2022: &arr[23],
            pool_program: &arr[24],
            event_authority: &arr[25],
        }
    }
}
pub const EXECUTIVE_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    159, 39, 110, 137, 100, 234, 204, 141,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecutiveWithdrawIxArgs {
    pub action: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecutiveWithdrawIxData(pub ExecutiveWithdrawIxArgs);
impl From<ExecutiveWithdrawIxArgs> for ExecutiveWithdrawIxData {
    fn from(args: ExecutiveWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl ExecutiveWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTIVE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let action: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ExecutiveWithdrawIxArgs { action }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTIVE_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.action, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn executive_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecutiveWithdrawKeys,
    args: ExecutiveWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTIVE_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecutiveWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn executive_withdraw_ix(
    keys: ExecutiveWithdrawKeys,
    args: ExecutiveWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    executive_withdraw_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn executive_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecutiveWithdrawAccounts<'_, '_>,
    args: ExecutiveWithdrawIxArgs,
) -> ProgramResult {
    let keys: ExecutiveWithdrawKeys = accounts.into();
    let ix = executive_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn executive_withdraw_invoke(
    accounts: ExecutiveWithdrawAccounts<'_, '_>,
    args: ExecutiveWithdrawIxArgs,
) -> ProgramResult {
    executive_withdraw_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn executive_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecutiveWithdrawAccounts<'_, '_>,
    args: ExecutiveWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecutiveWithdrawKeys = accounts.into();
    let ix = executive_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn executive_withdraw_invoke_signed(
    accounts: ExecutiveWithdrawAccounts<'_, '_>,
    args: ExecutiveWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    executive_withdraw_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn executive_withdraw_verify_account_keys(
    accounts: ExecutiveWithdrawAccounts<'_, '_>,
    keys: ExecutiveWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (
            *accounts.raydium_protocol_position_or_base_vault_authority.key,
            keys.raydium_protocol_position_or_base_vault_authority,
        ),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn executive_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: ExecutiveWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.strategy,
        accounts.pool,
        accounts.position,
        accounts.raydium_protocol_position_or_base_vault_authority,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn executive_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: ExecutiveWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn executive_withdraw_verify_account_privileges<'me, 'info>(
    accounts: ExecutiveWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    executive_withdraw_verify_writable_privileges(accounts)?;
    executive_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN: usize = 33;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeesAndRewardsAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub raydium_protocol_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub treasury_fee_token_a_vault: &'me AccountInfo<'info>,
    pub treasury_fee_token_b_vault: &'me AccountInfo<'info>,
    pub treasury_fee_vault_authority: &'me AccountInfo<'info>,
    pub reward0_vault: &'me AccountInfo<'info>,
    pub reward1_vault: &'me AccountInfo<'info>,
    pub reward2_vault: &'me AccountInfo<'info>,
    pub pool_reward_vault0: &'me AccountInfo<'info>,
    pub pool_reward_vault1: &'me AccountInfo<'info>,
    pub pool_reward_vault2: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeesAndRewardsKeys {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub position: Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub position_token_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub token_b_vault: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub treasury_fee_token_a_vault: Pubkey,
    pub treasury_fee_token_b_vault: Pubkey,
    pub treasury_fee_vault_authority: Pubkey,
    pub reward0_vault: Pubkey,
    pub reward1_vault: Pubkey,
    pub reward2_vault: Pubkey,
    pub pool_reward_vault0: Pubkey,
    pub pool_reward_vault1: Pubkey,
    pub pool_reward_vault2: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub pool_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub event_authority: Pubkey,
}
impl From<CollectFeesAndRewardsAccounts<'_, '_>> for CollectFeesAndRewardsKeys {
    fn from(accounts: CollectFeesAndRewardsAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool: *accounts.pool.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            position: *accounts.position.key,
            raydium_protocol_position_or_base_vault_authority: *accounts
                .raydium_protocol_position_or_base_vault_authority
                .key,
            position_token_account: *accounts.position_token_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            token_b_vault: *accounts.token_b_vault.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            treasury_fee_token_a_vault: *accounts.treasury_fee_token_a_vault.key,
            treasury_fee_token_b_vault: *accounts.treasury_fee_token_b_vault.key,
            treasury_fee_vault_authority: *accounts.treasury_fee_vault_authority.key,
            reward0_vault: *accounts.reward0_vault.key,
            reward1_vault: *accounts.reward1_vault.key,
            reward2_vault: *accounts.reward2_vault.key,
            pool_reward_vault0: *accounts.pool_reward_vault0.key,
            pool_reward_vault1: *accounts.pool_reward_vault1.key,
            pool_reward_vault2: *accounts.pool_reward_vault2.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            pool_program: *accounts.pool_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<CollectFeesAndRewardsKeys>
for [AccountMeta; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeesAndRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_protocol_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward2_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_reward_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_reward_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_reward_vault2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for CollectFeesAndRewardsKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            base_vault_authority: pubkeys[3],
            pool: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            position: pubkeys[7],
            raydium_protocol_position_or_base_vault_authority: pubkeys[8],
            position_token_account: pubkeys[9],
            token_a_vault: pubkeys[10],
            pool_token_vault_a: pubkeys[11],
            token_b_vault: pubkeys[12],
            pool_token_vault_b: pubkeys[13],
            treasury_fee_token_a_vault: pubkeys[14],
            treasury_fee_token_b_vault: pubkeys[15],
            treasury_fee_vault_authority: pubkeys[16],
            reward0_vault: pubkeys[17],
            reward1_vault: pubkeys[18],
            reward2_vault: pubkeys[19],
            pool_reward_vault0: pubkeys[20],
            pool_reward_vault1: pubkeys[21],
            pool_reward_vault2: pubkeys[22],
            token_a_mint: pubkeys[23],
            token_b_mint: pubkeys[24],
            token_a_token_program: pubkeys[25],
            token_b_token_program: pubkeys[26],
            memo_program: pubkeys[27],
            token_program: pubkeys[28],
            token_program2022: pubkeys[29],
            pool_program: pubkeys[30],
            instruction_sysvar_account: pubkeys[31],
            event_authority: pubkeys[32],
        }
    }
}
impl<'info> From<CollectFeesAndRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeesAndRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.position.clone(),
            accounts.raydium_protocol_position_or_base_vault_authority.clone(),
            accounts.position_token_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.token_b_vault.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.treasury_fee_token_a_vault.clone(),
            accounts.treasury_fee_token_b_vault.clone(),
            accounts.treasury_fee_vault_authority.clone(),
            accounts.reward0_vault.clone(),
            accounts.reward1_vault.clone(),
            accounts.reward2_vault.clone(),
            accounts.pool_reward_vault0.clone(),
            accounts.pool_reward_vault1.clone(),
            accounts.pool_reward_vault2.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.pool_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for CollectFeesAndRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            base_vault_authority: &arr[3],
            pool: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            position: &arr[7],
            raydium_protocol_position_or_base_vault_authority: &arr[8],
            position_token_account: &arr[9],
            token_a_vault: &arr[10],
            pool_token_vault_a: &arr[11],
            token_b_vault: &arr[12],
            pool_token_vault_b: &arr[13],
            treasury_fee_token_a_vault: &arr[14],
            treasury_fee_token_b_vault: &arr[15],
            treasury_fee_vault_authority: &arr[16],
            reward0_vault: &arr[17],
            reward1_vault: &arr[18],
            reward2_vault: &arr[19],
            pool_reward_vault0: &arr[20],
            pool_reward_vault1: &arr[21],
            pool_reward_vault2: &arr[22],
            token_a_mint: &arr[23],
            token_b_mint: &arr[24],
            token_a_token_program: &arr[25],
            token_b_token_program: &arr[26],
            memo_program: &arr[27],
            token_program: &arr[28],
            token_program2022: &arr[29],
            pool_program: &arr[30],
            instruction_sysvar_account: &arr[31],
            event_authority: &arr[32],
        }
    }
}
pub const COLLECT_FEES_AND_REWARDS_IX_DISCM: [u8; 8usize] = [
    113, 18, 75, 8, 182, 31, 105, 186,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesAndRewardsIxData;
impl CollectFeesAndRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEES_AND_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEES_AND_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fees_and_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectFeesAndRewardsIxData.try_to_vec()?,
    })
}
pub fn collect_fees_and_rewards_ix(
    keys: CollectFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    collect_fees_and_rewards_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn collect_fees_and_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeesAndRewardsKeys = accounts.into();
    let ix = collect_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fees_and_rewards_invoke(
    accounts: CollectFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    collect_fees_and_rewards_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn collect_fees_and_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeesAndRewardsKeys = accounts.into();
    let ix = collect_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fees_and_rewards_invoke_signed(
    accounts: CollectFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fees_and_rewards_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_fees_and_rewards_verify_account_keys(
    accounts: CollectFeesAndRewardsAccounts<'_, '_>,
    keys: CollectFeesAndRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.position.key, keys.position),
        (
            *accounts.raydium_protocol_position_or_base_vault_authority.key,
            keys.raydium_protocol_position_or_base_vault_authority,
        ),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.treasury_fee_token_a_vault.key, keys.treasury_fee_token_a_vault),
        (*accounts.treasury_fee_token_b_vault.key, keys.treasury_fee_token_b_vault),
        (*accounts.treasury_fee_vault_authority.key, keys.treasury_fee_vault_authority),
        (*accounts.reward0_vault.key, keys.reward0_vault),
        (*accounts.reward1_vault.key, keys.reward1_vault),
        (*accounts.reward2_vault.key, keys.reward2_vault),
        (*accounts.pool_reward_vault0.key, keys.pool_reward_vault0),
        (*accounts.pool_reward_vault1.key, keys.pool_reward_vault1),
        (*accounts.pool_reward_vault2.key, keys.pool_reward_vault2),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fees_and_rewards_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.strategy,
        accounts.base_vault_authority,
        accounts.pool,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.position,
        accounts.raydium_protocol_position_or_base_vault_authority,
        accounts.token_a_vault,
        accounts.pool_token_vault_a,
        accounts.token_b_vault,
        accounts.pool_token_vault_b,
        accounts.treasury_fee_token_a_vault,
        accounts.treasury_fee_token_b_vault,
        accounts.reward0_vault,
        accounts.reward1_vault,
        accounts.reward2_vault,
        accounts.pool_reward_vault0,
        accounts.pool_reward_vault1,
        accounts.pool_reward_vault2,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fees_and_rewards_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fees_and_rewards_verify_account_privileges<'me, 'info>(
    accounts: CollectFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fees_and_rewards_verify_writable_privileges(accounts)?;
    collect_fees_and_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_REWARDS_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct SwapRewardsAccounts<'me, 'info> {
    pub actions_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub treasury_fee_token_a_vault: &'me AccountInfo<'info>,
    pub treasury_fee_token_b_vault: &'me AccountInfo<'info>,
    pub treasury_fee_vault_authority: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_token_a_ata: &'me AccountInfo<'info>,
    pub user_token_b_ata: &'me AccountInfo<'info>,
    pub user_reward_token_account: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub scope_prices_reward: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub reward_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapRewardsKeys {
    pub actions_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub reward_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub treasury_fee_token_a_vault: Pubkey,
    pub treasury_fee_token_b_vault: Pubkey,
    pub treasury_fee_vault_authority: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub user_token_a_ata: Pubkey,
    pub user_token_b_ata: Pubkey,
    pub user_reward_token_account: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub scope_prices_reward: Pubkey,
    pub token_infos: Pubkey,
    pub system_program: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub reward_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<SwapRewardsAccounts<'_, '_>> for SwapRewardsKeys {
    fn from(accounts: SwapRewardsAccounts) -> Self {
        Self {
            actions_authority: *accounts.actions_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            reward_vault: *accounts.reward_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            treasury_fee_token_a_vault: *accounts.treasury_fee_token_a_vault.key,
            treasury_fee_token_b_vault: *accounts.treasury_fee_token_b_vault.key,
            treasury_fee_vault_authority: *accounts.treasury_fee_vault_authority.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            reward_mint: *accounts.reward_mint.key,
            user_token_a_ata: *accounts.user_token_a_ata.key,
            user_token_b_ata: *accounts.user_token_b_ata.key,
            user_reward_token_account: *accounts.user_reward_token_account.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            scope_prices_reward: *accounts.scope_prices_reward.key,
            token_infos: *accounts.token_infos.key,
            system_program: *accounts.system_program.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            reward_token_program: *accounts.reward_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<SwapRewardsKeys> for [AccountMeta; SWAP_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.actions_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_reward_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_reward,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_REWARDS_IX_ACCOUNTS_LEN]> for SwapRewardsKeys {
    fn from(pubkeys: [Pubkey; SWAP_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            actions_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            token_a_vault: pubkeys[4],
            token_b_vault: pubkeys[5],
            reward_vault: pubkeys[6],
            base_vault_authority: pubkeys[7],
            treasury_fee_token_a_vault: pubkeys[8],
            treasury_fee_token_b_vault: pubkeys[9],
            treasury_fee_vault_authority: pubkeys[10],
            token_a_mint: pubkeys[11],
            token_b_mint: pubkeys[12],
            reward_mint: pubkeys[13],
            user_token_a_ata: pubkeys[14],
            user_token_b_ata: pubkeys[15],
            user_reward_token_account: pubkeys[16],
            scope_prices_a: pubkeys[17],
            scope_prices_b: pubkeys[18],
            scope_prices_reward: pubkeys[19],
            token_infos: pubkeys[20],
            system_program: pubkeys[21],
            token_a_token_program: pubkeys[22],
            token_b_token_program: pubkeys[23],
            reward_token_program: pubkeys[24],
            instruction_sysvar_account: pubkeys[25],
        }
    }
}
impl<'info> From<SwapRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.actions_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.reward_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.treasury_fee_token_a_vault.clone(),
            accounts.treasury_fee_token_b_vault.clone(),
            accounts.treasury_fee_vault_authority.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.reward_mint.clone(),
            accounts.user_token_a_ata.clone(),
            accounts.user_token_b_ata.clone(),
            accounts.user_reward_token_account.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.scope_prices_reward.clone(),
            accounts.token_infos.clone(),
            accounts.system_program.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.reward_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_REWARDS_IX_ACCOUNTS_LEN]>
for SwapRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            actions_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            token_a_vault: &arr[4],
            token_b_vault: &arr[5],
            reward_vault: &arr[6],
            base_vault_authority: &arr[7],
            treasury_fee_token_a_vault: &arr[8],
            treasury_fee_token_b_vault: &arr[9],
            treasury_fee_vault_authority: &arr[10],
            token_a_mint: &arr[11],
            token_b_mint: &arr[12],
            reward_mint: &arr[13],
            user_token_a_ata: &arr[14],
            user_token_b_ata: &arr[15],
            user_reward_token_account: &arr[16],
            scope_prices_a: &arr[17],
            scope_prices_b: &arr[18],
            scope_prices_reward: &arr[19],
            token_infos: &arr[20],
            system_program: &arr[21],
            token_a_token_program: &arr[22],
            token_b_token_program: &arr[23],
            reward_token_program: &arr[24],
            instruction_sysvar_account: &arr[25],
        }
    }
}
pub const SWAP_REWARDS_IX_DISCM: [u8; 8usize] = [92, 41, 172, 30, 190, 65, 174, 90];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapRewardsIxArgs {
    pub token_a_in: u64,
    pub token_b_in: u64,
    pub reward_index: u64,
    pub reward_collateral_id: u64,
    pub min_collateral_token_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapRewardsIxData(pub SwapRewardsIxArgs);
impl From<SwapRewardsIxArgs> for SwapRewardsIxData {
    fn from(args: SwapRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl SwapRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_a_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_collateral_token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapRewardsIxArgs {
                token_a_in,
                token_b_in,
                reward_index,
                reward_collateral_id,
                min_collateral_token_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_REWARDS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_a_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_b_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.reward_collateral_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_collateral_token_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapRewardsKeys,
    args: SwapRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_rewards_ix(
    keys: SwapRewardsKeys,
    args: SwapRewardsIxArgs,
) -> std::io::Result<Instruction> {
    swap_rewards_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn swap_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapRewardsAccounts<'_, '_>,
    args: SwapRewardsIxArgs,
) -> ProgramResult {
    let keys: SwapRewardsKeys = accounts.into();
    let ix = swap_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_rewards_invoke(
    accounts: SwapRewardsAccounts<'_, '_>,
    args: SwapRewardsIxArgs,
) -> ProgramResult {
    swap_rewards_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn swap_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapRewardsAccounts<'_, '_>,
    args: SwapRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapRewardsKeys = accounts.into();
    let ix = swap_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_rewards_invoke_signed(
    accounts: SwapRewardsAccounts<'_, '_>,
    args: SwapRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_rewards_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, args, seeds)
}
pub fn swap_rewards_verify_account_keys(
    accounts: SwapRewardsAccounts<'_, '_>,
    keys: SwapRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.actions_authority.key, keys.actions_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.treasury_fee_token_a_vault.key, keys.treasury_fee_token_a_vault),
        (*accounts.treasury_fee_token_b_vault.key, keys.treasury_fee_token_b_vault),
        (*accounts.treasury_fee_vault_authority.key, keys.treasury_fee_vault_authority),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.user_token_a_ata.key, keys.user_token_a_ata),
        (*accounts.user_token_b_ata.key, keys.user_token_b_ata),
        (*accounts.user_reward_token_account.key, keys.user_reward_token_account),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.scope_prices_reward.key, keys.scope_prices_reward),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.reward_token_program.key, keys.reward_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_rewards_verify_writable_privileges<'me, 'info>(
    accounts: SwapRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.actions_authority,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.reward_vault,
        accounts.base_vault_authority,
        accounts.treasury_fee_token_a_vault,
        accounts.treasury_fee_token_b_vault,
        accounts.user_token_a_ata,
        accounts.user_token_b_ata,
        accounts.user_reward_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_rewards_verify_signer_privileges<'me, 'info>(
    accounts: SwapRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.actions_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_rewards_verify_account_privileges<'me, 'info>(
    accounts: SwapRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_rewards_verify_writable_privileges(accounts)?;
    swap_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CheckExpectedVaultsBalancesAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CheckExpectedVaultsBalancesKeys {
    pub user: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
}
impl From<CheckExpectedVaultsBalancesAccounts<'_, '_>>
for CheckExpectedVaultsBalancesKeys {
    fn from(accounts: CheckExpectedVaultsBalancesAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
        }
    }
}
impl From<CheckExpectedVaultsBalancesKeys>
for [AccountMeta; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN] {
    fn from(keys: CheckExpectedVaultsBalancesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN]>
for CheckExpectedVaultsBalancesKeys {
    fn from(pubkeys: [Pubkey; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            token_a_ata: pubkeys[1],
            token_b_ata: pubkeys[2],
        }
    }
}
impl<'info> From<CheckExpectedVaultsBalancesAccounts<'_, 'info>>
for [AccountInfo<'info>; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN] {
    fn from(accounts: CheckExpectedVaultsBalancesAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN]>
for CheckExpectedVaultsBalancesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            token_a_ata: &arr[1],
            token_b_ata: &arr[2],
        }
    }
}
pub const CHECK_EXPECTED_VAULTS_BALANCES_IX_DISCM: [u8; 8usize] = [
    75, 151, 187, 125, 50, 4, 11, 71,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckExpectedVaultsBalancesIxArgs {
    pub token_a_ata_balance: u64,
    pub token_b_ata_balance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CheckExpectedVaultsBalancesIxData(pub CheckExpectedVaultsBalancesIxArgs);
impl From<CheckExpectedVaultsBalancesIxArgs> for CheckExpectedVaultsBalancesIxData {
    fn from(args: CheckExpectedVaultsBalancesIxArgs) -> Self {
        Self(args)
    }
}
impl CheckExpectedVaultsBalancesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHECK_EXPECTED_VAULTS_BALANCES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_a_ata_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_ata_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CheckExpectedVaultsBalancesIxArgs {
                token_a_ata_balance,
                token_b_ata_balance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHECK_EXPECTED_VAULTS_BALANCES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_a_ata_balance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token_b_ata_balance, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn check_expected_vaults_balances_ix_with_program_id(
    program_id: Pubkey,
    keys: CheckExpectedVaultsBalancesKeys,
    args: CheckExpectedVaultsBalancesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHECK_EXPECTED_VAULTS_BALANCES_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CheckExpectedVaultsBalancesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn check_expected_vaults_balances_ix(
    keys: CheckExpectedVaultsBalancesKeys,
    args: CheckExpectedVaultsBalancesIxArgs,
) -> std::io::Result<Instruction> {
    check_expected_vaults_balances_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn check_expected_vaults_balances_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CheckExpectedVaultsBalancesAccounts<'_, '_>,
    args: CheckExpectedVaultsBalancesIxArgs,
) -> ProgramResult {
    let keys: CheckExpectedVaultsBalancesKeys = accounts.into();
    let ix = check_expected_vaults_balances_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn check_expected_vaults_balances_invoke(
    accounts: CheckExpectedVaultsBalancesAccounts<'_, '_>,
    args: CheckExpectedVaultsBalancesIxArgs,
) -> ProgramResult {
    check_expected_vaults_balances_invoke_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn check_expected_vaults_balances_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CheckExpectedVaultsBalancesAccounts<'_, '_>,
    args: CheckExpectedVaultsBalancesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CheckExpectedVaultsBalancesKeys = accounts.into();
    let ix = check_expected_vaults_balances_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn check_expected_vaults_balances_invoke_signed(
    accounts: CheckExpectedVaultsBalancesAccounts<'_, '_>,
    args: CheckExpectedVaultsBalancesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    check_expected_vaults_balances_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn check_expected_vaults_balances_verify_account_keys(
    accounts: CheckExpectedVaultsBalancesAccounts<'_, '_>,
    keys: CheckExpectedVaultsBalancesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn check_expected_vaults_balances_verify_writable_privileges<'me, 'info>(
    accounts: CheckExpectedVaultsBalancesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn check_expected_vaults_balances_verify_signer_privileges<'me, 'info>(
    accounts: CheckExpectedVaultsBalancesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn check_expected_vaults_balances_verify_account_privileges<'me, 'info>(
    accounts: CheckExpectedVaultsBalancesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    check_expected_vaults_balances_verify_writable_privileges(accounts)?;
    check_expected_vaults_balances_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN: usize = 32;
#[derive(Copy, Clone, Debug)]
pub struct SingleTokenDepositAndInvestWithMinAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub raydium_protocol_position_or_base_vault_authority: &'me AccountInfo<'info>,
    pub position_token_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SingleTokenDepositAndInvestWithMinKeys {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: Pubkey,
    pub position_token_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub base_vault_authority: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub user_shares_ata: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub pool_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub event_authority: Pubkey,
}
impl From<SingleTokenDepositAndInvestWithMinAccounts<'_, '_>>
for SingleTokenDepositAndInvestWithMinKeys {
    fn from(accounts: SingleTokenDepositAndInvestWithMinAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            raydium_protocol_position_or_base_vault_authority: *accounts
                .raydium_protocol_position_or_base_vault_authority
                .key,
            position_token_account: *accounts.position_token_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            shares_mint: *accounts.shares_mint.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            pool_program: *accounts.pool_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            event_authority: *accounts.event_authority.key,
        }
    }
}
impl From<SingleTokenDepositAndInvestWithMinKeys>
for [AccountMeta; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SingleTokenDepositAndInvestWithMinKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_protocol_position_or_base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN]>
for SingleTokenDepositAndInvestWithMinKeys {
    fn from(
        pubkeys: [Pubkey; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            raydium_protocol_position_or_base_vault_authority: pubkeys[5],
            position_token_account: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            pool_token_vault_a: pubkeys[9],
            pool_token_vault_b: pubkeys[10],
            tick_array_lower: pubkeys[11],
            tick_array_upper: pubkeys[12],
            base_vault_authority: pubkeys[13],
            token_a_ata: pubkeys[14],
            token_b_ata: pubkeys[15],
            token_a_mint: pubkeys[16],
            token_b_mint: pubkeys[17],
            user_shares_ata: pubkeys[18],
            shares_mint: pubkeys[19],
            shares_mint_authority: pubkeys[20],
            scope_prices_a: pubkeys[21],
            scope_prices_b: pubkeys[22],
            token_infos: pubkeys[23],
            token_program: pubkeys[24],
            token_program2022: pubkeys[25],
            token_a_token_program: pubkeys[26],
            token_b_token_program: pubkeys[27],
            memo_program: pubkeys[28],
            pool_program: pubkeys[29],
            instruction_sysvar_account: pubkeys[30],
            event_authority: pubkeys[31],
        }
    }
}
impl<'info> From<SingleTokenDepositAndInvestWithMinAccounts<'_, 'info>>
for [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SingleTokenDepositAndInvestWithMinAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.raydium_protocol_position_or_base_vault_authority.clone(),
            accounts.position_token_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.base_vault_authority.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.user_shares_ata.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.pool_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.event_authority.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN],
> for SingleTokenDepositAndInvestWithMinAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            raydium_protocol_position_or_base_vault_authority: &arr[5],
            position_token_account: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            pool_token_vault_a: &arr[9],
            pool_token_vault_b: &arr[10],
            tick_array_lower: &arr[11],
            tick_array_upper: &arr[12],
            base_vault_authority: &arr[13],
            token_a_ata: &arr[14],
            token_b_ata: &arr[15],
            token_a_mint: &arr[16],
            token_b_mint: &arr[17],
            user_shares_ata: &arr[18],
            shares_mint: &arr[19],
            shares_mint_authority: &arr[20],
            scope_prices_a: &arr[21],
            scope_prices_b: &arr[22],
            token_infos: &arr[23],
            token_program: &arr[24],
            token_program2022: &arr[25],
            token_a_token_program: &arr[26],
            token_b_token_program: &arr[27],
            memo_program: &arr[28],
            pool_program: &arr[29],
            instruction_sysvar_account: &arr[30],
            event_authority: &arr[31],
        }
    }
}
pub const SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_DISCM: [u8; 8usize] = [
    118, 134, 143, 192, 188, 21, 131, 17,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SingleTokenDepositAndInvestWithMinIxArgs {
    pub token_a_min_post_deposit_balance: u64,
    pub token_b_min_post_deposit_balance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SingleTokenDepositAndInvestWithMinIxData(
    pub SingleTokenDepositAndInvestWithMinIxArgs,
);
impl From<SingleTokenDepositAndInvestWithMinIxArgs>
for SingleTokenDepositAndInvestWithMinIxData {
    fn from(args: SingleTokenDepositAndInvestWithMinIxArgs) -> Self {
        Self(args)
    }
}
impl SingleTokenDepositAndInvestWithMinIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_a_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_b_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SingleTokenDepositAndInvestWithMinIxArgs {
                token_a_min_post_deposit_balance,
                token_b_min_post_deposit_balance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.token_a_min_post_deposit_balance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.token_b_min_post_deposit_balance,
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
pub fn single_token_deposit_and_invest_with_min_ix_with_program_id(
    program_id: Pubkey,
    keys: SingleTokenDepositAndInvestWithMinKeys,
    args: SingleTokenDepositAndInvestWithMinIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SINGLE_TOKEN_DEPOSIT_AND_INVEST_WITH_MIN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SingleTokenDepositAndInvestWithMinIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn single_token_deposit_and_invest_with_min_ix(
    keys: SingleTokenDepositAndInvestWithMinKeys,
    args: SingleTokenDepositAndInvestWithMinIxArgs,
) -> std::io::Result<Instruction> {
    single_token_deposit_and_invest_with_min_ix_with_program_id(
        YVAULTS_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn single_token_deposit_and_invest_with_min_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'_, '_>,
    args: SingleTokenDepositAndInvestWithMinIxArgs,
) -> ProgramResult {
    let keys: SingleTokenDepositAndInvestWithMinKeys = accounts.into();
    let ix = single_token_deposit_and_invest_with_min_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn single_token_deposit_and_invest_with_min_invoke(
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'_, '_>,
    args: SingleTokenDepositAndInvestWithMinIxArgs,
) -> ProgramResult {
    single_token_deposit_and_invest_with_min_invoke_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn single_token_deposit_and_invest_with_min_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'_, '_>,
    args: SingleTokenDepositAndInvestWithMinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SingleTokenDepositAndInvestWithMinKeys = accounts.into();
    let ix = single_token_deposit_and_invest_with_min_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn single_token_deposit_and_invest_with_min_invoke_signed(
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'_, '_>,
    args: SingleTokenDepositAndInvestWithMinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    single_token_deposit_and_invest_with_min_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn single_token_deposit_and_invest_with_min_verify_account_keys(
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'_, '_>,
    keys: SingleTokenDepositAndInvestWithMinKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (
            *accounts.raydium_protocol_position_or_base_vault_authority.key,
            keys.raydium_protocol_position_or_base_vault_authority,
        ),
        (*accounts.position_token_account.key, keys.position_token_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn single_token_deposit_and_invest_with_min_verify_writable_privileges<'me, 'info>(
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.strategy,
        accounts.pool,
        accounts.position,
        accounts.raydium_protocol_position_or_base_vault_authority,
        accounts.position_token_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.base_vault_authority,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.user_shares_ata,
        accounts.shares_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn single_token_deposit_and_invest_with_min_verify_signer_privileges<'me, 'info>(
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn single_token_deposit_and_invest_with_min_verify_account_privileges<'me, 'info>(
    accounts: SingleTokenDepositAndInvestWithMinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    single_token_deposit_and_invest_with_min_verify_writable_privileges(accounts)?;
    single_token_deposit_and_invest_with_min_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct SingleTokenDepositWithMinAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub shares_mint: &'me AccountInfo<'info>,
    pub shares_mint_authority: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SingleTokenDepositWithMinKeys {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub user_shares_ata: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub token_program: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<SingleTokenDepositWithMinAccounts<'_, '_>> for SingleTokenDepositWithMinKeys {
    fn from(accounts: SingleTokenDepositWithMinAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            shares_mint: *accounts.shares_mint.key,
            shares_mint_authority: *accounts.shares_mint_authority.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            token_program: *accounts.token_program.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<SingleTokenDepositWithMinKeys>
for [AccountMeta; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SingleTokenDepositWithMinKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN]>
for SingleTokenDepositWithMinKeys {
    fn from(pubkeys: [Pubkey; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            base_vault_authority: pubkeys[9],
            token_a_ata: pubkeys[10],
            token_b_ata: pubkeys[11],
            token_a_mint: pubkeys[12],
            token_b_mint: pubkeys[13],
            user_shares_ata: pubkeys[14],
            shares_mint: pubkeys[15],
            shares_mint_authority: pubkeys[16],
            scope_prices_a: pubkeys[17],
            scope_prices_b: pubkeys[18],
            token_infos: pubkeys[19],
            token_program: pubkeys[20],
            token_a_token_program: pubkeys[21],
            token_b_token_program: pubkeys[22],
            instruction_sysvar_account: pubkeys[23],
        }
    }
}
impl<'info> From<SingleTokenDepositWithMinAccounts<'_, 'info>>
for [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SingleTokenDepositWithMinAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.user_shares_ata.clone(),
            accounts.shares_mint.clone(),
            accounts.shares_mint_authority.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.token_program.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN]>
for SingleTokenDepositWithMinAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            base_vault_authority: &arr[9],
            token_a_ata: &arr[10],
            token_b_ata: &arr[11],
            token_a_mint: &arr[12],
            token_b_mint: &arr[13],
            user_shares_ata: &arr[14],
            shares_mint: &arr[15],
            shares_mint_authority: &arr[16],
            scope_prices_a: &arr[17],
            scope_prices_b: &arr[18],
            token_infos: &arr[19],
            token_program: &arr[20],
            token_a_token_program: &arr[21],
            token_b_token_program: &arr[22],
            instruction_sysvar_account: &arr[23],
        }
    }
}
pub const SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_DISCM: [u8; 8usize] = [
    250, 142, 102, 160, 72, 12, 83, 139,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SingleTokenDepositWithMinIxArgs {
    pub token_a_min_post_deposit_balance: u64,
    pub token_b_min_post_deposit_balance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SingleTokenDepositWithMinIxData(pub SingleTokenDepositWithMinIxArgs);
impl From<SingleTokenDepositWithMinIxArgs> for SingleTokenDepositWithMinIxData {
    fn from(args: SingleTokenDepositWithMinIxArgs) -> Self {
        Self(args)
    }
}
impl SingleTokenDepositWithMinIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_a_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_b_min_post_deposit_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SingleTokenDepositWithMinIxArgs {
                token_a_min_post_deposit_balance,
                token_b_min_post_deposit_balance,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.token_a_min_post_deposit_balance,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.token_b_min_post_deposit_balance,
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
pub fn single_token_deposit_with_min_ix_with_program_id(
    program_id: Pubkey,
    keys: SingleTokenDepositWithMinKeys,
    args: SingleTokenDepositWithMinIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SINGLE_TOKEN_DEPOSIT_WITH_MIN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SingleTokenDepositWithMinIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn single_token_deposit_with_min_ix(
    keys: SingleTokenDepositWithMinKeys,
    args: SingleTokenDepositWithMinIxArgs,
) -> std::io::Result<Instruction> {
    single_token_deposit_with_min_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn single_token_deposit_with_min_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SingleTokenDepositWithMinAccounts<'_, '_>,
    args: SingleTokenDepositWithMinIxArgs,
) -> ProgramResult {
    let keys: SingleTokenDepositWithMinKeys = accounts.into();
    let ix = single_token_deposit_with_min_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn single_token_deposit_with_min_invoke(
    accounts: SingleTokenDepositWithMinAccounts<'_, '_>,
    args: SingleTokenDepositWithMinIxArgs,
) -> ProgramResult {
    single_token_deposit_with_min_invoke_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn single_token_deposit_with_min_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SingleTokenDepositWithMinAccounts<'_, '_>,
    args: SingleTokenDepositWithMinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SingleTokenDepositWithMinKeys = accounts.into();
    let ix = single_token_deposit_with_min_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn single_token_deposit_with_min_invoke_signed(
    accounts: SingleTokenDepositWithMinAccounts<'_, '_>,
    args: SingleTokenDepositWithMinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    single_token_deposit_with_min_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn single_token_deposit_with_min_verify_account_keys(
    accounts: SingleTokenDepositWithMinAccounts<'_, '_>,
    keys: SingleTokenDepositWithMinKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.shares_mint.key, keys.shares_mint),
        (*accounts.shares_mint_authority.key, keys.shares_mint_authority),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn single_token_deposit_with_min_verify_writable_privileges<'me, 'info>(
    accounts: SingleTokenDepositWithMinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.user_shares_ata,
        accounts.shares_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn single_token_deposit_with_min_verify_signer_privileges<'me, 'info>(
    accounts: SingleTokenDepositWithMinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn single_token_deposit_with_min_verify_account_privileges<'me, 'info>(
    accounts: SingleTokenDepositWithMinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    single_token_deposit_with_min_verify_writable_privileges(accounts)?;
    single_token_deposit_with_min_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct FlashSwapUnevenVaultsStartAccounts<'me, 'info> {
    pub swapper: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub consensus_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashSwapUnevenVaultsStartKeys {
    pub swapper: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub consensus_account: Pubkey,
}
impl From<FlashSwapUnevenVaultsStartAccounts<'_, '_>>
for FlashSwapUnevenVaultsStartKeys {
    fn from(accounts: FlashSwapUnevenVaultsStartAccounts) -> Self {
        Self {
            swapper: *accounts.swapper.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            consensus_account: *accounts.consensus_account.key,
        }
    }
}
impl From<FlashSwapUnevenVaultsStartKeys>
for [AccountMeta; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashSwapUnevenVaultsStartKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swapper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consensus_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN]>
for FlashSwapUnevenVaultsStartKeys {
    fn from(pubkeys: [Pubkey; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swapper: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            token_a_vault: pubkeys[3],
            token_b_vault: pubkeys[4],
            token_a_ata: pubkeys[5],
            token_b_ata: pubkeys[6],
            base_vault_authority: pubkeys[7],
            pool: pubkeys[8],
            position: pubkeys[9],
            scope_prices_a: pubkeys[10],
            scope_prices_b: pubkeys[11],
            token_infos: pubkeys[12],
            tick_array_lower: pubkeys[13],
            tick_array_upper: pubkeys[14],
            token_a_mint: pubkeys[15],
            token_b_mint: pubkeys[16],
            token_a_token_program: pubkeys[17],
            token_b_token_program: pubkeys[18],
            instruction_sysvar_account: pubkeys[19],
            consensus_account: pubkeys[20],
        }
    }
}
impl<'info> From<FlashSwapUnevenVaultsStartAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashSwapUnevenVaultsStartAccounts<'_, 'info>) -> Self {
        [
            accounts.swapper.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.consensus_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN]>
for FlashSwapUnevenVaultsStartAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swapper: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            token_a_vault: &arr[3],
            token_b_vault: &arr[4],
            token_a_ata: &arr[5],
            token_b_ata: &arr[6],
            base_vault_authority: &arr[7],
            pool: &arr[8],
            position: &arr[9],
            scope_prices_a: &arr[10],
            scope_prices_b: &arr[11],
            token_infos: &arr[12],
            tick_array_lower: &arr[13],
            tick_array_upper: &arr[14],
            token_a_mint: &arr[15],
            token_b_mint: &arr[16],
            token_a_token_program: &arr[17],
            token_b_token_program: &arr[18],
            instruction_sysvar_account: &arr[19],
            consensus_account: &arr[20],
        }
    }
}
pub const FLASH_SWAP_UNEVEN_VAULTS_START_IX_DISCM: [u8; 8usize] = [
    129, 111, 174, 12, 10, 60, 149, 193,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashSwapUnevenVaultsStartIxArgs {
    pub amount: u64,
    pub a_to_b: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashSwapUnevenVaultsStartIxData(pub FlashSwapUnevenVaultsStartIxArgs);
impl From<FlashSwapUnevenVaultsStartIxArgs> for FlashSwapUnevenVaultsStartIxData {
    fn from(args: FlashSwapUnevenVaultsStartIxArgs) -> Self {
        Self(args)
    }
}
impl FlashSwapUnevenVaultsStartIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_SWAP_UNEVEN_VAULTS_START_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FlashSwapUnevenVaultsStartIxArgs {
                amount,
                a_to_b,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_SWAP_UNEVEN_VAULTS_START_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_swap_uneven_vaults_start_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashSwapUnevenVaultsStartKeys,
    args: FlashSwapUnevenVaultsStartIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_SWAP_UNEVEN_VAULTS_START_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: FlashSwapUnevenVaultsStartIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_swap_uneven_vaults_start_ix(
    keys: FlashSwapUnevenVaultsStartKeys,
    args: FlashSwapUnevenVaultsStartIxArgs,
) -> std::io::Result<Instruction> {
    flash_swap_uneven_vaults_start_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn flash_swap_uneven_vaults_start_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashSwapUnevenVaultsStartAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsStartIxArgs,
) -> ProgramResult {
    let keys: FlashSwapUnevenVaultsStartKeys = accounts.into();
    let ix = flash_swap_uneven_vaults_start_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_swap_uneven_vaults_start_invoke(
    accounts: FlashSwapUnevenVaultsStartAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsStartIxArgs,
) -> ProgramResult {
    flash_swap_uneven_vaults_start_invoke_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn flash_swap_uneven_vaults_start_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashSwapUnevenVaultsStartAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsStartIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashSwapUnevenVaultsStartKeys = accounts.into();
    let ix = flash_swap_uneven_vaults_start_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_swap_uneven_vaults_start_invoke_signed(
    accounts: FlashSwapUnevenVaultsStartAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsStartIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_swap_uneven_vaults_start_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_swap_uneven_vaults_start_verify_account_keys(
    accounts: FlashSwapUnevenVaultsStartAccounts<'_, '_>,
    keys: FlashSwapUnevenVaultsStartKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swapper.key, keys.swapper),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.consensus_account.key, keys.consensus_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_swap_uneven_vaults_start_verify_writable_privileges<'me, 'info>(
    accounts: FlashSwapUnevenVaultsStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swapper,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.base_vault_authority,
        accounts.pool,
        accounts.position,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_swap_uneven_vaults_start_verify_signer_privileges<'me, 'info>(
    accounts: FlashSwapUnevenVaultsStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.swapper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_swap_uneven_vaults_start_verify_account_privileges<'me, 'info>(
    accounts: FlashSwapUnevenVaultsStartAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_swap_uneven_vaults_start_verify_writable_privileges(accounts)?;
    flash_swap_uneven_vaults_start_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct FlashSwapUnevenVaultsEndAccounts<'me, 'info> {
    pub swapper: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_ata: &'me AccountInfo<'info>,
    pub token_b_ata: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
    pub consensus_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashSwapUnevenVaultsEndKeys {
    pub swapper: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
    pub consensus_account: Pubkey,
}
impl From<FlashSwapUnevenVaultsEndAccounts<'_, '_>> for FlashSwapUnevenVaultsEndKeys {
    fn from(accounts: FlashSwapUnevenVaultsEndAccounts) -> Self {
        Self {
            swapper: *accounts.swapper.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_ata: *accounts.token_a_ata.key,
            token_b_ata: *accounts.token_b_ata.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
            consensus_account: *accounts.consensus_account.key,
        }
    }
}
impl From<FlashSwapUnevenVaultsEndKeys>
for [AccountMeta; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashSwapUnevenVaultsEndKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swapper,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.consensus_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN]>
for FlashSwapUnevenVaultsEndKeys {
    fn from(pubkeys: [Pubkey; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swapper: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            token_a_vault: pubkeys[3],
            token_b_vault: pubkeys[4],
            token_a_ata: pubkeys[5],
            token_b_ata: pubkeys[6],
            base_vault_authority: pubkeys[7],
            pool: pubkeys[8],
            position: pubkeys[9],
            scope_prices_a: pubkeys[10],
            scope_prices_b: pubkeys[11],
            token_infos: pubkeys[12],
            tick_array_lower: pubkeys[13],
            tick_array_upper: pubkeys[14],
            token_a_mint: pubkeys[15],
            token_b_mint: pubkeys[16],
            token_a_token_program: pubkeys[17],
            token_b_token_program: pubkeys[18],
            instruction_sysvar_account: pubkeys[19],
            consensus_account: pubkeys[20],
        }
    }
}
impl<'info> From<FlashSwapUnevenVaultsEndAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashSwapUnevenVaultsEndAccounts<'_, 'info>) -> Self {
        [
            accounts.swapper.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_ata.clone(),
            accounts.token_b_ata.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
            accounts.consensus_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN]>
for FlashSwapUnevenVaultsEndAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swapper: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            token_a_vault: &arr[3],
            token_b_vault: &arr[4],
            token_a_ata: &arr[5],
            token_b_ata: &arr[6],
            base_vault_authority: &arr[7],
            pool: &arr[8],
            position: &arr[9],
            scope_prices_a: &arr[10],
            scope_prices_b: &arr[11],
            token_infos: &arr[12],
            tick_array_lower: &arr[13],
            tick_array_upper: &arr[14],
            token_a_mint: &arr[15],
            token_b_mint: &arr[16],
            token_a_token_program: &arr[17],
            token_b_token_program: &arr[18],
            instruction_sysvar_account: &arr[19],
            consensus_account: &arr[20],
        }
    }
}
pub const FLASH_SWAP_UNEVEN_VAULTS_END_IX_DISCM: [u8; 8usize] = [
    226, 2, 190, 101, 202, 132, 156, 20,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlashSwapUnevenVaultsEndIxArgs {
    pub min_repay_amount: u64,
    pub amount_to_leave_to_user: u64,
    pub a_to_b: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashSwapUnevenVaultsEndIxData(pub FlashSwapUnevenVaultsEndIxArgs);
impl From<FlashSwapUnevenVaultsEndIxArgs> for FlashSwapUnevenVaultsEndIxData {
    fn from(args: FlashSwapUnevenVaultsEndIxArgs) -> Self {
        Self(args)
    }
}
impl FlashSwapUnevenVaultsEndIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_SWAP_UNEVEN_VAULTS_END_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let min_repay_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_to_leave_to_user: u64 = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(FlashSwapUnevenVaultsEndIxArgs {
                min_repay_amount,
                amount_to_leave_to_user,
                a_to_b,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_SWAP_UNEVEN_VAULTS_END_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.min_repay_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_to_leave_to_user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_swap_uneven_vaults_end_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashSwapUnevenVaultsEndKeys,
    args: FlashSwapUnevenVaultsEndIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_SWAP_UNEVEN_VAULTS_END_IX_ACCOUNTS_LEN] = keys.into();
    let data: FlashSwapUnevenVaultsEndIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_swap_uneven_vaults_end_ix(
    keys: FlashSwapUnevenVaultsEndKeys,
    args: FlashSwapUnevenVaultsEndIxArgs,
) -> std::io::Result<Instruction> {
    flash_swap_uneven_vaults_end_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn flash_swap_uneven_vaults_end_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashSwapUnevenVaultsEndAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsEndIxArgs,
) -> ProgramResult {
    let keys: FlashSwapUnevenVaultsEndKeys = accounts.into();
    let ix = flash_swap_uneven_vaults_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_swap_uneven_vaults_end_invoke(
    accounts: FlashSwapUnevenVaultsEndAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsEndIxArgs,
) -> ProgramResult {
    flash_swap_uneven_vaults_end_invoke_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn flash_swap_uneven_vaults_end_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashSwapUnevenVaultsEndAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashSwapUnevenVaultsEndKeys = accounts.into();
    let ix = flash_swap_uneven_vaults_end_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_swap_uneven_vaults_end_invoke_signed(
    accounts: FlashSwapUnevenVaultsEndAccounts<'_, '_>,
    args: FlashSwapUnevenVaultsEndIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_swap_uneven_vaults_end_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_swap_uneven_vaults_end_verify_account_keys(
    accounts: FlashSwapUnevenVaultsEndAccounts<'_, '_>,
    keys: FlashSwapUnevenVaultsEndKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swapper.key, keys.swapper),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_ata.key, keys.token_a_ata),
        (*accounts.token_b_ata.key, keys.token_b_ata),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
        (*accounts.consensus_account.key, keys.consensus_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_swap_uneven_vaults_end_verify_writable_privileges<'me, 'info>(
    accounts: FlashSwapUnevenVaultsEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swapper,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.token_a_ata,
        accounts.token_b_ata,
        accounts.base_vault_authority,
        accounts.pool,
        accounts.position,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_swap_uneven_vaults_end_verify_signer_privileges<'me, 'info>(
    accounts: FlashSwapUnevenVaultsEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.swapper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_swap_uneven_vaults_end_verify_account_privileges<'me, 'info>(
    accounts: FlashSwapUnevenVaultsEndAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_swap_uneven_vaults_end_verify_writable_privileges(accounts)?;
    flash_swap_uneven_vaults_end_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EMERGENCY_SWAP_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct EmergencySwapAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub pool_token_vault_a: &'me AccountInfo<'info>,
    pub pool_token_vault_b: &'me AccountInfo<'info>,
    pub tick_array0: &'me AccountInfo<'info>,
    pub tick_array1: &'me AccountInfo<'info>,
    pub tick_array2: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub pool_program: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmergencySwapKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub pool_token_vault_a: Pubkey,
    pub pool_token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
    pub pool_program: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
    pub token_infos: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
}
impl From<EmergencySwapAccounts<'_, '_>> for EmergencySwapKeys {
    fn from(accounts: EmergencySwapAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            pool_token_vault_a: *accounts.pool_token_vault_a.key,
            pool_token_vault_b: *accounts.pool_token_vault_b.key,
            tick_array0: *accounts.tick_array0.key,
            tick_array1: *accounts.tick_array1.key,
            tick_array2: *accounts.tick_array2.key,
            oracle: *accounts.oracle.key,
            pool_program: *accounts.pool_program.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
            token_infos: *accounts.token_infos.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<EmergencySwapKeys> for [AccountMeta; EMERGENCY_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: EmergencySwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EMERGENCY_SWAP_IX_ACCOUNTS_LEN]> for EmergencySwapKeys {
    fn from(pubkeys: [Pubkey; EMERGENCY_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            token_a_mint: pubkeys[3],
            token_b_mint: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            base_vault_authority: pubkeys[7],
            pool: pubkeys[8],
            position: pubkeys[9],
            pool_token_vault_a: pubkeys[10],
            pool_token_vault_b: pubkeys[11],
            tick_array0: pubkeys[12],
            tick_array1: pubkeys[13],
            tick_array2: pubkeys[14],
            oracle: pubkeys[15],
            pool_program: pubkeys[16],
            scope_prices_a: pubkeys[17],
            scope_prices_b: pubkeys[18],
            token_infos: pubkeys[19],
            token_a_token_program: pubkeys[20],
            token_b_token_program: pubkeys[21],
            memo_program: pubkeys[22],
        }
    }
}
impl<'info> From<EmergencySwapAccounts<'_, 'info>>
for [AccountInfo<'info>; EMERGENCY_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: EmergencySwapAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.pool_token_vault_a.clone(),
            accounts.pool_token_vault_b.clone(),
            accounts.tick_array0.clone(),
            accounts.tick_array1.clone(),
            accounts.tick_array2.clone(),
            accounts.oracle.clone(),
            accounts.pool_program.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
            accounts.token_infos.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EMERGENCY_SWAP_IX_ACCOUNTS_LEN]>
for EmergencySwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EMERGENCY_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            token_a_mint: &arr[3],
            token_b_mint: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            base_vault_authority: &arr[7],
            pool: &arr[8],
            position: &arr[9],
            pool_token_vault_a: &arr[10],
            pool_token_vault_b: &arr[11],
            tick_array0: &arr[12],
            tick_array1: &arr[13],
            tick_array2: &arr[14],
            oracle: &arr[15],
            pool_program: &arr[16],
            scope_prices_a: &arr[17],
            scope_prices_b: &arr[18],
            token_infos: &arr[19],
            token_a_token_program: &arr[20],
            token_b_token_program: &arr[21],
            memo_program: &arr[22],
        }
    }
}
pub const EMERGENCY_SWAP_IX_DISCM: [u8; 8usize] = [73, 226, 248, 215, 5, 197, 211, 229];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmergencySwapIxArgs {
    pub a_to_b: bool,
    pub target_limit_bps: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EmergencySwapIxData(pub EmergencySwapIxArgs);
impl From<EmergencySwapIxArgs> for EmergencySwapIxData {
    fn from(args: EmergencySwapIxArgs) -> Self {
        Self(args)
    }
}
impl EmergencySwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EMERGENCY_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let target_limit_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(EmergencySwapIxArgs {
                a_to_b,
                target_limit_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EMERGENCY_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.target_limit_bps, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn emergency_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: EmergencySwapKeys,
    args: EmergencySwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EMERGENCY_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: EmergencySwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn emergency_swap_ix(
    keys: EmergencySwapKeys,
    args: EmergencySwapIxArgs,
) -> std::io::Result<Instruction> {
    emergency_swap_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn emergency_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EmergencySwapAccounts<'_, '_>,
    args: EmergencySwapIxArgs,
) -> ProgramResult {
    let keys: EmergencySwapKeys = accounts.into();
    let ix = emergency_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn emergency_swap_invoke(
    accounts: EmergencySwapAccounts<'_, '_>,
    args: EmergencySwapIxArgs,
) -> ProgramResult {
    emergency_swap_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn emergency_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EmergencySwapAccounts<'_, '_>,
    args: EmergencySwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EmergencySwapKeys = accounts.into();
    let ix = emergency_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn emergency_swap_invoke_signed(
    accounts: EmergencySwapAccounts<'_, '_>,
    args: EmergencySwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    emergency_swap_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn emergency_swap_verify_account_keys(
    accounts: EmergencySwapAccounts<'_, '_>,
    keys: EmergencySwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.pool_token_vault_a.key, keys.pool_token_vault_a),
        (*accounts.pool_token_vault_b.key, keys.pool_token_vault_b),
        (*accounts.tick_array0.key, keys.tick_array0),
        (*accounts.tick_array1.key, keys.tick_array1),
        (*accounts.tick_array2.key, keys.tick_array2),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.pool_program.key, keys.pool_program),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn emergency_swap_verify_writable_privileges<'me, 'info>(
    accounts: EmergencySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.strategy,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.base_vault_authority,
        accounts.pool,
        accounts.position,
        accounts.pool_token_vault_a,
        accounts.pool_token_vault_b,
        accounts.tick_array0,
        accounts.tick_array1,
        accounts.tick_array2,
        accounts.oracle,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn emergency_swap_verify_signer_privileges<'me, 'info>(
    accounts: EmergencySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn emergency_swap_verify_account_privileges<'me, 'info>(
    accounts: EmergencySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    emergency_swap_verify_writable_privileges(accounts)?;
    emergency_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromTreasuryAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub treasury_fee_vault: &'me AccountInfo<'info>,
    pub treasury_fee_vault_authority: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromTreasuryKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub mint: Pubkey,
    pub treasury_fee_vault: Pubkey,
    pub treasury_fee_vault_authority: Pubkey,
    pub token_account: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFromTreasuryAccounts<'_, '_>> for WithdrawFromTreasuryKeys {
    fn from(accounts: WithdrawFromTreasuryAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            mint: *accounts.mint.key,
            treasury_fee_vault: *accounts.treasury_fee_vault.key,
            treasury_fee_vault_authority: *accounts.treasury_fee_vault_authority.key,
            token_account: *accounts.token_account.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFromTreasuryKeys>
for [AccountMeta; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromTreasuryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault_authority,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.rent,
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
impl From<[Pubkey; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN]>
for WithdrawFromTreasuryKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            mint: pubkeys[2],
            treasury_fee_vault: pubkeys[3],
            treasury_fee_vault_authority: pubkeys[4],
            token_account: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawFromTreasuryAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromTreasuryAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.mint.clone(),
            accounts.treasury_fee_vault.clone(),
            accounts.treasury_fee_vault_authority.clone(),
            accounts.token_account.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN]>
for WithdrawFromTreasuryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            mint: &arr[2],
            treasury_fee_vault: &arr[3],
            treasury_fee_vault_authority: &arr[4],
            token_account: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const WITHDRAW_FROM_TREASURY_IX_DISCM: [u8; 8usize] = [
    0, 164, 86, 76, 56, 72, 12, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromTreasuryIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromTreasuryIxData(pub WithdrawFromTreasuryIxArgs);
impl From<WithdrawFromTreasuryIxArgs> for WithdrawFromTreasuryIxData {
    fn from(args: WithdrawFromTreasuryIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromTreasuryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_TREASURY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawFromTreasuryIxArgs {
                amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_TREASURY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_treasury_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromTreasuryKeys,
    args: WithdrawFromTreasuryIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromTreasuryIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_treasury_ix(
    keys: WithdrawFromTreasuryKeys,
    args: WithdrawFromTreasuryIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_treasury_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_treasury_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromTreasuryAccounts<'_, '_>,
    args: WithdrawFromTreasuryIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromTreasuryKeys = accounts.into();
    let ix = withdraw_from_treasury_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_treasury_invoke(
    accounts: WithdrawFromTreasuryAccounts<'_, '_>,
    args: WithdrawFromTreasuryIxArgs,
) -> ProgramResult {
    withdraw_from_treasury_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_treasury_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromTreasuryAccounts<'_, '_>,
    args: WithdrawFromTreasuryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromTreasuryKeys = accounts.into();
    let ix = withdraw_from_treasury_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_treasury_invoke_signed(
    accounts: WithdrawFromTreasuryAccounts<'_, '_>,
    args: WithdrawFromTreasuryIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_treasury_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_treasury_verify_account_keys(
    accounts: WithdrawFromTreasuryAccounts<'_, '_>,
    keys: WithdrawFromTreasuryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.treasury_fee_vault.key, keys.treasury_fee_vault),
        (*accounts.treasury_fee_vault_authority.key, keys.treasury_fee_vault_authority),
        (*accounts.token_account.key, keys.token_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_treasury_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_authority,
        accounts.treasury_fee_vault,
        accounts.treasury_fee_vault_authority,
        accounts.token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_treasury_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_treasury_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_treasury_verify_writable_privileges(accounts)?;
    withdraw_from_treasury_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct PermisionlessWithdrawFromTreasuryAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub treasury_fee_vault: &'me AccountInfo<'info>,
    pub treasury_fee_vault_authority: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PermisionlessWithdrawFromTreasuryKeys {
    pub signer: Pubkey,
    pub global_config: Pubkey,
    pub mint: Pubkey,
    pub treasury_fee_vault: Pubkey,
    pub treasury_fee_vault_authority: Pubkey,
    pub token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<PermisionlessWithdrawFromTreasuryAccounts<'_, '_>>
for PermisionlessWithdrawFromTreasuryKeys {
    fn from(accounts: PermisionlessWithdrawFromTreasuryAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            global_config: *accounts.global_config.key,
            mint: *accounts.mint.key,
            treasury_fee_vault: *accounts.treasury_fee_vault.key,
            treasury_fee_vault_authority: *accounts.treasury_fee_vault_authority.key,
            token_account: *accounts.token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PermisionlessWithdrawFromTreasuryKeys>
for [AccountMeta; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(keys: PermisionlessWithdrawFromTreasuryKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_fee_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
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
impl From<[Pubkey; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN]>
for PermisionlessWithdrawFromTreasuryKeys {
    fn from(
        pubkeys: [Pubkey; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: pubkeys[0],
            global_config: pubkeys[1],
            mint: pubkeys[2],
            treasury_fee_vault: pubkeys[3],
            treasury_fee_vault_authority: pubkeys[4],
            token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<PermisionlessWithdrawFromTreasuryAccounts<'_, 'info>>
for [AccountInfo<'info>; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN] {
    fn from(accounts: PermisionlessWithdrawFromTreasuryAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.global_config.clone(),
            accounts.mint.clone(),
            accounts.treasury_fee_vault.clone(),
            accounts.treasury_fee_vault_authority.clone(),
            accounts.token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN]>
for PermisionlessWithdrawFromTreasuryAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            global_config: &arr[1],
            mint: &arr[2],
            treasury_fee_vault: &arr[3],
            treasury_fee_vault_authority: &arr[4],
            token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_DISCM: [u8; 8usize] = [
    167, 36, 32, 79, 97, 170, 183, 108,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PermisionlessWithdrawFromTreasuryIxData;
impl PermisionlessWithdrawFromTreasuryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn permisionless_withdraw_from_treasury_ix_with_program_id(
    program_id: Pubkey,
    keys: PermisionlessWithdrawFromTreasuryKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERMISIONLESS_WITHDRAW_FROM_TREASURY_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PermisionlessWithdrawFromTreasuryIxData.try_to_vec()?,
    })
}
pub fn permisionless_withdraw_from_treasury_ix(
    keys: PermisionlessWithdrawFromTreasuryKeys,
) -> std::io::Result<Instruction> {
    permisionless_withdraw_from_treasury_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn permisionless_withdraw_from_treasury_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PermisionlessWithdrawFromTreasuryKeys = accounts.into();
    let ix = permisionless_withdraw_from_treasury_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn permisionless_withdraw_from_treasury_invoke(
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'_, '_>,
) -> ProgramResult {
    permisionless_withdraw_from_treasury_invoke_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
    )
}
pub fn permisionless_withdraw_from_treasury_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PermisionlessWithdrawFromTreasuryKeys = accounts.into();
    let ix = permisionless_withdraw_from_treasury_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn permisionless_withdraw_from_treasury_invoke_signed(
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    permisionless_withdraw_from_treasury_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn permisionless_withdraw_from_treasury_verify_account_keys(
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'_, '_>,
    keys: PermisionlessWithdrawFromTreasuryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.treasury_fee_vault.key, keys.treasury_fee_vault),
        (*accounts.treasury_fee_vault_authority.key, keys.treasury_fee_vault_authority),
        (*accounts.token_account.key, keys.token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn permisionless_withdraw_from_treasury_verify_writable_privileges<'me, 'info>(
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.treasury_fee_vault,
        accounts.treasury_fee_vault_authority,
        accounts.token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn permisionless_withdraw_from_treasury_verify_signer_privileges<'me, 'info>(
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn permisionless_withdraw_from_treasury_verify_account_privileges<'me, 'info>(
    accounts: PermisionlessWithdrawFromTreasuryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    permisionless_withdraw_from_treasury_verify_writable_privileges(accounts)?;
    permisionless_withdraw_from_treasury_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromTopupAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub topup_vault: &'me AccountInfo<'info>,
    pub system: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromTopupKeys {
    pub admin_authority: Pubkey,
    pub topup_vault: Pubkey,
    pub system: Pubkey,
}
impl From<WithdrawFromTopupAccounts<'_, '_>> for WithdrawFromTopupKeys {
    fn from(accounts: WithdrawFromTopupAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            topup_vault: *accounts.topup_vault.key,
            system: *accounts.system.key,
        }
    }
}
impl From<WithdrawFromTopupKeys> for [AccountMeta; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromTopupKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.topup_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN]> for WithdrawFromTopupKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            topup_vault: pubkeys[1],
            system: pubkeys[2],
        }
    }
}
impl<'info> From<WithdrawFromTopupAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromTopupAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.topup_vault.clone(),
            accounts.system.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN]>
for WithdrawFromTopupAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            topup_vault: &arr[1],
            system: &arr[2],
        }
    }
}
pub const WITHDRAW_FROM_TOPUP_IX_DISCM: [u8; 8usize] = [
    95, 227, 138, 220, 240, 95, 150, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromTopupIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromTopupIxData(pub WithdrawFromTopupIxArgs);
impl From<WithdrawFromTopupIxArgs> for WithdrawFromTopupIxData {
    fn from(args: WithdrawFromTopupIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromTopupIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_TOPUP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(WithdrawFromTopupIxArgs { amount }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_TOPUP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_topup_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromTopupKeys,
    args: WithdrawFromTopupIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_TOPUP_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromTopupIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_topup_ix(
    keys: WithdrawFromTopupKeys,
    args: WithdrawFromTopupIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_topup_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_topup_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromTopupAccounts<'_, '_>,
    args: WithdrawFromTopupIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromTopupKeys = accounts.into();
    let ix = withdraw_from_topup_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_topup_invoke(
    accounts: WithdrawFromTopupAccounts<'_, '_>,
    args: WithdrawFromTopupIxArgs,
) -> ProgramResult {
    withdraw_from_topup_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_topup_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromTopupAccounts<'_, '_>,
    args: WithdrawFromTopupIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromTopupKeys = accounts.into();
    let ix = withdraw_from_topup_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_topup_invoke_signed(
    accounts: WithdrawFromTopupAccounts<'_, '_>,
    args: WithdrawFromTopupIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_topup_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_topup_verify_account_keys(
    accounts: WithdrawFromTopupAccounts<'_, '_>,
    keys: WithdrawFromTopupKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.topup_vault.key, keys.topup_vault),
        (*accounts.system.key, keys.system),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_topup_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromTopupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.topup_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_topup_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromTopupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_topup_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromTopupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_topup_verify_writable_privileges(accounts)?;
    withdraw_from_topup_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_POOL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ChangePoolAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub old_position: &'me AccountInfo<'info>,
    pub base_vault_authority: &'me AccountInfo<'info>,
    pub new_pool: &'me AccountInfo<'info>,
    pub strategy_reward_vault0_or_base_vault_authority: &'me AccountInfo<'info>,
    pub strategy_reward_vault1_or_base_vault_authority: &'me AccountInfo<'info>,
    pub strategy_reward_vault2_or_base_vault_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangePoolKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub old_position: Pubkey,
    pub base_vault_authority: Pubkey,
    pub new_pool: Pubkey,
    pub strategy_reward_vault0_or_base_vault_authority: Pubkey,
    pub strategy_reward_vault1_or_base_vault_authority: Pubkey,
    pub strategy_reward_vault2_or_base_vault_authority: Pubkey,
}
impl From<ChangePoolAccounts<'_, '_>> for ChangePoolKeys {
    fn from(accounts: ChangePoolAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
            old_position: *accounts.old_position.key,
            base_vault_authority: *accounts.base_vault_authority.key,
            new_pool: *accounts.new_pool.key,
            strategy_reward_vault0_or_base_vault_authority: *accounts
                .strategy_reward_vault0_or_base_vault_authority
                .key,
            strategy_reward_vault1_or_base_vault_authority: *accounts
                .strategy_reward_vault1_or_base_vault_authority
                .key,
            strategy_reward_vault2_or_base_vault_authority: *accounts
                .strategy_reward_vault2_or_base_vault_authority
                .key,
        }
    }
}
impl From<ChangePoolKeys> for [AccountMeta; CHANGE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_reward_vault0_or_base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_reward_vault1_or_base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy_reward_vault2_or_base_vault_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_POOL_IX_ACCOUNTS_LEN]> for ChangePoolKeys {
    fn from(pubkeys: [Pubkey; CHANGE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
            old_position: pubkeys[3],
            base_vault_authority: pubkeys[4],
            new_pool: pubkeys[5],
            strategy_reward_vault0_or_base_vault_authority: pubkeys[6],
            strategy_reward_vault1_or_base_vault_authority: pubkeys[7],
            strategy_reward_vault2_or_base_vault_authority: pubkeys[8],
        }
    }
}
impl<'info> From<ChangePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
            accounts.old_position.clone(),
            accounts.base_vault_authority.clone(),
            accounts.new_pool.clone(),
            accounts.strategy_reward_vault0_or_base_vault_authority.clone(),
            accounts.strategy_reward_vault1_or_base_vault_authority.clone(),
            accounts.strategy_reward_vault2_or_base_vault_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_POOL_IX_ACCOUNTS_LEN]>
for ChangePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
            old_position: &arr[3],
            base_vault_authority: &arr[4],
            new_pool: &arr[5],
            strategy_reward_vault0_or_base_vault_authority: &arr[6],
            strategy_reward_vault1_or_base_vault_authority: &arr[7],
            strategy_reward_vault2_or_base_vault_authority: &arr[8],
        }
    }
}
pub const CHANGE_POOL_IX_DISCM: [u8; 8usize] = [141, 221, 123, 235, 35, 9, 145, 201];
#[derive(Clone, Debug, PartialEq)]
pub struct ChangePoolIxData;
impl ChangePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_POOL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_POOL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangePoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ChangePoolIxData.try_to_vec()?,
    })
}
pub fn change_pool_ix(keys: ChangePoolKeys) -> std::io::Result<Instruction> {
    change_pool_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn change_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangePoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ChangePoolKeys = accounts.into();
    let ix = change_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_pool_invoke(accounts: ChangePoolAccounts<'_, '_>) -> ProgramResult {
    change_pool_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn change_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangePoolKeys = accounts.into();
    let ix = change_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_pool_invoke_signed(
    accounts: ChangePoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_pool_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, seeds)
}
pub fn change_pool_verify_account_keys(
    accounts: ChangePoolAccounts<'_, '_>,
    keys: ChangePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.old_position.key, keys.old_position),
        (*accounts.base_vault_authority.key, keys.base_vault_authority),
        (*accounts.new_pool.key, keys.new_pool),
        (
            *accounts.strategy_reward_vault0_or_base_vault_authority.key,
            keys.strategy_reward_vault0_or_base_vault_authority,
        ),
        (
            *accounts.strategy_reward_vault1_or_base_vault_authority.key,
            keys.strategy_reward_vault1_or_base_vault_authority,
        ),
        (
            *accounts.strategy_reward_vault2_or_base_vault_authority.key,
            keys.strategy_reward_vault2_or_base_vault_authority,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_pool_verify_writable_privileges<'me, 'info>(
    accounts: ChangePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.strategy] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_pool_verify_signer_privileges<'me, 'info>(
    accounts: ChangePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_pool_verify_account_privileges<'me, 'info>(
    accounts: ChangePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_pool_verify_writable_privileges(accounts)?;
    change_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseProgramAccountAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub program_data: &'me AccountInfo<'info>,
    pub closing_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseProgramAccountKeys {
    pub admin_authority: Pubkey,
    pub program: Pubkey,
    pub program_data: Pubkey,
    pub closing_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<CloseProgramAccountAccounts<'_, '_>> for CloseProgramAccountKeys {
    fn from(accounts: CloseProgramAccountAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            program: *accounts.program.key,
            program_data: *accounts.program_data.key,
            closing_account: *accounts.closing_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CloseProgramAccountKeys>
for [AccountMeta; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseProgramAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.closing_account,
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
impl From<[Pubkey; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN]> for CloseProgramAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            program: pubkeys[1],
            program_data: pubkeys[2],
            closing_account: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseProgramAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseProgramAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.program.clone(),
            accounts.program_data.clone(),
            accounts.closing_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN]>
for CloseProgramAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            program: &arr[1],
            program_data: &arr[2],
            closing_account: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CLOSE_PROGRAM_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    245, 14, 192, 211, 99, 42, 170, 187,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseProgramAccountIxData;
impl CloseProgramAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PROGRAM_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PROGRAM_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_program_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseProgramAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PROGRAM_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseProgramAccountIxData.try_to_vec()?,
    })
}
pub fn close_program_account_ix(
    keys: CloseProgramAccountKeys,
) -> std::io::Result<Instruction> {
    close_program_account_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn close_program_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseProgramAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseProgramAccountKeys = accounts.into();
    let ix = close_program_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_program_account_invoke(
    accounts: CloseProgramAccountAccounts<'_, '_>,
) -> ProgramResult {
    close_program_account_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn close_program_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseProgramAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseProgramAccountKeys = accounts.into();
    let ix = close_program_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_program_account_invoke_signed(
    accounts: CloseProgramAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_program_account_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_program_account_verify_account_keys(
    accounts: CloseProgramAccountAccounts<'_, '_>,
    keys: CloseProgramAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.program.key, keys.program),
        (*accounts.program_data.key, keys.program_data),
        (*accounts.closing_account.key, keys.closing_account),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_program_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseProgramAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.closing_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_program_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseProgramAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_program_account_verify_account_privileges<'me, 'info>(
    accounts: CloseProgramAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_program_account_verify_writable_privileges(accounts)?;
    close_program_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ORCA_SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct OrcaSwapAccounts<'me, 'info> {
    pub funder: &'me AccountInfo<'info>,
    pub token_a_token_program: &'me AccountInfo<'info>,
    pub token_b_token_program: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub tick_array0: &'me AccountInfo<'info>,
    pub tick_array1: &'me AccountInfo<'info>,
    pub tick_array2: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub whirlpool_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OrcaSwapKeys {
    pub funder: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub memo_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
    pub whirlpool_program: Pubkey,
}
impl From<OrcaSwapAccounts<'_, '_>> for OrcaSwapKeys {
    fn from(accounts: OrcaSwapAccounts) -> Self {
        Self {
            funder: *accounts.funder.key,
            token_a_token_program: *accounts.token_a_token_program.key,
            token_b_token_program: *accounts.token_b_token_program.key,
            memo_program: *accounts.memo_program.key,
            token_authority: *accounts.token_authority.key,
            whirlpool: *accounts.whirlpool.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_b: *accounts.token_vault_b.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            tick_array0: *accounts.tick_array0.key,
            tick_array1: *accounts.tick_array1.key,
            tick_array2: *accounts.tick_array2.key,
            oracle: *accounts.oracle.key,
            whirlpool_program: *accounts.whirlpool_program.key,
        }
    }
}
impl From<OrcaSwapKeys> for [AccountMeta; ORCA_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: OrcaSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array2,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ORCA_SWAP_IX_ACCOUNTS_LEN]> for OrcaSwapKeys {
    fn from(pubkeys: [Pubkey; ORCA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: pubkeys[0],
            token_a_token_program: pubkeys[1],
            token_b_token_program: pubkeys[2],
            memo_program: pubkeys[3],
            token_authority: pubkeys[4],
            whirlpool: pubkeys[5],
            token_owner_account_a: pubkeys[6],
            token_vault_a: pubkeys[7],
            token_owner_account_b: pubkeys[8],
            token_vault_b: pubkeys[9],
            token_mint_a: pubkeys[10],
            token_mint_b: pubkeys[11],
            tick_array0: pubkeys[12],
            tick_array1: pubkeys[13],
            tick_array2: pubkeys[14],
            oracle: pubkeys[15],
            whirlpool_program: pubkeys[16],
        }
    }
}
impl<'info> From<OrcaSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; ORCA_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: OrcaSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.funder.clone(),
            accounts.token_a_token_program.clone(),
            accounts.token_b_token_program.clone(),
            accounts.memo_program.clone(),
            accounts.token_authority.clone(),
            accounts.whirlpool.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_b.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.tick_array0.clone(),
            accounts.tick_array1.clone(),
            accounts.tick_array2.clone(),
            accounts.oracle.clone(),
            accounts.whirlpool_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ORCA_SWAP_IX_ACCOUNTS_LEN]>
for OrcaSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ORCA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funder: &arr[0],
            token_a_token_program: &arr[1],
            token_b_token_program: &arr[2],
            memo_program: &arr[3],
            token_authority: &arr[4],
            whirlpool: &arr[5],
            token_owner_account_a: &arr[6],
            token_vault_a: &arr[7],
            token_owner_account_b: &arr[8],
            token_vault_b: &arr[9],
            token_mint_a: &arr[10],
            token_mint_b: &arr[11],
            tick_array0: &arr[12],
            tick_array1: &arr[13],
            tick_array2: &arr[14],
            oracle: &arr[15],
            whirlpool_program: &arr[16],
        }
    }
}
pub const ORCA_SWAP_IX_DISCM: [u8; 8usize] = [33, 94, 249, 97, 250, 254, 198, 93];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrcaSwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrcaSwapIxData(pub OrcaSwapIxArgs);
impl From<OrcaSwapIxArgs> for OrcaSwapIxData {
    fn from(args: OrcaSwapIxArgs) -> Self {
        Self(args)
    }
}
impl OrcaSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORCA_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let other_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_limit: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_specified_is_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(OrcaSwapIxArgs {
                amount,
                other_amount_threshold,
                sqrt_price_limit,
                amount_specified_is_input,
                a_to_b,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORCA_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.other_amount_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.sqrt_price_limit, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.amount_specified_is_input,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.a_to_b, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn orca_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: OrcaSwapKeys,
    args: OrcaSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ORCA_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: OrcaSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn orca_swap_ix(
    keys: OrcaSwapKeys,
    args: OrcaSwapIxArgs,
) -> std::io::Result<Instruction> {
    orca_swap_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn orca_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OrcaSwapAccounts<'_, '_>,
    args: OrcaSwapIxArgs,
) -> ProgramResult {
    let keys: OrcaSwapKeys = accounts.into();
    let ix = orca_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn orca_swap_invoke(
    accounts: OrcaSwapAccounts<'_, '_>,
    args: OrcaSwapIxArgs,
) -> ProgramResult {
    orca_swap_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn orca_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OrcaSwapAccounts<'_, '_>,
    args: OrcaSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OrcaSwapKeys = accounts.into();
    let ix = orca_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn orca_swap_invoke_signed(
    accounts: OrcaSwapAccounts<'_, '_>,
    args: OrcaSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    orca_swap_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, args, seeds)
}
pub fn orca_swap_verify_account_keys(
    accounts: OrcaSwapAccounts<'_, '_>,
    keys: OrcaSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.funder.key, keys.funder),
        (*accounts.token_a_token_program.key, keys.token_a_token_program),
        (*accounts.token_b_token_program.key, keys.token_b_token_program),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_owner_account_a.key, keys.token_owner_account_a),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (*accounts.token_owner_account_b.key, keys.token_owner_account_b),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (*accounts.tick_array0.key, keys.tick_array0),
        (*accounts.tick_array1.key, keys.tick_array1),
        (*accounts.tick_array2.key, keys.tick_array2),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.whirlpool_program.key, keys.whirlpool_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn orca_swap_verify_writable_privileges<'me, 'info>(
    accounts: OrcaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn orca_swap_verify_signer_privileges<'me, 'info>(
    accounts: OrcaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn orca_swap_verify_account_privileges<'me, 'info>(
    accounts: OrcaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    orca_swap_verify_writable_privileges(accounts)?;
    orca_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SIGN_TERMS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SignTermsAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub owner_signature_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SignTermsKeys {
    pub owner: Pubkey,
    pub owner_signature_state: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<SignTermsAccounts<'_, '_>> for SignTermsKeys {
    fn from(accounts: SignTermsAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            owner_signature_state: *accounts.owner_signature_state.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<SignTermsKeys> for [AccountMeta; SIGN_TERMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SignTermsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_signature_state,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; SIGN_TERMS_IX_ACCOUNTS_LEN]> for SignTermsKeys {
    fn from(pubkeys: [Pubkey; SIGN_TERMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            owner_signature_state: pubkeys[1],
            system_program: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<SignTermsAccounts<'_, 'info>>
for [AccountInfo<'info>; SIGN_TERMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SignTermsAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.owner_signature_state.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SIGN_TERMS_IX_ACCOUNTS_LEN]>
for SignTermsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SIGN_TERMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            owner_signature_state: &arr[1],
            system_program: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const SIGN_TERMS_IX_DISCM: [u8; 8usize] = [226, 42, 174, 143, 144, 159, 139, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignTermsIxArgs {
    #[serde(with = "crate::big_array_serde")]
    pub signature: [u8; 64],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SignTermsIxData(pub SignTermsIxArgs);
impl From<SignTermsIxArgs> for SignTermsIxData {
    fn from(args: SignTermsIxArgs) -> Self {
        Self(args)
    }
}
impl SignTermsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SIGN_TERMS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let signature = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(Self(SignTermsIxArgs { signature }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SIGN_TERMS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.signature, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sign_terms_ix_with_program_id(
    program_id: Pubkey,
    keys: SignTermsKeys,
    args: SignTermsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SIGN_TERMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SignTermsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sign_terms_ix(
    keys: SignTermsKeys,
    args: SignTermsIxArgs,
) -> std::io::Result<Instruction> {
    sign_terms_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn sign_terms_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SignTermsAccounts<'_, '_>,
    args: SignTermsIxArgs,
) -> ProgramResult {
    let keys: SignTermsKeys = accounts.into();
    let ix = sign_terms_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sign_terms_invoke(
    accounts: SignTermsAccounts<'_, '_>,
    args: SignTermsIxArgs,
) -> ProgramResult {
    sign_terms_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn sign_terms_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SignTermsAccounts<'_, '_>,
    args: SignTermsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SignTermsKeys = accounts.into();
    let ix = sign_terms_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sign_terms_invoke_signed(
    accounts: SignTermsAccounts<'_, '_>,
    args: SignTermsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sign_terms_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, args, seeds)
}
pub fn sign_terms_verify_account_keys(
    accounts: SignTermsAccounts<'_, '_>,
    keys: SignTermsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.owner_signature_state.key, keys.owner_signature_state),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sign_terms_verify_writable_privileges<'me, 'info>(
    accounts: SignTermsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.owner_signature_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sign_terms_verify_signer_privileges<'me, 'info>(
    accounts: SignTermsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sign_terms_verify_account_privileges<'me, 'info>(
    accounts: SignTermsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sign_terms_verify_writable_privileges(accounts)?;
    sign_terms_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateStrategyAdminAccounts<'me, 'info> {
    pub pending_admin: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateStrategyAdminKeys {
    pub pending_admin: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
}
impl From<UpdateStrategyAdminAccounts<'_, '_>> for UpdateStrategyAdminKeys {
    fn from(accounts: UpdateStrategyAdminAccounts) -> Self {
        Self {
            pending_admin: *accounts.pending_admin.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<UpdateStrategyAdminKeys>
for [AccountMeta; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateStrategyAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pending_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN]> for UpdateStrategyAdminKeys {
    fn from(pubkeys: [Pubkey; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pending_admin: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateStrategyAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateStrategyAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.pending_admin.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateStrategyAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pending_admin: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
        }
    }
}
pub const UPDATE_STRATEGY_ADMIN_IX_DISCM: [u8; 8usize] = [
    13, 227, 164, 236, 32, 39, 6, 255,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStrategyAdminIxData;
impl UpdateStrategyAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_STRATEGY_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_STRATEGY_ADMIN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_strategy_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateStrategyAdminKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_STRATEGY_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateStrategyAdminIxData.try_to_vec()?,
    })
}
pub fn update_strategy_admin_ix(
    keys: UpdateStrategyAdminKeys,
) -> std::io::Result<Instruction> {
    update_strategy_admin_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn update_strategy_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStrategyAdminAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateStrategyAdminKeys = accounts.into();
    let ix = update_strategy_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_strategy_admin_invoke(
    accounts: UpdateStrategyAdminAccounts<'_, '_>,
) -> ProgramResult {
    update_strategy_admin_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn update_strategy_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStrategyAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateStrategyAdminKeys = accounts.into();
    let ix = update_strategy_admin_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_strategy_admin_invoke_signed(
    accounts: UpdateStrategyAdminAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_strategy_admin_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_strategy_admin_verify_account_keys(
    accounts: UpdateStrategyAdminAccounts<'_, '_>,
    keys: UpdateStrategyAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pending_admin.key, keys.pending_admin),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_strategy_admin_verify_writable_privileges<'me, 'info>(
    accounts: UpdateStrategyAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pending_admin, accounts.strategy] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_strategy_admin_verify_signer_privileges<'me, 'info>(
    accounts: UpdateStrategyAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pending_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_strategy_admin_verify_account_privileges<'me, 'info>(
    accounts: UpdateStrategyAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_strategy_admin_verify_writable_privileges(accounts)?;
    update_strategy_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ResizeTokenInfosAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResizeTokenInfosKeys {
    pub signer: Pubkey,
    pub token_infos: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<ResizeTokenInfosAccounts<'_, '_>> for ResizeTokenInfosKeys {
    fn from(accounts: ResizeTokenInfosAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            token_infos: *accounts.token_infos.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<ResizeTokenInfosKeys> for [AccountMeta; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN] {
    fn from(keys: ResizeTokenInfosKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN]> for ResizeTokenInfosKeys {
    fn from(pubkeys: [Pubkey; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            token_infos: pubkeys[1],
            system_program: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<ResizeTokenInfosAccounts<'_, 'info>>
for [AccountInfo<'info>; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResizeTokenInfosAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.token_infos.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN]>
for ResizeTokenInfosAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            token_infos: &arr[1],
            system_program: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const RESIZE_TOKEN_INFOS_IX_DISCM: [u8; 8usize] = [
    29, 211, 127, 192, 213, 128, 200, 187,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResizeTokenInfosIxData;
impl ResizeTokenInfosIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESIZE_TOKEN_INFOS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESIZE_TOKEN_INFOS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resize_token_infos_ix_with_program_id(
    program_id: Pubkey,
    keys: ResizeTokenInfosKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESIZE_TOKEN_INFOS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResizeTokenInfosIxData.try_to_vec()?,
    })
}
pub fn resize_token_infos_ix(
    keys: ResizeTokenInfosKeys,
) -> std::io::Result<Instruction> {
    resize_token_infos_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn resize_token_infos_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResizeTokenInfosAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResizeTokenInfosKeys = accounts.into();
    let ix = resize_token_infos_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn resize_token_infos_invoke(
    accounts: ResizeTokenInfosAccounts<'_, '_>,
) -> ProgramResult {
    resize_token_infos_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn resize_token_infos_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResizeTokenInfosAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResizeTokenInfosKeys = accounts.into();
    let ix = resize_token_infos_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn resize_token_infos_invoke_signed(
    accounts: ResizeTokenInfosAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    resize_token_infos_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, seeds)
}
pub fn resize_token_infos_verify_account_keys(
    accounts: ResizeTokenInfosAccounts<'_, '_>,
    keys: ResizeTokenInfosKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn resize_token_infos_verify_writable_privileges<'me, 'info>(
    accounts: ResizeTokenInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.token_infos] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn resize_token_infos_verify_signer_privileges<'me, 'info>(
    accounts: ResizeTokenInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn resize_token_infos_verify_account_privileges<'me, 'info>(
    accounts: ResizeTokenInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    resize_token_infos_verify_writable_privileges(accounts)?;
    resize_token_infos_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DeprecateCollateralInfoAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub token_infos: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeprecateCollateralInfoKeys {
    pub admin_authority: Pubkey,
    pub global_config: Pubkey,
    pub token_infos: Pubkey,
}
impl From<DeprecateCollateralInfoAccounts<'_, '_>> for DeprecateCollateralInfoKeys {
    fn from(accounts: DeprecateCollateralInfoAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            global_config: *accounts.global_config.key,
            token_infos: *accounts.token_infos.key,
        }
    }
}
impl From<DeprecateCollateralInfoKeys>
for [AccountMeta; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: DeprecateCollateralInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for DeprecateCollateralInfoKeys {
    fn from(pubkeys: [Pubkey; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            global_config: pubkeys[1],
            token_infos: pubkeys[2],
        }
    }
}
impl<'info> From<DeprecateCollateralInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeprecateCollateralInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.global_config.clone(),
            accounts.token_infos.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN]>
for DeprecateCollateralInfoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin_authority: &arr[0],
            global_config: &arr[1],
            token_infos: &arr[2],
        }
    }
}
pub const DEPRECATE_COLLATERAL_INFO_IX_DISCM: [u8; 8usize] = [
    143, 221, 36, 89, 12, 97, 29, 173,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeprecateCollateralInfoIxArgs {
    pub index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecateCollateralInfoIxData(pub DeprecateCollateralInfoIxArgs);
impl From<DeprecateCollateralInfoIxArgs> for DeprecateCollateralInfoIxData {
    fn from(args: DeprecateCollateralInfoIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecateCollateralInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPRECATE_COLLATERAL_INFO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DeprecateCollateralInfoIxArgs {
                index,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPRECATE_COLLATERAL_INFO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deprecate_collateral_info_ix_with_program_id(
    program_id: Pubkey,
    keys: DeprecateCollateralInfoKeys,
    args: DeprecateCollateralInfoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPRECATE_COLLATERAL_INFO_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeprecateCollateralInfoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecate_collateral_info_ix(
    keys: DeprecateCollateralInfoKeys,
    args: DeprecateCollateralInfoIxArgs,
) -> std::io::Result<Instruction> {
    deprecate_collateral_info_ix_with_program_id(YVAULTS_PROGRAM_ID, keys, args)
}
pub fn deprecate_collateral_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeprecateCollateralInfoAccounts<'_, '_>,
    args: DeprecateCollateralInfoIxArgs,
) -> ProgramResult {
    let keys: DeprecateCollateralInfoKeys = accounts.into();
    let ix = deprecate_collateral_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deprecate_collateral_info_invoke(
    accounts: DeprecateCollateralInfoAccounts<'_, '_>,
    args: DeprecateCollateralInfoIxArgs,
) -> ProgramResult {
    deprecate_collateral_info_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts, args)
}
pub fn deprecate_collateral_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeprecateCollateralInfoAccounts<'_, '_>,
    args: DeprecateCollateralInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeprecateCollateralInfoKeys = accounts.into();
    let ix = deprecate_collateral_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deprecate_collateral_info_invoke_signed(
    accounts: DeprecateCollateralInfoAccounts<'_, '_>,
    args: DeprecateCollateralInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deprecate_collateral_info_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deprecate_collateral_info_verify_account_keys(
    accounts: DeprecateCollateralInfoAccounts<'_, '_>,
    keys: DeprecateCollateralInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.token_infos.key, keys.token_infos),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deprecate_collateral_info_verify_writable_privileges<'me, 'info>(
    accounts: DeprecateCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin_authority, accounts.token_infos] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deprecate_collateral_info_verify_signer_privileges<'me, 'info>(
    accounts: DeprecateCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deprecate_collateral_info_verify_account_privileges<'me, 'info>(
    accounts: DeprecateCollateralInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deprecate_collateral_info_verify_writable_privileges(accounts)?;
    deprecate_collateral_info_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DeprecateStrategyAccounts<'me, 'info> {
    pub admin_authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeprecateStrategyKeys {
    pub admin_authority: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
}
impl From<DeprecateStrategyAccounts<'_, '_>> for DeprecateStrategyKeys {
    fn from(accounts: DeprecateStrategyAccounts) -> Self {
        Self {
            admin_authority: *accounts.admin_authority.key,
            strategy: *accounts.strategy.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<DeprecateStrategyKeys> for [AccountMeta; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: DeprecateStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN]> for DeprecateStrategyKeys {
    fn from(pubkeys: [Pubkey; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: pubkeys[0],
            strategy: pubkeys[1],
            global_config: pubkeys[2],
        }
    }
}
impl<'info> From<DeprecateStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeprecateStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_authority.clone(),
            accounts.strategy.clone(),
            accounts.global_config.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN]>
for DeprecateStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_authority: &arr[0],
            strategy: &arr[1],
            global_config: &arr[2],
        }
    }
}
pub const DEPRECATE_STRATEGY_IX_DISCM: [u8; 8usize] = [
    73, 76, 95, 59, 38, 251, 117, 161,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecateStrategyIxData;
impl DeprecateStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPRECATE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPRECATE_STRATEGY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deprecate_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: DeprecateStrategyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPRECATE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeprecateStrategyIxData.try_to_vec()?,
    })
}
pub fn deprecate_strategy_ix(
    keys: DeprecateStrategyKeys,
) -> std::io::Result<Instruction> {
    deprecate_strategy_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn deprecate_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeprecateStrategyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeprecateStrategyKeys = accounts.into();
    let ix = deprecate_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn deprecate_strategy_invoke(
    accounts: DeprecateStrategyAccounts<'_, '_>,
) -> ProgramResult {
    deprecate_strategy_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn deprecate_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeprecateStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeprecateStrategyKeys = accounts.into();
    let ix = deprecate_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deprecate_strategy_invoke_signed(
    accounts: DeprecateStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deprecate_strategy_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, seeds)
}
pub fn deprecate_strategy_verify_account_keys(
    accounts: DeprecateStrategyAccounts<'_, '_>,
    keys: DeprecateStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_authority.key, keys.admin_authority),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deprecate_strategy_verify_writable_privileges<'me, 'info>(
    accounts: DeprecateStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.strategy] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deprecate_strategy_verify_signer_privileges<'me, 'info>(
    accounts: DeprecateStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deprecate_strategy_verify_account_privileges<'me, 'info>(
    accounts: DeprecateStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deprecate_strategy_verify_writable_privileges(accounts)?;
    deprecate_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ResetStrategyPaddingAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResetStrategyPaddingKeys {
    pub authority: Pubkey,
    pub strategy: Pubkey,
}
impl From<ResetStrategyPaddingAccounts<'_, '_>> for ResetStrategyPaddingKeys {
    fn from(accounts: ResetStrategyPaddingAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            strategy: *accounts.strategy.key,
        }
    }
}
impl From<ResetStrategyPaddingKeys>
for [AccountMeta; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN] {
    fn from(keys: ResetStrategyPaddingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN]>
for ResetStrategyPaddingKeys {
    fn from(pubkeys: [Pubkey; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            strategy: pubkeys[1],
        }
    }
}
impl<'info> From<ResetStrategyPaddingAccounts<'_, 'info>>
for [AccountInfo<'info>; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN] {
    fn from(accounts: ResetStrategyPaddingAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.strategy.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN]>
for ResetStrategyPaddingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            strategy: &arr[1],
        }
    }
}
pub const RESET_STRATEGY_PADDING_IX_DISCM: [u8; 8usize] = [
    168, 99, 222, 231, 25, 197, 174, 19,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ResetStrategyPaddingIxData;
impl ResetStrategyPaddingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESET_STRATEGY_PADDING_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESET_STRATEGY_PADDING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reset_strategy_padding_ix_with_program_id(
    program_id: Pubkey,
    keys: ResetStrategyPaddingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RESET_STRATEGY_PADDING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ResetStrategyPaddingIxData.try_to_vec()?,
    })
}
pub fn reset_strategy_padding_ix(
    keys: ResetStrategyPaddingKeys,
) -> std::io::Result<Instruction> {
    reset_strategy_padding_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn reset_strategy_padding_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ResetStrategyPaddingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ResetStrategyPaddingKeys = accounts.into();
    let ix = reset_strategy_padding_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn reset_strategy_padding_invoke(
    accounts: ResetStrategyPaddingAccounts<'_, '_>,
) -> ProgramResult {
    reset_strategy_padding_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn reset_strategy_padding_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ResetStrategyPaddingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ResetStrategyPaddingKeys = accounts.into();
    let ix = reset_strategy_padding_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reset_strategy_padding_invoke_signed(
    accounts: ResetStrategyPaddingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reset_strategy_padding_invoke_signed_with_program_id(
        YVAULTS_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn reset_strategy_padding_verify_account_keys(
    accounts: ResetStrategyPaddingAccounts<'_, '_>,
    keys: ResetStrategyPaddingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.strategy.key, keys.strategy),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn reset_strategy_padding_verify_writable_privileges<'me, 'info>(
    accounts: ResetStrategyPaddingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.strategy] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reset_strategy_padding_verify_signer_privileges<'me, 'info>(
    accounts: ResetStrategyPaddingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reset_strategy_padding_verify_account_privileges<'me, 'info>(
    accounts: ResetStrategyPaddingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reset_strategy_padding_verify_writable_privileges(accounts)?;
    reset_strategy_padding_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct GetKtokenPriceAccounts<'me, 'info> {
    pub token_infos: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub scope_prices_a: &'me AccountInfo<'info>,
    pub scope_prices_b: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetKtokenPriceKeys {
    pub token_infos: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub strategy: Pubkey,
    pub scope_prices_a: Pubkey,
    pub scope_prices_b: Pubkey,
}
impl From<GetKtokenPriceAccounts<'_, '_>> for GetKtokenPriceKeys {
    fn from(accounts: GetKtokenPriceAccounts) -> Self {
        Self {
            token_infos: *accounts.token_infos.key,
            global_config: *accounts.global_config.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            strategy: *accounts.strategy.key,
            scope_prices_a: *accounts.scope_prices_a.key,
            scope_prices_b: *accounts.scope_prices_b.key,
        }
    }
}
impl From<GetKtokenPriceKeys> for [AccountMeta; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: GetKtokenPriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_infos,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices_b,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN]> for GetKtokenPriceKeys {
    fn from(pubkeys: [Pubkey; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_infos: pubkeys[0],
            global_config: pubkeys[1],
            pool: pubkeys[2],
            position: pubkeys[3],
            strategy: pubkeys[4],
            scope_prices_a: pubkeys[5],
            scope_prices_b: pubkeys[6],
        }
    }
}
impl<'info> From<GetKtokenPriceAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetKtokenPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.token_infos.clone(),
            accounts.global_config.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.strategy.clone(),
            accounts.scope_prices_a.clone(),
            accounts.scope_prices_b.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN]>
for GetKtokenPriceAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_infos: &arr[0],
            global_config: &arr[1],
            pool: &arr[2],
            position: &arr[3],
            strategy: &arr[4],
            scope_prices_a: &arr[5],
            scope_prices_b: &arr[6],
        }
    }
}
pub const GET_KTOKEN_PRICE_IX_DISCM: [u8; 8usize] = [
    215, 107, 174, 156, 212, 156, 22, 130,
];
#[derive(Clone, Debug, PartialEq)]
pub struct GetKtokenPriceIxData;
impl GetKtokenPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_KTOKEN_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_KTOKEN_PRICE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_ktoken_price_ix_with_program_id(
    program_id: Pubkey,
    keys: GetKtokenPriceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_KTOKEN_PRICE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetKtokenPriceIxData.try_to_vec()?,
    })
}
pub fn get_ktoken_price_ix(keys: GetKtokenPriceKeys) -> std::io::Result<Instruction> {
    get_ktoken_price_ix_with_program_id(YVAULTS_PROGRAM_ID, keys)
}
pub fn get_ktoken_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetKtokenPriceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetKtokenPriceKeys = accounts.into();
    let ix = get_ktoken_price_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_ktoken_price_invoke(
    accounts: GetKtokenPriceAccounts<'_, '_>,
) -> ProgramResult {
    get_ktoken_price_invoke_with_program_id(YVAULTS_PROGRAM_ID, accounts)
}
pub fn get_ktoken_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetKtokenPriceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetKtokenPriceKeys = accounts.into();
    let ix = get_ktoken_price_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_ktoken_price_invoke_signed(
    accounts: GetKtokenPriceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_ktoken_price_invoke_signed_with_program_id(YVAULTS_PROGRAM_ID, accounts, seeds)
}
pub fn get_ktoken_price_verify_account_keys(
    accounts: GetKtokenPriceAccounts<'_, '_>,
    keys: GetKtokenPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_infos.key, keys.token_infos),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.scope_prices_a.key, keys.scope_prices_a),
        (*accounts.scope_prices_b.key, keys.scope_prices_b),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
