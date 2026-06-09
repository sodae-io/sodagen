use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum DeltafiProgramIx {
    CreateMarketConfig(CreateMarketConfigIxArgs),
    CreateSwap(CreateSwapIxArgs),
    UpdateSwapConfig(UpdateSwapConfigIxArgs),
    UpdateFarmConfig(UpdateFarmConfigIxArgs),
    InitNormalSwap(InitNormalSwapIxArgs),
    InitStableSwap(InitStableSwapIxArgs),
    InitSerumSwap(InitSerumSwapIxArgs),
    CreateLiquidityProviderV2(CreateLiquidityProviderV2IxArgs),
    DepositToNormalSwap(DepositToNormalSwapIxArgs),
    DepositToStableSwap(DepositToStableSwapIxArgs),
    DepositToSerumSwap(DepositToSerumSwapIxArgs),
    WithdrawFromNormalSwap(WithdrawFromNormalSwapIxArgs),
    WithdrawFromStableSwap(WithdrawFromStableSwapIxArgs),
    WithdrawFromSerumSwap(WithdrawFromSerumSwapIxArgs),
    NormalSwap(NormalSwapIxArgs),
    NormalSwapWithReferrer(NormalSwapWithReferrerIxArgs),
    NormalSwapWithRebate(NormalSwapWithRebateIxArgs),
    StableSwap(StableSwapIxArgs),
    StableSwapWithReferrer(StableSwapWithReferrerIxArgs),
    StableSwapWithRebate(StableSwapWithRebateIxArgs),
    SerumSwap(SerumSwapIxArgs),
    SerumSwapWithReferrer(SerumSwapWithReferrerIxArgs),
    CreateFarm(CreateFarmIxArgs),
    DepositToFarm(DepositToFarmIxArgs),
    WithdrawFromFarm(WithdrawFromFarmIxArgs),
    ClaimFarmRewards,
    CreateDeltafiUser(CreateDeltafiUserIxArgs),
    CreateDeltafiUserWithReferrer(CreateDeltafiUserWithReferrerIxArgs),
    ClaimSwapRewards,
    ClaimTradeRewards,
    ClaimReferralRewards,
    CreateFarmUserV2(CreateFarmUserV2IxArgs),
}
impl DeltafiProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&CREATE_MARKET_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_MARKET_CONFIG_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateMarketConfig(CreateMarketConfigIxArgs { bump }));
        }
        if buf.starts_with(&CREATE_SWAP_IX_DISCM) {
            let mut reader = &buf[CREATE_SWAP_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
            let swap_config = if reader.is_empty() {
                Default::default()
            } else {
                <SwapConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateSwap(CreateSwapIxArgs {
                    bump,
                    swap_type,
                    swap_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SWAP_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_SWAP_CONFIG_IX_DISCM.len()..];
            let swap_config = if reader.is_empty() {
                Default::default()
            } else {
                <SwapConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateSwapConfig(UpdateSwapConfigIxArgs {
                    swap_config,
                }),
            );
        }
        if buf.starts_with(&UPDATE_FARM_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_FARM_CONFIG_IX_DISCM.len()..];
            let farm_config = if reader.is_empty() {
                Default::default()
            } else {
                <FarmConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateFarmConfig(UpdateFarmConfigIxArgs {
                    farm_config,
                }),
            );
        }
        if buf.starts_with(&INIT_NORMAL_SWAP_IX_DISCM) {
            let mut reader = &buf[INIT_NORMAL_SWAP_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitNormalSwap(InitNormalSwapIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&INIT_STABLE_SWAP_IX_DISCM) {
            let mut reader = &buf[INIT_STABLE_SWAP_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitStableSwap(InitStableSwapIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&INIT_SERUM_SWAP_IX_DISCM) {
            let mut reader = &buf[INIT_SERUM_SWAP_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitSerumSwap(InitSerumSwapIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&CREATE_LIQUIDITY_PROVIDER_V2_IX_DISCM) {
            let mut reader = &buf[CREATE_LIQUIDITY_PROVIDER_V2_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateLiquidityProviderV2(CreateLiquidityProviderV2IxArgs {
                    bump,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_TO_NORMAL_SWAP_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_TO_NORMAL_SWAP_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositToNormalSwap(DepositToNormalSwapIxArgs {
                    base_amount,
                    quote_amount,
                    min_base_share,
                    min_quote_share,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_TO_STABLE_SWAP_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_TO_STABLE_SWAP_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositToStableSwap(DepositToStableSwapIxArgs {
                    base_amount,
                    quote_amount,
                    min_base_share,
                    min_quote_share,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_TO_SERUM_SWAP_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_TO_SERUM_SWAP_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositToSerumSwap(DepositToSerumSwapIxArgs {
                    base_amount,
                    quote_amount,
                    min_base_share,
                    min_quote_share,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FROM_NORMAL_SWAP_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_NORMAL_SWAP_IX_DISCM.len()..];
            let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawFromNormalSwap(WithdrawFromNormalSwapIxArgs {
                    base_share,
                    quote_share,
                    min_base_amount,
                    min_quote_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FROM_STABLE_SWAP_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_STABLE_SWAP_IX_DISCM.len()..];
            let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawFromStableSwap(WithdrawFromStableSwapIxArgs {
                    base_share,
                    quote_share,
                    min_base_amount,
                    min_quote_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FROM_SERUM_SWAP_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_SERUM_SWAP_IX_DISCM.len()..];
            let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawFromSerumSwap(WithdrawFromSerumSwapIxArgs {
                    base_share,
                    quote_share,
                    min_base_amount,
                    min_quote_amount,
                }),
            );
        }
        if buf.starts_with(&NORMAL_SWAP_IX_DISCM) {
            let mut reader = &buf[NORMAL_SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::NormalSwap(NormalSwapIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&NORMAL_SWAP_WITH_REFERRER_IX_DISCM) {
            let mut reader = &buf[NORMAL_SWAP_WITH_REFERRER_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::NormalSwapWithReferrer(NormalSwapWithReferrerIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&NORMAL_SWAP_WITH_REBATE_IX_DISCM) {
            let mut reader = &buf[NORMAL_SWAP_WITH_REBATE_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::NormalSwapWithRebate(NormalSwapWithRebateIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&STABLE_SWAP_IX_DISCM) {
            let mut reader = &buf[STABLE_SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::StableSwap(StableSwapIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&STABLE_SWAP_WITH_REFERRER_IX_DISCM) {
            let mut reader = &buf[STABLE_SWAP_WITH_REFERRER_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::StableSwapWithReferrer(StableSwapWithReferrerIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&STABLE_SWAP_WITH_REBATE_IX_DISCM) {
            let mut reader = &buf[STABLE_SWAP_WITH_REBATE_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::StableSwapWithRebate(StableSwapWithRebateIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&SERUM_SWAP_IX_DISCM) {
            let mut reader = &buf[SERUM_SWAP_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SerumSwap(SerumSwapIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&SERUM_SWAP_WITH_REFERRER_IX_DISCM) {
            let mut reader = &buf[SERUM_SWAP_WITH_REFERRER_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SerumSwapWithReferrer(SerumSwapWithReferrerIxArgs {
                    amount_in,
                    min_amount_out,
                }),
            );
        }
        if buf.starts_with(&CREATE_FARM_IX_DISCM) {
            let mut reader = &buf[CREATE_FARM_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            let seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let farm_config = if reader.is_empty() {
                Default::default()
            } else {
                <FarmConfig>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateFarm(CreateFarmIxArgs {
                    bump,
                    seed,
                    farm_config,
                }),
            );
        }
        if buf.starts_with(&DEPOSIT_TO_FARM_IX_DISCM) {
            let mut reader = &buf[DEPOSIT_TO_FARM_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DepositToFarm(DepositToFarmIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&WITHDRAW_FROM_FARM_IX_DISCM) {
            let mut reader = &buf[WITHDRAW_FROM_FARM_IX_DISCM.len()..];
            let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WithdrawFromFarm(WithdrawFromFarmIxArgs {
                    base_amount,
                    quote_amount,
                }),
            );
        }
        if buf.starts_with(&CLAIM_FARM_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimFarmRewards);
        }
        if buf.starts_with(&CREATE_DELTAFI_USER_IX_DISCM) {
            let mut reader = &buf[CREATE_DELTAFI_USER_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateDeltafiUser(CreateDeltafiUserIxArgs { bump }));
        }
        if buf.starts_with(&CREATE_DELTAFI_USER_WITH_REFERRER_IX_DISCM) {
            let mut reader = &buf[CREATE_DELTAFI_USER_WITH_REFERRER_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateDeltafiUserWithReferrer(CreateDeltafiUserWithReferrerIxArgs {
                    bump,
                }),
            );
        }
        if buf.starts_with(&CLAIM_SWAP_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimSwapRewards);
        }
        if buf.starts_with(&CLAIM_TRADE_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimTradeRewards);
        }
        if buf.starts_with(&CLAIM_REFERRAL_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimReferralRewards);
        }
        if buf.starts_with(&CREATE_FARM_USER_V2_IX_DISCM) {
            let mut reader = &buf[CREATE_FARM_USER_V2_IX_DISCM.len()..];
            let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::CreateFarmUserV2(CreateFarmUserV2IxArgs { bump }));
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateMarketConfig(args) => {
                writer.write_all(&CREATE_MARKET_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CreateSwap(args) => {
                writer.write_all(&CREATE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.swap_config, &mut writer)?;
                Ok(())
            }
            Self::UpdateSwapConfig(args) => {
                writer.write_all(&UPDATE_SWAP_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.swap_config, &mut writer)?;
                Ok(())
            }
            Self::UpdateFarmConfig(args) => {
                writer.write_all(&UPDATE_FARM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.farm_config, &mut writer)?;
                Ok(())
            }
            Self::InitNormalSwap(args) => {
                writer.write_all(&INIT_NORMAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::InitStableSwap(args) => {
                writer.write_all(&INIT_STABLE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::InitSerumSwap(args) => {
                writer.write_all(&INIT_SERUM_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::CreateLiquidityProviderV2(args) => {
                writer.write_all(&CREATE_LIQUIDITY_PROVIDER_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::DepositToNormalSwap(args) => {
                writer.write_all(&DEPOSIT_TO_NORMAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_base_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_quote_share, &mut writer)?;
                Ok(())
            }
            Self::DepositToStableSwap(args) => {
                writer.write_all(&DEPOSIT_TO_STABLE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_base_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_quote_share, &mut writer)?;
                Ok(())
            }
            Self::DepositToSerumSwap(args) => {
                writer.write_all(&DEPOSIT_TO_SERUM_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_base_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_quote_share, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromNormalSwap(args) => {
                writer.write_all(&WITHDRAW_FROM_NORMAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_quote_amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromStableSwap(args) => {
                writer.write_all(&WITHDRAW_FROM_STABLE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_quote_amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromSerumSwap(args) => {
                writer.write_all(&WITHDRAW_FROM_SERUM_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_share, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_quote_amount, &mut writer)?;
                Ok(())
            }
            Self::NormalSwap(args) => {
                writer.write_all(&NORMAL_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::NormalSwapWithReferrer(args) => {
                writer.write_all(&NORMAL_SWAP_WITH_REFERRER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::NormalSwapWithRebate(args) => {
                writer.write_all(&NORMAL_SWAP_WITH_REBATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::StableSwap(args) => {
                writer.write_all(&STABLE_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::StableSwapWithReferrer(args) => {
                writer.write_all(&STABLE_SWAP_WITH_REFERRER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::StableSwapWithRebate(args) => {
                writer.write_all(&STABLE_SWAP_WITH_REBATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SerumSwap(args) => {
                writer.write_all(&SERUM_SWAP_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::SerumSwapWithReferrer(args) => {
                writer.write_all(&SERUM_SWAP_WITH_REFERRER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_amount_out, &mut writer)?;
                Ok(())
            }
            Self::CreateFarm(args) => {
                writer.write_all(&CREATE_FARM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.seed, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.farm_config, &mut writer)?;
                Ok(())
            }
            Self::DepositToFarm(args) => {
                writer.write_all(&DEPOSIT_TO_FARM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::WithdrawFromFarm(args) => {
                writer.write_all(&WITHDRAW_FROM_FARM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.quote_amount, &mut writer)?;
                Ok(())
            }
            Self::ClaimFarmRewards => writer.write_all(&CLAIM_FARM_REWARDS_IX_DISCM),
            Self::CreateDeltafiUser(args) => {
                writer.write_all(&CREATE_DELTAFI_USER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::CreateDeltafiUserWithReferrer(args) => {
                writer.write_all(&CREATE_DELTAFI_USER_WITH_REFERRER_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
                Ok(())
            }
            Self::ClaimSwapRewards => writer.write_all(&CLAIM_SWAP_REWARDS_IX_DISCM),
            Self::ClaimTradeRewards => writer.write_all(&CLAIM_TRADE_REWARDS_IX_DISCM),
            Self::ClaimReferralRewards => {
                writer.write_all(&CLAIM_REFERRAL_REWARDS_IX_DISCM)
            }
            Self::CreateFarmUserV2(args) => {
                writer.write_all(&CREATE_FARM_USER_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.bump, &mut writer)?;
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
pub const CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CreateMarketConfigAccounts<'me, 'info> {
    pub seed: &'me AccountInfo<'info>,
    pub market_config: &'me AccountInfo<'info>,
    pub deltafi_mint: &'me AccountInfo<'info>,
    pub deltafi_token: &'me AccountInfo<'info>,
    pub pyth_program: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMarketConfigKeys {
    pub seed: Pubkey,
    pub market_config: Pubkey,
    pub deltafi_mint: Pubkey,
    pub deltafi_token: Pubkey,
    pub pyth_program: Pubkey,
    pub serum_program: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateMarketConfigAccounts<'_, '_>> for CreateMarketConfigKeys {
    fn from(accounts: CreateMarketConfigAccounts) -> Self {
        Self {
            seed: *accounts.seed.key,
            market_config: *accounts.market_config.key,
            deltafi_mint: *accounts.deltafi_mint.key,
            deltafi_token: *accounts.deltafi_token.key,
            pyth_program: *accounts.pyth_program.key,
            serum_program: *accounts.serum_program.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateMarketConfigKeys>
for [AccountMeta; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMarketConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.seed,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deltafi_token,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN]> for CreateMarketConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seed: pubkeys[0],
            market_config: pubkeys[1],
            deltafi_mint: pubkeys[2],
            deltafi_token: pubkeys[3],
            pyth_program: pubkeys[4],
            serum_program: pubkeys[5],
            admin: pubkeys[6],
            payer: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<CreateMarketConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMarketConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.seed.clone(),
            accounts.market_config.clone(),
            accounts.deltafi_mint.clone(),
            accounts.deltafi_token.clone(),
            accounts.pyth_program.clone(),
            accounts.serum_program.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN]>
for CreateMarketConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            seed: &arr[0],
            market_config: &arr[1],
            deltafi_mint: &arr[2],
            deltafi_token: &arr[3],
            pyth_program: &arr[4],
            serum_program: &arr[5],
            admin: &arr[6],
            payer: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
pub const CREATE_MARKET_CONFIG_IX_DISCM: [u8; 8usize] = [
    33, 94, 138, 19, 111, 112, 91, 111,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketConfigIxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMarketConfigIxData(pub CreateMarketConfigIxArgs);
impl From<CreateMarketConfigIxArgs> for CreateMarketConfigIxData {
    fn from(args: CreateMarketConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMarketConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MARKET_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateMarketConfigIxArgs { bump }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MARKET_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_market_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMarketConfigKeys,
    args: CreateMarketConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MARKET_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateMarketConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_market_config_ix(
    keys: CreateMarketConfigKeys,
    args: CreateMarketConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_market_config_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_market_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketConfigAccounts<'_, '_>,
    args: CreateMarketConfigIxArgs,
) -> ProgramResult {
    let keys: CreateMarketConfigKeys = accounts.into();
    let ix = create_market_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_market_config_invoke(
    accounts: CreateMarketConfigAccounts<'_, '_>,
    args: CreateMarketConfigIxArgs,
) -> ProgramResult {
    create_market_config_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn create_market_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMarketConfigAccounts<'_, '_>,
    args: CreateMarketConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMarketConfigKeys = accounts.into();
    let ix = create_market_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_market_config_invoke_signed(
    accounts: CreateMarketConfigAccounts<'_, '_>,
    args: CreateMarketConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_market_config_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_market_config_verify_account_keys(
    accounts: CreateMarketConfigAccounts<'_, '_>,
    keys: CreateMarketConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.seed.key, keys.seed),
        (*accounts.market_config.key, keys.market_config),
        (*accounts.deltafi_mint.key, keys.deltafi_mint),
        (*accounts.deltafi_token.key, keys.deltafi_token),
        (*accounts.pyth_program.key, keys.pyth_program),
        (*accounts.serum_program.key, keys.serum_program),
        (*accounts.admin.key, keys.admin),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_market_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateMarketConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market_config,
        accounts.deltafi_token,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_market_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateMarketConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.seed,
        accounts.deltafi_token,
        accounts.admin,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_market_config_verify_account_privileges<'me, 'info>(
    accounts: CreateMarketConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_market_config_verify_writable_privileges(accounts)?;
    create_market_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CreateSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub seed: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub mint_base: &'me AccountInfo<'info>,
    pub mint_quote: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub admin_fee_token_base: &'me AccountInfo<'info>,
    pub admin_fee_token_quote: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateSwapKeys {
    pub market_config: Pubkey,
    pub seed: Pubkey,
    pub swap_info: Pubkey,
    pub mint_base: Pubkey,
    pub mint_quote: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub admin_fee_token_base: Pubkey,
    pub admin_fee_token_quote: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateSwapAccounts<'_, '_>> for CreateSwapKeys {
    fn from(accounts: CreateSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            seed: *accounts.seed.key,
            swap_info: *accounts.swap_info.key,
            mint_base: *accounts.mint_base.key,
            mint_quote: *accounts.mint_quote.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            admin_fee_token_base: *accounts.admin_fee_token_base.key,
            admin_fee_token_quote: *accounts.admin_fee_token_quote.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateSwapKeys> for [AccountMeta; CREATE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seed,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_SWAP_IX_ACCOUNTS_LEN]> for CreateSwapKeys {
    fn from(pubkeys: [Pubkey; CREATE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            seed: pubkeys[1],
            swap_info: pubkeys[2],
            mint_base: pubkeys[3],
            mint_quote: pubkeys[4],
            token_base: pubkeys[5],
            token_quote: pubkeys[6],
            admin_fee_token_base: pubkeys[7],
            admin_fee_token_quote: pubkeys[8],
            admin: pubkeys[9],
            payer: pubkeys[10],
            token_program: pubkeys[11],
            system_program: pubkeys[12],
            rent: pubkeys[13],
        }
    }
}
impl<'info> From<CreateSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.seed.clone(),
            accounts.swap_info.clone(),
            accounts.mint_base.clone(),
            accounts.mint_quote.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.admin_fee_token_base.clone(),
            accounts.admin_fee_token_quote.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_SWAP_IX_ACCOUNTS_LEN]>
for CreateSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            seed: &arr[1],
            swap_info: &arr[2],
            mint_base: &arr[3],
            mint_quote: &arr[4],
            token_base: &arr[5],
            token_quote: &arr[6],
            admin_fee_token_base: &arr[7],
            admin_fee_token_quote: &arr[8],
            admin: &arr[9],
            payer: &arr[10],
            token_program: &arr[11],
            system_program: &arr[12],
            rent: &arr[13],
        }
    }
}
pub const CREATE_SWAP_IX_DISCM: [u8; 8usize] = [176, 207, 238, 60, 195, 2, 203, 91];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSwapIxArgs {
    pub bump: u8,
    pub swap_type: SwapType,
    pub swap_config: SwapConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateSwapIxData(pub CreateSwapIxArgs);
impl From<CreateSwapIxArgs> for CreateSwapIxData {
    fn from(args: CreateSwapIxArgs) -> Self {
        Self(args)
    }
}
impl CreateSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let swap_config = if reader.is_empty() {
            Default::default()
        } else {
            <SwapConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateSwapIxArgs {
                bump,
                swap_type,
                swap_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.swap_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateSwapKeys,
    args: CreateSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_swap_ix(
    keys: CreateSwapKeys,
    args: CreateSwapIxArgs,
) -> std::io::Result<Instruction> {
    create_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateSwapAccounts<'_, '_>,
    args: CreateSwapIxArgs,
) -> ProgramResult {
    let keys: CreateSwapKeys = accounts.into();
    let ix = create_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_swap_invoke(
    accounts: CreateSwapAccounts<'_, '_>,
    args: CreateSwapIxArgs,
) -> ProgramResult {
    create_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn create_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateSwapAccounts<'_, '_>,
    args: CreateSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateSwapKeys = accounts.into();
    let ix = create_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_swap_invoke_signed(
    accounts: CreateSwapAccounts<'_, '_>,
    args: CreateSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_swap_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_swap_verify_account_keys(
    accounts: CreateSwapAccounts<'_, '_>,
    keys: CreateSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.seed.key, keys.seed),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.mint_base.key, keys.mint_base),
        (*accounts.mint_quote.key, keys.mint_quote),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.admin_fee_token_base.key, keys.admin_fee_token_base),
        (*accounts.admin_fee_token_quote.key, keys.admin_fee_token_quote),
        (*accounts.admin.key, keys.admin),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_swap_verify_writable_privileges<'me, 'info>(
    accounts: CreateSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.token_base,
        accounts.token_quote,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_swap_verify_signer_privileges<'me, 'info>(
    accounts: CreateSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.seed,
        accounts.token_base,
        accounts.token_quote,
        accounts.admin,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_swap_verify_account_privileges<'me, 'info>(
    accounts: CreateSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_swap_verify_writable_privileges(accounts)?;
    create_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSwapConfigAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSwapConfigKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub admin: Pubkey,
}
impl From<UpdateSwapConfigAccounts<'_, '_>> for UpdateSwapConfigKeys {
    fn from(accounts: UpdateSwapConfigAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<UpdateSwapConfigKeys> for [AccountMeta; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSwapConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN]> for UpdateSwapConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateSwapConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSwapConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateSwapConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const UPDATE_SWAP_CONFIG_IX_DISCM: [u8; 8usize] = [
    123, 172, 158, 60, 104, 133, 39, 13,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateSwapConfigIxArgs {
    pub swap_config: SwapConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSwapConfigIxData(pub UpdateSwapConfigIxArgs);
impl From<UpdateSwapConfigIxArgs> for UpdateSwapConfigIxData {
    fn from(args: UpdateSwapConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSwapConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SWAP_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let swap_config = if reader.is_empty() {
            Default::default()
        } else {
            <SwapConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateSwapConfigIxArgs {
                swap_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SWAP_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.swap_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_swap_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSwapConfigKeys,
    args: UpdateSwapConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SWAP_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateSwapConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_swap_config_ix(
    keys: UpdateSwapConfigKeys,
    args: UpdateSwapConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_swap_config_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn update_swap_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSwapConfigAccounts<'_, '_>,
    args: UpdateSwapConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateSwapConfigKeys = accounts.into();
    let ix = update_swap_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_swap_config_invoke(
    accounts: UpdateSwapConfigAccounts<'_, '_>,
    args: UpdateSwapConfigIxArgs,
) -> ProgramResult {
    update_swap_config_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn update_swap_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSwapConfigAccounts<'_, '_>,
    args: UpdateSwapConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSwapConfigKeys = accounts.into();
    let ix = update_swap_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_swap_config_invoke_signed(
    accounts: UpdateSwapConfigAccounts<'_, '_>,
    args: UpdateSwapConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_swap_config_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_swap_config_verify_account_keys(
    accounts: UpdateSwapConfigAccounts<'_, '_>,
    keys: UpdateSwapConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_swap_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSwapConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.swap_info] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_swap_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateSwapConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_swap_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateSwapConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_swap_config_verify_writable_privileges(accounts)?;
    update_swap_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFarmConfigAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub farm_info: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFarmConfigKeys {
    pub market_config: Pubkey,
    pub farm_info: Pubkey,
    pub admin: Pubkey,
}
impl From<UpdateFarmConfigAccounts<'_, '_>> for UpdateFarmConfigKeys {
    fn from(accounts: UpdateFarmConfigAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            farm_info: *accounts.farm_info.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<UpdateFarmConfigKeys> for [AccountMeta; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFarmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]> for UpdateFarmConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            farm_info: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateFarmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFarmConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.farm_info.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateFarmConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            farm_info: &arr[1],
            admin: &arr[2],
        }
    }
}
pub const UPDATE_FARM_CONFIG_IX_DISCM: [u8; 8usize] = [
    214, 176, 188, 244, 203, 59, 230, 207,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFarmConfigIxArgs {
    pub farm_config: FarmConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFarmConfigIxData(pub UpdateFarmConfigIxArgs);
impl From<UpdateFarmConfigIxArgs> for UpdateFarmConfigIxData {
    fn from(args: UpdateFarmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFarmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FARM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let farm_config = if reader.is_empty() {
            Default::default()
        } else {
            <FarmConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateFarmConfigIxArgs {
                farm_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FARM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.farm_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_farm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFarmConfigKeys,
    args: UpdateFarmConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FARM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFarmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_farm_config_ix(
    keys: UpdateFarmConfigKeys,
    args: UpdateFarmConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_farm_config_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn update_farm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateFarmConfigKeys = accounts.into();
    let ix = update_farm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_farm_config_invoke(
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
) -> ProgramResult {
    update_farm_config_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn update_farm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFarmConfigKeys = accounts.into();
    let ix = update_farm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_farm_config_invoke_signed(
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    args: UpdateFarmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_farm_config_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_farm_config_verify_account_keys(
    accounts: UpdateFarmConfigAccounts<'_, '_>,
    keys: UpdateFarmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.farm_info.key, keys.farm_info),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_farm_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFarmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.farm_info] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_farm_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFarmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_farm_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateFarmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_farm_config_verify_writable_privileges(accounts)?;
    update_farm_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitNormalSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitNormalSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitNormalSwapAccounts<'_, '_>> for InitNormalSwapKeys {
    fn from(accounts: InitNormalSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitNormalSwapKeys> for [AccountMeta; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: InitNormalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN]> for InitNormalSwapKeys {
    fn from(pubkeys: [Pubkey; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_token_base: pubkeys[2],
            user_token_quote: pubkeys[3],
            liquidity_provider: pubkeys[4],
            token_base: pubkeys[5],
            token_quote: pubkeys[6],
            pyth_price_base: pubkeys[7],
            pyth_price_quote: pubkeys[8],
            user_authority: pubkeys[9],
            admin: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<InitNormalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitNormalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN]>
for InitNormalSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_token_base: &arr[2],
            user_token_quote: &arr[3],
            liquidity_provider: &arr[4],
            token_base: &arr[5],
            token_quote: &arr[6],
            pyth_price_base: &arr[7],
            pyth_price_quote: &arr[8],
            user_authority: &arr[9],
            admin: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const INIT_NORMAL_SWAP_IX_DISCM: [u8; 8usize] = [
    171, 235, 255, 67, 227, 195, 223, 227,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitNormalSwapIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitNormalSwapIxData(pub InitNormalSwapIxArgs);
impl From<InitNormalSwapIxArgs> for InitNormalSwapIxData {
    fn from(args: InitNormalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl InitNormalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_NORMAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitNormalSwapIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_NORMAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_normal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: InitNormalSwapKeys,
    args: InitNormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_NORMAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitNormalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_normal_swap_ix(
    keys: InitNormalSwapKeys,
    args: InitNormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    init_normal_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn init_normal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitNormalSwapAccounts<'_, '_>,
    args: InitNormalSwapIxArgs,
) -> ProgramResult {
    let keys: InitNormalSwapKeys = accounts.into();
    let ix = init_normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_normal_swap_invoke(
    accounts: InitNormalSwapAccounts<'_, '_>,
    args: InitNormalSwapIxArgs,
) -> ProgramResult {
    init_normal_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn init_normal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitNormalSwapAccounts<'_, '_>,
    args: InitNormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitNormalSwapKeys = accounts.into();
    let ix = init_normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_normal_swap_invoke_signed(
    accounts: InitNormalSwapAccounts<'_, '_>,
    args: InitNormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_normal_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_normal_swap_verify_account_keys(
    accounts: InitNormalSwapAccounts<'_, '_>,
    keys: InitNormalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_normal_swap_verify_writable_privileges<'me, 'info>(
    accounts: InitNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_normal_swap_verify_signer_privileges<'me, 'info>(
    accounts: InitNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_normal_swap_verify_account_privileges<'me, 'info>(
    accounts: InitNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_normal_swap_verify_writable_privileges(accounts)?;
    init_normal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_STABLE_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitStableSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitStableSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitStableSwapAccounts<'_, '_>> for InitStableSwapKeys {
    fn from(accounts: InitStableSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitStableSwapKeys> for [AccountMeta; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: InitStableSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN]> for InitStableSwapKeys {
    fn from(pubkeys: [Pubkey; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_token_base: pubkeys[2],
            user_token_quote: pubkeys[3],
            liquidity_provider: pubkeys[4],
            token_base: pubkeys[5],
            token_quote: pubkeys[6],
            pyth_price_base: pubkeys[7],
            pyth_price_quote: pubkeys[8],
            user_authority: pubkeys[9],
            admin: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<InitStableSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitStableSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN]>
for InitStableSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_token_base: &arr[2],
            user_token_quote: &arr[3],
            liquidity_provider: &arr[4],
            token_base: &arr[5],
            token_quote: &arr[6],
            pyth_price_base: &arr[7],
            pyth_price_quote: &arr[8],
            user_authority: &arr[9],
            admin: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const INIT_STABLE_SWAP_IX_DISCM: [u8; 8usize] = [10, 83, 211, 228, 77, 4, 220, 12];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitStableSwapIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitStableSwapIxData(pub InitStableSwapIxArgs);
impl From<InitStableSwapIxArgs> for InitStableSwapIxData {
    fn from(args: InitStableSwapIxArgs) -> Self {
        Self(args)
    }
}
impl InitStableSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_STABLE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitStableSwapIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_STABLE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_stable_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: InitStableSwapKeys,
    args: InitStableSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_STABLE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitStableSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_stable_swap_ix(
    keys: InitStableSwapKeys,
    args: InitStableSwapIxArgs,
) -> std::io::Result<Instruction> {
    init_stable_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn init_stable_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitStableSwapAccounts<'_, '_>,
    args: InitStableSwapIxArgs,
) -> ProgramResult {
    let keys: InitStableSwapKeys = accounts.into();
    let ix = init_stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_stable_swap_invoke(
    accounts: InitStableSwapAccounts<'_, '_>,
    args: InitStableSwapIxArgs,
) -> ProgramResult {
    init_stable_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn init_stable_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitStableSwapAccounts<'_, '_>,
    args: InitStableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitStableSwapKeys = accounts.into();
    let ix = init_stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_stable_swap_invoke_signed(
    accounts: InitStableSwapAccounts<'_, '_>,
    args: InitStableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_stable_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_stable_swap_verify_account_keys(
    accounts: InitStableSwapAccounts<'_, '_>,
    keys: InitStableSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_stable_swap_verify_writable_privileges<'me, 'info>(
    accounts: InitStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_stable_swap_verify_signer_privileges<'me, 'info>(
    accounts: InitStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_stable_swap_verify_account_privileges<'me, 'info>(
    accounts: InitStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_stable_swap_verify_writable_privileges(accounts)?;
    init_stable_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_SERUM_SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitSerumSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitSerumSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub user_authority: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
}
impl From<InitSerumSwapAccounts<'_, '_>> for InitSerumSwapKeys {
    fn from(accounts: InitSerumSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            user_authority: *accounts.user_authority.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitSerumSwapKeys> for [AccountMeta; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: InitSerumSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
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
impl From<[Pubkey; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN]> for InitSerumSwapKeys {
    fn from(pubkeys: [Pubkey; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_token_base: pubkeys[2],
            user_token_quote: pubkeys[3],
            liquidity_provider: pubkeys[4],
            token_base: pubkeys[5],
            token_quote: pubkeys[6],
            serum_market: pubkeys[7],
            serum_bids: pubkeys[8],
            serum_asks: pubkeys[9],
            user_authority: pubkeys[10],
            admin: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<InitSerumSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitSerumSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.user_authority.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN]>
for InitSerumSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_token_base: &arr[2],
            user_token_quote: &arr[3],
            liquidity_provider: &arr[4],
            token_base: &arr[5],
            token_quote: &arr[6],
            serum_market: &arr[7],
            serum_bids: &arr[8],
            serum_asks: &arr[9],
            user_authority: &arr[10],
            admin: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const INIT_SERUM_SWAP_IX_DISCM: [u8; 8usize] = [65, 87, 10, 15, 99, 93, 113, 206];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitSerumSwapIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitSerumSwapIxData(pub InitSerumSwapIxArgs);
impl From<InitSerumSwapIxArgs> for InitSerumSwapIxData {
    fn from(args: InitSerumSwapIxArgs) -> Self {
        Self(args)
    }
}
impl InitSerumSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_SERUM_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitSerumSwapIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_SERUM_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_serum_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: InitSerumSwapKeys,
    args: InitSerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_SERUM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitSerumSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_serum_swap_ix(
    keys: InitSerumSwapKeys,
    args: InitSerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    init_serum_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn init_serum_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitSerumSwapAccounts<'_, '_>,
    args: InitSerumSwapIxArgs,
) -> ProgramResult {
    let keys: InitSerumSwapKeys = accounts.into();
    let ix = init_serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_serum_swap_invoke(
    accounts: InitSerumSwapAccounts<'_, '_>,
    args: InitSerumSwapIxArgs,
) -> ProgramResult {
    init_serum_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn init_serum_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitSerumSwapAccounts<'_, '_>,
    args: InitSerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitSerumSwapKeys = accounts.into();
    let ix = init_serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_serum_swap_invoke_signed(
    accounts: InitSerumSwapAccounts<'_, '_>,
    args: InitSerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_serum_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_serum_swap_verify_account_keys(
    accounts: InitSerumSwapAccounts<'_, '_>,
    keys: InitSerumSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.serum_market.key, keys.serum_market),
        (*accounts.serum_bids.key, keys.serum_bids),
        (*accounts.serum_asks.key, keys.serum_asks),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_serum_swap_verify_writable_privileges<'me, 'info>(
    accounts: InitSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_serum_swap_verify_signer_privileges<'me, 'info>(
    accounts: InitSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_serum_swap_verify_account_privileges<'me, 'info>(
    accounts: InitSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_serum_swap_verify_writable_privileges(accounts)?;
    init_serum_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateLiquidityProviderV2Accounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLiquidityProviderV2Keys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub liquidity_provider: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateLiquidityProviderV2Accounts<'_, '_>> for CreateLiquidityProviderV2Keys {
    fn from(accounts: CreateLiquidityProviderV2Accounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateLiquidityProviderV2Keys>
for [AccountMeta; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLiquidityProviderV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN]>
for CreateLiquidityProviderV2Keys {
    fn from(pubkeys: [Pubkey; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            liquidity_provider: pubkeys[2],
            owner: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<CreateLiquidityProviderV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLiquidityProviderV2Accounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.liquidity_provider.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN]>
for CreateLiquidityProviderV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            liquidity_provider: &arr[2],
            owner: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
pub const CREATE_LIQUIDITY_PROVIDER_V2_IX_DISCM: [u8; 8usize] = [
    173, 173, 108, 177, 152, 203, 107, 134,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateLiquidityProviderV2IxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLiquidityProviderV2IxData(pub CreateLiquidityProviderV2IxArgs);
impl From<CreateLiquidityProviderV2IxArgs> for CreateLiquidityProviderV2IxData {
    fn from(args: CreateLiquidityProviderV2IxArgs) -> Self {
        Self(args)
    }
}
impl CreateLiquidityProviderV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LIQUIDITY_PROVIDER_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateLiquidityProviderV2IxArgs {
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LIQUIDITY_PROVIDER_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_liquidity_provider_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateLiquidityProviderV2Keys,
    args: CreateLiquidityProviderV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_LIQUIDITY_PROVIDER_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateLiquidityProviderV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_liquidity_provider_v2_ix(
    keys: CreateLiquidityProviderV2Keys,
    args: CreateLiquidityProviderV2IxArgs,
) -> std::io::Result<Instruction> {
    create_liquidity_provider_v2_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_liquidity_provider_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateLiquidityProviderV2Accounts<'_, '_>,
    args: CreateLiquidityProviderV2IxArgs,
) -> ProgramResult {
    let keys: CreateLiquidityProviderV2Keys = accounts.into();
    let ix = create_liquidity_provider_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_liquidity_provider_v2_invoke(
    accounts: CreateLiquidityProviderV2Accounts<'_, '_>,
    args: CreateLiquidityProviderV2IxArgs,
) -> ProgramResult {
    create_liquidity_provider_v2_invoke_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_liquidity_provider_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateLiquidityProviderV2Accounts<'_, '_>,
    args: CreateLiquidityProviderV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateLiquidityProviderV2Keys = accounts.into();
    let ix = create_liquidity_provider_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_liquidity_provider_v2_invoke_signed(
    accounts: CreateLiquidityProviderV2Accounts<'_, '_>,
    args: CreateLiquidityProviderV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_liquidity_provider_v2_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_liquidity_provider_v2_verify_account_keys(
    accounts: CreateLiquidityProviderV2Accounts<'_, '_>,
    keys: CreateLiquidityProviderV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_liquidity_provider_v2_verify_writable_privileges<'me, 'info>(
    accounts: CreateLiquidityProviderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidity_provider,
        accounts.owner,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_liquidity_provider_v2_verify_signer_privileges<'me, 'info>(
    accounts: CreateLiquidityProviderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_liquidity_provider_v2_verify_account_privileges<'me, 'info>(
    accounts: CreateLiquidityProviderV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_liquidity_provider_v2_verify_writable_privileges(accounts)?;
    create_liquidity_provider_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositToNormalSwapAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositToNormalSwapKeys {
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositToNormalSwapAccounts<'_, '_>> for DepositToNormalSwapKeys {
    fn from(accounts: DepositToNormalSwapAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositToNormalSwapKeys>
for [AccountMeta; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositToNormalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN]> for DepositToNormalSwapKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            user_token_base: pubkeys[1],
            user_token_quote: pubkeys[2],
            liquidity_provider: pubkeys[3],
            token_base: pubkeys[4],
            token_quote: pubkeys[5],
            pyth_price_base: pubkeys[6],
            pyth_price_quote: pubkeys[7],
            user_authority: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<DepositToNormalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositToNormalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN]>
for DepositToNormalSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_info: &arr[0],
            user_token_base: &arr[1],
            user_token_quote: &arr[2],
            liquidity_provider: &arr[3],
            token_base: &arr[4],
            token_quote: &arr[5],
            pyth_price_base: &arr[6],
            pyth_price_quote: &arr[7],
            user_authority: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const DEPOSIT_TO_NORMAL_SWAP_IX_DISCM: [u8; 8usize] = [
    86, 251, 171, 167, 234, 9, 223, 241,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositToNormalSwapIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
    pub min_base_share: u64,
    pub min_quote_share: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositToNormalSwapIxData(pub DepositToNormalSwapIxArgs);
impl From<DepositToNormalSwapIxArgs> for DepositToNormalSwapIxData {
    fn from(args: DepositToNormalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl DepositToNormalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_TO_NORMAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositToNormalSwapIxArgs {
                base_amount,
                quote_amount,
                min_base_share,
                min_quote_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_TO_NORMAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_quote_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_to_normal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositToNormalSwapKeys,
    args: DepositToNormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_TO_NORMAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositToNormalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_to_normal_swap_ix(
    keys: DepositToNormalSwapKeys,
    args: DepositToNormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    deposit_to_normal_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn deposit_to_normal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositToNormalSwapAccounts<'_, '_>,
    args: DepositToNormalSwapIxArgs,
) -> ProgramResult {
    let keys: DepositToNormalSwapKeys = accounts.into();
    let ix = deposit_to_normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_to_normal_swap_invoke(
    accounts: DepositToNormalSwapAccounts<'_, '_>,
    args: DepositToNormalSwapIxArgs,
) -> ProgramResult {
    deposit_to_normal_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn deposit_to_normal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositToNormalSwapAccounts<'_, '_>,
    args: DepositToNormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositToNormalSwapKeys = accounts.into();
    let ix = deposit_to_normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_to_normal_swap_invoke_signed(
    accounts: DepositToNormalSwapAccounts<'_, '_>,
    args: DepositToNormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_to_normal_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_to_normal_swap_verify_account_keys(
    accounts: DepositToNormalSwapAccounts<'_, '_>,
    keys: DepositToNormalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_to_normal_swap_verify_writable_privileges<'me, 'info>(
    accounts: DepositToNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_to_normal_swap_verify_signer_privileges<'me, 'info>(
    accounts: DepositToNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_to_normal_swap_verify_account_privileges<'me, 'info>(
    accounts: DepositToNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_to_normal_swap_verify_writable_privileges(accounts)?;
    deposit_to_normal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositToStableSwapAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositToStableSwapKeys {
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositToStableSwapAccounts<'_, '_>> for DepositToStableSwapKeys {
    fn from(accounts: DepositToStableSwapAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositToStableSwapKeys>
for [AccountMeta; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositToStableSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN]> for DepositToStableSwapKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            user_token_base: pubkeys[1],
            user_token_quote: pubkeys[2],
            liquidity_provider: pubkeys[3],
            token_base: pubkeys[4],
            token_quote: pubkeys[5],
            pyth_price_base: pubkeys[6],
            pyth_price_quote: pubkeys[7],
            user_authority: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<DepositToStableSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositToStableSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN]>
for DepositToStableSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_info: &arr[0],
            user_token_base: &arr[1],
            user_token_quote: &arr[2],
            liquidity_provider: &arr[3],
            token_base: &arr[4],
            token_quote: &arr[5],
            pyth_price_base: &arr[6],
            pyth_price_quote: &arr[7],
            user_authority: &arr[8],
            token_program: &arr[9],
        }
    }
}
pub const DEPOSIT_TO_STABLE_SWAP_IX_DISCM: [u8; 8usize] = [
    54, 175, 75, 157, 101, 197, 152, 250,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositToStableSwapIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
    pub min_base_share: u64,
    pub min_quote_share: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositToStableSwapIxData(pub DepositToStableSwapIxArgs);
impl From<DepositToStableSwapIxArgs> for DepositToStableSwapIxData {
    fn from(args: DepositToStableSwapIxArgs) -> Self {
        Self(args)
    }
}
impl DepositToStableSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_TO_STABLE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositToStableSwapIxArgs {
                base_amount,
                quote_amount,
                min_base_share,
                min_quote_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_TO_STABLE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_quote_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_to_stable_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositToStableSwapKeys,
    args: DepositToStableSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_TO_STABLE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositToStableSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_to_stable_swap_ix(
    keys: DepositToStableSwapKeys,
    args: DepositToStableSwapIxArgs,
) -> std::io::Result<Instruction> {
    deposit_to_stable_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn deposit_to_stable_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositToStableSwapAccounts<'_, '_>,
    args: DepositToStableSwapIxArgs,
) -> ProgramResult {
    let keys: DepositToStableSwapKeys = accounts.into();
    let ix = deposit_to_stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_to_stable_swap_invoke(
    accounts: DepositToStableSwapAccounts<'_, '_>,
    args: DepositToStableSwapIxArgs,
) -> ProgramResult {
    deposit_to_stable_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn deposit_to_stable_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositToStableSwapAccounts<'_, '_>,
    args: DepositToStableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositToStableSwapKeys = accounts.into();
    let ix = deposit_to_stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_to_stable_swap_invoke_signed(
    accounts: DepositToStableSwapAccounts<'_, '_>,
    args: DepositToStableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_to_stable_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_to_stable_swap_verify_account_keys(
    accounts: DepositToStableSwapAccounts<'_, '_>,
    keys: DepositToStableSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_to_stable_swap_verify_writable_privileges<'me, 'info>(
    accounts: DepositToStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_to_stable_swap_verify_signer_privileges<'me, 'info>(
    accounts: DepositToStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_to_stable_swap_verify_account_privileges<'me, 'info>(
    accounts: DepositToStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_to_stable_swap_verify_writable_privileges(accounts)?;
    deposit_to_stable_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DepositToSerumSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositToSerumSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<DepositToSerumSwapAccounts<'_, '_>> for DepositToSerumSwapKeys {
    fn from(accounts: DepositToSerumSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DepositToSerumSwapKeys>
for [AccountMeta; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositToSerumSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN]> for DepositToSerumSwapKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_token_base: pubkeys[2],
            user_token_quote: pubkeys[3],
            liquidity_provider: pubkeys[4],
            token_base: pubkeys[5],
            token_quote: pubkeys[6],
            serum_market: pubkeys[7],
            serum_bids: pubkeys[8],
            serum_asks: pubkeys[9],
            user_authority: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<DepositToSerumSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositToSerumSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN]>
for DepositToSerumSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_token_base: &arr[2],
            user_token_quote: &arr[3],
            liquidity_provider: &arr[4],
            token_base: &arr[5],
            token_quote: &arr[6],
            serum_market: &arr[7],
            serum_bids: &arr[8],
            serum_asks: &arr[9],
            user_authority: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const DEPOSIT_TO_SERUM_SWAP_IX_DISCM: [u8; 8usize] = [
    115, 76, 233, 218, 220, 188, 90, 121,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositToSerumSwapIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
    pub min_base_share: u64,
    pub min_quote_share: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositToSerumSwapIxData(pub DepositToSerumSwapIxArgs);
impl From<DepositToSerumSwapIxArgs> for DepositToSerumSwapIxData {
    fn from(args: DepositToSerumSwapIxArgs) -> Self {
        Self(args)
    }
}
impl DepositToSerumSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_TO_SERUM_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositToSerumSwapIxArgs {
                base_amount,
                quote_amount,
                min_base_share,
                min_quote_share,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_TO_SERUM_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_quote_share, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_to_serum_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositToSerumSwapKeys,
    args: DepositToSerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_TO_SERUM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositToSerumSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_to_serum_swap_ix(
    keys: DepositToSerumSwapKeys,
    args: DepositToSerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    deposit_to_serum_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn deposit_to_serum_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositToSerumSwapAccounts<'_, '_>,
    args: DepositToSerumSwapIxArgs,
) -> ProgramResult {
    let keys: DepositToSerumSwapKeys = accounts.into();
    let ix = deposit_to_serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_to_serum_swap_invoke(
    accounts: DepositToSerumSwapAccounts<'_, '_>,
    args: DepositToSerumSwapIxArgs,
) -> ProgramResult {
    deposit_to_serum_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn deposit_to_serum_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositToSerumSwapAccounts<'_, '_>,
    args: DepositToSerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositToSerumSwapKeys = accounts.into();
    let ix = deposit_to_serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_to_serum_swap_invoke_signed(
    accounts: DepositToSerumSwapAccounts<'_, '_>,
    args: DepositToSerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_to_serum_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_to_serum_swap_verify_account_keys(
    accounts: DepositToSerumSwapAccounts<'_, '_>,
    keys: DepositToSerumSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.serum_market.key, keys.serum_market),
        (*accounts.serum_bids.key, keys.serum_bids),
        (*accounts.serum_asks.key, keys.serum_asks),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_to_serum_swap_verify_writable_privileges<'me, 'info>(
    accounts: DepositToSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_to_serum_swap_verify_signer_privileges<'me, 'info>(
    accounts: DepositToSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_to_serum_swap_verify_account_privileges<'me, 'info>(
    accounts: DepositToSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_to_serum_swap_verify_writable_privileges(accounts)?;
    deposit_to_serum_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromNormalSwapAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub admin_fee_token_base: &'me AccountInfo<'info>,
    pub admin_fee_token_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromNormalSwapKeys {
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub admin_fee_token_base: Pubkey,
    pub admin_fee_token_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFromNormalSwapAccounts<'_, '_>> for WithdrawFromNormalSwapKeys {
    fn from(accounts: WithdrawFromNormalSwapAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            admin_fee_token_base: *accounts.admin_fee_token_base.key,
            admin_fee_token_quote: *accounts.admin_fee_token_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFromNormalSwapKeys>
for [AccountMeta; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromNormalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN]>
for WithdrawFromNormalSwapKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            user_token_base: pubkeys[1],
            user_token_quote: pubkeys[2],
            liquidity_provider: pubkeys[3],
            token_base: pubkeys[4],
            token_quote: pubkeys[5],
            pyth_price_base: pubkeys[6],
            pyth_price_quote: pubkeys[7],
            admin_fee_token_base: pubkeys[8],
            admin_fee_token_quote: pubkeys[9],
            user_authority: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawFromNormalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromNormalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.admin_fee_token_base.clone(),
            accounts.admin_fee_token_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN]>
for WithdrawFromNormalSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_info: &arr[0],
            user_token_base: &arr[1],
            user_token_quote: &arr[2],
            liquidity_provider: &arr[3],
            token_base: &arr[4],
            token_quote: &arr[5],
            pyth_price_base: &arr[6],
            pyth_price_quote: &arr[7],
            admin_fee_token_base: &arr[8],
            admin_fee_token_quote: &arr[9],
            user_authority: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const WITHDRAW_FROM_NORMAL_SWAP_IX_DISCM: [u8; 8usize] = [
    199, 192, 194, 134, 151, 127, 234, 120,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromNormalSwapIxArgs {
    pub base_share: u64,
    pub quote_share: u64,
    pub min_base_amount: u64,
    pub min_quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromNormalSwapIxData(pub WithdrawFromNormalSwapIxArgs);
impl From<WithdrawFromNormalSwapIxArgs> for WithdrawFromNormalSwapIxData {
    fn from(args: WithdrawFromNormalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromNormalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_NORMAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawFromNormalSwapIxArgs {
                base_share,
                quote_share,
                min_base_amount,
                min_quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_NORMAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_normal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromNormalSwapKeys,
    args: WithdrawFromNormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_NORMAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromNormalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_normal_swap_ix(
    keys: WithdrawFromNormalSwapKeys,
    args: WithdrawFromNormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_normal_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_normal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromNormalSwapAccounts<'_, '_>,
    args: WithdrawFromNormalSwapIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromNormalSwapKeys = accounts.into();
    let ix = withdraw_from_normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_normal_swap_invoke(
    accounts: WithdrawFromNormalSwapAccounts<'_, '_>,
    args: WithdrawFromNormalSwapIxArgs,
) -> ProgramResult {
    withdraw_from_normal_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_normal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromNormalSwapAccounts<'_, '_>,
    args: WithdrawFromNormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromNormalSwapKeys = accounts.into();
    let ix = withdraw_from_normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_normal_swap_invoke_signed(
    accounts: WithdrawFromNormalSwapAccounts<'_, '_>,
    args: WithdrawFromNormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_normal_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_normal_swap_verify_account_keys(
    accounts: WithdrawFromNormalSwapAccounts<'_, '_>,
    keys: WithdrawFromNormalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.admin_fee_token_base.key, keys.admin_fee_token_base),
        (*accounts.admin_fee_token_quote.key, keys.admin_fee_token_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_normal_swap_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
        accounts.admin_fee_token_base,
        accounts.admin_fee_token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_normal_swap_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_normal_swap_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromNormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_normal_swap_verify_writable_privileges(accounts)?;
    withdraw_from_normal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromStableSwapAccounts<'me, 'info> {
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub admin_fee_token_base: &'me AccountInfo<'info>,
    pub admin_fee_token_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromStableSwapKeys {
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub admin_fee_token_base: Pubkey,
    pub admin_fee_token_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFromStableSwapAccounts<'_, '_>> for WithdrawFromStableSwapKeys {
    fn from(accounts: WithdrawFromStableSwapAccounts) -> Self {
        Self {
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            admin_fee_token_base: *accounts.admin_fee_token_base.key,
            admin_fee_token_quote: *accounts.admin_fee_token_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFromStableSwapKeys>
for [AccountMeta; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromStableSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN]>
for WithdrawFromStableSwapKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_info: pubkeys[0],
            user_token_base: pubkeys[1],
            user_token_quote: pubkeys[2],
            liquidity_provider: pubkeys[3],
            token_base: pubkeys[4],
            token_quote: pubkeys[5],
            pyth_price_base: pubkeys[6],
            pyth_price_quote: pubkeys[7],
            admin_fee_token_base: pubkeys[8],
            admin_fee_token_quote: pubkeys[9],
            user_authority: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawFromStableSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromStableSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.admin_fee_token_base.clone(),
            accounts.admin_fee_token_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN]>
for WithdrawFromStableSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_info: &arr[0],
            user_token_base: &arr[1],
            user_token_quote: &arr[2],
            liquidity_provider: &arr[3],
            token_base: &arr[4],
            token_quote: &arr[5],
            pyth_price_base: &arr[6],
            pyth_price_quote: &arr[7],
            admin_fee_token_base: &arr[8],
            admin_fee_token_quote: &arr[9],
            user_authority: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const WITHDRAW_FROM_STABLE_SWAP_IX_DISCM: [u8; 8usize] = [
    136, 158, 171, 145, 82, 167, 195, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromStableSwapIxArgs {
    pub base_share: u64,
    pub quote_share: u64,
    pub min_base_amount: u64,
    pub min_quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromStableSwapIxData(pub WithdrawFromStableSwapIxArgs);
impl From<WithdrawFromStableSwapIxArgs> for WithdrawFromStableSwapIxData {
    fn from(args: WithdrawFromStableSwapIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromStableSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_STABLE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawFromStableSwapIxArgs {
                base_share,
                quote_share,
                min_base_amount,
                min_quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_STABLE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_stable_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromStableSwapKeys,
    args: WithdrawFromStableSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_STABLE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromStableSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_stable_swap_ix(
    keys: WithdrawFromStableSwapKeys,
    args: WithdrawFromStableSwapIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_stable_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_stable_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromStableSwapAccounts<'_, '_>,
    args: WithdrawFromStableSwapIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromStableSwapKeys = accounts.into();
    let ix = withdraw_from_stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_stable_swap_invoke(
    accounts: WithdrawFromStableSwapAccounts<'_, '_>,
    args: WithdrawFromStableSwapIxArgs,
) -> ProgramResult {
    withdraw_from_stable_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_stable_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromStableSwapAccounts<'_, '_>,
    args: WithdrawFromStableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromStableSwapKeys = accounts.into();
    let ix = withdraw_from_stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_stable_swap_invoke_signed(
    accounts: WithdrawFromStableSwapAccounts<'_, '_>,
    args: WithdrawFromStableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_stable_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_stable_swap_verify_account_keys(
    accounts: WithdrawFromStableSwapAccounts<'_, '_>,
    keys: WithdrawFromStableSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.admin_fee_token_base.key, keys.admin_fee_token_base),
        (*accounts.admin_fee_token_quote.key, keys.admin_fee_token_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_stable_swap_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
        accounts.admin_fee_token_base,
        accounts.admin_fee_token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_stable_swap_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_stable_swap_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_stable_swap_verify_writable_privileges(accounts)?;
    withdraw_from_stable_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromSerumSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_token_base: &'me AccountInfo<'info>,
    pub user_token_quote: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub token_base: &'me AccountInfo<'info>,
    pub token_quote: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub admin_fee_token_base: &'me AccountInfo<'info>,
    pub admin_fee_token_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromSerumSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_token_base: Pubkey,
    pub user_token_quote: Pubkey,
    pub liquidity_provider: Pubkey,
    pub token_base: Pubkey,
    pub token_quote: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub admin_fee_token_base: Pubkey,
    pub admin_fee_token_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawFromSerumSwapAccounts<'_, '_>> for WithdrawFromSerumSwapKeys {
    fn from(accounts: WithdrawFromSerumSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_token_base: *accounts.user_token_base.key,
            user_token_quote: *accounts.user_token_quote.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            token_base: *accounts.token_base.key,
            token_quote: *accounts.token_quote.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            admin_fee_token_base: *accounts.admin_fee_token_base.key,
            admin_fee_token_quote: *accounts.admin_fee_token_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawFromSerumSwapKeys>
for [AccountMeta; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromSerumSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_fee_token_quote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN]>
for WithdrawFromSerumSwapKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_token_base: pubkeys[2],
            user_token_quote: pubkeys[3],
            liquidity_provider: pubkeys[4],
            token_base: pubkeys[5],
            token_quote: pubkeys[6],
            serum_market: pubkeys[7],
            serum_bids: pubkeys[8],
            serum_asks: pubkeys[9],
            admin_fee_token_base: pubkeys[10],
            admin_fee_token_quote: pubkeys[11],
            user_authority: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<WithdrawFromSerumSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromSerumSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_token_base.clone(),
            accounts.user_token_quote.clone(),
            accounts.liquidity_provider.clone(),
            accounts.token_base.clone(),
            accounts.token_quote.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.admin_fee_token_base.clone(),
            accounts.admin_fee_token_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN]>
for WithdrawFromSerumSwapAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_token_base: &arr[2],
            user_token_quote: &arr[3],
            liquidity_provider: &arr[4],
            token_base: &arr[5],
            token_quote: &arr[6],
            serum_market: &arr[7],
            serum_bids: &arr[8],
            serum_asks: &arr[9],
            admin_fee_token_base: &arr[10],
            admin_fee_token_quote: &arr[11],
            user_authority: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const WITHDRAW_FROM_SERUM_SWAP_IX_DISCM: [u8; 8usize] = [
    151, 148, 122, 209, 27, 7, 154, 99,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromSerumSwapIxArgs {
    pub base_share: u64,
    pub quote_share: u64,
    pub min_base_amount: u64,
    pub min_quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromSerumSwapIxData(pub WithdrawFromSerumSwapIxArgs);
impl From<WithdrawFromSerumSwapIxArgs> for WithdrawFromSerumSwapIxData {
    fn from(args: WithdrawFromSerumSwapIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromSerumSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_SERUM_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawFromSerumSwapIxArgs {
                base_share,
                quote_share,
                min_base_amount,
                min_quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_SERUM_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_serum_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromSerumSwapKeys,
    args: WithdrawFromSerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_SERUM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromSerumSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_serum_swap_ix(
    keys: WithdrawFromSerumSwapKeys,
    args: WithdrawFromSerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_serum_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_serum_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromSerumSwapAccounts<'_, '_>,
    args: WithdrawFromSerumSwapIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromSerumSwapKeys = accounts.into();
    let ix = withdraw_from_serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_serum_swap_invoke(
    accounts: WithdrawFromSerumSwapAccounts<'_, '_>,
    args: WithdrawFromSerumSwapIxArgs,
) -> ProgramResult {
    withdraw_from_serum_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_serum_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromSerumSwapAccounts<'_, '_>,
    args: WithdrawFromSerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromSerumSwapKeys = accounts.into();
    let ix = withdraw_from_serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_serum_swap_invoke_signed(
    accounts: WithdrawFromSerumSwapAccounts<'_, '_>,
    args: WithdrawFromSerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_serum_swap_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_serum_swap_verify_account_keys(
    accounts: WithdrawFromSerumSwapAccounts<'_, '_>,
    keys: WithdrawFromSerumSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_token_base.key, keys.user_token_base),
        (*accounts.user_token_quote.key, keys.user_token_quote),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.token_base.key, keys.token_base),
        (*accounts.token_quote.key, keys.token_quote),
        (*accounts.serum_market.key, keys.serum_market),
        (*accounts.serum_bids.key, keys.serum_bids),
        (*accounts.serum_asks.key, keys.serum_asks),
        (*accounts.admin_fee_token_base.key, keys.admin_fee_token_base),
        (*accounts.admin_fee_token_quote.key, keys.admin_fee_token_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_serum_swap_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_token_base,
        accounts.user_token_quote,
        accounts.liquidity_provider,
        accounts.token_base,
        accounts.token_quote,
        accounts.admin_fee_token_base,
        accounts.admin_fee_token_quote,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_serum_swap_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_serum_swap_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromSerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_serum_swap_verify_writable_privileges(accounts)?;
    withdraw_from_serum_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NORMAL_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct NormalSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<NormalSwapAccounts<'_, '_>> for NormalSwapKeys {
    fn from(accounts: NormalSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<NormalSwapKeys> for [AccountMeta; NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: NormalSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; NORMAL_SWAP_IX_ACCOUNTS_LEN]> for NormalSwapKeys {
    fn from(pubkeys: [Pubkey; NORMAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            admin_destination_token: pubkeys[7],
            pyth_price_base: pubkeys[8],
            pyth_price_quote: pubkeys[9],
            user_authority: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<NormalSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; NORMAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: NormalSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NORMAL_SWAP_IX_ACCOUNTS_LEN]>
for NormalSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NORMAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            admin_destination_token: &arr[7],
            pyth_price_base: &arr[8],
            pyth_price_quote: &arr[9],
            user_authority: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const NORMAL_SWAP_IX_DISCM: [u8; 8usize] = [43, 79, 212, 62, 177, 177, 234, 155];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NormalSwapIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NormalSwapIxData(pub NormalSwapIxArgs);
impl From<NormalSwapIxArgs> for NormalSwapIxData {
    fn from(args: NormalSwapIxArgs) -> Self {
        Self(args)
    }
}
impl NormalSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NORMAL_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(NormalSwapIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NORMAL_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn normal_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: NormalSwapKeys,
    args: NormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NORMAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: NormalSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn normal_swap_ix(
    keys: NormalSwapKeys,
    args: NormalSwapIxArgs,
) -> std::io::Result<Instruction> {
    normal_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn normal_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NormalSwapAccounts<'_, '_>,
    args: NormalSwapIxArgs,
) -> ProgramResult {
    let keys: NormalSwapKeys = accounts.into();
    let ix = normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn normal_swap_invoke(
    accounts: NormalSwapAccounts<'_, '_>,
    args: NormalSwapIxArgs,
) -> ProgramResult {
    normal_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn normal_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NormalSwapAccounts<'_, '_>,
    args: NormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NormalSwapKeys = accounts.into();
    let ix = normal_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn normal_swap_invoke_signed(
    accounts: NormalSwapAccounts<'_, '_>,
    args: NormalSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    normal_swap_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn normal_swap_verify_account_keys(
    accounts: NormalSwapAccounts<'_, '_>,
    keys: NormalSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn normal_swap_verify_writable_privileges<'me, 'info>(
    accounts: NormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn normal_swap_verify_signer_privileges<'me, 'info>(
    accounts: NormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn normal_swap_verify_account_privileges<'me, 'info>(
    accounts: NormalSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    normal_swap_verify_writable_privileges(accounts)?;
    normal_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct NormalSwapWithReferrerAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalSwapWithReferrerKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub referrer: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<NormalSwapWithReferrerAccounts<'_, '_>> for NormalSwapWithReferrerKeys {
    fn from(accounts: NormalSwapWithReferrerAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            referrer: *accounts.referrer.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<NormalSwapWithReferrerKeys>
for [AccountMeta; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(keys: NormalSwapWithReferrerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for NormalSwapWithReferrerKeys {
    fn from(pubkeys: [Pubkey; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            referrer: pubkeys[7],
            admin_destination_token: pubkeys[8],
            pyth_price_base: pubkeys[9],
            pyth_price_quote: pubkeys[10],
            user_authority: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<NormalSwapWithReferrerAccounts<'_, 'info>>
for [AccountInfo<'info>; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(accounts: NormalSwapWithReferrerAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.referrer.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for NormalSwapWithReferrerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            referrer: &arr[7],
            admin_destination_token: &arr[8],
            pyth_price_base: &arr[9],
            pyth_price_quote: &arr[10],
            user_authority: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const NORMAL_SWAP_WITH_REFERRER_IX_DISCM: [u8; 8usize] = [
    35, 17, 129, 10, 167, 164, 24, 16,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NormalSwapWithReferrerIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NormalSwapWithReferrerIxData(pub NormalSwapWithReferrerIxArgs);
impl From<NormalSwapWithReferrerIxArgs> for NormalSwapWithReferrerIxData {
    fn from(args: NormalSwapWithReferrerIxArgs) -> Self {
        Self(args)
    }
}
impl NormalSwapWithReferrerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NORMAL_SWAP_WITH_REFERRER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(NormalSwapWithReferrerIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NORMAL_SWAP_WITH_REFERRER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn normal_swap_with_referrer_ix_with_program_id(
    program_id: Pubkey,
    keys: NormalSwapWithReferrerKeys,
    args: NormalSwapWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NORMAL_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] = keys.into();
    let data: NormalSwapWithReferrerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn normal_swap_with_referrer_ix(
    keys: NormalSwapWithReferrerKeys,
    args: NormalSwapWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    normal_swap_with_referrer_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn normal_swap_with_referrer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NormalSwapWithReferrerAccounts<'_, '_>,
    args: NormalSwapWithReferrerIxArgs,
) -> ProgramResult {
    let keys: NormalSwapWithReferrerKeys = accounts.into();
    let ix = normal_swap_with_referrer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn normal_swap_with_referrer_invoke(
    accounts: NormalSwapWithReferrerAccounts<'_, '_>,
    args: NormalSwapWithReferrerIxArgs,
) -> ProgramResult {
    normal_swap_with_referrer_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn normal_swap_with_referrer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NormalSwapWithReferrerAccounts<'_, '_>,
    args: NormalSwapWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NormalSwapWithReferrerKeys = accounts.into();
    let ix = normal_swap_with_referrer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn normal_swap_with_referrer_invoke_signed(
    accounts: NormalSwapWithReferrerAccounts<'_, '_>,
    args: NormalSwapWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    normal_swap_with_referrer_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn normal_swap_with_referrer_verify_account_keys(
    accounts: NormalSwapWithReferrerAccounts<'_, '_>,
    keys: NormalSwapWithReferrerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn normal_swap_with_referrer_verify_writable_privileges<'me, 'info>(
    accounts: NormalSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.referrer,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn normal_swap_with_referrer_verify_signer_privileges<'me, 'info>(
    accounts: NormalSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn normal_swap_with_referrer_verify_account_privileges<'me, 'info>(
    accounts: NormalSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    normal_swap_with_referrer_verify_writable_privileges(accounts)?;
    normal_swap_with_referrer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct NormalSwapWithRebateAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub rebate_token: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalSwapWithRebateKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub rebate_token: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<NormalSwapWithRebateAccounts<'_, '_>> for NormalSwapWithRebateKeys {
    fn from(accounts: NormalSwapWithRebateAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            rebate_token: *accounts.rebate_token.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<NormalSwapWithRebateKeys>
for [AccountMeta; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN] {
    fn from(keys: NormalSwapWithRebateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebate_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN]>
for NormalSwapWithRebateKeys {
    fn from(pubkeys: [Pubkey; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            rebate_token: pubkeys[7],
            admin_destination_token: pubkeys[8],
            pyth_price_base: pubkeys[9],
            pyth_price_quote: pubkeys[10],
            user_authority: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<NormalSwapWithRebateAccounts<'_, 'info>>
for [AccountInfo<'info>; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: NormalSwapWithRebateAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.rebate_token.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN]>
for NormalSwapWithRebateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            rebate_token: &arr[7],
            admin_destination_token: &arr[8],
            pyth_price_base: &arr[9],
            pyth_price_quote: &arr[10],
            user_authority: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const NORMAL_SWAP_WITH_REBATE_IX_DISCM: [u8; 8usize] = [
    80, 34, 18, 92, 201, 97, 168, 68,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NormalSwapWithRebateIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NormalSwapWithRebateIxData(pub NormalSwapWithRebateIxArgs);
impl From<NormalSwapWithRebateIxArgs> for NormalSwapWithRebateIxData {
    fn from(args: NormalSwapWithRebateIxArgs) -> Self {
        Self(args)
    }
}
impl NormalSwapWithRebateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NORMAL_SWAP_WITH_REBATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(NormalSwapWithRebateIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NORMAL_SWAP_WITH_REBATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn normal_swap_with_rebate_ix_with_program_id(
    program_id: Pubkey,
    keys: NormalSwapWithRebateKeys,
    args: NormalSwapWithRebateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NORMAL_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: NormalSwapWithRebateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn normal_swap_with_rebate_ix(
    keys: NormalSwapWithRebateKeys,
    args: NormalSwapWithRebateIxArgs,
) -> std::io::Result<Instruction> {
    normal_swap_with_rebate_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn normal_swap_with_rebate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NormalSwapWithRebateAccounts<'_, '_>,
    args: NormalSwapWithRebateIxArgs,
) -> ProgramResult {
    let keys: NormalSwapWithRebateKeys = accounts.into();
    let ix = normal_swap_with_rebate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn normal_swap_with_rebate_invoke(
    accounts: NormalSwapWithRebateAccounts<'_, '_>,
    args: NormalSwapWithRebateIxArgs,
) -> ProgramResult {
    normal_swap_with_rebate_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn normal_swap_with_rebate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NormalSwapWithRebateAccounts<'_, '_>,
    args: NormalSwapWithRebateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NormalSwapWithRebateKeys = accounts.into();
    let ix = normal_swap_with_rebate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn normal_swap_with_rebate_invoke_signed(
    accounts: NormalSwapWithRebateAccounts<'_, '_>,
    args: NormalSwapWithRebateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    normal_swap_with_rebate_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn normal_swap_with_rebate_verify_account_keys(
    accounts: NormalSwapWithRebateAccounts<'_, '_>,
    keys: NormalSwapWithRebateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.rebate_token.key, keys.rebate_token),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn normal_swap_with_rebate_verify_writable_privileges<'me, 'info>(
    accounts: NormalSwapWithRebateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.rebate_token,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn normal_swap_with_rebate_verify_signer_privileges<'me, 'info>(
    accounts: NormalSwapWithRebateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn normal_swap_with_rebate_verify_account_privileges<'me, 'info>(
    accounts: NormalSwapWithRebateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    normal_swap_with_rebate_verify_writable_privileges(accounts)?;
    normal_swap_with_rebate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STABLE_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct StableSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StableSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<StableSwapAccounts<'_, '_>> for StableSwapKeys {
    fn from(accounts: StableSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StableSwapKeys> for [AccountMeta; STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: StableSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; STABLE_SWAP_IX_ACCOUNTS_LEN]> for StableSwapKeys {
    fn from(pubkeys: [Pubkey; STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            admin_destination_token: pubkeys[7],
            pyth_price_base: pubkeys[8],
            pyth_price_quote: pubkeys[9],
            user_authority: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<StableSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: StableSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STABLE_SWAP_IX_ACCOUNTS_LEN]>
for StableSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            admin_destination_token: &arr[7],
            pyth_price_base: &arr[8],
            pyth_price_quote: &arr[9],
            user_authority: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const STABLE_SWAP_IX_DISCM: [u8; 8usize] = [77, 218, 214, 182, 145, 78, 196, 142];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StableSwapIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StableSwapIxData(pub StableSwapIxArgs);
impl From<StableSwapIxArgs> for StableSwapIxData {
    fn from(args: StableSwapIxArgs) -> Self {
        Self(args)
    }
}
impl StableSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STABLE_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(StableSwapIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STABLE_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stable_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: StableSwapKeys,
    args: StableSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STABLE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: StableSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stable_swap_ix(
    keys: StableSwapKeys,
    args: StableSwapIxArgs,
) -> std::io::Result<Instruction> {
    stable_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn stable_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StableSwapAccounts<'_, '_>,
    args: StableSwapIxArgs,
) -> ProgramResult {
    let keys: StableSwapKeys = accounts.into();
    let ix = stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stable_swap_invoke(
    accounts: StableSwapAccounts<'_, '_>,
    args: StableSwapIxArgs,
) -> ProgramResult {
    stable_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn stable_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StableSwapAccounts<'_, '_>,
    args: StableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StableSwapKeys = accounts.into();
    let ix = stable_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stable_swap_invoke_signed(
    accounts: StableSwapAccounts<'_, '_>,
    args: StableSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stable_swap_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn stable_swap_verify_account_keys(
    accounts: StableSwapAccounts<'_, '_>,
    keys: StableSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stable_swap_verify_writable_privileges<'me, 'info>(
    accounts: StableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stable_swap_verify_signer_privileges<'me, 'info>(
    accounts: StableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stable_swap_verify_account_privileges<'me, 'info>(
    accounts: StableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stable_swap_verify_writable_privileges(accounts)?;
    stable_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct StableSwapWithReferrerAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StableSwapWithReferrerKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub referrer: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<StableSwapWithReferrerAccounts<'_, '_>> for StableSwapWithReferrerKeys {
    fn from(accounts: StableSwapWithReferrerAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            referrer: *accounts.referrer.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StableSwapWithReferrerKeys>
for [AccountMeta; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(keys: StableSwapWithReferrerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for StableSwapWithReferrerKeys {
    fn from(pubkeys: [Pubkey; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            referrer: pubkeys[7],
            admin_destination_token: pubkeys[8],
            pyth_price_base: pubkeys[9],
            pyth_price_quote: pubkeys[10],
            user_authority: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<StableSwapWithReferrerAccounts<'_, 'info>>
for [AccountInfo<'info>; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(accounts: StableSwapWithReferrerAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.referrer.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for StableSwapWithReferrerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            referrer: &arr[7],
            admin_destination_token: &arr[8],
            pyth_price_base: &arr[9],
            pyth_price_quote: &arr[10],
            user_authority: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const STABLE_SWAP_WITH_REFERRER_IX_DISCM: [u8; 8usize] = [
    172, 101, 4, 110, 22, 134, 242, 209,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StableSwapWithReferrerIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StableSwapWithReferrerIxData(pub StableSwapWithReferrerIxArgs);
impl From<StableSwapWithReferrerIxArgs> for StableSwapWithReferrerIxData {
    fn from(args: StableSwapWithReferrerIxArgs) -> Self {
        Self(args)
    }
}
impl StableSwapWithReferrerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STABLE_SWAP_WITH_REFERRER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(StableSwapWithReferrerIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STABLE_SWAP_WITH_REFERRER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stable_swap_with_referrer_ix_with_program_id(
    program_id: Pubkey,
    keys: StableSwapWithReferrerKeys,
    args: StableSwapWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STABLE_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] = keys.into();
    let data: StableSwapWithReferrerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stable_swap_with_referrer_ix(
    keys: StableSwapWithReferrerKeys,
    args: StableSwapWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    stable_swap_with_referrer_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn stable_swap_with_referrer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StableSwapWithReferrerAccounts<'_, '_>,
    args: StableSwapWithReferrerIxArgs,
) -> ProgramResult {
    let keys: StableSwapWithReferrerKeys = accounts.into();
    let ix = stable_swap_with_referrer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stable_swap_with_referrer_invoke(
    accounts: StableSwapWithReferrerAccounts<'_, '_>,
    args: StableSwapWithReferrerIxArgs,
) -> ProgramResult {
    stable_swap_with_referrer_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn stable_swap_with_referrer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StableSwapWithReferrerAccounts<'_, '_>,
    args: StableSwapWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StableSwapWithReferrerKeys = accounts.into();
    let ix = stable_swap_with_referrer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stable_swap_with_referrer_invoke_signed(
    accounts: StableSwapWithReferrerAccounts<'_, '_>,
    args: StableSwapWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stable_swap_with_referrer_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn stable_swap_with_referrer_verify_account_keys(
    accounts: StableSwapWithReferrerAccounts<'_, '_>,
    keys: StableSwapWithReferrerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stable_swap_with_referrer_verify_writable_privileges<'me, 'info>(
    accounts: StableSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.referrer,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stable_swap_with_referrer_verify_signer_privileges<'me, 'info>(
    accounts: StableSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stable_swap_with_referrer_verify_account_privileges<'me, 'info>(
    accounts: StableSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stable_swap_with_referrer_verify_writable_privileges(accounts)?;
    stable_swap_with_referrer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct StableSwapWithRebateAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub rebate_token: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StableSwapWithRebateKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub rebate_token: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<StableSwapWithRebateAccounts<'_, '_>> for StableSwapWithRebateKeys {
    fn from(accounts: StableSwapWithRebateAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            rebate_token: *accounts.rebate_token.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StableSwapWithRebateKeys>
for [AccountMeta; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN] {
    fn from(keys: StableSwapWithRebateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rebate_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN]>
for StableSwapWithRebateKeys {
    fn from(pubkeys: [Pubkey; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            rebate_token: pubkeys[7],
            admin_destination_token: pubkeys[8],
            pyth_price_base: pubkeys[9],
            pyth_price_quote: pubkeys[10],
            user_authority: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<StableSwapWithRebateAccounts<'_, 'info>>
for [AccountInfo<'info>; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: StableSwapWithRebateAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.rebate_token.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN]>
for StableSwapWithRebateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            rebate_token: &arr[7],
            admin_destination_token: &arr[8],
            pyth_price_base: &arr[9],
            pyth_price_quote: &arr[10],
            user_authority: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const STABLE_SWAP_WITH_REBATE_IX_DISCM: [u8; 8usize] = [
    167, 249, 225, 86, 192, 159, 185, 54,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StableSwapWithRebateIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StableSwapWithRebateIxData(pub StableSwapWithRebateIxArgs);
impl From<StableSwapWithRebateIxArgs> for StableSwapWithRebateIxData {
    fn from(args: StableSwapWithRebateIxArgs) -> Self {
        Self(args)
    }
}
impl StableSwapWithRebateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STABLE_SWAP_WITH_REBATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(StableSwapWithRebateIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STABLE_SWAP_WITH_REBATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stable_swap_with_rebate_ix_with_program_id(
    program_id: Pubkey,
    keys: StableSwapWithRebateKeys,
    args: StableSwapWithRebateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STABLE_SWAP_WITH_REBATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: StableSwapWithRebateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stable_swap_with_rebate_ix(
    keys: StableSwapWithRebateKeys,
    args: StableSwapWithRebateIxArgs,
) -> std::io::Result<Instruction> {
    stable_swap_with_rebate_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn stable_swap_with_rebate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StableSwapWithRebateAccounts<'_, '_>,
    args: StableSwapWithRebateIxArgs,
) -> ProgramResult {
    let keys: StableSwapWithRebateKeys = accounts.into();
    let ix = stable_swap_with_rebate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stable_swap_with_rebate_invoke(
    accounts: StableSwapWithRebateAccounts<'_, '_>,
    args: StableSwapWithRebateIxArgs,
) -> ProgramResult {
    stable_swap_with_rebate_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn stable_swap_with_rebate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StableSwapWithRebateAccounts<'_, '_>,
    args: StableSwapWithRebateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StableSwapWithRebateKeys = accounts.into();
    let ix = stable_swap_with_rebate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stable_swap_with_rebate_invoke_signed(
    accounts: StableSwapWithRebateAccounts<'_, '_>,
    args: StableSwapWithRebateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stable_swap_with_rebate_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn stable_swap_with_rebate_verify_account_keys(
    accounts: StableSwapWithRebateAccounts<'_, '_>,
    keys: StableSwapWithRebateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.rebate_token.key, keys.rebate_token),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stable_swap_with_rebate_verify_writable_privileges<'me, 'info>(
    accounts: StableSwapWithRebateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.rebate_token,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stable_swap_with_rebate_verify_signer_privileges<'me, 'info>(
    accounts: StableSwapWithRebateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stable_swap_with_rebate_verify_account_privileges<'me, 'info>(
    accounts: StableSwapWithRebateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stable_swap_with_rebate_verify_writable_privileges(accounts)?;
    stable_swap_with_rebate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SERUM_SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SerumSwapAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SerumSwapKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub admin_destination_token: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<SerumSwapAccounts<'_, '_>> for SerumSwapKeys {
    fn from(accounts: SerumSwapAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SerumSwapKeys> for [AccountMeta; SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SerumSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; SERUM_SWAP_IX_ACCOUNTS_LEN]> for SerumSwapKeys {
    fn from(pubkeys: [Pubkey; SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            admin_destination_token: pubkeys[7],
            serum_market: pubkeys[8],
            serum_bids: pubkeys[9],
            serum_asks: pubkeys[10],
            user_authority: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<SerumSwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SerumSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.admin_destination_token.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SERUM_SWAP_IX_ACCOUNTS_LEN]>
for SerumSwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            admin_destination_token: &arr[7],
            serum_market: &arr[8],
            serum_bids: &arr[9],
            serum_asks: &arr[10],
            user_authority: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const SERUM_SWAP_IX_DISCM: [u8; 8usize] = [88, 183, 70, 249, 214, 118, 82, 210];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerumSwapIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SerumSwapIxData(pub SerumSwapIxArgs);
impl From<SerumSwapIxArgs> for SerumSwapIxData {
    fn from(args: SerumSwapIxArgs) -> Self {
        Self(args)
    }
}
impl SerumSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SERUM_SWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SerumSwapIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SERUM_SWAP_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn serum_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SerumSwapKeys,
    args: SerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SERUM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SerumSwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn serum_swap_ix(
    keys: SerumSwapKeys,
    args: SerumSwapIxArgs,
) -> std::io::Result<Instruction> {
    serum_swap_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn serum_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SerumSwapAccounts<'_, '_>,
    args: SerumSwapIxArgs,
) -> ProgramResult {
    let keys: SerumSwapKeys = accounts.into();
    let ix = serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn serum_swap_invoke(
    accounts: SerumSwapAccounts<'_, '_>,
    args: SerumSwapIxArgs,
) -> ProgramResult {
    serum_swap_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn serum_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SerumSwapAccounts<'_, '_>,
    args: SerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SerumSwapKeys = accounts.into();
    let ix = serum_swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn serum_swap_invoke_signed(
    accounts: SerumSwapAccounts<'_, '_>,
    args: SerumSwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    serum_swap_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn serum_swap_verify_account_keys(
    accounts: SerumSwapAccounts<'_, '_>,
    keys: SerumSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.serum_market.key, keys.serum_market),
        (*accounts.serum_bids.key, keys.serum_bids),
        (*accounts.serum_asks.key, keys.serum_asks),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn serum_swap_verify_writable_privileges<'me, 'info>(
    accounts: SerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn serum_swap_verify_signer_privileges<'me, 'info>(
    accounts: SerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn serum_swap_verify_account_privileges<'me, 'info>(
    accounts: SerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    serum_swap_verify_writable_privileges(accounts)?;
    serum_swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SerumSwapWithReferrerAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SerumSwapWithReferrerKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub referrer: Pubkey,
    pub admin_destination_token: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<SerumSwapWithReferrerAccounts<'_, '_>> for SerumSwapWithReferrerKeys {
    fn from(accounts: SerumSwapWithReferrerAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            referrer: *accounts.referrer.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SerumSwapWithReferrerKeys>
for [AccountMeta; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(keys: SerumSwapWithReferrerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: true,
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
impl From<[Pubkey; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for SerumSwapWithReferrerKeys {
    fn from(pubkeys: [Pubkey; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            swap_source_token: pubkeys[4],
            swap_destination_token: pubkeys[5],
            deltafi_user: pubkeys[6],
            referrer: pubkeys[7],
            admin_destination_token: pubkeys[8],
            serum_market: pubkeys[9],
            serum_bids: pubkeys[10],
            serum_asks: pubkeys[11],
            user_authority: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<SerumSwapWithReferrerAccounts<'_, 'info>>
for [AccountInfo<'info>; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SerumSwapWithReferrerAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.referrer.clone(),
            accounts.admin_destination_token.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for SerumSwapWithReferrerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            swap_source_token: &arr[4],
            swap_destination_token: &arr[5],
            deltafi_user: &arr[6],
            referrer: &arr[7],
            admin_destination_token: &arr[8],
            serum_market: &arr[9],
            serum_bids: &arr[10],
            serum_asks: &arr[11],
            user_authority: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const SERUM_SWAP_WITH_REFERRER_IX_DISCM: [u8; 8usize] = [
    23, 154, 51, 135, 128, 124, 155, 62,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerumSwapWithReferrerIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SerumSwapWithReferrerIxData(pub SerumSwapWithReferrerIxArgs);
impl From<SerumSwapWithReferrerIxArgs> for SerumSwapWithReferrerIxData {
    fn from(args: SerumSwapWithReferrerIxArgs) -> Self {
        Self(args)
    }
}
impl SerumSwapWithReferrerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SERUM_SWAP_WITH_REFERRER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SerumSwapWithReferrerIxArgs {
                amount_in,
                min_amount_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SERUM_SWAP_WITH_REFERRER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_amount_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn serum_swap_with_referrer_ix_with_program_id(
    program_id: Pubkey,
    keys: SerumSwapWithReferrerKeys,
    args: SerumSwapWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SERUM_SWAP_WITH_REFERRER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SerumSwapWithReferrerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn serum_swap_with_referrer_ix(
    keys: SerumSwapWithReferrerKeys,
    args: SerumSwapWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    serum_swap_with_referrer_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn serum_swap_with_referrer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SerumSwapWithReferrerAccounts<'_, '_>,
    args: SerumSwapWithReferrerIxArgs,
) -> ProgramResult {
    let keys: SerumSwapWithReferrerKeys = accounts.into();
    let ix = serum_swap_with_referrer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn serum_swap_with_referrer_invoke(
    accounts: SerumSwapWithReferrerAccounts<'_, '_>,
    args: SerumSwapWithReferrerIxArgs,
) -> ProgramResult {
    serum_swap_with_referrer_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn serum_swap_with_referrer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SerumSwapWithReferrerAccounts<'_, '_>,
    args: SerumSwapWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SerumSwapWithReferrerKeys = accounts.into();
    let ix = serum_swap_with_referrer_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn serum_swap_with_referrer_invoke_signed(
    accounts: SerumSwapWithReferrerAccounts<'_, '_>,
    args: SerumSwapWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    serum_swap_with_referrer_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn serum_swap_with_referrer_verify_account_keys(
    accounts: SerumSwapWithReferrerAccounts<'_, '_>,
    keys: SerumSwapWithReferrerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (*accounts.swap_destination_token.key, keys.swap_destination_token),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.admin_destination_token.key, keys.admin_destination_token),
        (*accounts.serum_market.key, keys.serum_market),
        (*accounts.serum_bids.key, keys.serum_bids),
        (*accounts.serum_asks.key, keys.serum_asks),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn serum_swap_with_referrer_verify_writable_privileges<'me, 'info>(
    accounts: SerumSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.referrer,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn serum_swap_with_referrer_verify_signer_privileges<'me, 'info>(
    accounts: SerumSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn serum_swap_with_referrer_verify_account_privileges<'me, 'info>(
    accounts: SerumSwapWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    serum_swap_with_referrer_verify_writable_privileges(accounts)?;
    serum_swap_with_referrer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FARM_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateFarmAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub farm_info: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFarmKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub farm_info: Pubkey,
    pub admin: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateFarmAccounts<'_, '_>> for CreateFarmKeys {
    fn from(accounts: CreateFarmAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            farm_info: *accounts.farm_info.key,
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateFarmKeys> for [AccountMeta; CREATE_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFarmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; CREATE_FARM_IX_ACCOUNTS_LEN]> for CreateFarmKeys {
    fn from(pubkeys: [Pubkey; CREATE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            farm_info: pubkeys[2],
            admin: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<CreateFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.farm_info.clone(),
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FARM_IX_ACCOUNTS_LEN]>
for CreateFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            farm_info: &arr[2],
            admin: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
pub const CREATE_FARM_IX_DISCM: [u8; 8usize] = [74, 59, 128, 160, 87, 174, 153, 194];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFarmIxArgs {
    pub bump: u8,
    pub seed: Pubkey,
    pub farm_config: FarmConfig,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFarmIxData(pub CreateFarmIxArgs);
impl From<CreateFarmIxArgs> for CreateFarmIxData {
    fn from(args: CreateFarmIxArgs) -> Self {
        Self(args)
    }
}
impl CreateFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_config = if reader.is_empty() {
            Default::default()
        } else {
            <FarmConfig>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateFarmIxArgs {
                bump,
                seed,
                farm_config,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FARM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.farm_config, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFarmKeys,
    args: CreateFarmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FARM_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateFarmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_farm_ix(
    keys: CreateFarmKeys,
    args: CreateFarmIxArgs,
) -> std::io::Result<Instruction> {
    create_farm_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
) -> ProgramResult {
    let keys: CreateFarmKeys = accounts.into();
    let ix = create_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_farm_invoke(
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
) -> ProgramResult {
    create_farm_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn create_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFarmKeys = accounts.into();
    let ix = create_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_farm_invoke_signed(
    accounts: CreateFarmAccounts<'_, '_>,
    args: CreateFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_farm_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_farm_verify_account_keys(
    accounts: CreateFarmAccounts<'_, '_>,
    keys: CreateFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.farm_info.key, keys.farm_info),
        (*accounts.admin.key, keys.admin),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_farm_verify_writable_privileges<'me, 'info>(
    accounts: CreateFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.farm_info, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_farm_verify_signer_privileges<'me, 'info>(
    accounts: CreateFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_farm_verify_account_privileges<'me, 'info>(
    accounts: CreateFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_farm_verify_writable_privileges(accounts)?;
    create_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DepositToFarmAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub farm_info: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub farm_user: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositToFarmKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub farm_info: Pubkey,
    pub liquidity_provider: Pubkey,
    pub farm_user: Pubkey,
    pub owner: Pubkey,
}
impl From<DepositToFarmAccounts<'_, '_>> for DepositToFarmKeys {
    fn from(accounts: DepositToFarmAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            farm_info: *accounts.farm_info.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            farm_user: *accounts.farm_user.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<DepositToFarmKeys> for [AccountMeta; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositToFarmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN]> for DepositToFarmKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            farm_info: pubkeys[2],
            liquidity_provider: pubkeys[3],
            farm_user: pubkeys[4],
            owner: pubkeys[5],
        }
    }
}
impl<'info> From<DepositToFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositToFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.farm_info.clone(),
            accounts.liquidity_provider.clone(),
            accounts.farm_user.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN]>
for DepositToFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            farm_info: &arr[2],
            liquidity_provider: &arr[3],
            farm_user: &arr[4],
            owner: &arr[5],
        }
    }
}
pub const DEPOSIT_TO_FARM_IX_DISCM: [u8; 8usize] = [75, 183, 114, 14, 145, 19, 249, 167];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositToFarmIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositToFarmIxData(pub DepositToFarmIxArgs);
impl From<DepositToFarmIxArgs> for DepositToFarmIxData {
    fn from(args: DepositToFarmIxArgs) -> Self {
        Self(args)
    }
}
impl DepositToFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_TO_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DepositToFarmIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_TO_FARM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_to_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositToFarmKeys,
    args: DepositToFarmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_TO_FARM_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositToFarmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_to_farm_ix(
    keys: DepositToFarmKeys,
    args: DepositToFarmIxArgs,
) -> std::io::Result<Instruction> {
    deposit_to_farm_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn deposit_to_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositToFarmAccounts<'_, '_>,
    args: DepositToFarmIxArgs,
) -> ProgramResult {
    let keys: DepositToFarmKeys = accounts.into();
    let ix = deposit_to_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_to_farm_invoke(
    accounts: DepositToFarmAccounts<'_, '_>,
    args: DepositToFarmIxArgs,
) -> ProgramResult {
    deposit_to_farm_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn deposit_to_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositToFarmAccounts<'_, '_>,
    args: DepositToFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositToFarmKeys = accounts.into();
    let ix = deposit_to_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_to_farm_invoke_signed(
    accounts: DepositToFarmAccounts<'_, '_>,
    args: DepositToFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_to_farm_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_to_farm_verify_account_keys(
    accounts: DepositToFarmAccounts<'_, '_>,
    keys: DepositToFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.farm_info.key, keys.farm_info),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.farm_user.key, keys.farm_user),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_to_farm_verify_writable_privileges<'me, 'info>(
    accounts: DepositToFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_info,
        accounts.liquidity_provider,
        accounts.farm_user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_to_farm_verify_signer_privileges<'me, 'info>(
    accounts: DepositToFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_to_farm_verify_account_privileges<'me, 'info>(
    accounts: DepositToFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_to_farm_verify_writable_privileges(accounts)?;
    deposit_to_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFromFarmAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub farm_info: &'me AccountInfo<'info>,
    pub liquidity_provider: &'me AccountInfo<'info>,
    pub farm_user: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawFromFarmKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub farm_info: Pubkey,
    pub liquidity_provider: Pubkey,
    pub farm_user: Pubkey,
    pub owner: Pubkey,
}
impl From<WithdrawFromFarmAccounts<'_, '_>> for WithdrawFromFarmKeys {
    fn from(accounts: WithdrawFromFarmAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            farm_info: *accounts.farm_info.key,
            liquidity_provider: *accounts.liquidity_provider.key,
            farm_user: *accounts.farm_user.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<WithdrawFromFarmKeys> for [AccountMeta; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFromFarmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidity_provider,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN]> for WithdrawFromFarmKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            farm_info: pubkeys[2],
            liquidity_provider: pubkeys[3],
            farm_user: pubkeys[4],
            owner: pubkeys[5],
        }
    }
}
impl<'info> From<WithdrawFromFarmAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawFromFarmAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.farm_info.clone(),
            accounts.liquidity_provider.clone(),
            accounts.farm_user.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN]>
for WithdrawFromFarmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            farm_info: &arr[2],
            liquidity_provider: &arr[3],
            farm_user: &arr[4],
            owner: &arr[5],
        }
    }
}
pub const WITHDRAW_FROM_FARM_IX_DISCM: [u8; 8usize] = [
    119, 13, 141, 28, 62, 197, 68, 246,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFromFarmIxArgs {
    pub base_amount: u64,
    pub quote_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFromFarmIxData(pub WithdrawFromFarmIxArgs);
impl From<WithdrawFromFarmIxArgs> for WithdrawFromFarmIxData {
    fn from(args: WithdrawFromFarmIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFromFarmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_FROM_FARM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WithdrawFromFarmIxArgs {
                base_amount,
                quote_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_FROM_FARM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.quote_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_from_farm_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFromFarmKeys,
    args: WithdrawFromFarmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FROM_FARM_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFromFarmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_from_farm_ix(
    keys: WithdrawFromFarmKeys,
    args: WithdrawFromFarmIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_from_farm_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn withdraw_from_farm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromFarmAccounts<'_, '_>,
    args: WithdrawFromFarmIxArgs,
) -> ProgramResult {
    let keys: WithdrawFromFarmKeys = accounts.into();
    let ix = withdraw_from_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_from_farm_invoke(
    accounts: WithdrawFromFarmAccounts<'_, '_>,
    args: WithdrawFromFarmIxArgs,
) -> ProgramResult {
    withdraw_from_farm_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn withdraw_from_farm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFromFarmAccounts<'_, '_>,
    args: WithdrawFromFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFromFarmKeys = accounts.into();
    let ix = withdraw_from_farm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_from_farm_invoke_signed(
    accounts: WithdrawFromFarmAccounts<'_, '_>,
    args: WithdrawFromFarmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_from_farm_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_from_farm_verify_account_keys(
    accounts: WithdrawFromFarmAccounts<'_, '_>,
    keys: WithdrawFromFarmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.farm_info.key, keys.farm_info),
        (*accounts.liquidity_provider.key, keys.liquidity_provider),
        (*accounts.farm_user.key, keys.farm_user),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_from_farm_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFromFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_info,
        accounts.liquidity_provider,
        accounts.farm_user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_from_farm_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFromFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_from_farm_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFromFarmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_from_farm_verify_writable_privileges(accounts)?;
    withdraw_from_farm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFarmRewardsAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub farm_info: &'me AccountInfo<'info>,
    pub farm_user: &'me AccountInfo<'info>,
    pub user_deltafi_token: &'me AccountInfo<'info>,
    pub swap_deltafi_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFarmRewardsKeys {
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub farm_info: Pubkey,
    pub farm_user: Pubkey,
    pub user_deltafi_token: Pubkey,
    pub swap_deltafi_token: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimFarmRewardsAccounts<'_, '_>> for ClaimFarmRewardsKeys {
    fn from(accounts: ClaimFarmRewardsAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            farm_info: *accounts.farm_info.key,
            farm_user: *accounts.farm_user.key,
            user_deltafi_token: *accounts.user_deltafi_token.key,
            swap_deltafi_token: *accounts.swap_deltafi_token.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimFarmRewardsKeys> for [AccountMeta; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFarmRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN]> for ClaimFarmRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            swap_info: pubkeys[1],
            farm_info: pubkeys[2],
            farm_user: pubkeys[3],
            user_deltafi_token: pubkeys[4],
            swap_deltafi_token: pubkeys[5],
            owner: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<ClaimFarmRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFarmRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.farm_info.clone(),
            accounts.farm_user.clone(),
            accounts.user_deltafi_token.clone(),
            accounts.swap_deltafi_token.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimFarmRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            swap_info: &arr[1],
            farm_info: &arr[2],
            farm_user: &arr[3],
            user_deltafi_token: &arr[4],
            swap_deltafi_token: &arr[5],
            owner: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const CLAIM_FARM_REWARDS_IX_DISCM: [u8; 8usize] = [
    102, 40, 223, 149, 90, 81, 228, 23,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFarmRewardsIxData;
impl ClaimFarmRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FARM_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FARM_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_farm_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFarmRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FARM_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFarmRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_farm_rewards_ix(
    keys: ClaimFarmRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_farm_rewards_ix_with_program_id(DELTAFI_PROGRAM_ID, keys)
}
pub fn claim_farm_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFarmRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFarmRewardsKeys = accounts.into();
    let ix = claim_farm_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_farm_rewards_invoke(
    accounts: ClaimFarmRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_farm_rewards_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts)
}
pub fn claim_farm_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFarmRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFarmRewardsKeys = accounts.into();
    let ix = claim_farm_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_farm_rewards_invoke_signed(
    accounts: ClaimFarmRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_farm_rewards_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, seeds)
}
pub fn claim_farm_rewards_verify_account_keys(
    accounts: ClaimFarmRewardsAccounts<'_, '_>,
    keys: ClaimFarmRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.farm_info.key, keys.farm_info),
        (*accounts.farm_user.key, keys.farm_user),
        (*accounts.user_deltafi_token.key, keys.user_deltafi_token),
        (*accounts.swap_deltafi_token.key, keys.swap_deltafi_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_farm_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFarmRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.farm_user,
        accounts.user_deltafi_token,
        accounts.swap_deltafi_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_farm_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFarmRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_farm_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimFarmRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_farm_rewards_verify_writable_privileges(accounts)?;
    claim_farm_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateDeltafiUserAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDeltafiUserKeys {
    pub market_config: Pubkey,
    pub deltafi_user: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateDeltafiUserAccounts<'_, '_>> for CreateDeltafiUserKeys {
    fn from(accounts: CreateDeltafiUserAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            deltafi_user: *accounts.deltafi_user.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateDeltafiUserKeys> for [AccountMeta; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDeltafiUserKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN]> for CreateDeltafiUserKeys {
    fn from(pubkeys: [Pubkey; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            deltafi_user: pubkeys[1],
            owner: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<CreateDeltafiUserAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDeltafiUserAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.deltafi_user.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN]>
for CreateDeltafiUserAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            deltafi_user: &arr[1],
            owner: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const CREATE_DELTAFI_USER_IX_DISCM: [u8; 8usize] = [
    181, 233, 44, 118, 190, 100, 81, 229,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDeltafiUserIxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDeltafiUserIxData(pub CreateDeltafiUserIxArgs);
impl From<CreateDeltafiUserIxArgs> for CreateDeltafiUserIxData {
    fn from(args: CreateDeltafiUserIxArgs) -> Self {
        Self(args)
    }
}
impl CreateDeltafiUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DELTAFI_USER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateDeltafiUserIxArgs { bump }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DELTAFI_USER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_deltafi_user_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDeltafiUserKeys,
    args: CreateDeltafiUserIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DELTAFI_USER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateDeltafiUserIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_deltafi_user_ix(
    keys: CreateDeltafiUserKeys,
    args: CreateDeltafiUserIxArgs,
) -> std::io::Result<Instruction> {
    create_deltafi_user_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_deltafi_user_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDeltafiUserAccounts<'_, '_>,
    args: CreateDeltafiUserIxArgs,
) -> ProgramResult {
    let keys: CreateDeltafiUserKeys = accounts.into();
    let ix = create_deltafi_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_deltafi_user_invoke(
    accounts: CreateDeltafiUserAccounts<'_, '_>,
    args: CreateDeltafiUserIxArgs,
) -> ProgramResult {
    create_deltafi_user_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn create_deltafi_user_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDeltafiUserAccounts<'_, '_>,
    args: CreateDeltafiUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDeltafiUserKeys = accounts.into();
    let ix = create_deltafi_user_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_deltafi_user_invoke_signed(
    accounts: CreateDeltafiUserAccounts<'_, '_>,
    args: CreateDeltafiUserIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_deltafi_user_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_deltafi_user_verify_account_keys(
    accounts: CreateDeltafiUserAccounts<'_, '_>,
    keys: CreateDeltafiUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_deltafi_user_verify_writable_privileges<'me, 'info>(
    accounts: CreateDeltafiUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.deltafi_user, accounts.owner] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_deltafi_user_verify_signer_privileges<'me, 'info>(
    accounts: CreateDeltafiUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_deltafi_user_verify_account_privileges<'me, 'info>(
    accounts: CreateDeltafiUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_deltafi_user_verify_writable_privileges(accounts)?;
    create_deltafi_user_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateDeltafiUserWithReferrerAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDeltafiUserWithReferrerKeys {
    pub market_config: Pubkey,
    pub deltafi_user: Pubkey,
    pub referrer: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateDeltafiUserWithReferrerAccounts<'_, '_>>
for CreateDeltafiUserWithReferrerKeys {
    fn from(accounts: CreateDeltafiUserWithReferrerAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            deltafi_user: *accounts.deltafi_user.key,
            referrer: *accounts.referrer.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateDeltafiUserWithReferrerKeys>
for [AccountMeta; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDeltafiUserWithReferrerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for CreateDeltafiUserWithReferrerKeys {
    fn from(
        pubkeys: [Pubkey; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: pubkeys[0],
            deltafi_user: pubkeys[1],
            referrer: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
        }
    }
}
impl<'info> From<CreateDeltafiUserWithReferrerAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDeltafiUserWithReferrerAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.deltafi_user.clone(),
            accounts.referrer.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN]>
for CreateDeltafiUserWithReferrerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            deltafi_user: &arr[1],
            referrer: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
        }
    }
}
pub const CREATE_DELTAFI_USER_WITH_REFERRER_IX_DISCM: [u8; 8usize] = [
    250, 223, 53, 85, 221, 28, 196, 76,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDeltafiUserWithReferrerIxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDeltafiUserWithReferrerIxData(pub CreateDeltafiUserWithReferrerIxArgs);
impl From<CreateDeltafiUserWithReferrerIxArgs> for CreateDeltafiUserWithReferrerIxData {
    fn from(args: CreateDeltafiUserWithReferrerIxArgs) -> Self {
        Self(args)
    }
}
impl CreateDeltafiUserWithReferrerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DELTAFI_USER_WITH_REFERRER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateDeltafiUserWithReferrerIxArgs {
                bump,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DELTAFI_USER_WITH_REFERRER_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_deltafi_user_with_referrer_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDeltafiUserWithReferrerKeys,
    args: CreateDeltafiUserWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DELTAFI_USER_WITH_REFERRER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateDeltafiUserWithReferrerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_deltafi_user_with_referrer_ix(
    keys: CreateDeltafiUserWithReferrerKeys,
    args: CreateDeltafiUserWithReferrerIxArgs,
) -> std::io::Result<Instruction> {
    create_deltafi_user_with_referrer_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_deltafi_user_with_referrer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDeltafiUserWithReferrerAccounts<'_, '_>,
    args: CreateDeltafiUserWithReferrerIxArgs,
) -> ProgramResult {
    let keys: CreateDeltafiUserWithReferrerKeys = accounts.into();
    let ix = create_deltafi_user_with_referrer_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_deltafi_user_with_referrer_invoke(
    accounts: CreateDeltafiUserWithReferrerAccounts<'_, '_>,
    args: CreateDeltafiUserWithReferrerIxArgs,
) -> ProgramResult {
    create_deltafi_user_with_referrer_invoke_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_deltafi_user_with_referrer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDeltafiUserWithReferrerAccounts<'_, '_>,
    args: CreateDeltafiUserWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDeltafiUserWithReferrerKeys = accounts.into();
    let ix = create_deltafi_user_with_referrer_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_deltafi_user_with_referrer_invoke_signed(
    accounts: CreateDeltafiUserWithReferrerAccounts<'_, '_>,
    args: CreateDeltafiUserWithReferrerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_deltafi_user_with_referrer_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_deltafi_user_with_referrer_verify_account_keys(
    accounts: CreateDeltafiUserWithReferrerAccounts<'_, '_>,
    keys: CreateDeltafiUserWithReferrerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_deltafi_user_with_referrer_verify_writable_privileges<'me, 'info>(
    accounts: CreateDeltafiUserWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.deltafi_user, accounts.owner] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_deltafi_user_with_referrer_verify_signer_privileges<'me, 'info>(
    accounts: CreateDeltafiUserWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_deltafi_user_with_referrer_verify_account_privileges<'me, 'info>(
    accounts: CreateDeltafiUserWithReferrerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_deltafi_user_with_referrer_verify_writable_privileges(accounts)?;
    create_deltafi_user_with_referrer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClaimSwapRewardsAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub user_deltafi_token: &'me AccountInfo<'info>,
    pub swap_deltafi_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimSwapRewardsKeys {
    pub market_config: Pubkey,
    pub deltafi_user: Pubkey,
    pub user_deltafi_token: Pubkey,
    pub swap_deltafi_token: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimSwapRewardsAccounts<'_, '_>> for ClaimSwapRewardsKeys {
    fn from(accounts: ClaimSwapRewardsAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            deltafi_user: *accounts.deltafi_user.key,
            user_deltafi_token: *accounts.user_deltafi_token.key,
            swap_deltafi_token: *accounts.swap_deltafi_token.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimSwapRewardsKeys> for [AccountMeta; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimSwapRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN]> for ClaimSwapRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            deltafi_user: pubkeys[1],
            user_deltafi_token: pubkeys[2],
            swap_deltafi_token: pubkeys[3],
            owner: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<ClaimSwapRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimSwapRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.deltafi_user.clone(),
            accounts.user_deltafi_token.clone(),
            accounts.swap_deltafi_token.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimSwapRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: &arr[0],
            deltafi_user: &arr[1],
            user_deltafi_token: &arr[2],
            swap_deltafi_token: &arr[3],
            owner: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CLAIM_SWAP_REWARDS_IX_DISCM: [u8; 8usize] = [
    41, 121, 18, 52, 159, 208, 126, 115,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimSwapRewardsIxData;
impl ClaimSwapRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_SWAP_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_SWAP_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_swap_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimSwapRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_SWAP_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimSwapRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_swap_rewards_ix(
    keys: ClaimSwapRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_swap_rewards_ix_with_program_id(DELTAFI_PROGRAM_ID, keys)
}
pub fn claim_swap_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimSwapRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimSwapRewardsKeys = accounts.into();
    let ix = claim_swap_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_swap_rewards_invoke(
    accounts: ClaimSwapRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_swap_rewards_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts)
}
pub fn claim_swap_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimSwapRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimSwapRewardsKeys = accounts.into();
    let ix = claim_swap_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_swap_rewards_invoke_signed(
    accounts: ClaimSwapRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_swap_rewards_invoke_signed_with_program_id(DELTAFI_PROGRAM_ID, accounts, seeds)
}
pub fn claim_swap_rewards_verify_account_keys(
    accounts: ClaimSwapRewardsAccounts<'_, '_>,
    keys: ClaimSwapRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.user_deltafi_token.key, keys.user_deltafi_token),
        (*accounts.swap_deltafi_token.key, keys.swap_deltafi_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_swap_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimSwapRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deltafi_user,
        accounts.user_deltafi_token,
        accounts.swap_deltafi_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_swap_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimSwapRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_swap_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimSwapRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_swap_rewards_verify_writable_privileges(accounts)?;
    claim_swap_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTradeRewardsAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub user_deltafi_token: &'me AccountInfo<'info>,
    pub swap_deltafi_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTradeRewardsKeys {
    pub market_config: Pubkey,
    pub deltafi_user: Pubkey,
    pub user_deltafi_token: Pubkey,
    pub swap_deltafi_token: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimTradeRewardsAccounts<'_, '_>> for ClaimTradeRewardsKeys {
    fn from(accounts: ClaimTradeRewardsAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            deltafi_user: *accounts.deltafi_user.key,
            user_deltafi_token: *accounts.user_deltafi_token.key,
            swap_deltafi_token: *accounts.swap_deltafi_token.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimTradeRewardsKeys> for [AccountMeta; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTradeRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN]> for ClaimTradeRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            deltafi_user: pubkeys[1],
            user_deltafi_token: pubkeys[2],
            swap_deltafi_token: pubkeys[3],
            owner: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<ClaimTradeRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTradeRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.deltafi_user.clone(),
            accounts.user_deltafi_token.clone(),
            accounts.swap_deltafi_token.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimTradeRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            deltafi_user: &arr[1],
            user_deltafi_token: &arr[2],
            swap_deltafi_token: &arr[3],
            owner: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CLAIM_TRADE_REWARDS_IX_DISCM: [u8; 8usize] = [
    58, 174, 130, 162, 37, 87, 34, 57,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTradeRewardsIxData;
impl ClaimTradeRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TRADE_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TRADE_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_trade_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTradeRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TRADE_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimTradeRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_trade_rewards_ix(
    keys: ClaimTradeRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_trade_rewards_ix_with_program_id(DELTAFI_PROGRAM_ID, keys)
}
pub fn claim_trade_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTradeRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimTradeRewardsKeys = accounts.into();
    let ix = claim_trade_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_trade_rewards_invoke(
    accounts: ClaimTradeRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_trade_rewards_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts)
}
pub fn claim_trade_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTradeRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTradeRewardsKeys = accounts.into();
    let ix = claim_trade_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_trade_rewards_invoke_signed(
    accounts: ClaimTradeRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_trade_rewards_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_trade_rewards_verify_account_keys(
    accounts: ClaimTradeRewardsAccounts<'_, '_>,
    keys: ClaimTradeRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.user_deltafi_token.key, keys.user_deltafi_token),
        (*accounts.swap_deltafi_token.key, keys.swap_deltafi_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_trade_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTradeRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deltafi_user,
        accounts.user_deltafi_token,
        accounts.swap_deltafi_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_trade_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTradeRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_trade_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimTradeRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_trade_rewards_verify_writable_privileges(accounts)?;
    claim_trade_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClaimReferralRewardsAccounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub user_deltafi_token: &'me AccountInfo<'info>,
    pub swap_deltafi_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimReferralRewardsKeys {
    pub market_config: Pubkey,
    pub deltafi_user: Pubkey,
    pub user_deltafi_token: Pubkey,
    pub swap_deltafi_token: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
}
impl From<ClaimReferralRewardsAccounts<'_, '_>> for ClaimReferralRewardsKeys {
    fn from(accounts: ClaimReferralRewardsAccounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            deltafi_user: *accounts.deltafi_user.key,
            user_deltafi_token: *accounts.user_deltafi_token.key,
            swap_deltafi_token: *accounts.swap_deltafi_token.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ClaimReferralRewardsKeys>
for [AccountMeta; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimReferralRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_deltafi_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
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
impl From<[Pubkey; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimReferralRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            deltafi_user: pubkeys[1],
            user_deltafi_token: pubkeys[2],
            swap_deltafi_token: pubkeys[3],
            owner: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<ClaimReferralRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimReferralRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.deltafi_user.clone(),
            accounts.user_deltafi_token.clone(),
            accounts.swap_deltafi_token.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimReferralRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            deltafi_user: &arr[1],
            user_deltafi_token: &arr[2],
            swap_deltafi_token: &arr[3],
            owner: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CLAIM_REFERRAL_REWARDS_IX_DISCM: [u8; 8usize] = [
    23, 112, 76, 162, 157, 106, 203, 246,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimReferralRewardsIxData;
impl ClaimReferralRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REFERRAL_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REFERRAL_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_referral_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimReferralRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimReferralRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_referral_rewards_ix(
    keys: ClaimReferralRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_referral_rewards_ix_with_program_id(DELTAFI_PROGRAM_ID, keys)
}
pub fn claim_referral_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimReferralRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimReferralRewardsKeys = accounts.into();
    let ix = claim_referral_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_referral_rewards_invoke(
    accounts: ClaimReferralRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_referral_rewards_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts)
}
pub fn claim_referral_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimReferralRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimReferralRewardsKeys = accounts.into();
    let ix = claim_referral_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_referral_rewards_invoke_signed(
    accounts: ClaimReferralRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_referral_rewards_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_referral_rewards_verify_account_keys(
    accounts: ClaimReferralRewardsAccounts<'_, '_>,
    keys: ClaimReferralRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (*accounts.user_deltafi_token.key, keys.user_deltafi_token),
        (*accounts.swap_deltafi_token.key, keys.swap_deltafi_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_referral_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimReferralRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deltafi_user,
        accounts.user_deltafi_token,
        accounts.swap_deltafi_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_referral_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimReferralRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_referral_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimReferralRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_referral_rewards_verify_writable_privileges(accounts)?;
    claim_referral_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateFarmUserV2Accounts<'me, 'info> {
    pub market_config: &'me AccountInfo<'info>,
    pub farm_info: &'me AccountInfo<'info>,
    pub farm_user: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateFarmUserV2Keys {
    pub market_config: Pubkey,
    pub farm_info: Pubkey,
    pub farm_user: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateFarmUserV2Accounts<'_, '_>> for CreateFarmUserV2Keys {
    fn from(accounts: CreateFarmUserV2Accounts) -> Self {
        Self {
            market_config: *accounts.market_config.key,
            farm_info: *accounts.farm_info.key,
            farm_user: *accounts.farm_user.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateFarmUserV2Keys> for [AccountMeta; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateFarmUserV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN]> for CreateFarmUserV2Keys {
    fn from(pubkeys: [Pubkey; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_config: pubkeys[0],
            farm_info: pubkeys[1],
            farm_user: pubkeys[2],
            owner: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<CreateFarmUserV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateFarmUserV2Accounts<'_, 'info>) -> Self {
        [
            accounts.market_config.clone(),
            accounts.farm_info.clone(),
            accounts.farm_user.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN]>
for CreateFarmUserV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            market_config: &arr[0],
            farm_info: &arr[1],
            farm_user: &arr[2],
            owner: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
pub const CREATE_FARM_USER_V2_IX_DISCM: [u8; 8usize] = [
    144, 6, 165, 125, 77, 201, 230, 220,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFarmUserV2IxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateFarmUserV2IxData(pub CreateFarmUserV2IxArgs);
impl From<CreateFarmUserV2IxArgs> for CreateFarmUserV2IxData {
    fn from(args: CreateFarmUserV2IxArgs) -> Self {
        Self(args)
    }
}
impl CreateFarmUserV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_FARM_USER_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(CreateFarmUserV2IxArgs { bump }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_FARM_USER_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.bump, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_farm_user_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateFarmUserV2Keys,
    args: CreateFarmUserV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_FARM_USER_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateFarmUserV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_farm_user_v2_ix(
    keys: CreateFarmUserV2Keys,
    args: CreateFarmUserV2IxArgs,
) -> std::io::Result<Instruction> {
    create_farm_user_v2_ix_with_program_id(DELTAFI_PROGRAM_ID, keys, args)
}
pub fn create_farm_user_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateFarmUserV2Accounts<'_, '_>,
    args: CreateFarmUserV2IxArgs,
) -> ProgramResult {
    let keys: CreateFarmUserV2Keys = accounts.into();
    let ix = create_farm_user_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_farm_user_v2_invoke(
    accounts: CreateFarmUserV2Accounts<'_, '_>,
    args: CreateFarmUserV2IxArgs,
) -> ProgramResult {
    create_farm_user_v2_invoke_with_program_id(DELTAFI_PROGRAM_ID, accounts, args)
}
pub fn create_farm_user_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateFarmUserV2Accounts<'_, '_>,
    args: CreateFarmUserV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateFarmUserV2Keys = accounts.into();
    let ix = create_farm_user_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_farm_user_v2_invoke_signed(
    accounts: CreateFarmUserV2Accounts<'_, '_>,
    args: CreateFarmUserV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_farm_user_v2_invoke_signed_with_program_id(
        DELTAFI_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_farm_user_v2_verify_account_keys(
    accounts: CreateFarmUserV2Accounts<'_, '_>,
    keys: CreateFarmUserV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_config.key, keys.market_config),
        (*accounts.farm_info.key, keys.farm_info),
        (*accounts.farm_user.key, keys.farm_user),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_farm_user_v2_verify_writable_privileges<'me, 'info>(
    accounts: CreateFarmUserV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.farm_user, accounts.owner, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_farm_user_v2_verify_signer_privileges<'me, 'info>(
    accounts: CreateFarmUserV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_farm_user_v2_verify_account_privileges<'me, 'info>(
    accounts: CreateFarmUserV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_farm_user_v2_verify_writable_privileges(accounts)?;
    create_farm_user_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
