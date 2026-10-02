use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum JupiterLendAmmProgramIx {
    Borrow(BorrowIxArgs),
    BorrowPerfect(BorrowPerfectIxArgs),
    Deposit(DepositIxArgs),
    DepositPerfect(DepositPerfectIxArgs),
    InitDex(InitDexIxArgs),
    InitDexAdmin(InitDexAdminIxArgs),
    InitPosition(InitPositionIxArgs),
    PauseDex,
    PauseSwapAndArbitrage,
    PauseUser(PauseUserIxArgs),
    Payback(PaybackIxArgs),
    PaybackPerfect(PaybackPerfectIxArgs),
    PaybackPerfectInOneToken(PaybackPerfectInOneTokenIxArgs),
    PreviewDexShares(PreviewDexSharesIxArgs),
    SwapIn(SwapInIxArgs),
    SwapOut(SwapOutIxArgs),
    TurnOnSmartCol(TurnOnSmartColIxArgs),
    TurnOnSmartDebt(TurnOnSmartDebtIxArgs),
    UnpauseDex,
    UnpauseSwapAndArbitrage,
    UnpauseUser(UnpauseUserIxArgs),
    UpdateAuthority(UpdateAuthorityIxArgs),
    UpdateAuths(UpdateAuthsIxArgs),
    UpdateCenterPriceAddress(UpdateCenterPriceAddressIxArgs),
    UpdateCenterPriceLimits(UpdateCenterPriceLimitsIxArgs),
    UpdateFeeAndRevenueCut(UpdateFeeAndRevenueCutIxArgs),
    UpdateMaxBorrowShares(UpdateMaxBorrowSharesIxArgs),
    UpdateMaxSupplyShares(UpdateMaxSupplySharesIxArgs),
    UpdateRangePercents(UpdateRangePercentsIxArgs),
    UpdateThresholdPercent(UpdateThresholdPercentIxArgs),
    UpdateUserBorrowConfig(UpdateUserBorrowConfigIxArgs),
    UpdateUserSupplyConfig(UpdateUserSupplyConfigIxArgs),
    UpdateUserWithdrawalLimit(UpdateUserWithdrawalLimitIxArgs),
    UpdateUtilizationLimit(UpdateUtilizationLimitIxArgs),
    Withdraw(WithdrawIxArgs),
    WithdrawPerfect(WithdrawPerfectIxArgs),
    WithdrawPerfectInOneToken(WithdrawPerfectInOneTokenIxArgs),
}
impl JupiterLendAmmProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BORROW_IX_DISCM) {
            let mut reader = &buf[BORROW_IX_DISCM.len()..];
            let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Borrow(BorrowIxArgs {
                    token0_amt,
                    token1_amt,
                    max_shares,
                }),
            );
        }
        if buf.starts_with(&BORROW_PERFECT_IX_DISCM) {
            let mut reader = &buf[BORROW_PERFECT_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BorrowPerfect(BorrowPerfectIxArgs {
                    shares,
                    min_token0,
                    min_token1,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_IX_DISCM.len()..];
            let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Deposit(DepositIxArgs {
                    token0_amt,
                    token1_amt,
                    min_shares,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_PERFECT_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_PERFECT_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositPerfect(DepositPerfectIxArgs {
                    shares,
                    max_token0,
                    max_token1,
                }),
            );
        }
        if buf.starts_with(&INIT_DEX_IX_DISCM) {
            let mut reader = &buf[INIT_DEX_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitDexParams>::deserialize(&mut reader)?
            };
            return Ok(Self::InitDex(InitDexIxArgs { params }));
        }
        if buf.starts_with(&INIT_DEX_ADMIN_IX_DISCM) {
            let mut reader = &buf[INIT_DEX_ADMIN_IX_DISCM.len()..];
            let liquidity: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitDexAdmin(InitDexAdminIxArgs {
                    liquidity,
                    authority,
                }),
            );
        }
        if buf.starts_with(&INIT_POSITION_IX_DISCM) {
            let mut reader = &buf[INIT_POSITION_IX_DISCM.len()..];
            let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::InitPosition(InitPositionIxArgs { protocol }));
        }
        if buf.starts_with(&PAUSE_DEX_IX_DISCM) {
            return Ok(Self::PauseDex);
        }
        if buf.starts_with(&PAUSE_SWAP_AND_ARBITRAGE_IX_DISCM) {
            return Ok(Self::PauseSwapAndArbitrage);
        }
        if buf.starts_with(&PAUSE_USER_IX_DISCM) {
            let mut reader = &buf[PAUSE_USER_IX_DISCM.len()..];
            let pause_supply: bool = crate::borsh_de_or_default(&mut reader)?;
            let pause_borrow: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PauseUser(PauseUserIxArgs {
                    pause_supply,
                    pause_borrow,
                }),
            );
        }
        if buf.starts_with(&PAYBACK_IX_DISCM) {
            let mut reader = &buf[PAYBACK_IX_DISCM.len()..];
            let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Payback(PaybackIxArgs {
                    token0_amt,
                    token1_amt,
                    min_shares,
                }),
            );
        }
        if buf.starts_with(&PAYBACK_PERFECT_IX_DISCM) {
            let mut reader = &buf[PAYBACK_PERFECT_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PaybackPerfect(PaybackPerfectIxArgs {
                    shares,
                    max_token0,
                    max_token1,
                }),
            );
        }
        if buf.starts_with(&PAYBACK_PERFECT_IN_ONE_TOKEN_IX_DISCM) {
            let mut reader = &buf[PAYBACK_PERFECT_IN_ONE_TOKEN_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PaybackPerfectInOneToken(PaybackPerfectInOneTokenIxArgs {
                    shares,
                    max_token0,
                    max_token1,
                }),
            );
        }
        if buf.starts_with(&PREVIEW_DEX_SHARES_IX_DISCM) {
            let mut reader = &buf[PREVIEW_DEX_SHARES_IX_DISCM.len()..];
            let col_token0: i64 = crate::borsh_de_or_default(&mut reader)?;
            let col_token1: i64 = crate::borsh_de_or_default(&mut reader)?;
            let debt_token0: i64 = crate::borsh_de_or_default(&mut reader)?;
            let debt_token1: i64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::PreviewDexShares(PreviewDexSharesIxArgs {
                    col_token0,
                    col_token1,
                    debt_token0,
                    debt_token1,
                }),
            );
        }
        if buf.starts_with(&SWAP_IN_IX_DISCM) {
            let mut reader = &buf[SWAP_IN_IX_DISCM.len()..];
            let swap0to1: bool = crate::borsh_de_or_default(&mut reader)?;
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_out_min: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapIn(SwapInIxArgs {
                    swap0to1,
                    amount_in,
                    amount_out_min,
                }),
            );
        }
        if buf.starts_with(&SWAP_OUT_IX_DISCM) {
            let mut reader = &buf[SWAP_OUT_IX_DISCM.len()..];
            let swap0to1: bool = crate::borsh_de_or_default(&mut reader)?;
            let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let amount_in_max: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SwapOut(SwapOutIxArgs {
                    swap0to1,
                    amount_out,
                    amount_in_max,
                }),
            );
        }
        if buf.starts_with(&TURN_ON_SMART_COL_IX_DISCM) {
            let mut reader = &buf[TURN_ON_SMART_COL_IX_DISCM.len()..];
            let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TurnOnSmartCol(TurnOnSmartColIxArgs {
                    token_0_amt,
                }),
            );
        }
        if buf.starts_with(&TURN_ON_SMART_DEBT_IX_DISCM) {
            let mut reader = &buf[TURN_ON_SMART_DEBT_IX_DISCM.len()..];
            let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::TurnOnSmartDebt(TurnOnSmartDebtIxArgs {
                    token_0_amt,
                }),
            );
        }
        if buf.starts_with(&UNPAUSE_DEX_IX_DISCM) {
            return Ok(Self::UnpauseDex);
        }
        if buf.starts_with(&UNPAUSE_SWAP_AND_ARBITRAGE_IX_DISCM) {
            return Ok(Self::UnpauseSwapAndArbitrage);
        }
        if buf.starts_with(&UNPAUSE_USER_IX_DISCM) {
            let mut reader = &buf[UNPAUSE_USER_IX_DISCM.len()..];
            let unpause_supply: bool = crate::borsh_de_or_default(&mut reader)?;
            let unpause_borrow: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UnpauseUser(UnpauseUserIxArgs {
                    unpause_supply,
                    unpause_borrow,
                }),
            );
        }
        if buf.starts_with(&UPDATE_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[UPDATE_AUTHORITY_IX_DISCM.len()..];
            let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateAuthority(UpdateAuthorityIxArgs {
                    new_authority,
                }),
            );
        }
        if buf.starts_with(&UPDATE_AUTHS_IX_DISCM) {
            let mut reader = &buf[UPDATE_AUTHS_IX_DISCM.len()..];
            let auth_status: Vec<AddressBool> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdateAuths(UpdateAuthsIxArgs { auth_status }));
        }
        if buf.starts_with(&UPDATE_CENTER_PRICE_ADDRESS_IX_DISCM) {
            let mut reader = &buf[UPDATE_CENTER_PRICE_ADDRESS_IX_DISCM.len()..];
            let center_price_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let percent: u32 = crate::borsh_de_or_default(&mut reader)?;
            let time: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateCenterPriceAddress(UpdateCenterPriceAddressIxArgs {
                    center_price_address,
                    percent,
                    time,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CENTER_PRICE_LIMITS_IX_DISCM) {
            let mut reader = &buf[UPDATE_CENTER_PRICE_LIMITS_IX_DISCM.len()..];
            let max_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateCenterPriceLimits(UpdateCenterPriceLimitsIxArgs {
                    max_center_price,
                    min_center_price,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FEE_AND_REVENUE_CUT_IX_DISCM) {
            let mut reader = &buf[UPDATE_FEE_AND_REVENUE_CUT_IX_DISCM.len()..];
            let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
            let revenue_cut: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateFeeAndRevenueCut(UpdateFeeAndRevenueCutIxArgs {
                    fee,
                    revenue_cut,
                }),
            );
        }
        if buf.starts_with(&UPDATE_MAX_BORROW_SHARES_IX_DISCM) {
            let mut reader = &buf[UPDATE_MAX_BORROW_SHARES_IX_DISCM.len()..];
            let max_borrow_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateMaxBorrowShares(UpdateMaxBorrowSharesIxArgs {
                    max_borrow_shares,
                }),
            );
        }
        if buf.starts_with(&UPDATE_MAX_SUPPLY_SHARES_IX_DISCM) {
            let mut reader = &buf[UPDATE_MAX_SUPPLY_SHARES_IX_DISCM.len()..];
            let max_supply_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateMaxSupplyShares(UpdateMaxSupplySharesIxArgs {
                    max_supply_shares,
                }),
            );
        }
        if buf.starts_with(&UPDATE_RANGE_PERCENTS_IX_DISCM) {
            let mut reader = &buf[UPDATE_RANGE_PERCENTS_IX_DISCM.len()..];
            let upper_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
            let lower_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
            let shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateRangePercents(UpdateRangePercentsIxArgs {
                    upper_percent,
                    lower_percent,
                    shift_time,
                }),
            );
        }
        if buf.starts_with(&UPDATE_THRESHOLD_PERCENT_IX_DISCM) {
            let mut reader = &buf[UPDATE_THRESHOLD_PERCENT_IX_DISCM.len()..];
            let upper_threshold_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
            let lower_threshold_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
            let threshold_shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
            let shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateThresholdPercent(UpdateThresholdPercentIxArgs {
                    upper_threshold_percent,
                    lower_threshold_percent,
                    threshold_shift_time,
                    shift_time,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USER_BORROW_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_USER_BORROW_CONFIG_IX_DISCM.len()..];
            let config = if reader.is_empty() {
                Default::default()
            } else {
                <UserBorrowConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateUserBorrowConfig(UpdateUserBorrowConfigIxArgs {
                    config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USER_SUPPLY_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_USER_SUPPLY_CONFIG_IX_DISCM.len()..];
            let config = if reader.is_empty() {
                Default::default()
            } else {
                <UserSupplyConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateUserSupplyConfig(UpdateUserSupplyConfigIxArgs {
                    config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_USER_WITHDRAWAL_LIMIT_IX_DISCM) {
            let mut reader = &buf[UPDATE_USER_WITHDRAWAL_LIMIT_IX_DISCM.len()..];
            let new_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UpdateUserWithdrawalLimit(UpdateUserWithdrawalLimitIxArgs {
                    new_limit,
                }),
            );
        }
        if buf.starts_with(&UPDATE_UTILIZATION_LIMIT_IX_DISCM) {
            let mut reader = &buf[UPDATE_UTILIZATION_LIMIT_IX_DISCM.len()..];
            let token_0_utilization_limit: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_1_utilization_limit: u16 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateUtilizationLimit(UpdateUtilizationLimitIxArgs {
                    token_0_utilization_limit,
                    token_1_utilization_limit,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_IX_DISCM.len()..];
            let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Withdraw(WithdrawIxArgs {
                    token0_amt,
                    token1_amt,
                    max_shares,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_PERFECT_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_PERFECT_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawPerfect(WithdrawPerfectIxArgs {
                    shares,
                    min_token0,
                    min_token1,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_DISCM.len()..];
            let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawPerfectInOneToken(WithdrawPerfectInOneTokenIxArgs {
                    shares,
                    min_token0,
                    min_token1,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Borrow(args) => {
                writer.write_all(&BORROW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token0_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token1_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_shares, &mut writer)?;
                Ok(())
            }
            Self::BorrowPerfect(args) => {
                writer.write_all(&BORROW_PERFECT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token1, &mut writer)?;
                Ok(())
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token0_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token1_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_shares, &mut writer)?;
                Ok(())
            }
            Self::DepositPerfect(args) => {
                writer.write_all(&DEPOSIT_PERFECT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_token1, &mut writer)?;
                Ok(())
            }
            Self::InitDex(args) => {
                writer.write_all(&INIT_DEX_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitDexAdmin(args) => {
                writer.write_all(&INIT_DEX_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.liquidity, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                Ok(())
            }
            Self::InitPosition(args) => {
                writer.write_all(&INIT_POSITION_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.protocol, &mut writer)?;
                Ok(())
            }
            Self::PauseDex => writer.write_all(&PAUSE_DEX_IX_DISCM),
            Self::PauseSwapAndArbitrage => {
                writer.write_all(&PAUSE_SWAP_AND_ARBITRAGE_IX_DISCM)
            }
            Self::PauseUser(args) => {
                writer.write_all(&PAUSE_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.pause_supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pause_borrow, &mut writer)?;
                Ok(())
            }
            Self::Payback(args) => {
                writer.write_all(&PAYBACK_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token0_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token1_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_shares, &mut writer)?;
                Ok(())
            }
            Self::PaybackPerfect(args) => {
                writer.write_all(&PAYBACK_PERFECT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_token1, &mut writer)?;
                Ok(())
            }
            Self::PaybackPerfectInOneToken(args) => {
                writer.write_all(&PAYBACK_PERFECT_IN_ONE_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_token1, &mut writer)?;
                Ok(())
            }
            Self::PreviewDexShares(args) => {
                writer.write_all(&PREVIEW_DEX_SHARES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.col_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.col_token1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.debt_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.debt_token1, &mut writer)?;
                Ok(())
            }
            Self::SwapIn(args) => {
                writer.write_all(&SWAP_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.swap0to1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out_min, &mut writer)?;
                Ok(())
            }
            Self::SwapOut(args) => {
                writer.write_all(&SWAP_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.swap0to1, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amount_in_max, &mut writer)?;
                Ok(())
            }
            Self::TurnOnSmartCol(args) => {
                writer.write_all(&TURN_ON_SMART_COL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_0_amt, &mut writer)?;
                Ok(())
            }
            Self::TurnOnSmartDebt(args) => {
                writer.write_all(&TURN_ON_SMART_DEBT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token_0_amt, &mut writer)?;
                Ok(())
            }
            Self::UnpauseDex => writer.write_all(&UNPAUSE_DEX_IX_DISCM),
            Self::UnpauseSwapAndArbitrage => {
                writer.write_all(&UNPAUSE_SWAP_AND_ARBITRAGE_IX_DISCM)
            }
            Self::UnpauseUser(args) => {
                writer.write_all(&UNPAUSE_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.unpause_supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.unpause_borrow, &mut writer)?;
                Ok(())
            }
            Self::UpdateAuthority(args) => {
                writer.write_all(&UPDATE_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_authority, &mut writer)?;
                Ok(())
            }
            Self::UpdateAuths(args) => {
                writer.write_all(&UPDATE_AUTHS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.auth_status, &mut writer)?;
                Ok(())
            }
            Self::UpdateCenterPriceAddress(args) => {
                writer.write_all(&UPDATE_CENTER_PRICE_ADDRESS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.center_price_address,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.percent, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.time, &mut writer)?;
                Ok(())
            }
            Self::UpdateCenterPriceLimits(args) => {
                writer.write_all(&UPDATE_CENTER_PRICE_LIMITS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_center_price, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_center_price, &mut writer)?;
                Ok(())
            }
            Self::UpdateFeeAndRevenueCut(args) => {
                writer.write_all(&UPDATE_FEE_AND_REVENUE_CUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.revenue_cut, &mut writer)?;
                Ok(())
            }
            Self::UpdateMaxBorrowShares(args) => {
                writer.write_all(&UPDATE_MAX_BORROW_SHARES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_borrow_shares, &mut writer)?;
                Ok(())
            }
            Self::UpdateMaxSupplyShares(args) => {
                writer.write_all(&UPDATE_MAX_SUPPLY_SHARES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.max_supply_shares, &mut writer)?;
                Ok(())
            }
            Self::UpdateRangePercents(args) => {
                writer.write_all(&UPDATE_RANGE_PERCENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.upper_percent, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.lower_percent, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.shift_time, &mut writer)?;
                Ok(())
            }
            Self::UpdateThresholdPercent(args) => {
                writer.write_all(&UPDATE_THRESHOLD_PERCENT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.upper_threshold_percent,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.lower_threshold_percent,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.threshold_shift_time,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.shift_time, &mut writer)?;
                Ok(())
            }
            Self::UpdateUserBorrowConfig(args) => {
                writer.write_all(&UPDATE_USER_BORROW_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.config, &mut writer)?;
                Ok(())
            }
            Self::UpdateUserSupplyConfig(args) => {
                writer.write_all(&UPDATE_USER_SUPPLY_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.config, &mut writer)?;
                Ok(())
            }
            Self::UpdateUserWithdrawalLimit(args) => {
                writer.write_all(&UPDATE_USER_WITHDRAWAL_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_limit, &mut writer)?;
                Ok(())
            }
            Self::UpdateUtilizationLimit(args) => {
                writer.write_all(&UPDATE_UTILIZATION_LIMIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.token_0_utilization_limit,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.token_1_utilization_limit,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.token0_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.token1_amt, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_shares, &mut writer)?;
                Ok(())
            }
            Self::WithdrawPerfect(args) => {
                writer.write_all(&WITHDRAW_PERFECT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token1, &mut writer)?;
                Ok(())
            }
            Self::WithdrawPerfectInOneToken(args) => {
                writer.write_all(&WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.shares, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token0, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_token1, &mut writer)?;
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
pub const BORROW_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct BorrowAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<BorrowAccounts<'_, '_>> for BorrowKeys {
    fn from(accounts: BorrowAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<BorrowKeys> for [AccountMeta; BORROW_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BORROW_IX_ACCOUNTS_LEN]> for BorrowKeys {
    fn from(pubkeys: [Pubkey; BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<BorrowAccounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BORROW_IX_ACCOUNTS_LEN]>
for BorrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BORROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const BORROW_IX_DISCM: [u8; 8usize] = [228, 253, 131, 202, 207, 116, 89, 18];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowIxArgs {
    pub token0_amt: u64,
    pub token1_amt: u64,
    pub max_shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowIxData(pub BorrowIxArgs);
impl From<BorrowIxArgs> for BorrowIxData {
    fn from(args: BorrowIxArgs) -> Self {
        Self(args)
    }
}
impl BorrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BorrowIxArgs {
                token0_amt,
                token1_amt,
                max_shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowKeys,
    args: BorrowIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_IX_ACCOUNTS_LEN] = keys.into();
    let data: BorrowIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_ix(keys: BorrowKeys, args: BorrowIxArgs) -> std::io::Result<Instruction> {
    borrow_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn borrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
) -> ProgramResult {
    let keys: BorrowKeys = accounts.into();
    let ix = borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_invoke(
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
) -> ProgramResult {
    borrow_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn borrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowKeys = accounts.into();
    let ix = borrow_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_invoke_signed(
    accounts: BorrowAccounts<'_, '_>,
    args: BorrowIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn borrow_verify_account_keys(
    accounts: BorrowAccounts<'_, '_>,
    keys: BorrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_verify_writable_privileges<'me, 'info>(
    accounts: BorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_verify_signer_privileges<'me, 'info>(
    accounts: BorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_verify_account_privileges<'me, 'info>(
    accounts: BorrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_verify_writable_privileges(accounts)?;
    borrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BORROW_PERFECT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct BorrowPerfectAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowPerfectKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<BorrowPerfectAccounts<'_, '_>> for BorrowPerfectKeys {
    fn from(accounts: BorrowPerfectAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<BorrowPerfectKeys> for [AccountMeta; BORROW_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowPerfectKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BORROW_PERFECT_IX_ACCOUNTS_LEN]> for BorrowPerfectKeys {
    fn from(pubkeys: [Pubkey; BORROW_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<BorrowPerfectAccounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowPerfectAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BORROW_PERFECT_IX_ACCOUNTS_LEN]>
for BorrowPerfectAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BORROW_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const BORROW_PERFECT_IX_DISCM: [u8; 8usize] = [210, 67, 245, 255, 134, 200, 153, 58];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowPerfectIxArgs {
    pub shares: u64,
    pub min_token0: u64,
    pub min_token1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowPerfectIxData(pub BorrowPerfectIxArgs);
impl From<BorrowPerfectIxArgs> for BorrowPerfectIxData {
    fn from(args: BorrowPerfectIxArgs) -> Self {
        Self(args)
    }
}
impl BorrowPerfectIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_PERFECT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BorrowPerfectIxArgs {
                shares,
                min_token0,
                min_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_PERFECT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_perfect_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowPerfectKeys,
    args: BorrowPerfectIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_PERFECT_IX_ACCOUNTS_LEN] = keys.into();
    let data: BorrowPerfectIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_perfect_ix(
    keys: BorrowPerfectKeys,
    args: BorrowPerfectIxArgs,
) -> std::io::Result<Instruction> {
    borrow_perfect_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn borrow_perfect_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowPerfectAccounts<'_, '_>,
    args: BorrowPerfectIxArgs,
) -> ProgramResult {
    let keys: BorrowPerfectKeys = accounts.into();
    let ix = borrow_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_perfect_invoke(
    accounts: BorrowPerfectAccounts<'_, '_>,
    args: BorrowPerfectIxArgs,
) -> ProgramResult {
    borrow_perfect_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn borrow_perfect_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowPerfectAccounts<'_, '_>,
    args: BorrowPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowPerfectKeys = accounts.into();
    let ix = borrow_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_perfect_invoke_signed(
    accounts: BorrowPerfectAccounts<'_, '_>,
    args: BorrowPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_perfect_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn borrow_perfect_verify_account_keys(
    accounts: BorrowPerfectAccounts<'_, '_>,
    keys: BorrowPerfectKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_perfect_verify_writable_privileges<'me, 'info>(
    accounts: BorrowPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_perfect_verify_signer_privileges<'me, 'info>(
    accounts: BorrowPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_perfect_verify_account_privileges<'me, 'info>(
    accounts: BorrowPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_perfect_verify_writable_privileges(accounts)?;
    borrow_perfect_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const DEPOSIT_IX_DISCM: [u8; 8usize] = [242, 35, 198, 137, 82, 225, 242, 182];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositIxArgs {
    pub token0_amt: u64,
    pub token1_amt: u64,
    pub min_shares: u64,
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
        let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositIxArgs {
                token0_amt,
                token1_amt,
                min_shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_shares, &mut writer)?;
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
    deposit_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
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
    deposit_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
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
    deposit_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
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
    for should_be_signer in [accounts.signer, accounts.user] {
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
pub const DEPOSIT_PERFECT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct DepositPerfectAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositPerfectKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<DepositPerfectAccounts<'_, '_>> for DepositPerfectKeys {
    fn from(accounts: DepositPerfectAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<DepositPerfectKeys> for [AccountMeta; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositPerfectKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN]> for DepositPerfectKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<DepositPerfectAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositPerfectAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN]>
for DepositPerfectAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const DEPOSIT_PERFECT_IX_DISCM: [u8; 8usize] = [4, 98, 66, 216, 110, 126, 154, 8];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositPerfectIxArgs {
    pub shares: u64,
    pub max_token0: u64,
    pub max_token1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositPerfectIxData(pub DepositPerfectIxArgs);
impl From<DepositPerfectIxArgs> for DepositPerfectIxData {
    fn from(args: DepositPerfectIxArgs) -> Self {
        Self(args)
    }
}
impl DepositPerfectIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_PERFECT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositPerfectIxArgs {
                shares,
                max_token0,
                max_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_PERFECT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_perfect_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositPerfectKeys,
    args: DepositPerfectIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_PERFECT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositPerfectIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_perfect_ix(
    keys: DepositPerfectKeys,
    args: DepositPerfectIxArgs,
) -> std::io::Result<Instruction> {
    deposit_perfect_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn deposit_perfect_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositPerfectAccounts<'_, '_>,
    args: DepositPerfectIxArgs,
) -> ProgramResult {
    let keys: DepositPerfectKeys = accounts.into();
    let ix = deposit_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_perfect_invoke(
    accounts: DepositPerfectAccounts<'_, '_>,
    args: DepositPerfectIxArgs,
) -> ProgramResult {
    deposit_perfect_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn deposit_perfect_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositPerfectAccounts<'_, '_>,
    args: DepositPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositPerfectKeys = accounts.into();
    let ix = deposit_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_perfect_invoke_signed(
    accounts: DepositPerfectAccounts<'_, '_>,
    args: DepositPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_perfect_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_perfect_verify_account_keys(
    accounts: DepositPerfectAccounts<'_, '_>,
    keys: DepositPerfectKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_perfect_verify_writable_privileges<'me, 'info>(
    accounts: DepositPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_perfect_verify_signer_privileges<'me, 'info>(
    accounts: DepositPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_perfect_verify_account_privileges<'me, 'info>(
    accounts: DepositPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_perfect_verify_writable_privileges(accounts)?;
    deposit_perfect_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_DEX_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitDexAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitDexKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitDexAccounts<'_, '_>> for InitDexKeys {
    fn from(accounts: InitDexAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitDexKeys> for [AccountMeta; INIT_DEX_IX_ACCOUNTS_LEN] {
    fn from(keys: InitDexKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
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
impl From<[Pubkey; INIT_DEX_IX_ACCOUNTS_LEN]> for InitDexKeys {
    fn from(pubkeys: [Pubkey; INIT_DEX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            token_0: pubkeys[3],
            token_1: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitDexAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_DEX_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitDexAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_DEX_IX_ACCOUNTS_LEN]>
for InitDexAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_DEX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            token_0: &arr[3],
            token_1: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INIT_DEX_IX_DISCM: [u8; 8usize] = [222, 187, 81, 48, 89, 117, 230, 164];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitDexIxArgs {
    pub params: InitDexParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitDexIxData(pub InitDexIxArgs);
impl From<InitDexIxArgs> for InitDexIxData {
    fn from(args: InitDexIxArgs) -> Self {
        Self(args)
    }
}
impl InitDexIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_DEX_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitDexParams>::deserialize(&mut reader)?
        };
        Ok(Self(InitDexIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_DEX_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_dex_ix_with_program_id(
    program_id: Pubkey,
    keys: InitDexKeys,
    args: InitDexIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_DEX_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitDexIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_dex_ix(
    keys: InitDexKeys,
    args: InitDexIxArgs,
) -> std::io::Result<Instruction> {
    init_dex_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn init_dex_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitDexAccounts<'_, '_>,
    args: InitDexIxArgs,
) -> ProgramResult {
    let keys: InitDexKeys = accounts.into();
    let ix = init_dex_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_dex_invoke(
    accounts: InitDexAccounts<'_, '_>,
    args: InitDexIxArgs,
) -> ProgramResult {
    init_dex_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn init_dex_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitDexAccounts<'_, '_>,
    args: InitDexIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitDexKeys = accounts.into();
    let ix = init_dex_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_dex_invoke_signed(
    accounts: InitDexAccounts<'_, '_>,
    args: InitDexIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_dex_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_dex_verify_account_keys(
    accounts: InitDexAccounts<'_, '_>,
    keys: InitDexKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_dex_verify_writable_privileges<'me, 'info>(
    accounts: InitDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.dex_admin, accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_dex_verify_signer_privileges<'me, 'info>(
    accounts: InitDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_dex_verify_account_privileges<'me, 'info>(
    accounts: InitDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_dex_verify_writable_privileges(accounts)?;
    init_dex_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_DEX_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitDexAdminAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitDexAdminKeys {
    pub signer: Pubkey,
    pub dex_admin: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitDexAdminAccounts<'_, '_>> for InitDexAdminKeys {
    fn from(accounts: InitDexAdminAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex_admin: *accounts.dex_admin.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitDexAdminKeys> for [AccountMeta; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: InitDexAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
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
impl From<[Pubkey; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN]> for InitDexAdminKeys {
    fn from(pubkeys: [Pubkey; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex_admin: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitDexAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitDexAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex_admin.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN]>
for InitDexAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex_admin: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INIT_DEX_ADMIN_IX_DISCM: [u8; 8usize] = [16, 61, 98, 61, 189, 243, 52, 252];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitDexAdminIxArgs {
    pub liquidity: Pubkey,
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitDexAdminIxData(pub InitDexAdminIxArgs);
impl From<InitDexAdminIxArgs> for InitDexAdminIxData {
    fn from(args: InitDexAdminIxArgs) -> Self {
        Self(args)
    }
}
impl InitDexAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_DEX_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let liquidity: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitDexAdminIxArgs {
                liquidity,
                authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_DEX_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_dex_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: InitDexAdminKeys,
    args: InitDexAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_DEX_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitDexAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_dex_admin_ix(
    keys: InitDexAdminKeys,
    args: InitDexAdminIxArgs,
) -> std::io::Result<Instruction> {
    init_dex_admin_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn init_dex_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitDexAdminAccounts<'_, '_>,
    args: InitDexAdminIxArgs,
) -> ProgramResult {
    let keys: InitDexAdminKeys = accounts.into();
    let ix = init_dex_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_dex_admin_invoke(
    accounts: InitDexAdminAccounts<'_, '_>,
    args: InitDexAdminIxArgs,
) -> ProgramResult {
    init_dex_admin_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn init_dex_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitDexAdminAccounts<'_, '_>,
    args: InitDexAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitDexAdminKeys = accounts.into();
    let ix = init_dex_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_dex_admin_invoke_signed(
    accounts: InitDexAdminAccounts<'_, '_>,
    args: InitDexAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_dex_admin_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_dex_admin_verify_account_keys(
    accounts: InitDexAdminAccounts<'_, '_>,
    keys: InitDexAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_dex_admin_verify_writable_privileges<'me, 'info>(
    accounts: InitDexAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.dex_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_dex_admin_verify_signer_privileges<'me, 'info>(
    accounts: InitDexAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_dex_admin_verify_account_privileges<'me, 'info>(
    accounts: InitDexAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_dex_admin_verify_writable_privileges(accounts)?;
    init_dex_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_POSITION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitPositionAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitPositionKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub position: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitPositionAccounts<'_, '_>> for InitPositionKeys {
    fn from(accounts: InitPositionAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            position: *accounts.position.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitPositionKeys> for [AccountMeta; INIT_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
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
impl From<[Pubkey; INIT_POSITION_IX_ACCOUNTS_LEN]> for InitPositionKeys {
    fn from(pubkeys: [Pubkey; INIT_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            position: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitPositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.position.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_POSITION_IX_ACCOUNTS_LEN]>
for InitPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            position: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INIT_POSITION_IX_DISCM: [u8; 8usize] = [197, 20, 10, 1, 97, 160, 177, 91];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitPositionIxArgs {
    pub protocol: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitPositionIxData(pub InitPositionIxArgs);
impl From<InitPositionIxArgs> for InitPositionIxData {
    fn from(args: InitPositionIxArgs) -> Self {
        Self(args)
    }
}
impl InitPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_POSITION_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let protocol: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(InitPositionIxArgs { protocol }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_POSITION_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.protocol, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InitPositionKeys,
    args: InitPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_position_ix(
    keys: InitPositionKeys,
    args: InitPositionIxArgs,
) -> std::io::Result<Instruction> {
    init_position_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn init_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitPositionAccounts<'_, '_>,
    args: InitPositionIxArgs,
) -> ProgramResult {
    let keys: InitPositionKeys = accounts.into();
    let ix = init_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_position_invoke(
    accounts: InitPositionAccounts<'_, '_>,
    args: InitPositionIxArgs,
) -> ProgramResult {
    init_position_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn init_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitPositionAccounts<'_, '_>,
    args: InitPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitPositionKeys = accounts.into();
    let ix = init_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_position_invoke_signed(
    accounts: InitPositionAccounts<'_, '_>,
    args: InitPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_position_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_position_verify_account_keys(
    accounts: InitPositionAccounts<'_, '_>,
    keys: InitPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_position_verify_writable_privileges<'me, 'info>(
    accounts: InitPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_position_verify_signer_privileges<'me, 'info>(
    accounts: InitPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_position_verify_account_privileges<'me, 'info>(
    accounts: InitPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_position_verify_writable_privileges(accounts)?;
    init_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_DEX_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseDexAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseDexKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<PauseDexAccounts<'_, '_>> for PauseDexKeys {
    fn from(accounts: PauseDexAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<PauseDexKeys> for [AccountMeta; PAUSE_DEX_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseDexKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_DEX_IX_ACCOUNTS_LEN]> for PauseDexKeys {
    fn from(pubkeys: [Pubkey; PAUSE_DEX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<PauseDexAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_DEX_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseDexAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_DEX_IX_ACCOUNTS_LEN]>
for PauseDexAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_DEX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const PAUSE_DEX_IX_DISCM: [u8; 8usize] = [138, 255, 101, 0, 116, 202, 128, 100];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseDexIxData;
impl PauseDexIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_DEX_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_DEX_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_dex_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseDexKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_DEX_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseDexIxData.try_to_vec()?,
    })
}
pub fn pause_dex_ix(keys: PauseDexKeys) -> std::io::Result<Instruction> {
    pause_dex_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys)
}
pub fn pause_dex_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseDexAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseDexKeys = accounts.into();
    let ix = pause_dex_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_dex_invoke(accounts: PauseDexAccounts<'_, '_>) -> ProgramResult {
    pause_dex_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts)
}
pub fn pause_dex_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseDexAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseDexKeys = accounts.into();
    let ix = pause_dex_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_dex_invoke_signed(
    accounts: PauseDexAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_dex_invoke_signed_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, seeds)
}
pub fn pause_dex_verify_account_keys(
    accounts: PauseDexAccounts<'_, '_>,
    keys: PauseDexKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_dex_verify_writable_privileges<'me, 'info>(
    accounts: PauseDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_dex_verify_signer_privileges<'me, 'info>(
    accounts: PauseDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_dex_verify_account_privileges<'me, 'info>(
    accounts: PauseDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_dex_verify_writable_privileges(accounts)?;
    pause_dex_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PauseSwapAndArbitrageAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseSwapAndArbitrageKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<PauseSwapAndArbitrageAccounts<'_, '_>> for PauseSwapAndArbitrageKeys {
    fn from(accounts: PauseSwapAndArbitrageAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<PauseSwapAndArbitrageKeys>
for [AccountMeta; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseSwapAndArbitrageKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN]>
for PauseSwapAndArbitrageKeys {
    fn from(pubkeys: [Pubkey; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<PauseSwapAndArbitrageAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseSwapAndArbitrageAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN]>
for PauseSwapAndArbitrageAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const PAUSE_SWAP_AND_ARBITRAGE_IX_DISCM: [u8; 8usize] = [
    252, 67, 166, 62, 45, 136, 88, 76,
];
#[derive(Clone, Debug, PartialEq)]
pub struct PauseSwapAndArbitrageIxData;
impl PauseSwapAndArbitrageIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_SWAP_AND_ARBITRAGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_SWAP_AND_ARBITRAGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_swap_and_arbitrage_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseSwapAndArbitrageKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PauseSwapAndArbitrageIxData.try_to_vec()?,
    })
}
pub fn pause_swap_and_arbitrage_ix(
    keys: PauseSwapAndArbitrageKeys,
) -> std::io::Result<Instruction> {
    pause_swap_and_arbitrage_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys)
}
pub fn pause_swap_and_arbitrage_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseSwapAndArbitrageAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PauseSwapAndArbitrageKeys = accounts.into();
    let ix = pause_swap_and_arbitrage_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_swap_and_arbitrage_invoke(
    accounts: PauseSwapAndArbitrageAccounts<'_, '_>,
) -> ProgramResult {
    pause_swap_and_arbitrage_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
    )
}
pub fn pause_swap_and_arbitrage_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseSwapAndArbitrageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseSwapAndArbitrageKeys = accounts.into();
    let ix = pause_swap_and_arbitrage_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_swap_and_arbitrage_invoke_signed(
    accounts: PauseSwapAndArbitrageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_swap_and_arbitrage_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn pause_swap_and_arbitrage_verify_account_keys(
    accounts: PauseSwapAndArbitrageAccounts<'_, '_>,
    keys: PauseSwapAndArbitrageKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_swap_and_arbitrage_verify_writable_privileges<'me, 'info>(
    accounts: PauseSwapAndArbitrageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_swap_and_arbitrage_verify_signer_privileges<'me, 'info>(
    accounts: PauseSwapAndArbitrageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_swap_and_arbitrage_verify_account_privileges<'me, 'info>(
    accounts: PauseSwapAndArbitrageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_swap_and_arbitrage_verify_writable_privileges(accounts)?;
    pause_swap_and_arbitrage_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_USER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PauseUserAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseUserKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub position: Pubkey,
}
impl From<PauseUserAccounts<'_, '_>> for PauseUserKeys {
    fn from(accounts: PauseUserAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            position: *accounts.position.key,
        }
    }
}
impl From<PauseUserKeys> for [AccountMeta; PAUSE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_USER_IX_ACCOUNTS_LEN]> for PauseUserKeys {
    fn from(pubkeys: [Pubkey; PAUSE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            position: pubkeys[3],
        }
    }
}
impl<'info> From<PauseUserAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseUserAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.position.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_USER_IX_ACCOUNTS_LEN]>
for PauseUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            position: &arr[3],
        }
    }
}
pub const PAUSE_USER_IX_DISCM: [u8; 8usize] = [18, 63, 43, 94, 239, 53, 101, 14];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PauseUserIxArgs {
    pub pause_supply: bool,
    pub pause_borrow: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PauseUserIxData(pub PauseUserIxArgs);
impl From<PauseUserIxArgs> for PauseUserIxData {
    fn from(args: PauseUserIxArgs) -> Self {
        Self(args)
    }
}
impl PauseUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let pause_supply: bool = crate::borsh_de_or_default(&mut reader)?;
        let pause_borrow: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PauseUserIxArgs {
                pause_supply,
                pause_borrow,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.pause_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pause_borrow, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_user_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseUserKeys,
    args: PauseUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: PauseUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pause_user_ix(
    keys: PauseUserKeys,
    args: PauseUserIxArgs,
) -> std::io::Result<Instruction> {
    pause_user_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn pause_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseUserAccounts<'_, '_>,
    args: PauseUserIxArgs,
) -> ProgramResult {
    let keys: PauseUserKeys = accounts.into();
    let ix = pause_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_user_invoke(
    accounts: PauseUserAccounts<'_, '_>,
    args: PauseUserIxArgs,
) -> ProgramResult {
    pause_user_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn pause_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseUserAccounts<'_, '_>,
    args: PauseUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseUserKeys = accounts.into();
    let ix = pause_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_user_invoke_signed(
    accounts: PauseUserAccounts<'_, '_>,
    args: PauseUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_user_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn pause_user_verify_account_keys(
    accounts: PauseUserAccounts<'_, '_>,
    keys: PauseUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_user_verify_writable_privileges<'me, 'info>(
    accounts: PauseUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_user_verify_signer_privileges<'me, 'info>(
    accounts: PauseUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_user_verify_account_privileges<'me, 'info>(
    accounts: PauseUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_user_verify_writable_privileges(accounts)?;
    pause_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAYBACK_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct PaybackAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PaybackKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<PaybackAccounts<'_, '_>> for PaybackKeys {
    fn from(accounts: PaybackAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<PaybackKeys> for [AccountMeta; PAYBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: PaybackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAYBACK_IX_ACCOUNTS_LEN]> for PaybackKeys {
    fn from(pubkeys: [Pubkey; PAYBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<PaybackAccounts<'_, 'info>>
for [AccountInfo<'info>; PAYBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: PaybackAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAYBACK_IX_ACCOUNTS_LEN]>
for PaybackAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAYBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const PAYBACK_IX_DISCM: [u8; 8usize] = [148, 144, 50, 144, 8, 112, 203, 3];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaybackIxArgs {
    pub token0_amt: u64,
    pub token1_amt: u64,
    pub min_shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PaybackIxData(pub PaybackIxArgs);
impl From<PaybackIxArgs> for PaybackIxData {
    fn from(args: PaybackIxArgs) -> Self {
        Self(args)
    }
}
impl PaybackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAYBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PaybackIxArgs {
                token0_amt,
                token1_amt,
                min_shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAYBACK_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn payback_ix_with_program_id(
    program_id: Pubkey,
    keys: PaybackKeys,
    args: PaybackIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAYBACK_IX_ACCOUNTS_LEN] = keys.into();
    let data: PaybackIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn payback_ix(
    keys: PaybackKeys,
    args: PaybackIxArgs,
) -> std::io::Result<Instruction> {
    payback_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn payback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PaybackAccounts<'_, '_>,
    args: PaybackIxArgs,
) -> ProgramResult {
    let keys: PaybackKeys = accounts.into();
    let ix = payback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn payback_invoke(
    accounts: PaybackAccounts<'_, '_>,
    args: PaybackIxArgs,
) -> ProgramResult {
    payback_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn payback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PaybackAccounts<'_, '_>,
    args: PaybackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PaybackKeys = accounts.into();
    let ix = payback_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn payback_invoke_signed(
    accounts: PaybackAccounts<'_, '_>,
    args: PaybackIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    payback_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn payback_verify_account_keys(
    accounts: PaybackAccounts<'_, '_>,
    keys: PaybackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn payback_verify_writable_privileges<'me, 'info>(
    accounts: PaybackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn payback_verify_signer_privileges<'me, 'info>(
    accounts: PaybackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn payback_verify_account_privileges<'me, 'info>(
    accounts: PaybackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    payback_verify_writable_privileges(accounts)?;
    payback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAYBACK_PERFECT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct PaybackPerfectAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PaybackPerfectKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<PaybackPerfectAccounts<'_, '_>> for PaybackPerfectKeys {
    fn from(accounts: PaybackPerfectAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<PaybackPerfectKeys> for [AccountMeta; PAYBACK_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(keys: PaybackPerfectKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAYBACK_PERFECT_IX_ACCOUNTS_LEN]> for PaybackPerfectKeys {
    fn from(pubkeys: [Pubkey; PAYBACK_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<PaybackPerfectAccounts<'_, 'info>>
for [AccountInfo<'info>; PAYBACK_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(accounts: PaybackPerfectAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAYBACK_PERFECT_IX_ACCOUNTS_LEN]>
for PaybackPerfectAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAYBACK_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const PAYBACK_PERFECT_IX_DISCM: [u8; 8usize] = [39, 2, 197, 102, 11, 186, 97, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaybackPerfectIxArgs {
    pub shares: u64,
    pub max_token0: u64,
    pub max_token1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PaybackPerfectIxData(pub PaybackPerfectIxArgs);
impl From<PaybackPerfectIxArgs> for PaybackPerfectIxData {
    fn from(args: PaybackPerfectIxArgs) -> Self {
        Self(args)
    }
}
impl PaybackPerfectIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAYBACK_PERFECT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PaybackPerfectIxArgs {
                shares,
                max_token0,
                max_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAYBACK_PERFECT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn payback_perfect_ix_with_program_id(
    program_id: Pubkey,
    keys: PaybackPerfectKeys,
    args: PaybackPerfectIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAYBACK_PERFECT_IX_ACCOUNTS_LEN] = keys.into();
    let data: PaybackPerfectIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn payback_perfect_ix(
    keys: PaybackPerfectKeys,
    args: PaybackPerfectIxArgs,
) -> std::io::Result<Instruction> {
    payback_perfect_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn payback_perfect_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PaybackPerfectAccounts<'_, '_>,
    args: PaybackPerfectIxArgs,
) -> ProgramResult {
    let keys: PaybackPerfectKeys = accounts.into();
    let ix = payback_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn payback_perfect_invoke(
    accounts: PaybackPerfectAccounts<'_, '_>,
    args: PaybackPerfectIxArgs,
) -> ProgramResult {
    payback_perfect_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn payback_perfect_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PaybackPerfectAccounts<'_, '_>,
    args: PaybackPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PaybackPerfectKeys = accounts.into();
    let ix = payback_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn payback_perfect_invoke_signed(
    accounts: PaybackPerfectAccounts<'_, '_>,
    args: PaybackPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    payback_perfect_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn payback_perfect_verify_account_keys(
    accounts: PaybackPerfectAccounts<'_, '_>,
    keys: PaybackPerfectKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn payback_perfect_verify_writable_privileges<'me, 'info>(
    accounts: PaybackPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn payback_perfect_verify_signer_privileges<'me, 'info>(
    accounts: PaybackPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn payback_perfect_verify_account_privileges<'me, 'info>(
    accounts: PaybackPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    payback_perfect_verify_writable_privileges(accounts)?;
    payback_perfect_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct PaybackPerfectInOneTokenAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PaybackPerfectInOneTokenKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<PaybackPerfectInOneTokenAccounts<'_, '_>> for PaybackPerfectInOneTokenKeys {
    fn from(accounts: PaybackPerfectInOneTokenAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<PaybackPerfectInOneTokenKeys>
for [AccountMeta; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: PaybackPerfectInOneTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN]>
for PaybackPerfectInOneTokenKeys {
    fn from(pubkeys: [Pubkey; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<PaybackPerfectInOneTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: PaybackPerfectInOneTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN]>
for PaybackPerfectInOneTokenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const PAYBACK_PERFECT_IN_ONE_TOKEN_IX_DISCM: [u8; 8usize] = [
    134, 210, 96, 6, 127, 183, 182, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaybackPerfectInOneTokenIxArgs {
    pub shares: u64,
    pub max_token0: u64,
    pub max_token1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PaybackPerfectInOneTokenIxData(pub PaybackPerfectInOneTokenIxArgs);
impl From<PaybackPerfectInOneTokenIxArgs> for PaybackPerfectInOneTokenIxData {
    fn from(args: PaybackPerfectInOneTokenIxArgs) -> Self {
        Self(args)
    }
}
impl PaybackPerfectInOneTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAYBACK_PERFECT_IN_ONE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PaybackPerfectInOneTokenIxArgs {
                shares,
                max_token0,
                max_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAYBACK_PERFECT_IN_ONE_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn payback_perfect_in_one_token_ix_with_program_id(
    program_id: Pubkey,
    keys: PaybackPerfectInOneTokenKeys,
    args: PaybackPerfectInOneTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAYBACK_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: PaybackPerfectInOneTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn payback_perfect_in_one_token_ix(
    keys: PaybackPerfectInOneTokenKeys,
    args: PaybackPerfectInOneTokenIxArgs,
) -> std::io::Result<Instruction> {
    payback_perfect_in_one_token_ix_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn payback_perfect_in_one_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PaybackPerfectInOneTokenAccounts<'_, '_>,
    args: PaybackPerfectInOneTokenIxArgs,
) -> ProgramResult {
    let keys: PaybackPerfectInOneTokenKeys = accounts.into();
    let ix = payback_perfect_in_one_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn payback_perfect_in_one_token_invoke(
    accounts: PaybackPerfectInOneTokenAccounts<'_, '_>,
    args: PaybackPerfectInOneTokenIxArgs,
) -> ProgramResult {
    payback_perfect_in_one_token_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn payback_perfect_in_one_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PaybackPerfectInOneTokenAccounts<'_, '_>,
    args: PaybackPerfectInOneTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PaybackPerfectInOneTokenKeys = accounts.into();
    let ix = payback_perfect_in_one_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn payback_perfect_in_one_token_invoke_signed(
    accounts: PaybackPerfectInOneTokenAccounts<'_, '_>,
    args: PaybackPerfectInOneTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    payback_perfect_in_one_token_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn payback_perfect_in_one_token_verify_account_keys(
    accounts: PaybackPerfectInOneTokenAccounts<'_, '_>,
    keys: PaybackPerfectInOneTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn payback_perfect_in_one_token_verify_writable_privileges<'me, 'info>(
    accounts: PaybackPerfectInOneTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn payback_perfect_in_one_token_verify_signer_privileges<'me, 'info>(
    accounts: PaybackPerfectInOneTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn payback_perfect_in_one_token_verify_account_privileges<'me, 'info>(
    accounts: PaybackPerfectInOneTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    payback_perfect_in_one_token_verify_writable_privileges(accounts)?;
    payback_perfect_in_one_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct PreviewDexSharesAccounts<'me, 'info> {
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PreviewDexSharesKeys {
    pub dex: Pubkey,
    pub position: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub oracle_program: Pubkey,
}
impl From<PreviewDexSharesAccounts<'_, '_>> for PreviewDexSharesKeys {
    fn from(accounts: PreviewDexSharesAccounts) -> Self {
        Self {
            dex: *accounts.dex.key,
            position: *accounts.position.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            oracle_program: *accounts.oracle_program.key,
        }
    }
}
impl From<PreviewDexSharesKeys> for [AccountMeta; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: PreviewDexSharesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN]> for PreviewDexSharesKeys {
    fn from(pubkeys: [Pubkey; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dex: pubkeys[0],
            position: pubkeys[1],
            token_0_reserve: pubkeys[2],
            token_1_reserve: pubkeys[3],
            dex_supply_position_token_0: pubkeys[4],
            dex_supply_position_token_1: pubkeys[5],
            dex_borrow_position_token_0: pubkeys[6],
            dex_borrow_position_token_1: pubkeys[7],
            oracle_program: pubkeys[8],
        }
    }
}
impl<'info> From<PreviewDexSharesAccounts<'_, 'info>>
for [AccountInfo<'info>; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN] {
    fn from(accounts: PreviewDexSharesAccounts<'_, 'info>) -> Self {
        [
            accounts.dex.clone(),
            accounts.position.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.oracle_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN]>
for PreviewDexSharesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            dex: &arr[0],
            position: &arr[1],
            token_0_reserve: &arr[2],
            token_1_reserve: &arr[3],
            dex_supply_position_token_0: &arr[4],
            dex_supply_position_token_1: &arr[5],
            dex_borrow_position_token_0: &arr[6],
            dex_borrow_position_token_1: &arr[7],
            oracle_program: &arr[8],
        }
    }
}
pub const PREVIEW_DEX_SHARES_IX_DISCM: [u8; 8usize] = [
    246, 97, 50, 171, 63, 142, 62, 229,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreviewDexSharesIxArgs {
    pub col_token0: i64,
    pub col_token1: i64,
    pub debt_token0: i64,
    pub debt_token1: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewDexSharesIxData(pub PreviewDexSharesIxArgs);
impl From<PreviewDexSharesIxArgs> for PreviewDexSharesIxData {
    fn from(args: PreviewDexSharesIxArgs) -> Self {
        Self(args)
    }
}
impl PreviewDexSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PREVIEW_DEX_SHARES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let col_token0: i64 = crate::borsh_de_or_default(&mut reader)?;
        let col_token1: i64 = crate::borsh_de_or_default(&mut reader)?;
        let debt_token0: i64 = crate::borsh_de_or_default(&mut reader)?;
        let debt_token1: i64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(PreviewDexSharesIxArgs {
                col_token0,
                col_token1,
                debt_token0,
                debt_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PREVIEW_DEX_SHARES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.col_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.col_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.debt_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.debt_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn preview_dex_shares_ix_with_program_id(
    program_id: Pubkey,
    keys: PreviewDexSharesKeys,
    args: PreviewDexSharesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PREVIEW_DEX_SHARES_IX_ACCOUNTS_LEN] = keys.into();
    let data: PreviewDexSharesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn preview_dex_shares_ix(
    keys: PreviewDexSharesKeys,
    args: PreviewDexSharesIxArgs,
) -> std::io::Result<Instruction> {
    preview_dex_shares_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn preview_dex_shares_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PreviewDexSharesAccounts<'_, '_>,
    args: PreviewDexSharesIxArgs,
) -> ProgramResult {
    let keys: PreviewDexSharesKeys = accounts.into();
    let ix = preview_dex_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn preview_dex_shares_invoke(
    accounts: PreviewDexSharesAccounts<'_, '_>,
    args: PreviewDexSharesIxArgs,
) -> ProgramResult {
    preview_dex_shares_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn preview_dex_shares_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PreviewDexSharesAccounts<'_, '_>,
    args: PreviewDexSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PreviewDexSharesKeys = accounts.into();
    let ix = preview_dex_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn preview_dex_shares_invoke_signed(
    accounts: PreviewDexSharesAccounts<'_, '_>,
    args: PreviewDexSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    preview_dex_shares_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn preview_dex_shares_verify_account_keys(
    accounts: PreviewDexSharesAccounts<'_, '_>,
    keys: PreviewDexSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.oracle_program.key, keys.oracle_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const SWAP_IN_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapInAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapInKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub recipient: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub oracle_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<SwapInAccounts<'_, '_>> for SwapInKeys {
    fn from(accounts: SwapInAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            recipient: *accounts.recipient.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            oracle_program: *accounts.oracle_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<SwapInKeys> for [AccountMeta; SWAP_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IN_IX_ACCOUNTS_LEN]> for SwapInKeys {
    fn from(pubkeys: [Pubkey; SWAP_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user_token_0_account: pubkeys[2],
            user_token_1_account: pubkeys[3],
            recipient: pubkeys[4],
            recipient_token_0_account: pubkeys[5],
            recipient_token_1_account: pubkeys[6],
            token_0: pubkeys[7],
            token_1: pubkeys[8],
            token_0_reserve: pubkeys[9],
            token_1_reserve: pubkeys[10],
            token_0_rate_model: pubkeys[11],
            token_1_rate_model: pubkeys[12],
            token_0_vault: pubkeys[13],
            token_1_vault: pubkeys[14],
            dex_supply_position_token_0: pubkeys[15],
            dex_supply_position_token_1: pubkeys[16],
            dex_borrow_position_token_0: pubkeys[17],
            dex_borrow_position_token_1: pubkeys[18],
            liquidity: pubkeys[19],
            liquidity_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            oracle_program: pubkeys[23],
            associated_token_program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapInAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapInAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.recipient.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.oracle_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IN_IX_ACCOUNTS_LEN]>
for SwapInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user_token_0_account: &arr[2],
            user_token_1_account: &arr[3],
            recipient: &arr[4],
            recipient_token_0_account: &arr[5],
            recipient_token_1_account: &arr[6],
            token_0: &arr[7],
            token_1: &arr[8],
            token_0_reserve: &arr[9],
            token_1_reserve: &arr[10],
            token_0_rate_model: &arr[11],
            token_1_rate_model: &arr[12],
            token_0_vault: &arr[13],
            token_1_vault: &arr[14],
            dex_supply_position_token_0: &arr[15],
            dex_supply_position_token_1: &arr[16],
            dex_borrow_position_token_0: &arr[17],
            dex_borrow_position_token_1: &arr[18],
            liquidity: &arr[19],
            liquidity_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            oracle_program: &arr[23],
            associated_token_program: &arr[24],
        }
    }
}
pub const SWAP_IN_IX_DISCM: [u8; 8usize] = [141, 172, 10, 208, 69, 9, 56, 154];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapInIxArgs {
    pub swap0to1: bool,
    pub amount_in: u64,
    pub amount_out_min: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapInIxData(pub SwapInIxArgs);
impl From<SwapInIxArgs> for SwapInIxData {
    fn from(args: SwapInIxArgs) -> Self {
        Self(args)
    }
}
impl SwapInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let swap0to1: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out_min: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapInIxArgs {
                swap0to1,
                amount_in,
                amount_out_min,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.swap0to1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out_min, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapInKeys,
    args: SwapInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_in_ix(keys: SwapInKeys, args: SwapInIxArgs) -> std::io::Result<Instruction> {
    swap_in_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn swap_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapInAccounts<'_, '_>,
    args: SwapInIxArgs,
) -> ProgramResult {
    let keys: SwapInKeys = accounts.into();
    let ix = swap_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_in_invoke(
    accounts: SwapInAccounts<'_, '_>,
    args: SwapInIxArgs,
) -> ProgramResult {
    swap_in_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn swap_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapInAccounts<'_, '_>,
    args: SwapInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapInKeys = accounts.into();
    let ix = swap_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_in_invoke_signed(
    accounts: SwapInAccounts<'_, '_>,
    args: SwapInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_in_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_in_verify_account_keys(
    accounts: SwapInAccounts<'_, '_>,
    keys: SwapInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_in_verify_writable_privileges<'me, 'info>(
    accounts: SwapInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_in_verify_signer_privileges<'me, 'info>(
    accounts: SwapInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_in_verify_account_privileges<'me, 'info>(
    accounts: SwapInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_in_verify_writable_privileges(accounts)?;
    swap_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_OUT_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct SwapOutAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub recipient_token_0_account: &'me AccountInfo<'info>,
    pub recipient_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapOutKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub recipient: Pubkey,
    pub recipient_token_0_account: Pubkey,
    pub recipient_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub oracle_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<SwapOutAccounts<'_, '_>> for SwapOutKeys {
    fn from(accounts: SwapOutAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            recipient: *accounts.recipient.key,
            recipient_token_0_account: *accounts.recipient_token_0_account.key,
            recipient_token_1_account: *accounts.recipient_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            oracle_program: *accounts.oracle_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<SwapOutKeys> for [AccountMeta; SWAP_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_OUT_IX_ACCOUNTS_LEN]> for SwapOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user_token_0_account: pubkeys[2],
            user_token_1_account: pubkeys[3],
            recipient: pubkeys[4],
            recipient_token_0_account: pubkeys[5],
            recipient_token_1_account: pubkeys[6],
            token_0: pubkeys[7],
            token_1: pubkeys[8],
            token_0_reserve: pubkeys[9],
            token_1_reserve: pubkeys[10],
            token_0_rate_model: pubkeys[11],
            token_1_rate_model: pubkeys[12],
            token_0_vault: pubkeys[13],
            token_1_vault: pubkeys[14],
            dex_supply_position_token_0: pubkeys[15],
            dex_supply_position_token_1: pubkeys[16],
            dex_borrow_position_token_0: pubkeys[17],
            dex_borrow_position_token_1: pubkeys[18],
            liquidity: pubkeys[19],
            liquidity_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            oracle_program: pubkeys[23],
            associated_token_program: pubkeys[24],
        }
    }
}
impl<'info> From<SwapOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapOutAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.recipient.clone(),
            accounts.recipient_token_0_account.clone(),
            accounts.recipient_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.oracle_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_OUT_IX_ACCOUNTS_LEN]>
for SwapOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user_token_0_account: &arr[2],
            user_token_1_account: &arr[3],
            recipient: &arr[4],
            recipient_token_0_account: &arr[5],
            recipient_token_1_account: &arr[6],
            token_0: &arr[7],
            token_1: &arr[8],
            token_0_reserve: &arr[9],
            token_1_reserve: &arr[10],
            token_0_rate_model: &arr[11],
            token_1_rate_model: &arr[12],
            token_0_vault: &arr[13],
            token_1_vault: &arr[14],
            dex_supply_position_token_0: &arr[15],
            dex_supply_position_token_1: &arr[16],
            dex_borrow_position_token_0: &arr[17],
            dex_borrow_position_token_1: &arr[18],
            liquidity: &arr[19],
            liquidity_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            oracle_program: &arr[23],
            associated_token_program: &arr[24],
        }
    }
}
pub const SWAP_OUT_IX_DISCM: [u8; 8usize] = [206, 36, 149, 14, 163, 132, 148, 1];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapOutIxArgs {
    pub swap0to1: bool,
    pub amount_out: u64,
    pub amount_in_max: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapOutIxData(pub SwapOutIxArgs);
impl From<SwapOutIxArgs> for SwapOutIxData {
    fn from(args: SwapOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let swap0to1: bool = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_max: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SwapOutIxArgs {
                swap0to1,
                amount_out,
                amount_in_max,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.swap0to1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in_max, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapOutKeys,
    args: SwapOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_out_ix(
    keys: SwapOutKeys,
    args: SwapOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_out_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn swap_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapOutAccounts<'_, '_>,
    args: SwapOutIxArgs,
) -> ProgramResult {
    let keys: SwapOutKeys = accounts.into();
    let ix = swap_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_out_invoke(
    accounts: SwapOutAccounts<'_, '_>,
    args: SwapOutIxArgs,
) -> ProgramResult {
    swap_out_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn swap_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapOutAccounts<'_, '_>,
    args: SwapOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapOutKeys = accounts.into();
    let ix = swap_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_out_invoke_signed(
    accounts: SwapOutAccounts<'_, '_>,
    args: SwapOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_out_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_out_verify_account_keys(
    accounts: SwapOutAccounts<'_, '_>,
    keys: SwapOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.recipient_token_0_account.key, keys.recipient_token_0_account),
        (*accounts.recipient_token_1_account.key, keys.recipient_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_out_verify_writable_privileges<'me, 'info>(
    accounts: SwapOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.recipient_token_0_account,
        accounts.recipient_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_out_verify_signer_privileges<'me, 'info>(
    accounts: SwapOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_out_verify_account_privileges<'me, 'info>(
    accounts: SwapOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_out_verify_writable_privileges(accounts)?;
    swap_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TURN_ON_SMART_COL_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct TurnOnSmartColAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub admin_token_0_account: &'me AccountInfo<'info>,
    pub admin_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TurnOnSmartColKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub admin_token_0_account: Pubkey,
    pub admin_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<TurnOnSmartColAccounts<'_, '_>> for TurnOnSmartColKeys {
    fn from(accounts: TurnOnSmartColAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            admin_token_0_account: *accounts.admin_token_0_account.key,
            admin_token_1_account: *accounts.admin_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<TurnOnSmartColKeys> for [AccountMeta; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN] {
    fn from(keys: TurnOnSmartColKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN]> for TurnOnSmartColKeys {
    fn from(pubkeys: [Pubkey; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            admin_token_0_account: pubkeys[3],
            admin_token_1_account: pubkeys[4],
            token_0: pubkeys[5],
            token_1: pubkeys[6],
            token_0_reserve: pubkeys[7],
            token_1_reserve: pubkeys[8],
            token_0_rate_model: pubkeys[9],
            token_1_rate_model: pubkeys[10],
            token_0_vault: pubkeys[11],
            token_1_vault: pubkeys[12],
            dex_supply_position_token_0: pubkeys[13],
            dex_supply_position_token_1: pubkeys[14],
            dex_borrow_position_token_0: pubkeys[15],
            dex_borrow_position_token_1: pubkeys[16],
            liquidity: pubkeys[17],
            liquidity_program: pubkeys[18],
            token_0_program: pubkeys[19],
            token_1_program: pubkeys[20],
            associated_token_program: pubkeys[21],
        }
    }
}
impl<'info> From<TurnOnSmartColAccounts<'_, 'info>>
for [AccountInfo<'info>; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN] {
    fn from(accounts: TurnOnSmartColAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.admin_token_0_account.clone(),
            accounts.admin_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN]>
for TurnOnSmartColAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            admin_token_0_account: &arr[3],
            admin_token_1_account: &arr[4],
            token_0: &arr[5],
            token_1: &arr[6],
            token_0_reserve: &arr[7],
            token_1_reserve: &arr[8],
            token_0_rate_model: &arr[9],
            token_1_rate_model: &arr[10],
            token_0_vault: &arr[11],
            token_1_vault: &arr[12],
            dex_supply_position_token_0: &arr[13],
            dex_supply_position_token_1: &arr[14],
            dex_borrow_position_token_0: &arr[15],
            dex_borrow_position_token_1: &arr[16],
            liquidity: &arr[17],
            liquidity_program: &arr[18],
            token_0_program: &arr[19],
            token_1_program: &arr[20],
            associated_token_program: &arr[21],
        }
    }
}
pub const TURN_ON_SMART_COL_IX_DISCM: [u8; 8usize] = [
    143, 236, 131, 173, 22, 90, 214, 202,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TurnOnSmartColIxArgs {
    pub token_0_amt: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TurnOnSmartColIxData(pub TurnOnSmartColIxArgs);
impl From<TurnOnSmartColIxArgs> for TurnOnSmartColIxData {
    fn from(args: TurnOnSmartColIxArgs) -> Self {
        Self(args)
    }
}
impl TurnOnSmartColIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TURN_ON_SMART_COL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TurnOnSmartColIxArgs {
                token_0_amt,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TURN_ON_SMART_COL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_0_amt, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn turn_on_smart_col_ix_with_program_id(
    program_id: Pubkey,
    keys: TurnOnSmartColKeys,
    args: TurnOnSmartColIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TURN_ON_SMART_COL_IX_ACCOUNTS_LEN] = keys.into();
    let data: TurnOnSmartColIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn turn_on_smart_col_ix(
    keys: TurnOnSmartColKeys,
    args: TurnOnSmartColIxArgs,
) -> std::io::Result<Instruction> {
    turn_on_smart_col_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn turn_on_smart_col_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TurnOnSmartColAccounts<'_, '_>,
    args: TurnOnSmartColIxArgs,
) -> ProgramResult {
    let keys: TurnOnSmartColKeys = accounts.into();
    let ix = turn_on_smart_col_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn turn_on_smart_col_invoke(
    accounts: TurnOnSmartColAccounts<'_, '_>,
    args: TurnOnSmartColIxArgs,
) -> ProgramResult {
    turn_on_smart_col_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn turn_on_smart_col_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TurnOnSmartColAccounts<'_, '_>,
    args: TurnOnSmartColIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TurnOnSmartColKeys = accounts.into();
    let ix = turn_on_smart_col_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn turn_on_smart_col_invoke_signed(
    accounts: TurnOnSmartColAccounts<'_, '_>,
    args: TurnOnSmartColIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    turn_on_smart_col_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn turn_on_smart_col_verify_account_keys(
    accounts: TurnOnSmartColAccounts<'_, '_>,
    keys: TurnOnSmartColKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.admin_token_0_account.key, keys.admin_token_0_account),
        (*accounts.admin_token_1_account.key, keys.admin_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn turn_on_smart_col_verify_writable_privileges<'me, 'info>(
    accounts: TurnOnSmartColAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.dex,
        accounts.admin_token_0_account,
        accounts.admin_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn turn_on_smart_col_verify_signer_privileges<'me, 'info>(
    accounts: TurnOnSmartColAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn turn_on_smart_col_verify_account_privileges<'me, 'info>(
    accounts: TurnOnSmartColAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    turn_on_smart_col_verify_writable_privileges(accounts)?;
    turn_on_smart_col_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct TurnOnSmartDebtAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub admin_token_0_account: &'me AccountInfo<'info>,
    pub admin_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TurnOnSmartDebtKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub admin_token_0_account: Pubkey,
    pub admin_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<TurnOnSmartDebtAccounts<'_, '_>> for TurnOnSmartDebtKeys {
    fn from(accounts: TurnOnSmartDebtAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            admin_token_0_account: *accounts.admin_token_0_account.key,
            admin_token_1_account: *accounts.admin_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<TurnOnSmartDebtKeys> for [AccountMeta; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN] {
    fn from(keys: TurnOnSmartDebtKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN]> for TurnOnSmartDebtKeys {
    fn from(pubkeys: [Pubkey; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            admin_token_0_account: pubkeys[3],
            admin_token_1_account: pubkeys[4],
            token_0: pubkeys[5],
            token_1: pubkeys[6],
            token_0_reserve: pubkeys[7],
            token_1_reserve: pubkeys[8],
            token_0_rate_model: pubkeys[9],
            token_1_rate_model: pubkeys[10],
            token_0_vault: pubkeys[11],
            token_1_vault: pubkeys[12],
            dex_supply_position_token_0: pubkeys[13],
            dex_supply_position_token_1: pubkeys[14],
            dex_borrow_position_token_0: pubkeys[15],
            dex_borrow_position_token_1: pubkeys[16],
            liquidity: pubkeys[17],
            liquidity_program: pubkeys[18],
            token_0_program: pubkeys[19],
            token_1_program: pubkeys[20],
            associated_token_program: pubkeys[21],
        }
    }
}
impl<'info> From<TurnOnSmartDebtAccounts<'_, 'info>>
for [AccountInfo<'info>; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN] {
    fn from(accounts: TurnOnSmartDebtAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.admin_token_0_account.clone(),
            accounts.admin_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN]>
for TurnOnSmartDebtAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            admin_token_0_account: &arr[3],
            admin_token_1_account: &arr[4],
            token_0: &arr[5],
            token_1: &arr[6],
            token_0_reserve: &arr[7],
            token_1_reserve: &arr[8],
            token_0_rate_model: &arr[9],
            token_1_rate_model: &arr[10],
            token_0_vault: &arr[11],
            token_1_vault: &arr[12],
            dex_supply_position_token_0: &arr[13],
            dex_supply_position_token_1: &arr[14],
            dex_borrow_position_token_0: &arr[15],
            dex_borrow_position_token_1: &arr[16],
            liquidity: &arr[17],
            liquidity_program: &arr[18],
            token_0_program: &arr[19],
            token_1_program: &arr[20],
            associated_token_program: &arr[21],
        }
    }
}
pub const TURN_ON_SMART_DEBT_IX_DISCM: [u8; 8usize] = [
    177, 184, 215, 221, 92, 231, 153, 83,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TurnOnSmartDebtIxArgs {
    pub token_0_amt: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TurnOnSmartDebtIxData(pub TurnOnSmartDebtIxArgs);
impl From<TurnOnSmartDebtIxArgs> for TurnOnSmartDebtIxData {
    fn from(args: TurnOnSmartDebtIxArgs) -> Self {
        Self(args)
    }
}
impl TurnOnSmartDebtIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TURN_ON_SMART_DEBT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(TurnOnSmartDebtIxArgs {
                token_0_amt,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TURN_ON_SMART_DEBT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token_0_amt, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn turn_on_smart_debt_ix_with_program_id(
    program_id: Pubkey,
    keys: TurnOnSmartDebtKeys,
    args: TurnOnSmartDebtIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TURN_ON_SMART_DEBT_IX_ACCOUNTS_LEN] = keys.into();
    let data: TurnOnSmartDebtIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn turn_on_smart_debt_ix(
    keys: TurnOnSmartDebtKeys,
    args: TurnOnSmartDebtIxArgs,
) -> std::io::Result<Instruction> {
    turn_on_smart_debt_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn turn_on_smart_debt_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TurnOnSmartDebtAccounts<'_, '_>,
    args: TurnOnSmartDebtIxArgs,
) -> ProgramResult {
    let keys: TurnOnSmartDebtKeys = accounts.into();
    let ix = turn_on_smart_debt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn turn_on_smart_debt_invoke(
    accounts: TurnOnSmartDebtAccounts<'_, '_>,
    args: TurnOnSmartDebtIxArgs,
) -> ProgramResult {
    turn_on_smart_debt_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn turn_on_smart_debt_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TurnOnSmartDebtAccounts<'_, '_>,
    args: TurnOnSmartDebtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TurnOnSmartDebtKeys = accounts.into();
    let ix = turn_on_smart_debt_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn turn_on_smart_debt_invoke_signed(
    accounts: TurnOnSmartDebtAccounts<'_, '_>,
    args: TurnOnSmartDebtIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    turn_on_smart_debt_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn turn_on_smart_debt_verify_account_keys(
    accounts: TurnOnSmartDebtAccounts<'_, '_>,
    keys: TurnOnSmartDebtKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.admin_token_0_account.key, keys.admin_token_0_account),
        (*accounts.admin_token_1_account.key, keys.admin_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn turn_on_smart_debt_verify_writable_privileges<'me, 'info>(
    accounts: TurnOnSmartDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.dex,
        accounts.admin_token_0_account,
        accounts.admin_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn turn_on_smart_debt_verify_signer_privileges<'me, 'info>(
    accounts: TurnOnSmartDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn turn_on_smart_debt_verify_account_privileges<'me, 'info>(
    accounts: TurnOnSmartDebtAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    turn_on_smart_debt_verify_writable_privileges(accounts)?;
    turn_on_smart_debt_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_DEX_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseDexAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseDexKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UnpauseDexAccounts<'_, '_>> for UnpauseDexKeys {
    fn from(accounts: UnpauseDexAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UnpauseDexKeys> for [AccountMeta; UNPAUSE_DEX_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseDexKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_DEX_IX_ACCOUNTS_LEN]> for UnpauseDexKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_DEX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UnpauseDexAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_DEX_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseDexAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_DEX_IX_ACCOUNTS_LEN]>
for UnpauseDexAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_DEX_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UNPAUSE_DEX_IX_DISCM: [u8; 8usize] = [88, 52, 175, 105, 210, 116, 178, 218];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseDexIxData;
impl UnpauseDexIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_DEX_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_DEX_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_dex_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseDexKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_DEX_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseDexIxData.try_to_vec()?,
    })
}
pub fn unpause_dex_ix(keys: UnpauseDexKeys) -> std::io::Result<Instruction> {
    unpause_dex_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys)
}
pub fn unpause_dex_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseDexAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseDexKeys = accounts.into();
    let ix = unpause_dex_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_dex_invoke(accounts: UnpauseDexAccounts<'_, '_>) -> ProgramResult {
    unpause_dex_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts)
}
pub fn unpause_dex_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseDexAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseDexKeys = accounts.into();
    let ix = unpause_dex_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_dex_invoke_signed(
    accounts: UnpauseDexAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_dex_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_dex_verify_account_keys(
    accounts: UnpauseDexAccounts<'_, '_>,
    keys: UnpauseDexKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_dex_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_dex_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_dex_verify_account_privileges<'me, 'info>(
    accounts: UnpauseDexAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_dex_verify_writable_privileges(accounts)?;
    unpause_dex_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseSwapAndArbitrageAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseSwapAndArbitrageKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UnpauseSwapAndArbitrageAccounts<'_, '_>> for UnpauseSwapAndArbitrageKeys {
    fn from(accounts: UnpauseSwapAndArbitrageAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UnpauseSwapAndArbitrageKeys>
for [AccountMeta; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseSwapAndArbitrageKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN]>
for UnpauseSwapAndArbitrageKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UnpauseSwapAndArbitrageAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseSwapAndArbitrageAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN]>
for UnpauseSwapAndArbitrageAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UNPAUSE_SWAP_AND_ARBITRAGE_IX_DISCM: [u8; 8usize] = [
    241, 4, 197, 110, 244, 255, 172, 184,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseSwapAndArbitrageIxData;
impl UnpauseSwapAndArbitrageIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_SWAP_AND_ARBITRAGE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_SWAP_AND_ARBITRAGE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_swap_and_arbitrage_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseSwapAndArbitrageKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_SWAP_AND_ARBITRAGE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnpauseSwapAndArbitrageIxData.try_to_vec()?,
    })
}
pub fn unpause_swap_and_arbitrage_ix(
    keys: UnpauseSwapAndArbitrageKeys,
) -> std::io::Result<Instruction> {
    unpause_swap_and_arbitrage_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys)
}
pub fn unpause_swap_and_arbitrage_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseSwapAndArbitrageAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnpauseSwapAndArbitrageKeys = accounts.into();
    let ix = unpause_swap_and_arbitrage_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_swap_and_arbitrage_invoke(
    accounts: UnpauseSwapAndArbitrageAccounts<'_, '_>,
) -> ProgramResult {
    unpause_swap_and_arbitrage_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
    )
}
pub fn unpause_swap_and_arbitrage_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseSwapAndArbitrageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseSwapAndArbitrageKeys = accounts.into();
    let ix = unpause_swap_and_arbitrage_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_swap_and_arbitrage_invoke_signed(
    accounts: UnpauseSwapAndArbitrageAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_swap_and_arbitrage_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn unpause_swap_and_arbitrage_verify_account_keys(
    accounts: UnpauseSwapAndArbitrageAccounts<'_, '_>,
    keys: UnpauseSwapAndArbitrageKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_swap_and_arbitrage_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseSwapAndArbitrageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_swap_and_arbitrage_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseSwapAndArbitrageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_swap_and_arbitrage_verify_account_privileges<'me, 'info>(
    accounts: UnpauseSwapAndArbitrageAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_swap_and_arbitrage_verify_writable_privileges(accounts)?;
    unpause_swap_and_arbitrage_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNPAUSE_USER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UnpauseUserAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnpauseUserKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub position: Pubkey,
}
impl From<UnpauseUserAccounts<'_, '_>> for UnpauseUserKeys {
    fn from(accounts: UnpauseUserAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            position: *accounts.position.key,
        }
    }
}
impl From<UnpauseUserKeys> for [AccountMeta; UNPAUSE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: UnpauseUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UNPAUSE_USER_IX_ACCOUNTS_LEN]> for UnpauseUserKeys {
    fn from(pubkeys: [Pubkey; UNPAUSE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            position: pubkeys[3],
        }
    }
}
impl<'info> From<UnpauseUserAccounts<'_, 'info>>
for [AccountInfo<'info>; UNPAUSE_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnpauseUserAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.position.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNPAUSE_USER_IX_ACCOUNTS_LEN]>
for UnpauseUserAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNPAUSE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            position: &arr[3],
        }
    }
}
pub const UNPAUSE_USER_IX_DISCM: [u8; 8usize] = [71, 115, 128, 252, 182, 126, 234, 62];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnpauseUserIxArgs {
    pub unpause_supply: bool,
    pub unpause_borrow: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnpauseUserIxData(pub UnpauseUserIxArgs);
impl From<UnpauseUserIxArgs> for UnpauseUserIxData {
    fn from(args: UnpauseUserIxArgs) -> Self {
        Self(args)
    }
}
impl UnpauseUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNPAUSE_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let unpause_supply: bool = crate::borsh_de_or_default(&mut reader)?;
        let unpause_borrow: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UnpauseUserIxArgs {
                unpause_supply,
                unpause_borrow,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNPAUSE_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.unpause_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.unpause_borrow, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unpause_user_ix_with_program_id(
    program_id: Pubkey,
    keys: UnpauseUserKeys,
    args: UnpauseUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNPAUSE_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnpauseUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unpause_user_ix(
    keys: UnpauseUserKeys,
    args: UnpauseUserIxArgs,
) -> std::io::Result<Instruction> {
    unpause_user_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn unpause_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseUserAccounts<'_, '_>,
    args: UnpauseUserIxArgs,
) -> ProgramResult {
    let keys: UnpauseUserKeys = accounts.into();
    let ix = unpause_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unpause_user_invoke(
    accounts: UnpauseUserAccounts<'_, '_>,
    args: UnpauseUserIxArgs,
) -> ProgramResult {
    unpause_user_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn unpause_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnpauseUserAccounts<'_, '_>,
    args: UnpauseUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnpauseUserKeys = accounts.into();
    let ix = unpause_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unpause_user_invoke_signed(
    accounts: UnpauseUserAccounts<'_, '_>,
    args: UnpauseUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unpause_user_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn unpause_user_verify_account_keys(
    accounts: UnpauseUserAccounts<'_, '_>,
    keys: UnpauseUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unpause_user_verify_writable_privileges<'me, 'info>(
    accounts: UnpauseUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unpause_user_verify_signer_privileges<'me, 'info>(
    accounts: UnpauseUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unpause_user_verify_account_privileges<'me, 'info>(
    accounts: UnpauseUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unpause_user_verify_writable_privileges(accounts)?;
    unpause_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAuthorityAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAuthorityKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
}
impl From<UpdateAuthorityAccounts<'_, '_>> for UpdateAuthorityKeys {
    fn from(accounts: UpdateAuthorityAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
        }
    }
}
impl From<UpdateAuthorityKeys> for [AccountMeta; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]> for UpdateAuthorityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
        }
    }
}
pub const UPDATE_AUTHORITY_IX_DISCM: [u8; 8usize] = [32, 46, 64, 28, 149, 75, 243, 88];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAuthorityIxArgs {
    pub new_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAuthorityIxData(pub UpdateAuthorityIxArgs);
impl From<UpdateAuthorityIxArgs> for UpdateAuthorityIxData {
    fn from(args: UpdateAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateAuthorityIxArgs {
                new_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAuthorityKeys,
    args: UpdateAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_authority_ix(
    keys: UpdateAuthorityKeys,
    args: UpdateAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    update_authority_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAuthorityAccounts<'_, '_>,
    args: UpdateAuthorityIxArgs,
) -> ProgramResult {
    let keys: UpdateAuthorityKeys = accounts.into();
    let ix = update_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_authority_invoke(
    accounts: UpdateAuthorityAccounts<'_, '_>,
    args: UpdateAuthorityIxArgs,
) -> ProgramResult {
    update_authority_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn update_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAuthorityAccounts<'_, '_>,
    args: UpdateAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAuthorityKeys = accounts.into();
    let ix = update_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_authority_invoke_signed(
    accounts: UpdateAuthorityAccounts<'_, '_>,
    args: UpdateAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_authority_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_authority_verify_account_keys(
    accounts: UpdateAuthorityAccounts<'_, '_>,
    keys: UpdateAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_authority_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_authority_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_authority_verify_account_privileges<'me, 'info>(
    accounts: UpdateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_authority_verify_writable_privileges(accounts)?;
    update_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_AUTHS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAuthsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAuthsKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
}
impl From<UpdateAuthsAccounts<'_, '_>> for UpdateAuthsKeys {
    fn from(accounts: UpdateAuthsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
        }
    }
}
impl From<UpdateAuthsKeys> for [AccountMeta; UPDATE_AUTHS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAuthsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_AUTHS_IX_ACCOUNTS_LEN]> for UpdateAuthsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AUTHS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAuthsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_AUTHS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAuthsAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AUTHS_IX_ACCOUNTS_LEN]>
for UpdateAuthsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AUTHS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
        }
    }
}
pub const UPDATE_AUTHS_IX_DISCM: [u8; 8usize] = [93, 96, 178, 156, 57, 117, 253, 209];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAuthsIxArgs {
    pub auth_status: Vec<AddressBool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAuthsIxData(pub UpdateAuthsIxArgs);
impl From<UpdateAuthsIxArgs> for UpdateAuthsIxData {
    fn from(args: UpdateAuthsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAuthsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_AUTHS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let auth_status: Vec<AddressBool> = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdateAuthsIxArgs { auth_status }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AUTHS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.auth_status, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_auths_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAuthsKeys,
    args: UpdateAuthsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_AUTHS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAuthsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_auths_ix(
    keys: UpdateAuthsKeys,
    args: UpdateAuthsIxArgs,
) -> std::io::Result<Instruction> {
    update_auths_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_auths_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAuthsAccounts<'_, '_>,
    args: UpdateAuthsIxArgs,
) -> ProgramResult {
    let keys: UpdateAuthsKeys = accounts.into();
    let ix = update_auths_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_auths_invoke(
    accounts: UpdateAuthsAccounts<'_, '_>,
    args: UpdateAuthsIxArgs,
) -> ProgramResult {
    update_auths_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn update_auths_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAuthsAccounts<'_, '_>,
    args: UpdateAuthsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAuthsKeys = accounts.into();
    let ix = update_auths_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_auths_invoke_signed(
    accounts: UpdateAuthsAccounts<'_, '_>,
    args: UpdateAuthsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_auths_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_auths_verify_account_keys(
    accounts: UpdateAuthsAccounts<'_, '_>,
    keys: UpdateAuthsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_auths_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAuthsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex_admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_auths_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAuthsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_auths_verify_account_privileges<'me, 'info>(
    accounts: UpdateAuthsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_auths_verify_writable_privileges(accounts)?;
    update_auths_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCenterPriceAddressAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCenterPriceAddressKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateCenterPriceAddressAccounts<'_, '_>> for UpdateCenterPriceAddressKeys {
    fn from(accounts: UpdateCenterPriceAddressAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateCenterPriceAddressKeys>
for [AccountMeta; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCenterPriceAddressKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN]>
for UpdateCenterPriceAddressKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateCenterPriceAddressAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCenterPriceAddressAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN]>
for UpdateCenterPriceAddressAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_CENTER_PRICE_ADDRESS_IX_DISCM: [u8; 8usize] = [
    45, 110, 96, 39, 201, 250, 142, 1,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCenterPriceAddressIxArgs {
    pub center_price_address: Pubkey,
    pub percent: u32,
    pub time: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCenterPriceAddressIxData(pub UpdateCenterPriceAddressIxArgs);
impl From<UpdateCenterPriceAddressIxArgs> for UpdateCenterPriceAddressIxData {
    fn from(args: UpdateCenterPriceAddressIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCenterPriceAddressIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CENTER_PRICE_ADDRESS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let center_price_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let time: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateCenterPriceAddressIxArgs {
                center_price_address,
                percent,
                time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CENTER_PRICE_ADDRESS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.center_price_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.time, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_center_price_address_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCenterPriceAddressKeys,
    args: UpdateCenterPriceAddressIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CENTER_PRICE_ADDRESS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCenterPriceAddressIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_center_price_address_ix(
    keys: UpdateCenterPriceAddressKeys,
    args: UpdateCenterPriceAddressIxArgs,
) -> std::io::Result<Instruction> {
    update_center_price_address_ix_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_center_price_address_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCenterPriceAddressAccounts<'_, '_>,
    args: UpdateCenterPriceAddressIxArgs,
) -> ProgramResult {
    let keys: UpdateCenterPriceAddressKeys = accounts.into();
    let ix = update_center_price_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_center_price_address_invoke(
    accounts: UpdateCenterPriceAddressAccounts<'_, '_>,
    args: UpdateCenterPriceAddressIxArgs,
) -> ProgramResult {
    update_center_price_address_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_center_price_address_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCenterPriceAddressAccounts<'_, '_>,
    args: UpdateCenterPriceAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCenterPriceAddressKeys = accounts.into();
    let ix = update_center_price_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_center_price_address_invoke_signed(
    accounts: UpdateCenterPriceAddressAccounts<'_, '_>,
    args: UpdateCenterPriceAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_center_price_address_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_center_price_address_verify_account_keys(
    accounts: UpdateCenterPriceAddressAccounts<'_, '_>,
    keys: UpdateCenterPriceAddressKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_center_price_address_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCenterPriceAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_center_price_address_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCenterPriceAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_center_price_address_verify_account_privileges<'me, 'info>(
    accounts: UpdateCenterPriceAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_center_price_address_verify_writable_privileges(accounts)?;
    update_center_price_address_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCenterPriceLimitsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCenterPriceLimitsKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateCenterPriceLimitsAccounts<'_, '_>> for UpdateCenterPriceLimitsKeys {
    fn from(accounts: UpdateCenterPriceLimitsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateCenterPriceLimitsKeys>
for [AccountMeta; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCenterPriceLimitsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN]>
for UpdateCenterPriceLimitsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateCenterPriceLimitsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCenterPriceLimitsAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN]>
for UpdateCenterPriceLimitsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_CENTER_PRICE_LIMITS_IX_DISCM: [u8; 8usize] = [
    17, 23, 56, 200, 237, 163, 24, 152,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCenterPriceLimitsIxArgs {
    pub max_center_price: u64,
    pub min_center_price: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCenterPriceLimitsIxData(pub UpdateCenterPriceLimitsIxArgs);
impl From<UpdateCenterPriceLimitsIxArgs> for UpdateCenterPriceLimitsIxData {
    fn from(args: UpdateCenterPriceLimitsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCenterPriceLimitsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CENTER_PRICE_LIMITS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateCenterPriceLimitsIxArgs {
                max_center_price,
                min_center_price,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CENTER_PRICE_LIMITS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_center_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_center_price, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_center_price_limits_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCenterPriceLimitsKeys,
    args: UpdateCenterPriceLimitsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CENTER_PRICE_LIMITS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCenterPriceLimitsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_center_price_limits_ix(
    keys: UpdateCenterPriceLimitsKeys,
    args: UpdateCenterPriceLimitsIxArgs,
) -> std::io::Result<Instruction> {
    update_center_price_limits_ix_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_center_price_limits_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCenterPriceLimitsAccounts<'_, '_>,
    args: UpdateCenterPriceLimitsIxArgs,
) -> ProgramResult {
    let keys: UpdateCenterPriceLimitsKeys = accounts.into();
    let ix = update_center_price_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_center_price_limits_invoke(
    accounts: UpdateCenterPriceLimitsAccounts<'_, '_>,
    args: UpdateCenterPriceLimitsIxArgs,
) -> ProgramResult {
    update_center_price_limits_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_center_price_limits_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCenterPriceLimitsAccounts<'_, '_>,
    args: UpdateCenterPriceLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCenterPriceLimitsKeys = accounts.into();
    let ix = update_center_price_limits_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_center_price_limits_invoke_signed(
    accounts: UpdateCenterPriceLimitsAccounts<'_, '_>,
    args: UpdateCenterPriceLimitsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_center_price_limits_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_center_price_limits_verify_account_keys(
    accounts: UpdateCenterPriceLimitsAccounts<'_, '_>,
    keys: UpdateCenterPriceLimitsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_center_price_limits_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCenterPriceLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_center_price_limits_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCenterPriceLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_center_price_limits_verify_account_privileges<'me, 'info>(
    accounts: UpdateCenterPriceLimitsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_center_price_limits_verify_writable_privileges(accounts)?;
    update_center_price_limits_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeAndRevenueCutAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeAndRevenueCutKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateFeeAndRevenueCutAccounts<'_, '_>> for UpdateFeeAndRevenueCutKeys {
    fn from(accounts: UpdateFeeAndRevenueCutAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateFeeAndRevenueCutKeys>
for [AccountMeta; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeAndRevenueCutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN]>
for UpdateFeeAndRevenueCutKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateFeeAndRevenueCutAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeAndRevenueCutAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN]>
for UpdateFeeAndRevenueCutAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_FEE_AND_REVENUE_CUT_IX_DISCM: [u8; 8usize] = [
    223, 251, 181, 7, 34, 61, 183, 122,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeAndRevenueCutIxArgs {
    pub fee: u32,
    pub revenue_cut: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeAndRevenueCutIxData(pub UpdateFeeAndRevenueCutIxArgs);
impl From<UpdateFeeAndRevenueCutIxArgs> for UpdateFeeAndRevenueCutIxData {
    fn from(args: UpdateFeeAndRevenueCutIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeeAndRevenueCutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_AND_REVENUE_CUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let revenue_cut: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateFeeAndRevenueCutIxArgs {
                fee,
                revenue_cut,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_AND_REVENUE_CUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.revenue_cut, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_and_revenue_cut_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeAndRevenueCutKeys,
    args: UpdateFeeAndRevenueCutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_AND_REVENUE_CUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeeAndRevenueCutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fee_and_revenue_cut_ix(
    keys: UpdateFeeAndRevenueCutKeys,
    args: UpdateFeeAndRevenueCutIxArgs,
) -> std::io::Result<Instruction> {
    update_fee_and_revenue_cut_ix_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_fee_and_revenue_cut_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeAndRevenueCutAccounts<'_, '_>,
    args: UpdateFeeAndRevenueCutIxArgs,
) -> ProgramResult {
    let keys: UpdateFeeAndRevenueCutKeys = accounts.into();
    let ix = update_fee_and_revenue_cut_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_and_revenue_cut_invoke(
    accounts: UpdateFeeAndRevenueCutAccounts<'_, '_>,
    args: UpdateFeeAndRevenueCutIxArgs,
) -> ProgramResult {
    update_fee_and_revenue_cut_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_fee_and_revenue_cut_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeAndRevenueCutAccounts<'_, '_>,
    args: UpdateFeeAndRevenueCutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeAndRevenueCutKeys = accounts.into();
    let ix = update_fee_and_revenue_cut_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_and_revenue_cut_invoke_signed(
    accounts: UpdateFeeAndRevenueCutAccounts<'_, '_>,
    args: UpdateFeeAndRevenueCutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_and_revenue_cut_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_fee_and_revenue_cut_verify_account_keys(
    accounts: UpdateFeeAndRevenueCutAccounts<'_, '_>,
    keys: UpdateFeeAndRevenueCutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_and_revenue_cut_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeeAndRevenueCutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fee_and_revenue_cut_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeeAndRevenueCutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_and_revenue_cut_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeAndRevenueCutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_and_revenue_cut_verify_writable_privileges(accounts)?;
    update_fee_and_revenue_cut_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMaxBorrowSharesAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateMaxBorrowSharesKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateMaxBorrowSharesAccounts<'_, '_>> for UpdateMaxBorrowSharesKeys {
    fn from(accounts: UpdateMaxBorrowSharesAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateMaxBorrowSharesKeys>
for [AccountMeta; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateMaxBorrowSharesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN]>
for UpdateMaxBorrowSharesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateMaxBorrowSharesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateMaxBorrowSharesAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN]>
for UpdateMaxBorrowSharesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_MAX_BORROW_SHARES_IX_DISCM: [u8; 8usize] = [
    176, 13, 121, 189, 225, 225, 238, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateMaxBorrowSharesIxArgs {
    pub max_borrow_shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMaxBorrowSharesIxData(pub UpdateMaxBorrowSharesIxArgs);
impl From<UpdateMaxBorrowSharesIxArgs> for UpdateMaxBorrowSharesIxData {
    fn from(args: UpdateMaxBorrowSharesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMaxBorrowSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MAX_BORROW_SHARES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_borrow_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateMaxBorrowSharesIxArgs {
                max_borrow_shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MAX_BORROW_SHARES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_borrow_shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_max_borrow_shares_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateMaxBorrowSharesKeys,
    args: UpdateMaxBorrowSharesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MAX_BORROW_SHARES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateMaxBorrowSharesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_max_borrow_shares_ix(
    keys: UpdateMaxBorrowSharesKeys,
    args: UpdateMaxBorrowSharesIxArgs,
) -> std::io::Result<Instruction> {
    update_max_borrow_shares_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_max_borrow_shares_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMaxBorrowSharesAccounts<'_, '_>,
    args: UpdateMaxBorrowSharesIxArgs,
) -> ProgramResult {
    let keys: UpdateMaxBorrowSharesKeys = accounts.into();
    let ix = update_max_borrow_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_max_borrow_shares_invoke(
    accounts: UpdateMaxBorrowSharesAccounts<'_, '_>,
    args: UpdateMaxBorrowSharesIxArgs,
) -> ProgramResult {
    update_max_borrow_shares_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_max_borrow_shares_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMaxBorrowSharesAccounts<'_, '_>,
    args: UpdateMaxBorrowSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateMaxBorrowSharesKeys = accounts.into();
    let ix = update_max_borrow_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_max_borrow_shares_invoke_signed(
    accounts: UpdateMaxBorrowSharesAccounts<'_, '_>,
    args: UpdateMaxBorrowSharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_max_borrow_shares_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_max_borrow_shares_verify_account_keys(
    accounts: UpdateMaxBorrowSharesAccounts<'_, '_>,
    keys: UpdateMaxBorrowSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_max_borrow_shares_verify_writable_privileges<'me, 'info>(
    accounts: UpdateMaxBorrowSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_max_borrow_shares_verify_signer_privileges<'me, 'info>(
    accounts: UpdateMaxBorrowSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_max_borrow_shares_verify_account_privileges<'me, 'info>(
    accounts: UpdateMaxBorrowSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_max_borrow_shares_verify_writable_privileges(accounts)?;
    update_max_borrow_shares_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMaxSupplySharesAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateMaxSupplySharesKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateMaxSupplySharesAccounts<'_, '_>> for UpdateMaxSupplySharesKeys {
    fn from(accounts: UpdateMaxSupplySharesAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateMaxSupplySharesKeys>
for [AccountMeta; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateMaxSupplySharesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN]>
for UpdateMaxSupplySharesKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateMaxSupplySharesAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateMaxSupplySharesAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN]>
for UpdateMaxSupplySharesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_MAX_SUPPLY_SHARES_IX_DISCM: [u8; 8usize] = [
    179, 157, 37, 206, 176, 51, 37, 79,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateMaxSupplySharesIxArgs {
    pub max_supply_shares: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMaxSupplySharesIxData(pub UpdateMaxSupplySharesIxArgs);
impl From<UpdateMaxSupplySharesIxArgs> for UpdateMaxSupplySharesIxData {
    fn from(args: UpdateMaxSupplySharesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMaxSupplySharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MAX_SUPPLY_SHARES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let max_supply_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateMaxSupplySharesIxArgs {
                max_supply_shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MAX_SUPPLY_SHARES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.max_supply_shares, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_max_supply_shares_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateMaxSupplySharesKeys,
    args: UpdateMaxSupplySharesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MAX_SUPPLY_SHARES_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateMaxSupplySharesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_max_supply_shares_ix(
    keys: UpdateMaxSupplySharesKeys,
    args: UpdateMaxSupplySharesIxArgs,
) -> std::io::Result<Instruction> {
    update_max_supply_shares_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_max_supply_shares_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMaxSupplySharesAccounts<'_, '_>,
    args: UpdateMaxSupplySharesIxArgs,
) -> ProgramResult {
    let keys: UpdateMaxSupplySharesKeys = accounts.into();
    let ix = update_max_supply_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_max_supply_shares_invoke(
    accounts: UpdateMaxSupplySharesAccounts<'_, '_>,
    args: UpdateMaxSupplySharesIxArgs,
) -> ProgramResult {
    update_max_supply_shares_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_max_supply_shares_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMaxSupplySharesAccounts<'_, '_>,
    args: UpdateMaxSupplySharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateMaxSupplySharesKeys = accounts.into();
    let ix = update_max_supply_shares_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_max_supply_shares_invoke_signed(
    accounts: UpdateMaxSupplySharesAccounts<'_, '_>,
    args: UpdateMaxSupplySharesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_max_supply_shares_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_max_supply_shares_verify_account_keys(
    accounts: UpdateMaxSupplySharesAccounts<'_, '_>,
    keys: UpdateMaxSupplySharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_max_supply_shares_verify_writable_privileges<'me, 'info>(
    accounts: UpdateMaxSupplySharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_max_supply_shares_verify_signer_privileges<'me, 'info>(
    accounts: UpdateMaxSupplySharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_max_supply_shares_verify_account_privileges<'me, 'info>(
    accounts: UpdateMaxSupplySharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_max_supply_shares_verify_writable_privileges(accounts)?;
    update_max_supply_shares_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRangePercentsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRangePercentsKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateRangePercentsAccounts<'_, '_>> for UpdateRangePercentsKeys {
    fn from(accounts: UpdateRangePercentsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateRangePercentsKeys>
for [AccountMeta; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRangePercentsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN]> for UpdateRangePercentsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateRangePercentsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRangePercentsAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN]>
for UpdateRangePercentsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_RANGE_PERCENTS_IX_DISCM: [u8; 8usize] = [
    51, 233, 228, 43, 91, 7, 62, 20,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRangePercentsIxArgs {
    pub upper_percent: u32,
    pub lower_percent: u32,
    pub shift_time: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRangePercentsIxData(pub UpdateRangePercentsIxArgs);
impl From<UpdateRangePercentsIxArgs> for UpdateRangePercentsIxData {
    fn from(args: UpdateRangePercentsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRangePercentsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_RANGE_PERCENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let upper_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateRangePercentsIxArgs {
                upper_percent,
                lower_percent,
                shift_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_RANGE_PERCENTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.upper_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lower_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.shift_time, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_range_percents_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRangePercentsKeys,
    args: UpdateRangePercentsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_RANGE_PERCENTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRangePercentsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_range_percents_ix(
    keys: UpdateRangePercentsKeys,
    args: UpdateRangePercentsIxArgs,
) -> std::io::Result<Instruction> {
    update_range_percents_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_range_percents_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRangePercentsAccounts<'_, '_>,
    args: UpdateRangePercentsIxArgs,
) -> ProgramResult {
    let keys: UpdateRangePercentsKeys = accounts.into();
    let ix = update_range_percents_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_range_percents_invoke(
    accounts: UpdateRangePercentsAccounts<'_, '_>,
    args: UpdateRangePercentsIxArgs,
) -> ProgramResult {
    update_range_percents_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_range_percents_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRangePercentsAccounts<'_, '_>,
    args: UpdateRangePercentsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRangePercentsKeys = accounts.into();
    let ix = update_range_percents_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_range_percents_invoke_signed(
    accounts: UpdateRangePercentsAccounts<'_, '_>,
    args: UpdateRangePercentsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_range_percents_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_range_percents_verify_account_keys(
    accounts: UpdateRangePercentsAccounts<'_, '_>,
    keys: UpdateRangePercentsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_range_percents_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRangePercentsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_range_percents_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRangePercentsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_range_percents_verify_account_privileges<'me, 'info>(
    accounts: UpdateRangePercentsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_range_percents_verify_writable_privileges(accounts)?;
    update_range_percents_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateThresholdPercentAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateThresholdPercentKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateThresholdPercentAccounts<'_, '_>> for UpdateThresholdPercentKeys {
    fn from(accounts: UpdateThresholdPercentAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateThresholdPercentKeys>
for [AccountMeta; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateThresholdPercentKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN]>
for UpdateThresholdPercentKeys {
    fn from(pubkeys: [Pubkey; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateThresholdPercentAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateThresholdPercentAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN]>
for UpdateThresholdPercentAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_THRESHOLD_PERCENT_IX_DISCM: [u8; 8usize] = [
    177, 125, 99, 134, 42, 254, 140, 234,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateThresholdPercentIxArgs {
    pub upper_threshold_percent: u32,
    pub lower_threshold_percent: u32,
    pub threshold_shift_time: u32,
    pub shift_time: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateThresholdPercentIxData(pub UpdateThresholdPercentIxArgs);
impl From<UpdateThresholdPercentIxArgs> for UpdateThresholdPercentIxData {
    fn from(args: UpdateThresholdPercentIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateThresholdPercentIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_THRESHOLD_PERCENT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let upper_threshold_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_threshold_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        let shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateThresholdPercentIxArgs {
                upper_threshold_percent,
                lower_threshold_percent,
                threshold_shift_time,
                shift_time,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_THRESHOLD_PERCENT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.upper_threshold_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.lower_threshold_percent, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.threshold_shift_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.shift_time, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_threshold_percent_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateThresholdPercentKeys,
    args: UpdateThresholdPercentIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_THRESHOLD_PERCENT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateThresholdPercentIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_threshold_percent_ix(
    keys: UpdateThresholdPercentKeys,
    args: UpdateThresholdPercentIxArgs,
) -> std::io::Result<Instruction> {
    update_threshold_percent_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_threshold_percent_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateThresholdPercentAccounts<'_, '_>,
    args: UpdateThresholdPercentIxArgs,
) -> ProgramResult {
    let keys: UpdateThresholdPercentKeys = accounts.into();
    let ix = update_threshold_percent_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_threshold_percent_invoke(
    accounts: UpdateThresholdPercentAccounts<'_, '_>,
    args: UpdateThresholdPercentIxArgs,
) -> ProgramResult {
    update_threshold_percent_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_threshold_percent_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateThresholdPercentAccounts<'_, '_>,
    args: UpdateThresholdPercentIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateThresholdPercentKeys = accounts.into();
    let ix = update_threshold_percent_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_threshold_percent_invoke_signed(
    accounts: UpdateThresholdPercentAccounts<'_, '_>,
    args: UpdateThresholdPercentIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_threshold_percent_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_threshold_percent_verify_account_keys(
    accounts: UpdateThresholdPercentAccounts<'_, '_>,
    keys: UpdateThresholdPercentKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_threshold_percent_verify_writable_privileges<'me, 'info>(
    accounts: UpdateThresholdPercentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_threshold_percent_verify_signer_privileges<'me, 'info>(
    accounts: UpdateThresholdPercentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_threshold_percent_verify_account_privileges<'me, 'info>(
    accounts: UpdateThresholdPercentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_threshold_percent_verify_writable_privileges(accounts)?;
    update_threshold_percent_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserBorrowConfigAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserBorrowConfigKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub position: Pubkey,
}
impl From<UpdateUserBorrowConfigAccounts<'_, '_>> for UpdateUserBorrowConfigKeys {
    fn from(accounts: UpdateUserBorrowConfigAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            position: *accounts.position.key,
        }
    }
}
impl From<UpdateUserBorrowConfigKeys>
for [AccountMeta; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUserBorrowConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateUserBorrowConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            position: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateUserBorrowConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUserBorrowConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.position.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateUserBorrowConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            position: &arr[3],
        }
    }
}
pub const UPDATE_USER_BORROW_CONFIG_IX_DISCM: [u8; 8usize] = [
    100, 176, 201, 174, 247, 2, 54, 168,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUserBorrowConfigIxArgs {
    pub config: UserBorrowConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserBorrowConfigIxData(pub UpdateUserBorrowConfigIxArgs);
impl From<UpdateUserBorrowConfigIxArgs> for UpdateUserBorrowConfigIxData {
    fn from(args: UpdateUserBorrowConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserBorrowConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_BORROW_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <UserBorrowConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateUserBorrowConfigIxArgs {
                config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_BORROW_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_borrow_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUserBorrowConfigKeys,
    args: UpdateUserBorrowConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USER_BORROW_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUserBorrowConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_borrow_config_ix(
    keys: UpdateUserBorrowConfigKeys,
    args: UpdateUserBorrowConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_user_borrow_config_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_user_borrow_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUserBorrowConfigAccounts<'_, '_>,
    args: UpdateUserBorrowConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateUserBorrowConfigKeys = accounts.into();
    let ix = update_user_borrow_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_user_borrow_config_invoke(
    accounts: UpdateUserBorrowConfigAccounts<'_, '_>,
    args: UpdateUserBorrowConfigIxArgs,
) -> ProgramResult {
    update_user_borrow_config_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_user_borrow_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUserBorrowConfigAccounts<'_, '_>,
    args: UpdateUserBorrowConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUserBorrowConfigKeys = accounts.into();
    let ix = update_user_borrow_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_user_borrow_config_invoke_signed(
    accounts: UpdateUserBorrowConfigAccounts<'_, '_>,
    args: UpdateUserBorrowConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_user_borrow_config_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_user_borrow_config_verify_account_keys(
    accounts: UpdateUserBorrowConfigAccounts<'_, '_>,
    keys: UpdateUserBorrowConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_user_borrow_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUserBorrowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_user_borrow_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUserBorrowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_user_borrow_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateUserBorrowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_user_borrow_config_verify_writable_privileges(accounts)?;
    update_user_borrow_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserSupplyConfigAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserSupplyConfigKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub position: Pubkey,
}
impl From<UpdateUserSupplyConfigAccounts<'_, '_>> for UpdateUserSupplyConfigKeys {
    fn from(accounts: UpdateUserSupplyConfigAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            position: *accounts.position.key,
        }
    }
}
impl From<UpdateUserSupplyConfigKeys>
for [AccountMeta; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUserSupplyConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateUserSupplyConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            position: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateUserSupplyConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUserSupplyConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.position.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateUserSupplyConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            position: &arr[3],
        }
    }
}
pub const UPDATE_USER_SUPPLY_CONFIG_IX_DISCM: [u8; 8usize] = [
    217, 239, 225, 218, 33, 49, 234, 183,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUserSupplyConfigIxArgs {
    pub config: UserSupplyConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserSupplyConfigIxData(pub UpdateUserSupplyConfigIxArgs);
impl From<UpdateUserSupplyConfigIxArgs> for UpdateUserSupplyConfigIxData {
    fn from(args: UpdateUserSupplyConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserSupplyConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_SUPPLY_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <UserSupplyConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateUserSupplyConfigIxArgs {
                config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_SUPPLY_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_supply_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUserSupplyConfigKeys,
    args: UpdateUserSupplyConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USER_SUPPLY_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUserSupplyConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_supply_config_ix(
    keys: UpdateUserSupplyConfigKeys,
    args: UpdateUserSupplyConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_user_supply_config_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_user_supply_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUserSupplyConfigAccounts<'_, '_>,
    args: UpdateUserSupplyConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateUserSupplyConfigKeys = accounts.into();
    let ix = update_user_supply_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_user_supply_config_invoke(
    accounts: UpdateUserSupplyConfigAccounts<'_, '_>,
    args: UpdateUserSupplyConfigIxArgs,
) -> ProgramResult {
    update_user_supply_config_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_user_supply_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUserSupplyConfigAccounts<'_, '_>,
    args: UpdateUserSupplyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUserSupplyConfigKeys = accounts.into();
    let ix = update_user_supply_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_user_supply_config_invoke_signed(
    accounts: UpdateUserSupplyConfigAccounts<'_, '_>,
    args: UpdateUserSupplyConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_user_supply_config_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_user_supply_config_verify_account_keys(
    accounts: UpdateUserSupplyConfigAccounts<'_, '_>,
    keys: UpdateUserSupplyConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_user_supply_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUserSupplyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_user_supply_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUserSupplyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_user_supply_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateUserSupplyConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_user_supply_config_verify_writable_privileges(accounts)?;
    update_user_supply_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserWithdrawalLimitAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserWithdrawalLimitKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
    pub position: Pubkey,
}
impl From<UpdateUserWithdrawalLimitAccounts<'_, '_>> for UpdateUserWithdrawalLimitKeys {
    fn from(accounts: UpdateUserWithdrawalLimitAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
            position: *accounts.position.key,
        }
    }
}
impl From<UpdateUserWithdrawalLimitKeys>
for [AccountMeta; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUserWithdrawalLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateUserWithdrawalLimitKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
            position: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateUserWithdrawalLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUserWithdrawalLimitAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dex_admin.clone(),
            accounts.dex.clone(),
            accounts.position.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateUserWithdrawalLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
            position: &arr[3],
        }
    }
}
pub const UPDATE_USER_WITHDRAWAL_LIMIT_IX_DISCM: [u8; 8usize] = [
    162, 9, 186, 9, 213, 30, 173, 78,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUserWithdrawalLimitIxArgs {
    pub new_limit: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserWithdrawalLimitIxData(pub UpdateUserWithdrawalLimitIxArgs);
impl From<UpdateUserWithdrawalLimitIxArgs> for UpdateUserWithdrawalLimitIxData {
    fn from(args: UpdateUserWithdrawalLimitIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserWithdrawalLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_WITHDRAWAL_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateUserWithdrawalLimitIxArgs {
                new_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_WITHDRAWAL_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_limit, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_withdrawal_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUserWithdrawalLimitKeys,
    args: UpdateUserWithdrawalLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_USER_WITHDRAWAL_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUserWithdrawalLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_withdrawal_limit_ix(
    keys: UpdateUserWithdrawalLimitKeys,
    args: UpdateUserWithdrawalLimitIxArgs,
) -> std::io::Result<Instruction> {
    update_user_withdrawal_limit_ix_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_user_withdrawal_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUserWithdrawalLimitAccounts<'_, '_>,
    args: UpdateUserWithdrawalLimitIxArgs,
) -> ProgramResult {
    let keys: UpdateUserWithdrawalLimitKeys = accounts.into();
    let ix = update_user_withdrawal_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_user_withdrawal_limit_invoke(
    accounts: UpdateUserWithdrawalLimitAccounts<'_, '_>,
    args: UpdateUserWithdrawalLimitIxArgs,
) -> ProgramResult {
    update_user_withdrawal_limit_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_user_withdrawal_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUserWithdrawalLimitAccounts<'_, '_>,
    args: UpdateUserWithdrawalLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUserWithdrawalLimitKeys = accounts.into();
    let ix = update_user_withdrawal_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_user_withdrawal_limit_invoke_signed(
    accounts: UpdateUserWithdrawalLimitAccounts<'_, '_>,
    args: UpdateUserWithdrawalLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_user_withdrawal_limit_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_user_withdrawal_limit_verify_account_keys(
    accounts: UpdateUserWithdrawalLimitAccounts<'_, '_>,
    keys: UpdateUserWithdrawalLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
        (*accounts.position.key, keys.position),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_user_withdrawal_limit_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUserWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_user_withdrawal_limit_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUserWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_user_withdrawal_limit_verify_account_privileges<'me, 'info>(
    accounts: UpdateUserWithdrawalLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_user_withdrawal_limit_verify_writable_privileges(accounts)?;
    update_user_withdrawal_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUtilizationLimitAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dex_admin: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUtilizationLimitKeys {
    pub authority: Pubkey,
    pub dex_admin: Pubkey,
    pub dex: Pubkey,
}
impl From<UpdateUtilizationLimitAccounts<'_, '_>> for UpdateUtilizationLimitKeys {
    fn from(accounts: UpdateUtilizationLimitAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dex_admin: *accounts.dex_admin.key,
            dex: *accounts.dex.key,
        }
    }
}
impl From<UpdateUtilizationLimitKeys>
for [AccountMeta; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUtilizationLimitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_admin,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateUtilizationLimitKeys {
    fn from(pubkeys: [Pubkey; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dex_admin: pubkeys[1],
            dex: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateUtilizationLimitAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUtilizationLimitAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.dex_admin.clone(), accounts.dex.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN]>
for UpdateUtilizationLimitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            dex_admin: &arr[1],
            dex: &arr[2],
        }
    }
}
pub const UPDATE_UTILIZATION_LIMIT_IX_DISCM: [u8; 8usize] = [
    48, 145, 0, 235, 118, 59, 55, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUtilizationLimitIxArgs {
    pub token_0_utilization_limit: u16,
    pub token_1_utilization_limit: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUtilizationLimitIxData(pub UpdateUtilizationLimitIxArgs);
impl From<UpdateUtilizationLimitIxArgs> for UpdateUtilizationLimitIxData {
    fn from(args: UpdateUtilizationLimitIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUtilizationLimitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_UTILIZATION_LIMIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let token_0_utilization_limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_utilization_limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateUtilizationLimitIxArgs {
                token_0_utilization_limit,
                token_1_utilization_limit,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_UTILIZATION_LIMIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.token_0_utilization_limit,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.token_1_utilization_limit,
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
pub fn update_utilization_limit_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUtilizationLimitKeys,
    args: UpdateUtilizationLimitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_UTILIZATION_LIMIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUtilizationLimitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_utilization_limit_ix(
    keys: UpdateUtilizationLimitKeys,
    args: UpdateUtilizationLimitIxArgs,
) -> std::io::Result<Instruction> {
    update_utilization_limit_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn update_utilization_limit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUtilizationLimitAccounts<'_, '_>,
    args: UpdateUtilizationLimitIxArgs,
) -> ProgramResult {
    let keys: UpdateUtilizationLimitKeys = accounts.into();
    let ix = update_utilization_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_utilization_limit_invoke(
    accounts: UpdateUtilizationLimitAccounts<'_, '_>,
    args: UpdateUtilizationLimitIxArgs,
) -> ProgramResult {
    update_utilization_limit_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_utilization_limit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUtilizationLimitAccounts<'_, '_>,
    args: UpdateUtilizationLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUtilizationLimitKeys = accounts.into();
    let ix = update_utilization_limit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_utilization_limit_invoke_signed(
    accounts: UpdateUtilizationLimitAccounts<'_, '_>,
    args: UpdateUtilizationLimitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_utilization_limit_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_utilization_limit_verify_account_keys(
    accounts: UpdateUtilizationLimitAccounts<'_, '_>,
    keys: UpdateUtilizationLimitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dex_admin.key, keys.dex_admin),
        (*accounts.dex.key, keys.dex),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_utilization_limit_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUtilizationLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.dex] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_utilization_limit_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUtilizationLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_utilization_limit_verify_account_privileges<'me, 'info>(
    accounts: UpdateUtilizationLimitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_utilization_limit_verify_writable_privileges(accounts)?;
    update_utilization_limit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8usize] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIxArgs {
    pub token0_amt: u64,
    pub token1_amt: u64,
    pub max_shares: u64,
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
        let token0_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token1_amt: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawIxArgs {
                token0_amt,
                token1_amt,
                max_shares,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.token0_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.token1_amt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_shares, &mut writer)?;
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
    withdraw_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
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
    withdraw_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
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
    withdraw_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
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
    for should_be_signer in [accounts.signer, accounts.user] {
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
pub const WITHDRAW_PERFECT_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawPerfectAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawPerfectKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<WithdrawPerfectAccounts<'_, '_>> for WithdrawPerfectKeys {
    fn from(accounts: WithdrawPerfectAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<WithdrawPerfectKeys> for [AccountMeta; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawPerfectKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN]> for WithdrawPerfectKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<WithdrawPerfectAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawPerfectAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN]>
for WithdrawPerfectAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const WITHDRAW_PERFECT_IX_DISCM: [u8; 8usize] = [
    41, 170, 122, 166, 20, 154, 180, 245,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawPerfectIxArgs {
    pub shares: u64,
    pub min_token0: u64,
    pub min_token1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawPerfectIxData(pub WithdrawPerfectIxArgs);
impl From<WithdrawPerfectIxArgs> for WithdrawPerfectIxData {
    fn from(args: WithdrawPerfectIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawPerfectIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PERFECT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawPerfectIxArgs {
                shares,
                min_token0,
                min_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PERFECT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_perfect_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawPerfectKeys,
    args: WithdrawPerfectIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PERFECT_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawPerfectIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_perfect_ix(
    keys: WithdrawPerfectKeys,
    args: WithdrawPerfectIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_perfect_ix_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, keys, args)
}
pub fn withdraw_perfect_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPerfectAccounts<'_, '_>,
    args: WithdrawPerfectIxArgs,
) -> ProgramResult {
    let keys: WithdrawPerfectKeys = accounts.into();
    let ix = withdraw_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_perfect_invoke(
    accounts: WithdrawPerfectAccounts<'_, '_>,
    args: WithdrawPerfectIxArgs,
) -> ProgramResult {
    withdraw_perfect_invoke_with_program_id(JUPITER_LEND_AMM_PROGRAM_ID, accounts, args)
}
pub fn withdraw_perfect_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPerfectAccounts<'_, '_>,
    args: WithdrawPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawPerfectKeys = accounts.into();
    let ix = withdraw_perfect_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_perfect_invoke_signed(
    accounts: WithdrawPerfectAccounts<'_, '_>,
    args: WithdrawPerfectIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_perfect_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_perfect_verify_account_keys(
    accounts: WithdrawPerfectAccounts<'_, '_>,
    keys: WithdrawPerfectKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_perfect_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_perfect_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_perfect_verify_account_privileges<'me, 'info>(
    accounts: WithdrawPerfectAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_perfect_verify_writable_privileges(accounts)?;
    withdraw_perfect_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawPerfectInOneTokenAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dex: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub user_token_0_account: &'me AccountInfo<'info>,
    pub user_token_1_account: &'me AccountInfo<'info>,
    pub token_0: &'me AccountInfo<'info>,
    pub token_1: &'me AccountInfo<'info>,
    pub token_0_reserve: &'me AccountInfo<'info>,
    pub token_1_reserve: &'me AccountInfo<'info>,
    pub token_0_rate_model: &'me AccountInfo<'info>,
    pub token_1_rate_model: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub dex_supply_position_token_0: &'me AccountInfo<'info>,
    pub dex_supply_position_token_1: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_0: &'me AccountInfo<'info>,
    pub dex_borrow_position_token_1: &'me AccountInfo<'info>,
    pub liquidity: &'me AccountInfo<'info>,
    pub liquidity_program: &'me AccountInfo<'info>,
    pub oracle_program: &'me AccountInfo<'info>,
    pub token_0_program: &'me AccountInfo<'info>,
    pub token_1_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawPerfectInOneTokenKeys {
    pub signer: Pubkey,
    pub dex: Pubkey,
    pub user: Pubkey,
    pub position: Pubkey,
    pub user_token_0_account: Pubkey,
    pub user_token_1_account: Pubkey,
    pub token_0: Pubkey,
    pub token_1: Pubkey,
    pub token_0_reserve: Pubkey,
    pub token_1_reserve: Pubkey,
    pub token_0_rate_model: Pubkey,
    pub token_1_rate_model: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub dex_supply_position_token_0: Pubkey,
    pub dex_supply_position_token_1: Pubkey,
    pub dex_borrow_position_token_0: Pubkey,
    pub dex_borrow_position_token_1: Pubkey,
    pub liquidity: Pubkey,
    pub liquidity_program: Pubkey,
    pub oracle_program: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<WithdrawPerfectInOneTokenAccounts<'_, '_>> for WithdrawPerfectInOneTokenKeys {
    fn from(accounts: WithdrawPerfectInOneTokenAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dex: *accounts.dex.key,
            user: *accounts.user.key,
            position: *accounts.position.key,
            user_token_0_account: *accounts.user_token_0_account.key,
            user_token_1_account: *accounts.user_token_1_account.key,
            token_0: *accounts.token_0.key,
            token_1: *accounts.token_1.key,
            token_0_reserve: *accounts.token_0_reserve.key,
            token_1_reserve: *accounts.token_1_reserve.key,
            token_0_rate_model: *accounts.token_0_rate_model.key,
            token_1_rate_model: *accounts.token_1_rate_model.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            dex_supply_position_token_0: *accounts.dex_supply_position_token_0.key,
            dex_supply_position_token_1: *accounts.dex_supply_position_token_1.key,
            dex_borrow_position_token_0: *accounts.dex_borrow_position_token_0.key,
            dex_borrow_position_token_1: *accounts.dex_borrow_position_token_1.key,
            liquidity: *accounts.liquidity.key,
            liquidity_program: *accounts.liquidity_program.key,
            oracle_program: *accounts.oracle_program.key,
            token_0_program: *accounts.token_0_program.key,
            token_1_program: *accounts.token_1_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<WithdrawPerfectInOneTokenKeys>
for [AccountMeta; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawPerfectInOneTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_0_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_1_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_0_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_rate_model,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_supply_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_borrow_position_token_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN]>
for WithdrawPerfectInOneTokenKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dex: pubkeys[1],
            user: pubkeys[2],
            position: pubkeys[3],
            user_token_0_account: pubkeys[4],
            user_token_1_account: pubkeys[5],
            token_0: pubkeys[6],
            token_1: pubkeys[7],
            token_0_reserve: pubkeys[8],
            token_1_reserve: pubkeys[9],
            token_0_rate_model: pubkeys[10],
            token_1_rate_model: pubkeys[11],
            token_0_vault: pubkeys[12],
            token_1_vault: pubkeys[13],
            dex_supply_position_token_0: pubkeys[14],
            dex_supply_position_token_1: pubkeys[15],
            dex_borrow_position_token_0: pubkeys[16],
            dex_borrow_position_token_1: pubkeys[17],
            liquidity: pubkeys[18],
            liquidity_program: pubkeys[19],
            oracle_program: pubkeys[20],
            token_0_program: pubkeys[21],
            token_1_program: pubkeys[22],
            associated_token_program: pubkeys[23],
        }
    }
}
impl<'info> From<WithdrawPerfectInOneTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawPerfectInOneTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dex.clone(),
            accounts.user.clone(),
            accounts.position.clone(),
            accounts.user_token_0_account.clone(),
            accounts.user_token_1_account.clone(),
            accounts.token_0.clone(),
            accounts.token_1.clone(),
            accounts.token_0_reserve.clone(),
            accounts.token_1_reserve.clone(),
            accounts.token_0_rate_model.clone(),
            accounts.token_1_rate_model.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.dex_supply_position_token_0.clone(),
            accounts.dex_supply_position_token_1.clone(),
            accounts.dex_borrow_position_token_0.clone(),
            accounts.dex_borrow_position_token_1.clone(),
            accounts.liquidity.clone(),
            accounts.liquidity_program.clone(),
            accounts.oracle_program.clone(),
            accounts.token_0_program.clone(),
            accounts.token_1_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN]>
for WithdrawPerfectInOneTokenAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            signer: &arr[0],
            dex: &arr[1],
            user: &arr[2],
            position: &arr[3],
            user_token_0_account: &arr[4],
            user_token_1_account: &arr[5],
            token_0: &arr[6],
            token_1: &arr[7],
            token_0_reserve: &arr[8],
            token_1_reserve: &arr[9],
            token_0_rate_model: &arr[10],
            token_1_rate_model: &arr[11],
            token_0_vault: &arr[12],
            token_1_vault: &arr[13],
            dex_supply_position_token_0: &arr[14],
            dex_supply_position_token_1: &arr[15],
            dex_borrow_position_token_0: &arr[16],
            dex_borrow_position_token_1: &arr[17],
            liquidity: &arr[18],
            liquidity_program: &arr[19],
            oracle_program: &arr[20],
            token_0_program: &arr[21],
            token_1_program: &arr[22],
            associated_token_program: &arr[23],
        }
    }
}
pub const WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_DISCM: [u8; 8usize] = [
    200, 72, 151, 197, 24, 15, 243, 165,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawPerfectInOneTokenIxArgs {
    pub shares: u64,
    pub min_token0: u64,
    pub min_token1: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawPerfectInOneTokenIxData(pub WithdrawPerfectInOneTokenIxArgs);
impl From<WithdrawPerfectInOneTokenIxArgs> for WithdrawPerfectInOneTokenIxData {
    fn from(args: WithdrawPerfectInOneTokenIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawPerfectInOneTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawPerfectInOneTokenIxArgs {
                shares,
                min_token0,
                min_token1,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_token1, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_perfect_in_one_token_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawPerfectInOneTokenKeys,
    args: WithdrawPerfectInOneTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PERFECT_IN_ONE_TOKEN_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawPerfectInOneTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_perfect_in_one_token_ix(
    keys: WithdrawPerfectInOneTokenKeys,
    args: WithdrawPerfectInOneTokenIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_perfect_in_one_token_ix_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn withdraw_perfect_in_one_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPerfectInOneTokenAccounts<'_, '_>,
    args: WithdrawPerfectInOneTokenIxArgs,
) -> ProgramResult {
    let keys: WithdrawPerfectInOneTokenKeys = accounts.into();
    let ix = withdraw_perfect_in_one_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_perfect_in_one_token_invoke(
    accounts: WithdrawPerfectInOneTokenAccounts<'_, '_>,
    args: WithdrawPerfectInOneTokenIxArgs,
) -> ProgramResult {
    withdraw_perfect_in_one_token_invoke_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn withdraw_perfect_in_one_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPerfectInOneTokenAccounts<'_, '_>,
    args: WithdrawPerfectInOneTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawPerfectInOneTokenKeys = accounts.into();
    let ix = withdraw_perfect_in_one_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_perfect_in_one_token_invoke_signed(
    accounts: WithdrawPerfectInOneTokenAccounts<'_, '_>,
    args: WithdrawPerfectInOneTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_perfect_in_one_token_invoke_signed_with_program_id(
        JUPITER_LEND_AMM_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_perfect_in_one_token_verify_account_keys(
    accounts: WithdrawPerfectInOneTokenAccounts<'_, '_>,
    keys: WithdrawPerfectInOneTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dex.key, keys.dex),
        (*accounts.user.key, keys.user),
        (*accounts.position.key, keys.position),
        (*accounts.user_token_0_account.key, keys.user_token_0_account),
        (*accounts.user_token_1_account.key, keys.user_token_1_account),
        (*accounts.token_0.key, keys.token_0),
        (*accounts.token_1.key, keys.token_1),
        (*accounts.token_0_reserve.key, keys.token_0_reserve),
        (*accounts.token_1_reserve.key, keys.token_1_reserve),
        (*accounts.token_0_rate_model.key, keys.token_0_rate_model),
        (*accounts.token_1_rate_model.key, keys.token_1_rate_model),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.dex_supply_position_token_0.key, keys.dex_supply_position_token_0),
        (*accounts.dex_supply_position_token_1.key, keys.dex_supply_position_token_1),
        (*accounts.dex_borrow_position_token_0.key, keys.dex_borrow_position_token_0),
        (*accounts.dex_borrow_position_token_1.key, keys.dex_borrow_position_token_1),
        (*accounts.liquidity.key, keys.liquidity),
        (*accounts.liquidity_program.key, keys.liquidity_program),
        (*accounts.oracle_program.key, keys.oracle_program),
        (*accounts.token_0_program.key, keys.token_0_program),
        (*accounts.token_1_program.key, keys.token_1_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_perfect_in_one_token_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawPerfectInOneTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.dex,
        accounts.position,
        accounts.user_token_0_account,
        accounts.user_token_1_account,
        accounts.token_0_reserve,
        accounts.token_1_reserve,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.dex_supply_position_token_0,
        accounts.dex_supply_position_token_1,
        accounts.dex_borrow_position_token_0,
        accounts.dex_borrow_position_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_perfect_in_one_token_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawPerfectInOneTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_perfect_in_one_token_verify_account_privileges<'me, 'info>(
    accounts: WithdrawPerfectInOneTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_perfect_in_one_token_verify_writable_privileges(accounts)?;
    withdraw_perfect_in_one_token_verify_signer_privileges(accounts)?;
    Ok(())
}
