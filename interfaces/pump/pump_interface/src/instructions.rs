use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum PumpProgramIx {
    AddQuoteControlMint(AddQuoteControlMintIxArgs),
    AddQuoteMint(AddQuoteMintIxArgs),
    AdminCto(AdminCtoIxArgs),
    AdminSetIdlAuthority(AdminSetIdlAuthorityIxArgs),
    AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesIxArgs),
    Buy(BuyIxArgs),
    BuyExactQuoteInV2(BuyExactQuoteInV2IxArgs),
    BuyExactSolIn(BuyExactSolInIxArgs),
    BuyV2(BuyV2IxArgs),
    ClaimCashback,
    ClaimCashbackV2,
    ClaimTokenIncentives,
    CloseUserVolumeAccumulator,
    CollectCreatorFee,
    CollectCreatorFeeV2,
    Create(CreateIxArgs),
    CreateV2(CreateV2IxArgs),
    DistributeCreatorFees,
    DistributeCreatorFeesV2(DistributeCreatorFeesV2IxArgs),
    DistributeFeeToHolders(DistributeFeeToHoldersIxArgs),
    ExtendAccount,
    GetMinimumDistributableFee,
    InitUserVolumeAccumulator,
    Initialize,
    InitializeQuoteControl,
    Migrate,
    MigrateBondingCurveCreator,
    MigrateV2,
    RemoveQuoteControlMint(RemoveQuoteControlMintIxArgs),
    RemoveQuoteMint(RemoveQuoteMintIxArgs),
    Sell(SellIxArgs),
    SellV2(SellV2IxArgs),
    SetCreator(SetCreatorIxArgs),
    SetMayhemVirtualParams,
    SetMetaplexCreator,
    SetParams(SetParamsIxArgs),
    SetQuoteControlAdmin(SetQuoteControlAdminIxArgs),
    SetReservedFeeRecipients(SetReservedFeeRecipientsIxArgs),
    SetVirtualQuoteReserves(SetVirtualQuoteReservesIxArgs),
    SyncUserVolumeAccumulator,
    ToggleCashbackEnabled(ToggleCashbackEnabledIxArgs),
    ToggleCreateV2(ToggleCreateV2IxArgs),
    ToggleMayhemMode(ToggleMayhemModeIxArgs),
    UpdateBuybackConfig(UpdateBuybackConfigIxArgs),
    UpdateCreatorFeeConfig(UpdateCreatorFeeConfigIxArgs),
    UpdateGlobalAuthority,
    UpdateHolderRewardConfig(UpdateHolderRewardConfigIxArgs),
}
impl PumpProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&ADD_QUOTE_CONTROL_MINT_IX_DISCM) {
            let mut reader = &buf[ADD_QUOTE_CONTROL_MINT_IX_DISCM.len()..];
            let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let initial_virtual_quote_reserves: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::AddQuoteControlMint(AddQuoteControlMintIxArgs {
                    quote_mint,
                    initial_virtual_quote_reserves,
                }),
            );
        }
        if buf.starts_with(&ADD_QUOTE_MINT_IX_DISCM) {
            let mut reader = &buf[ADD_QUOTE_MINT_IX_DISCM.len()..];
            let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::AddQuoteMint(AddQuoteMintIxArgs { quote_mint }));
        }
        if buf.starts_with(&ADMIN_CTO_IX_DISCM) {
            let mut reader = &buf[ADMIN_CTO_IX_DISCM.len()..];
            let is_holder_reward: Option<bool> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let creator_fee_bps: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
            let new_creator: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AdminCto(AdminCtoIxArgs {
                    is_holder_reward,
                    creator_fee_bps,
                    new_creator,
                }),
            );
        }
        if buf.starts_with(&ADMIN_SET_IDL_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[ADMIN_SET_IDL_AUTHORITY_IX_DISCM.len()..];
            let idl_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::AdminSetIdlAuthority(AdminSetIdlAuthorityIxArgs {
                    idl_authority,
                }),
            );
        }
        if buf.starts_with(&ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM) {
            let mut reader = &buf[ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM.len()..];
            let start_time: i64 = crate::borsh_de_or_default(&mut reader)?;
            let end_time: i64 = crate::borsh_de_or_default(&mut reader)?;
            let seconds_in_a_day: i64 = crate::borsh_de_or_default(&mut reader)?;
            let day_number: u64 = crate::borsh_de_or_default(&mut reader)?;
            let pump_token_supply_per_day: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesIxArgs {
                    start_time,
                    end_time,
                    seconds_in_a_day,
                    day_number,
                    pump_token_supply_per_day,
                }),
            );
        }
        if buf.starts_with(&BUY_IX_DISCM) {
            let mut reader = &buf[BUY_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_sol_cost: u64 = crate::borsh_de_or_default(&mut reader)?;
            let track_volume = if reader.is_empty() {
                Default::default()
            } else {
                <OptionBool>::deserialize(&mut reader)?
            };
            return Ok(
                Self::Buy(BuyIxArgs {
                    amount,
                    max_sol_cost,
                    track_volume,
                }),
            );
        }
        if buf.starts_with(&BUY_EXACT_QUOTE_IN_V2_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_QUOTE_IN_V2_IX_DISCM.len()..];
            let spendable_quote_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyExactQuoteInV2(BuyExactQuoteInV2IxArgs {
                    spendable_quote_in,
                    min_tokens_out,
                }),
            );
        }
        if buf.starts_with(&BUY_EXACT_SOL_IN_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_SOL_IN_IX_DISCM.len()..];
            let spendable_sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let track_volume = if reader.is_empty() {
                Default::default()
            } else {
                <OptionBool>::deserialize(&mut reader)?
            };
            return Ok(
                Self::BuyExactSolIn(BuyExactSolInIxArgs {
                    spendable_sol_in,
                    min_tokens_out,
                    track_volume,
                }),
            );
        }
        if buf.starts_with(&BUY_V2_IX_DISCM) {
            let mut reader = &buf[BUY_V2_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let max_sol_cost: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyV2(BuyV2IxArgs {
                    amount,
                    max_sol_cost,
                }),
            );
        }
        if buf.starts_with(&CLAIM_CASHBACK_IX_DISCM) {
            return Ok(Self::ClaimCashback);
        }
        if buf.starts_with(&CLAIM_CASHBACK_V2_IX_DISCM) {
            return Ok(Self::ClaimCashbackV2);
        }
        if buf.starts_with(&CLAIM_TOKEN_INCENTIVES_IX_DISCM) {
            return Ok(Self::ClaimTokenIncentives);
        }
        if buf.starts_with(&CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM) {
            return Ok(Self::CloseUserVolumeAccumulator);
        }
        if buf.starts_with(&COLLECT_CREATOR_FEE_IX_DISCM) {
            return Ok(Self::CollectCreatorFee);
        }
        if buf.starts_with(&COLLECT_CREATOR_FEE_V2_IX_DISCM) {
            return Ok(Self::CollectCreatorFeeV2);
        }
        if buf.starts_with(&CREATE_IX_DISCM) {
            let mut reader = &buf[CREATE_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Create(CreateIxArgs {
                    name,
                    symbol,
                    uri,
                    creator,
                }),
            );
        }
        if buf.starts_with(&CREATE_V2_IX_DISCM) {
            let mut reader = &buf[CREATE_V2_IX_DISCM.len()..];
            let name: String = crate::borsh_de_or_default(&mut reader)?;
            let symbol: String = crate::borsh_de_or_default(&mut reader)?;
            let uri: String = crate::borsh_de_or_default(&mut reader)?;
            let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let is_mayhem_mode: bool = crate::borsh_de_or_default(&mut reader)?;
            let is_cashback_enabled = if reader.is_empty() {
                Default::default()
            } else {
                <OptionBool>::deserialize(&mut reader)?
            };
            let creator_fee_bps = if reader.is_empty() {
                Default::default()
            } else {
                <OptionU64>::deserialize(&mut reader)?
            };
            let is_holder_reward = if reader.is_empty() {
                Default::default()
            } else {
                <OptionBool>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreateV2(CreateV2IxArgs {
                    name,
                    symbol,
                    uri,
                    creator,
                    is_mayhem_mode,
                    is_cashback_enabled,
                    creator_fee_bps,
                    is_holder_reward,
                }),
            );
        }
        if buf.starts_with(&DISTRIBUTE_CREATOR_FEES_IX_DISCM) {
            return Ok(Self::DistributeCreatorFees);
        }
        if buf.starts_with(&DISTRIBUTE_CREATOR_FEES_V2_IX_DISCM) {
            let mut reader = &buf[DISTRIBUTE_CREATOR_FEES_V2_IX_DISCM.len()..];
            let initialize_ata: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DistributeCreatorFeesV2(DistributeCreatorFeesV2IxArgs {
                    initialize_ata,
                }),
            );
        }
        if buf.starts_with(&DISTRIBUTE_FEE_TO_HOLDERS_IX_DISCM) {
            let mut reader = &buf[DISTRIBUTE_FEE_TO_HOLDERS_IX_DISCM.len()..];
            let amounts: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::DistributeFeeToHolders(DistributeFeeToHoldersIxArgs {
                    amounts,
                }),
            );
        }
        if buf.starts_with(&EXTEND_ACCOUNT_IX_DISCM) {
            return Ok(Self::ExtendAccount);
        }
        if buf.starts_with(&GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM) {
            return Ok(Self::GetMinimumDistributableFee);
        }
        if buf.starts_with(&INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM) {
            return Ok(Self::InitUserVolumeAccumulator);
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            return Ok(Self::Initialize);
        }
        if buf.starts_with(&INITIALIZE_QUOTE_CONTROL_IX_DISCM) {
            return Ok(Self::InitializeQuoteControl);
        }
        if buf.starts_with(&MIGRATE_IX_DISCM) {
            return Ok(Self::Migrate);
        }
        if buf.starts_with(&MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM) {
            return Ok(Self::MigrateBondingCurveCreator);
        }
        if buf.starts_with(&MIGRATE_V2_IX_DISCM) {
            return Ok(Self::MigrateV2);
        }
        if buf.starts_with(&REMOVE_QUOTE_CONTROL_MINT_IX_DISCM) {
            let mut reader = &buf[REMOVE_QUOTE_CONTROL_MINT_IX_DISCM.len()..];
            let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveQuoteControlMint(RemoveQuoteControlMintIxArgs {
                    quote_mint,
                }),
            );
        }
        if buf.starts_with(&REMOVE_QUOTE_MINT_IX_DISCM) {
            let mut reader = &buf[REMOVE_QUOTE_MINT_IX_DISCM.len()..];
            let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemoveQuoteMint(RemoveQuoteMintIxArgs {
                    quote_mint,
                }),
            );
        }
        if buf.starts_with(&SELL_IX_DISCM) {
            let mut reader = &buf[SELL_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_sol_output: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::Sell(SellIxArgs {
                    amount,
                    min_sol_output,
                }),
            );
        }
        if buf.starts_with(&SELL_V2_IX_DISCM) {
            let mut reader = &buf[SELL_V2_IX_DISCM.len()..];
            let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            let min_sol_output: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellV2(SellV2IxArgs {
                    amount,
                    min_sol_output,
                }),
            );
        }
        if buf.starts_with(&SET_CREATOR_IX_DISCM) {
            let mut reader = &buf[SET_CREATOR_IX_DISCM.len()..];
            let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::SetCreator(SetCreatorIxArgs { creator }));
        }
        if buf.starts_with(&SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM) {
            return Ok(Self::SetMayhemVirtualParams);
        }
        if buf.starts_with(&SET_METAPLEX_CREATOR_IX_DISCM) {
            return Ok(Self::SetMetaplexCreator);
        }
        if buf.starts_with(&SET_PARAMS_IX_DISCM) {
            let mut reader = &buf[SET_PARAMS_IX_DISCM.len()..];
            let initial_virtual_token_reserves: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let initial_virtual_sol_reserves: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let initial_real_token_reserves: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let token_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
            let fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
            let withdraw_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let enable_migrate: bool = crate::borsh_de_or_default(&mut reader)?;
            let pool_migration_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let creator_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
            let set_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let admin_set_creator_authority: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetParams(SetParamsIxArgs {
                    initial_virtual_token_reserves,
                    initial_virtual_sol_reserves,
                    initial_real_token_reserves,
                    token_total_supply,
                    fee_basis_points,
                    withdraw_authority,
                    enable_migrate,
                    pool_migration_fee,
                    creator_fee_basis_points,
                    set_creator_authority,
                    admin_set_creator_authority,
                }),
            );
        }
        if buf.starts_with(&SET_QUOTE_CONTROL_ADMIN_IX_DISCM) {
            let mut reader = &buf[SET_QUOTE_CONTROL_ADMIN_IX_DISCM.len()..];
            let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetQuoteControlAdmin(SetQuoteControlAdminIxArgs {
                    new_admin,
                }),
            );
        }
        if buf.starts_with(&SET_RESERVED_FEE_RECIPIENTS_IX_DISCM) {
            let mut reader = &buf[SET_RESERVED_FEE_RECIPIENTS_IX_DISCM.len()..];
            let whitelist_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SetReservedFeeRecipients(SetReservedFeeRecipientsIxArgs {
                    whitelist_pda,
                }),
            );
        }
        if buf.starts_with(&SET_VIRTUAL_QUOTE_RESERVES_IX_DISCM) {
            let mut reader = &buf[SET_VIRTUAL_QUOTE_RESERVES_IX_DISCM.len()..];
            let initial_virtual_quote_reserves: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetVirtualQuoteReserves(SetVirtualQuoteReservesIxArgs {
                    initial_virtual_quote_reserves,
                }),
            );
        }
        if buf.starts_with(&SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM) {
            return Ok(Self::SyncUserVolumeAccumulator);
        }
        if buf.starts_with(&TOGGLE_CASHBACK_ENABLED_IX_DISCM) {
            let mut reader = &buf[TOGGLE_CASHBACK_ENABLED_IX_DISCM.len()..];
            let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::ToggleCashbackEnabled(ToggleCashbackEnabledIxArgs {
                    enabled,
                }),
            );
        }
        if buf.starts_with(&TOGGLE_CREATE_V2_IX_DISCM) {
            let mut reader = &buf[TOGGLE_CREATE_V2_IX_DISCM.len()..];
            let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ToggleCreateV2(ToggleCreateV2IxArgs { enabled }));
        }
        if buf.starts_with(&TOGGLE_MAYHEM_MODE_IX_DISCM) {
            let mut reader = &buf[TOGGLE_MAYHEM_MODE_IX_DISCM.len()..];
            let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::ToggleMayhemMode(ToggleMayhemModeIxArgs { enabled }));
        }
        if buf.starts_with(&UPDATE_BUYBACK_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_BUYBACK_CONFIG_IX_DISCM.len()..];
            let buyback_basis_points: Option<u64> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateBuybackConfig(UpdateBuybackConfigIxArgs {
                    buyback_basis_points,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CREATOR_FEE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_CREATOR_FEE_CONFIG_IX_DISCM.len()..];
            let creator_fee_configurable: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let max_configurable_creator_fee_bps: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateCreatorFeeConfig(UpdateCreatorFeeConfigIxArgs {
                    creator_fee_configurable,
                    max_configurable_creator_fee_bps,
                }),
            );
        }
        if buf.starts_with(&UPDATE_GLOBAL_AUTHORITY_IX_DISCM) {
            return Ok(Self::UpdateGlobalAuthority);
        }
        if buf.starts_with(&UPDATE_HOLDER_REWARD_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_HOLDER_REWARD_CONFIG_IX_DISCM.len()..];
            let is_holder_reward_enabled: bool = crate::borsh_de_or_default(
                &mut reader,
            )?;
            let holder_reward_claim_authority: Pubkey = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdateHolderRewardConfig(UpdateHolderRewardConfigIxArgs {
                    is_holder_reward_enabled,
                    holder_reward_claim_authority,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddQuoteControlMint(args) => {
                writer.write_all(&ADD_QUOTE_CONTROL_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.quote_mint, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_quote_reserves,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::AddQuoteMint(args) => {
                writer.write_all(&ADD_QUOTE_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.quote_mint, &mut writer)?;
                Ok(())
            }
            Self::AdminCto(args) => {
                writer.write_all(&ADMIN_CTO_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.is_holder_reward, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.new_creator, &mut writer)?;
                Ok(())
            }
            Self::AdminSetIdlAuthority(args) => {
                writer.write_all(&ADMIN_SET_IDL_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.idl_authority, &mut writer)?;
                Ok(())
            }
            Self::AdminUpdateTokenIncentives(args) => {
                writer.write_all(&ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.start_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.end_time, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.seconds_in_a_day, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.day_number, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.pump_token_supply_per_day,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_sol_cost, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.track_volume, &mut writer)?;
                Ok(())
            }
            Self::BuyExactQuoteInV2(args) => {
                writer.write_all(&BUY_EXACT_QUOTE_IN_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.spendable_quote_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_tokens_out, &mut writer)?;
                Ok(())
            }
            Self::BuyExactSolIn(args) => {
                writer.write_all(&BUY_EXACT_SOL_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.spendable_sol_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_tokens_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.track_volume, &mut writer)?;
                Ok(())
            }
            Self::BuyV2(args) => {
                writer.write_all(&BUY_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.max_sol_cost, &mut writer)?;
                Ok(())
            }
            Self::ClaimCashback => writer.write_all(&CLAIM_CASHBACK_IX_DISCM),
            Self::ClaimCashbackV2 => writer.write_all(&CLAIM_CASHBACK_V2_IX_DISCM),
            Self::ClaimTokenIncentives => {
                writer.write_all(&CLAIM_TOKEN_INCENTIVES_IX_DISCM)
            }
            Self::CloseUserVolumeAccumulator => {
                writer.write_all(&CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM)
            }
            Self::CollectCreatorFee => writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM),
            Self::CollectCreatorFeeV2 => {
                writer.write_all(&COLLECT_CREATOR_FEE_V2_IX_DISCM)
            }
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator, &mut writer)?;
                Ok(())
            }
            Self::CreateV2(args) => {
                writer.write_all(&CREATE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.name, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.symbol, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.uri, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.creator, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_mayhem_mode, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.is_cashback_enabled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.creator_fee_bps, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.is_holder_reward, &mut writer)?;
                Ok(())
            }
            Self::DistributeCreatorFees => {
                writer.write_all(&DISTRIBUTE_CREATOR_FEES_IX_DISCM)
            }
            Self::DistributeCreatorFeesV2(args) => {
                writer.write_all(&DISTRIBUTE_CREATOR_FEES_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.initialize_ata, &mut writer)?;
                Ok(())
            }
            Self::DistributeFeeToHolders(args) => {
                writer.write_all(&DISTRIBUTE_FEE_TO_HOLDERS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amounts, &mut writer)?;
                Ok(())
            }
            Self::ExtendAccount => writer.write_all(&EXTEND_ACCOUNT_IX_DISCM),
            Self::GetMinimumDistributableFee => {
                writer.write_all(&GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM)
            }
            Self::InitUserVolumeAccumulator => {
                writer.write_all(&INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM)
            }
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::InitializeQuoteControl => {
                writer.write_all(&INITIALIZE_QUOTE_CONTROL_IX_DISCM)
            }
            Self::Migrate => writer.write_all(&MIGRATE_IX_DISCM),
            Self::MigrateBondingCurveCreator => {
                writer.write_all(&MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM)
            }
            Self::MigrateV2 => writer.write_all(&MIGRATE_V2_IX_DISCM),
            Self::RemoveQuoteControlMint(args) => {
                writer.write_all(&REMOVE_QUOTE_CONTROL_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.quote_mint, &mut writer)?;
                Ok(())
            }
            Self::RemoveQuoteMint(args) => {
                writer.write_all(&REMOVE_QUOTE_MINT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.quote_mint, &mut writer)?;
                Ok(())
            }
            Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_sol_output, &mut writer)?;
                Ok(())
            }
            Self::SellV2(args) => {
                writer.write_all(&SELL_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.min_sol_output, &mut writer)?;
                Ok(())
            }
            Self::SetCreator(args) => {
                writer.write_all(&SET_CREATOR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.creator, &mut writer)?;
                Ok(())
            }
            Self::SetMayhemVirtualParams => {
                writer.write_all(&SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM)
            }
            Self::SetMetaplexCreator => writer.write_all(&SET_METAPLEX_CREATOR_IX_DISCM),
            Self::SetParams(args) => {
                writer.write_all(&SET_PARAMS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_token_reserves,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_sol_reserves,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.initial_real_token_reserves,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(&args.token_total_supply, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.fee_basis_points, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.withdraw_authority, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.enable_migrate, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.pool_migration_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.creator_fee_basis_points,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.set_creator_authority,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.admin_set_creator_authority,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetQuoteControlAdmin(args) => {
                writer.write_all(&SET_QUOTE_CONTROL_ADMIN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_admin, &mut writer)?;
                Ok(())
            }
            Self::SetReservedFeeRecipients(args) => {
                writer.write_all(&SET_RESERVED_FEE_RECIPIENTS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.whitelist_pda, &mut writer)?;
                Ok(())
            }
            Self::SetVirtualQuoteReserves(args) => {
                writer.write_all(&SET_VIRTUAL_QUOTE_RESERVES_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.initial_virtual_quote_reserves,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SyncUserVolumeAccumulator => {
                writer.write_all(&SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM)
            }
            Self::ToggleCashbackEnabled(args) => {
                writer.write_all(&TOGGLE_CASHBACK_ENABLED_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enabled, &mut writer)?;
                Ok(())
            }
            Self::ToggleCreateV2(args) => {
                writer.write_all(&TOGGLE_CREATE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enabled, &mut writer)?;
                Ok(())
            }
            Self::ToggleMayhemMode(args) => {
                writer.write_all(&TOGGLE_MAYHEM_MODE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.enabled, &mut writer)?;
                Ok(())
            }
            Self::UpdateBuybackConfig(args) => {
                writer.write_all(&UPDATE_BUYBACK_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.buyback_basis_points,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateCreatorFeeConfig(args) => {
                writer.write_all(&UPDATE_CREATOR_FEE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.creator_fee_configurable,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.max_configurable_creator_fee_bps,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::UpdateGlobalAuthority => {
                writer.write_all(&UPDATE_GLOBAL_AUTHORITY_IX_DISCM)
            }
            Self::UpdateHolderRewardConfig(args) => {
                writer.write_all(&UPDATE_HOLDER_REWARD_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(
                    &args.is_holder_reward_enabled,
                    &mut writer,
                )?;
                borsh::BorshSerialize::serialize(
                    &args.holder_reward_claim_authority,
                    &mut writer,
                )?;
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
pub const ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddQuoteControlMintAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub quote_control: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddQuoteControlMintKeys {
    pub authority: Pubkey,
    pub global: Pubkey,
    pub quote_control: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddQuoteControlMintAccounts<'_, '_>> for AddQuoteControlMintKeys {
    fn from(accounts: AddQuoteControlMintAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            global: *accounts.global.key,
            quote_control: *accounts.quote_control.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddQuoteControlMintKeys>
for [AccountMeta; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: AddQuoteControlMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_control,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN]> for AddQuoteControlMintKeys {
    fn from(pubkeys: [Pubkey; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            global: pubkeys[1],
            quote_control: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<AddQuoteControlMintAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddQuoteControlMintAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.global.clone(),
            accounts.quote_control.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN]>
for AddQuoteControlMintAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            global: &arr[1],
            quote_control: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const ADD_QUOTE_CONTROL_MINT_IX_DISCM: [u8; 8usize] = [
    2, 14, 61, 138, 170, 142, 14, 95,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddQuoteControlMintIxArgs {
    pub quote_mint: Pubkey,
    pub initial_virtual_quote_reserves: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddQuoteControlMintIxData(pub AddQuoteControlMintIxArgs);
impl From<AddQuoteControlMintIxArgs> for AddQuoteControlMintIxData {
    fn from(args: AddQuoteControlMintIxArgs) -> Self {
        Self(args)
    }
}
impl AddQuoteControlMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_QUOTE_CONTROL_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_quote_reserves: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(AddQuoteControlMintIxArgs {
                quote_mint,
                initial_virtual_quote_reserves,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_QUOTE_CONTROL_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_quote_reserves,
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
pub fn add_quote_control_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: AddQuoteControlMintKeys,
    args: AddQuoteControlMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddQuoteControlMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_quote_control_mint_ix(
    keys: AddQuoteControlMintKeys,
    args: AddQuoteControlMintIxArgs,
) -> std::io::Result<Instruction> {
    add_quote_control_mint_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn add_quote_control_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddQuoteControlMintAccounts<'_, '_>,
    args: AddQuoteControlMintIxArgs,
) -> ProgramResult {
    let keys: AddQuoteControlMintKeys = accounts.into();
    let ix = add_quote_control_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_quote_control_mint_invoke(
    accounts: AddQuoteControlMintAccounts<'_, '_>,
    args: AddQuoteControlMintIxArgs,
) -> ProgramResult {
    add_quote_control_mint_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn add_quote_control_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddQuoteControlMintAccounts<'_, '_>,
    args: AddQuoteControlMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddQuoteControlMintKeys = accounts.into();
    let ix = add_quote_control_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_quote_control_mint_invoke_signed(
    accounts: AddQuoteControlMintAccounts<'_, '_>,
    args: AddQuoteControlMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_quote_control_mint_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_quote_control_mint_verify_account_keys(
    accounts: AddQuoteControlMintAccounts<'_, '_>,
    keys: AddQuoteControlMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.global.key, keys.global),
        (*accounts.quote_control.key, keys.quote_control),
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
pub fn add_quote_control_mint_verify_writable_privileges<'me, 'info>(
    accounts: AddQuoteControlMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.quote_control] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_quote_control_mint_verify_signer_privileges<'me, 'info>(
    accounts: AddQuoteControlMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_quote_control_mint_verify_account_privileges<'me, 'info>(
    accounts: AddQuoteControlMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_quote_control_mint_verify_writable_privileges(accounts)?;
    add_quote_control_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_QUOTE_MINT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct AddQuoteMintAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddQuoteMintKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddQuoteMintAccounts<'_, '_>> for AddQuoteMintKeys {
    fn from(accounts: AddQuoteMintAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddQuoteMintKeys> for [AccountMeta; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: AddQuoteMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN]> for AddQuoteMintKeys {
    fn from(pubkeys: [Pubkey; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<AddQuoteMintAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddQuoteMintAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN]>
for AddQuoteMintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const ADD_QUOTE_MINT_IX_DISCM: [u8; 8usize] = [111, 121, 21, 56, 40, 24, 94, 209];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddQuoteMintIxArgs {
    pub quote_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddQuoteMintIxData(pub AddQuoteMintIxArgs);
impl From<AddQuoteMintIxArgs> for AddQuoteMintIxData {
    fn from(args: AddQuoteMintIxArgs) -> Self {
        Self(args)
    }
}
impl AddQuoteMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_QUOTE_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(AddQuoteMintIxArgs { quote_mint }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_QUOTE_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.quote_mint, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_quote_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: AddQuoteMintKeys,
    args: AddQuoteMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_QUOTE_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddQuoteMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_quote_mint_ix(
    keys: AddQuoteMintKeys,
    args: AddQuoteMintIxArgs,
) -> std::io::Result<Instruction> {
    add_quote_mint_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn add_quote_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddQuoteMintAccounts<'_, '_>,
    args: AddQuoteMintIxArgs,
) -> ProgramResult {
    let keys: AddQuoteMintKeys = accounts.into();
    let ix = add_quote_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_quote_mint_invoke(
    accounts: AddQuoteMintAccounts<'_, '_>,
    args: AddQuoteMintIxArgs,
) -> ProgramResult {
    add_quote_mint_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn add_quote_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddQuoteMintAccounts<'_, '_>,
    args: AddQuoteMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddQuoteMintKeys = accounts.into();
    let ix = add_quote_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_quote_mint_invoke_signed(
    accounts: AddQuoteMintAccounts<'_, '_>,
    args: AddQuoteMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_quote_mint_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn add_quote_mint_verify_account_keys(
    accounts: AddQuoteMintAccounts<'_, '_>,
    keys: AddQuoteMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_quote_mint_verify_writable_privileges<'me, 'info>(
    accounts: AddQuoteMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_quote_mint_verify_signer_privileges<'me, 'info>(
    accounts: AddQuoteMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_quote_mint_verify_account_privileges<'me, 'info>(
    accounts: AddQuoteMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_quote_mint_verify_writable_privileges(accounts)?;
    add_quote_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_CTO_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct AdminCtoAccounts<'me, 'info> {
    pub admin_set_creator_authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub current_creator: &'me AccountInfo<'info>,
    pub current_creator_quote_token_account: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub creator_vault_quote_token_account: &'me AccountInfo<'info>,
    pub holder_creator_vault: &'me AccountInfo<'info>,
    pub holder_creator_vault_quote_token_account: &'me AccountInfo<'info>,
    pub pump_amm: &'me AccountInfo<'info>,
    pub amm_global_config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pump_amm_event_authority: &'me AccountInfo<'info>,
    pub coin_creator_vault_authority: &'me AccountInfo<'info>,
    pub coin_creator_vault_ata: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub pump_fees: &'me AccountInfo<'info>,
    pub pump_fees_event_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminCtoKeys {
    pub admin_set_creator_authority: Pubkey,
    pub global: Pubkey,
    pub mint: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub bonding_curve: Pubkey,
    pub current_creator: Pubkey,
    pub current_creator_quote_token_account: Pubkey,
    pub creator_vault: Pubkey,
    pub creator_vault_quote_token_account: Pubkey,
    pub holder_creator_vault: Pubkey,
    pub holder_creator_vault_quote_token_account: Pubkey,
    pub pump_amm: Pubkey,
    pub amm_global_config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub pump_amm_event_authority: Pubkey,
    pub coin_creator_vault_authority: Pubkey,
    pub coin_creator_vault_ata: Pubkey,
    pub sharing_config: Pubkey,
    pub pump_fees: Pubkey,
    pub pump_fees_event_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AdminCtoAccounts<'_, '_>> for AdminCtoKeys {
    fn from(accounts: AdminCtoAccounts) -> Self {
        Self {
            admin_set_creator_authority: *accounts.admin_set_creator_authority.key,
            global: *accounts.global.key,
            mint: *accounts.mint.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            bonding_curve: *accounts.bonding_curve.key,
            current_creator: *accounts.current_creator.key,
            current_creator_quote_token_account: *accounts
                .current_creator_quote_token_account
                .key,
            creator_vault: *accounts.creator_vault.key,
            creator_vault_quote_token_account: *accounts
                .creator_vault_quote_token_account
                .key,
            holder_creator_vault: *accounts.holder_creator_vault.key,
            holder_creator_vault_quote_token_account: *accounts
                .holder_creator_vault_quote_token_account
                .key,
            pump_amm: *accounts.pump_amm.key,
            amm_global_config: *accounts.amm_global_config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            pump_amm_event_authority: *accounts.pump_amm_event_authority.key,
            coin_creator_vault_authority: *accounts.coin_creator_vault_authority.key,
            coin_creator_vault_ata: *accounts.coin_creator_vault_ata.key,
            sharing_config: *accounts.sharing_config.key,
            pump_fees: *accounts.pump_fees.key,
            pump_fees_event_authority: *accounts.pump_fees_event_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AdminCtoKeys> for [AccountMeta; ADMIN_CTO_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminCtoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_set_creator_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.current_creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.current_creator_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_creator_vault_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pump_amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pump_amm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_creator_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_creator_vault_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pump_fees,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pump_fees_event_authority,
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
impl From<[Pubkey; ADMIN_CTO_IX_ACCOUNTS_LEN]> for AdminCtoKeys {
    fn from(pubkeys: [Pubkey; ADMIN_CTO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_set_creator_authority: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            quote_mint: pubkeys[3],
            quote_token_program: pubkeys[4],
            associated_token_program: pubkeys[5],
            system_program: pubkeys[6],
            bonding_curve: pubkeys[7],
            current_creator: pubkeys[8],
            current_creator_quote_token_account: pubkeys[9],
            creator_vault: pubkeys[10],
            creator_vault_quote_token_account: pubkeys[11],
            holder_creator_vault: pubkeys[12],
            holder_creator_vault_quote_token_account: pubkeys[13],
            pump_amm: pubkeys[14],
            amm_global_config: pubkeys[15],
            pool_authority: pubkeys[16],
            pool: pubkeys[17],
            pump_amm_event_authority: pubkeys[18],
            coin_creator_vault_authority: pubkeys[19],
            coin_creator_vault_ata: pubkeys[20],
            sharing_config: pubkeys[21],
            pump_fees: pubkeys[22],
            pump_fees_event_authority: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<AdminCtoAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_CTO_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminCtoAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_set_creator_authority.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.bonding_curve.clone(),
            accounts.current_creator.clone(),
            accounts.current_creator_quote_token_account.clone(),
            accounts.creator_vault.clone(),
            accounts.creator_vault_quote_token_account.clone(),
            accounts.holder_creator_vault.clone(),
            accounts.holder_creator_vault_quote_token_account.clone(),
            accounts.pump_amm.clone(),
            accounts.amm_global_config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.pump_amm_event_authority.clone(),
            accounts.coin_creator_vault_authority.clone(),
            accounts.coin_creator_vault_ata.clone(),
            accounts.sharing_config.clone(),
            accounts.pump_fees.clone(),
            accounts.pump_fees_event_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_CTO_IX_ACCOUNTS_LEN]>
for AdminCtoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADMIN_CTO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_set_creator_authority: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            quote_mint: &arr[3],
            quote_token_program: &arr[4],
            associated_token_program: &arr[5],
            system_program: &arr[6],
            bonding_curve: &arr[7],
            current_creator: &arr[8],
            current_creator_quote_token_account: &arr[9],
            creator_vault: &arr[10],
            creator_vault_quote_token_account: &arr[11],
            holder_creator_vault: &arr[12],
            holder_creator_vault_quote_token_account: &arr[13],
            pump_amm: &arr[14],
            amm_global_config: &arr[15],
            pool_authority: &arr[16],
            pool: &arr[17],
            pump_amm_event_authority: &arr[18],
            coin_creator_vault_authority: &arr[19],
            coin_creator_vault_ata: &arr[20],
            sharing_config: &arr[21],
            pump_fees: &arr[22],
            pump_fees_event_authority: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const ADMIN_CTO_IX_DISCM: [u8; 8usize] = [125, 126, 214, 134, 77, 229, 188, 89];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminCtoIxArgs {
    pub is_holder_reward: Option<bool>,
    pub creator_fee_bps: Option<u64>,
    pub new_creator: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminCtoIxData(pub AdminCtoIxArgs);
impl From<AdminCtoIxArgs> for AdminCtoIxData {
    fn from(args: AdminCtoIxArgs) -> Self {
        Self(args)
    }
}
impl AdminCtoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_CTO_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let is_holder_reward: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_bps: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let new_creator: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AdminCtoIxArgs {
                is_holder_reward,
                creator_fee_bps,
                new_creator,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_CTO_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.is_holder_reward, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.new_creator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_cto_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminCtoKeys,
    args: AdminCtoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_CTO_IX_ACCOUNTS_LEN] = keys.into();
    let data: AdminCtoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_cto_ix(
    keys: AdminCtoKeys,
    args: AdminCtoIxArgs,
) -> std::io::Result<Instruction> {
    admin_cto_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn admin_cto_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminCtoAccounts<'_, '_>,
    args: AdminCtoIxArgs,
) -> ProgramResult {
    let keys: AdminCtoKeys = accounts.into();
    let ix = admin_cto_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_cto_invoke(
    accounts: AdminCtoAccounts<'_, '_>,
    args: AdminCtoIxArgs,
) -> ProgramResult {
    admin_cto_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn admin_cto_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminCtoAccounts<'_, '_>,
    args: AdminCtoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminCtoKeys = accounts.into();
    let ix = admin_cto_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_cto_invoke_signed(
    accounts: AdminCtoAccounts<'_, '_>,
    args: AdminCtoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_cto_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn admin_cto_verify_account_keys(
    accounts: AdminCtoAccounts<'_, '_>,
    keys: AdminCtoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_set_creator_authority.key, keys.admin_set_creator_authority),
        (*accounts.global.key, keys.global),
        (*accounts.mint.key, keys.mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.current_creator.key, keys.current_creator),
        (
            *accounts.current_creator_quote_token_account.key,
            keys.current_creator_quote_token_account,
        ),
        (*accounts.creator_vault.key, keys.creator_vault),
        (
            *accounts.creator_vault_quote_token_account.key,
            keys.creator_vault_quote_token_account,
        ),
        (*accounts.holder_creator_vault.key, keys.holder_creator_vault),
        (
            *accounts.holder_creator_vault_quote_token_account.key,
            keys.holder_creator_vault_quote_token_account,
        ),
        (*accounts.pump_amm.key, keys.pump_amm),
        (*accounts.amm_global_config.key, keys.amm_global_config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.pump_amm_event_authority.key, keys.pump_amm_event_authority),
        (*accounts.coin_creator_vault_authority.key, keys.coin_creator_vault_authority),
        (*accounts.coin_creator_vault_ata.key, keys.coin_creator_vault_ata),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.pump_fees.key, keys.pump_fees),
        (*accounts.pump_fees_event_authority.key, keys.pump_fees_event_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_cto_verify_writable_privileges<'me, 'info>(
    accounts: AdminCtoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_set_creator_authority,
        accounts.bonding_curve,
        accounts.current_creator_quote_token_account,
        accounts.creator_vault,
        accounts.creator_vault_quote_token_account,
        accounts.holder_creator_vault,
        accounts.holder_creator_vault_quote_token_account,
        accounts.pool,
        accounts.coin_creator_vault_authority,
        accounts.coin_creator_vault_ata,
        accounts.sharing_config,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_cto_verify_signer_privileges<'me, 'info>(
    accounts: AdminCtoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_set_creator_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_cto_verify_account_privileges<'me, 'info>(
    accounts: AdminCtoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_cto_verify_writable_privileges(accounts)?;
    admin_cto_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AdminSetIdlAuthorityAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub idl_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program_signer: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminSetIdlAuthorityKeys {
    pub authority: Pubkey,
    pub global: Pubkey,
    pub idl_account: Pubkey,
    pub system_program: Pubkey,
    pub program_signer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AdminSetIdlAuthorityAccounts<'_, '_>> for AdminSetIdlAuthorityKeys {
    fn from(accounts: AdminSetIdlAuthorityAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            global: *accounts.global.key,
            idl_account: *accounts.idl_account.key,
            system_program: *accounts.system_program.key,
            program_signer: *accounts.program_signer.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AdminSetIdlAuthorityKeys>
for [AccountMeta; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminSetIdlAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.idl_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_signer,
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
impl From<[Pubkey; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]>
for AdminSetIdlAuthorityKeys {
    fn from(pubkeys: [Pubkey; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            global: pubkeys[1],
            idl_account: pubkeys[2],
            system_program: pubkeys[3],
            program_signer: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<AdminSetIdlAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminSetIdlAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.global.clone(),
            accounts.idl_account.clone(),
            accounts.system_program.clone(),
            accounts.program_signer.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]>
for AdminSetIdlAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            global: &arr[1],
            idl_account: &arr[2],
            system_program: &arr[3],
            program_signer: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const ADMIN_SET_IDL_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    8, 217, 96, 231, 144, 104, 192, 5,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminSetIdlAuthorityIxArgs {
    pub idl_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetIdlAuthorityIxData(pub AdminSetIdlAuthorityIxArgs);
impl From<AdminSetIdlAuthorityIxArgs> for AdminSetIdlAuthorityIxData {
    fn from(args: AdminSetIdlAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl AdminSetIdlAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_SET_IDL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let idl_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AdminSetIdlAuthorityIxArgs {
                idl_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_SET_IDL_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.idl_authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_set_idl_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminSetIdlAuthorityKeys,
    args: AdminSetIdlAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AdminSetIdlAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_set_idl_authority_ix(
    keys: AdminSetIdlAuthorityKeys,
    args: AdminSetIdlAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    admin_set_idl_authority_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn admin_set_idl_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
) -> ProgramResult {
    let keys: AdminSetIdlAuthorityKeys = accounts.into();
    let ix = admin_set_idl_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_set_idl_authority_invoke(
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
) -> ProgramResult {
    admin_set_idl_authority_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn admin_set_idl_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminSetIdlAuthorityKeys = accounts.into();
    let ix = admin_set_idl_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_set_idl_authority_invoke_signed(
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_set_idl_authority_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn admin_set_idl_authority_verify_account_keys(
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    keys: AdminSetIdlAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.global.key, keys.global),
        (*accounts.idl_account.key, keys.idl_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program_signer.key, keys.program_signer),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_set_idl_authority_verify_writable_privileges<'me, 'info>(
    accounts: AdminSetIdlAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.idl_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_set_idl_authority_verify_signer_privileges<'me, 'info>(
    accounts: AdminSetIdlAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_set_idl_authority_verify_account_privileges<'me, 'info>(
    accounts: AdminSetIdlAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_set_idl_authority_verify_writable_privileges(accounts)?;
    admin_set_idl_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct AdminUpdateTokenIncentivesAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global_incentive_token_account: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminUpdateTokenIncentivesKeys {
    pub authority: Pubkey,
    pub global: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub mint: Pubkey,
    pub global_incentive_token_account: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AdminUpdateTokenIncentivesAccounts<'_, '_>>
for AdminUpdateTokenIncentivesKeys {
    fn from(accounts: AdminUpdateTokenIncentivesAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            global: *accounts.global.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            mint: *accounts.mint.key,
            global_incentive_token_account: *accounts.global_incentive_token_account.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AdminUpdateTokenIncentivesKeys>
for [AccountMeta; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminUpdateTokenIncentivesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_incentive_token_account,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]>
for AdminUpdateTokenIncentivesKeys {
    fn from(pubkeys: [Pubkey; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            global: pubkeys[1],
            global_volume_accumulator: pubkeys[2],
            mint: pubkeys[3],
            global_incentive_token_account: pubkeys[4],
            associated_token_program: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<AdminUpdateTokenIncentivesAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminUpdateTokenIncentivesAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.global.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.mint.clone(),
            accounts.global_incentive_token_account.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]>
for AdminUpdateTokenIncentivesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            global: &arr[1],
            global_volume_accumulator: &arr[2],
            mint: &arr[3],
            global_incentive_token_account: &arr[4],
            associated_token_program: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM: [u8; 8usize] = [
    209, 11, 115, 87, 213, 23, 124, 204,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminUpdateTokenIncentivesIxArgs {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub pump_token_supply_per_day: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateTokenIncentivesIxData(pub AdminUpdateTokenIncentivesIxArgs);
impl From<AdminUpdateTokenIncentivesIxArgs> for AdminUpdateTokenIncentivesIxData {
    fn from(args: AdminUpdateTokenIncentivesIxArgs) -> Self {
        Self(args)
    }
}
impl AdminUpdateTokenIncentivesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let start_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_in_a_day: i64 = crate::borsh_de_or_default(&mut reader)?;
        let day_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pump_token_supply_per_day: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(AdminUpdateTokenIncentivesIxArgs {
                start_time,
                end_time,
                seconds_in_a_day,
                day_number,
                pump_token_supply_per_day,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.seconds_in_a_day, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.day_number, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.pump_token_supply_per_day,
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
pub fn admin_update_token_incentives_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminUpdateTokenIncentivesKeys,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AdminUpdateTokenIncentivesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_update_token_incentives_ix(
    keys: AdminUpdateTokenIncentivesKeys,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> std::io::Result<Instruction> {
    admin_update_token_incentives_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn admin_update_token_incentives_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> ProgramResult {
    let keys: AdminUpdateTokenIncentivesKeys = accounts.into();
    let ix = admin_update_token_incentives_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_update_token_incentives_invoke(
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> ProgramResult {
    admin_update_token_incentives_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn admin_update_token_incentives_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminUpdateTokenIncentivesKeys = accounts.into();
    let ix = admin_update_token_incentives_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_update_token_incentives_invoke_signed(
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_update_token_incentives_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn admin_update_token_incentives_verify_account_keys(
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    keys: AdminUpdateTokenIncentivesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.global.key, keys.global),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.mint.key, keys.mint),
        (
            *accounts.global_incentive_token_account.key,
            keys.global_incentive_token_account,
        ),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
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
pub fn admin_update_token_incentives_verify_writable_privileges<'me, 'info>(
    accounts: AdminUpdateTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.global_volume_accumulator,
        accounts.global_incentive_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_update_token_incentives_verify_signer_privileges<'me, 'info>(
    accounts: AdminUpdateTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_update_token_incentives_verify_account_privileges<'me, 'info>(
    accounts: AdminUpdateTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_update_token_incentives_verify_writable_privileges(accounts)?;
    admin_update_token_incentives_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub creator_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            creator_vault: *accounts.creator_vault.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
        }
    }
}
impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
            AccountMeta {
                pubkey: keys.creator_vault,
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
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            fee_recipient: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            associated_user: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            creator_vault: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
            global_volume_accumulator: pubkeys[12],
            user_volume_accumulator: pubkeys[13],
            fee_config: pubkeys[14],
            fee_program: pubkeys[15],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.creator_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]>
for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            fee_recipient: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            associated_user: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            creator_vault: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
            global_volume_accumulator: &arr[12],
            user_volume_accumulator: &arr[13],
            fee_config: &arr[14],
            fee_program: &arr[15],
        }
    }
}
pub const BUY_IX_DISCM: [u8; 8usize] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyIxArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
    pub track_volume: OptionBool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyIxData(pub BuyIxArgs);
impl From<BuyIxArgs> for BuyIxData {
    fn from(args: BuyIxArgs) -> Self {
        Self(args)
    }
}
impl BuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_sol_cost: u64 = crate::borsh_de_or_default(&mut reader)?;
        let track_volume = if reader.is_empty() {
            Default::default()
        } else {
            <OptionBool>::deserialize(&mut reader)?
        };
        Ok(
            Self(BuyIxArgs {
                amount,
                max_sol_cost,
                track_volume,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_sol_cost, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.track_volume, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyKeys,
    args: BuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_ix(keys: BuyKeys, args: BuyIxArgs) -> std::io::Result<Instruction> {
    buy_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_invoke(accounts: BuyAccounts<'_, '_>, args: BuyIxArgs) -> ProgramResult {
    buy_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_invoke_signed(
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_verify_writable_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.creator_vault,
        accounts.user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_verify_signer_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_verify_account_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_verify_writable_privileges(accounts)?;
    buy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactQuoteInV2Accounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub associated_quote_fee_recipient: &'me AccountInfo<'info>,
    pub buyback_fee_recipient: &'me AccountInfo<'info>,
    pub associated_quote_buyback_fee_recipient: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_base_bonding_curve: &'me AccountInfo<'info>,
    pub associated_quote_bonding_curve: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_base_user: &'me AccountInfo<'info>,
    pub associated_quote_user: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub associated_creator_vault: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub associated_user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactQuoteInV2Keys {
    pub global: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub fee_recipient: Pubkey,
    pub associated_quote_fee_recipient: Pubkey,
    pub buyback_fee_recipient: Pubkey,
    pub associated_quote_buyback_fee_recipient: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_base_bonding_curve: Pubkey,
    pub associated_quote_bonding_curve: Pubkey,
    pub user: Pubkey,
    pub associated_base_user: Pubkey,
    pub associated_quote_user: Pubkey,
    pub creator_vault: Pubkey,
    pub associated_creator_vault: Pubkey,
    pub sharing_config: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub associated_user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyExactQuoteInV2Accounts<'_, '_>> for BuyExactQuoteInV2Keys {
    fn from(accounts: BuyExactQuoteInV2Accounts) -> Self {
        Self {
            global: *accounts.global.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            fee_recipient: *accounts.fee_recipient.key,
            associated_quote_fee_recipient: *accounts.associated_quote_fee_recipient.key,
            buyback_fee_recipient: *accounts.buyback_fee_recipient.key,
            associated_quote_buyback_fee_recipient: *accounts
                .associated_quote_buyback_fee_recipient
                .key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_base_bonding_curve: *accounts.associated_base_bonding_curve.key,
            associated_quote_bonding_curve: *accounts.associated_quote_bonding_curve.key,
            user: *accounts.user.key,
            associated_base_user: *accounts.associated_base_user.key,
            associated_quote_user: *accounts.associated_quote_user.key,
            creator_vault: *accounts.creator_vault.key,
            associated_creator_vault: *accounts.associated_creator_vault.key,
            sharing_config: *accounts.sharing_config.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            associated_user_volume_accumulator: *accounts
                .associated_user_volume_accumulator
                .key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyExactQuoteInV2Keys>
for [AccountMeta; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactQuoteInV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyback_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_buyback_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
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
impl From<[Pubkey; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN]> for BuyExactQuoteInV2Keys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            base_token_program: pubkeys[3],
            quote_token_program: pubkeys[4],
            associated_token_program: pubkeys[5],
            fee_recipient: pubkeys[6],
            associated_quote_fee_recipient: pubkeys[7],
            buyback_fee_recipient: pubkeys[8],
            associated_quote_buyback_fee_recipient: pubkeys[9],
            bonding_curve: pubkeys[10],
            associated_base_bonding_curve: pubkeys[11],
            associated_quote_bonding_curve: pubkeys[12],
            user: pubkeys[13],
            associated_base_user: pubkeys[14],
            associated_quote_user: pubkeys[15],
            creator_vault: pubkeys[16],
            associated_creator_vault: pubkeys[17],
            sharing_config: pubkeys[18],
            global_volume_accumulator: pubkeys[19],
            user_volume_accumulator: pubkeys[20],
            associated_user_volume_accumulator: pubkeys[21],
            fee_config: pubkeys[22],
            fee_program: pubkeys[23],
            system_program: pubkeys[24],
            event_authority: pubkeys[25],
            program: pubkeys[26],
        }
    }
}
impl<'info> From<BuyExactQuoteInV2Accounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactQuoteInV2Accounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.fee_recipient.clone(),
            accounts.associated_quote_fee_recipient.clone(),
            accounts.buyback_fee_recipient.clone(),
            accounts.associated_quote_buyback_fee_recipient.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_base_bonding_curve.clone(),
            accounts.associated_quote_bonding_curve.clone(),
            accounts.user.clone(),
            accounts.associated_base_user.clone(),
            accounts.associated_quote_user.clone(),
            accounts.creator_vault.clone(),
            accounts.associated_creator_vault.clone(),
            accounts.sharing_config.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.associated_user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN]>
for BuyExactQuoteInV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            base_token_program: &arr[3],
            quote_token_program: &arr[4],
            associated_token_program: &arr[5],
            fee_recipient: &arr[6],
            associated_quote_fee_recipient: &arr[7],
            buyback_fee_recipient: &arr[8],
            associated_quote_buyback_fee_recipient: &arr[9],
            bonding_curve: &arr[10],
            associated_base_bonding_curve: &arr[11],
            associated_quote_bonding_curve: &arr[12],
            user: &arr[13],
            associated_base_user: &arr[14],
            associated_quote_user: &arr[15],
            creator_vault: &arr[16],
            associated_creator_vault: &arr[17],
            sharing_config: &arr[18],
            global_volume_accumulator: &arr[19],
            user_volume_accumulator: &arr[20],
            associated_user_volume_accumulator: &arr[21],
            fee_config: &arr[22],
            fee_program: &arr[23],
            system_program: &arr[24],
            event_authority: &arr[25],
            program: &arr[26],
        }
    }
}
pub const BUY_EXACT_QUOTE_IN_V2_IX_DISCM: [u8; 8usize] = [
    194, 171, 28, 70, 104, 77, 91, 47,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactQuoteInV2IxArgs {
    pub spendable_quote_in: u64,
    pub min_tokens_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactQuoteInV2IxData(pub BuyExactQuoteInV2IxArgs);
impl From<BuyExactQuoteInV2IxArgs> for BuyExactQuoteInV2IxData {
    fn from(args: BuyExactQuoteInV2IxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactQuoteInV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_QUOTE_IN_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let spendable_quote_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyExactQuoteInV2IxArgs {
                spendable_quote_in,
                min_tokens_out,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_QUOTE_IN_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.spendable_quote_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_tokens_out, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_quote_in_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactQuoteInV2Keys,
    args: BuyExactQuoteInV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_QUOTE_IN_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactQuoteInV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_quote_in_v2_ix(
    keys: BuyExactQuoteInV2Keys,
    args: BuyExactQuoteInV2IxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_quote_in_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn buy_exact_quote_in_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactQuoteInV2Accounts<'_, '_>,
    args: BuyExactQuoteInV2IxArgs,
) -> ProgramResult {
    let keys: BuyExactQuoteInV2Keys = accounts.into();
    let ix = buy_exact_quote_in_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_quote_in_v2_invoke(
    accounts: BuyExactQuoteInV2Accounts<'_, '_>,
    args: BuyExactQuoteInV2IxArgs,
) -> ProgramResult {
    buy_exact_quote_in_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_quote_in_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactQuoteInV2Accounts<'_, '_>,
    args: BuyExactQuoteInV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactQuoteInV2Keys = accounts.into();
    let ix = buy_exact_quote_in_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_quote_in_v2_invoke_signed(
    accounts: BuyExactQuoteInV2Accounts<'_, '_>,
    args: BuyExactQuoteInV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_quote_in_v2_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_quote_in_v2_verify_account_keys(
    accounts: BuyExactQuoteInV2Accounts<'_, '_>,
    keys: BuyExactQuoteInV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (
            *accounts.associated_quote_fee_recipient.key,
            keys.associated_quote_fee_recipient,
        ),
        (*accounts.buyback_fee_recipient.key, keys.buyback_fee_recipient),
        (
            *accounts.associated_quote_buyback_fee_recipient.key,
            keys.associated_quote_buyback_fee_recipient,
        ),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.associated_base_bonding_curve.key,
            keys.associated_base_bonding_curve,
        ),
        (
            *accounts.associated_quote_bonding_curve.key,
            keys.associated_quote_bonding_curve,
        ),
        (*accounts.user.key, keys.user),
        (*accounts.associated_base_user.key, keys.associated_base_user),
        (*accounts.associated_quote_user.key, keys.associated_quote_user),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.associated_creator_vault.key, keys.associated_creator_vault),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (
            *accounts.associated_user_volume_accumulator.key,
            keys.associated_user_volume_accumulator,
        ),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
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
pub fn buy_exact_quote_in_v2_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactQuoteInV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.associated_quote_fee_recipient,
        accounts.buyback_fee_recipient,
        accounts.associated_quote_buyback_fee_recipient,
        accounts.bonding_curve,
        accounts.associated_base_bonding_curve,
        accounts.associated_quote_bonding_curve,
        accounts.user,
        accounts.associated_base_user,
        accounts.associated_quote_user,
        accounts.creator_vault,
        accounts.associated_creator_vault,
        accounts.user_volume_accumulator,
        accounts.associated_user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_quote_in_v2_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactQuoteInV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_quote_in_v2_verify_account_privileges<'me, 'info>(
    accounts: BuyExactQuoteInV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_quote_in_v2_verify_writable_privileges(accounts)?;
    buy_exact_quote_in_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactSolInAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactSolInKeys {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub creator_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
}
impl From<BuyExactSolInAccounts<'_, '_>> for BuyExactSolInKeys {
    fn from(accounts: BuyExactSolInAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            creator_vault: *accounts.creator_vault.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
        }
    }
}
impl From<BuyExactSolInKeys> for [AccountMeta; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactSolInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
            AccountMeta {
                pubkey: keys.creator_vault,
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
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]> for BuyExactSolInKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            fee_recipient: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            associated_user: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            creator_vault: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
            global_volume_accumulator: pubkeys[12],
            user_volume_accumulator: pubkeys[13],
            fee_config: pubkeys[14],
            fee_program: pubkeys[15],
        }
    }
}
impl<'info> From<BuyExactSolInAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactSolInAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.creator_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]>
for BuyExactSolInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            fee_recipient: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            associated_user: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            creator_vault: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
            global_volume_accumulator: &arr[12],
            user_volume_accumulator: &arr[13],
            fee_config: &arr[14],
            fee_program: &arr[15],
        }
    }
}
pub const BUY_EXACT_SOL_IN_IX_DISCM: [u8; 8usize] = [56, 252, 116, 8, 158, 223, 205, 95];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactSolInIxArgs {
    pub spendable_sol_in: u64,
    pub min_tokens_out: u64,
    pub track_volume: OptionBool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactSolInIxData(pub BuyExactSolInIxArgs);
impl From<BuyExactSolInIxArgs> for BuyExactSolInIxData {
    fn from(args: BuyExactSolInIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactSolInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_SOL_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let spendable_sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_tokens_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let track_volume = if reader.is_empty() {
            Default::default()
        } else {
            <OptionBool>::deserialize(&mut reader)?
        };
        Ok(
            Self(BuyExactSolInIxArgs {
                spendable_sol_in,
                min_tokens_out,
                track_volume,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_SOL_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.spendable_sol_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_tokens_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.track_volume, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_sol_in_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactSolInKeys,
    args: BuyExactSolInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactSolInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_sol_in_ix(
    keys: BuyExactSolInKeys,
    args: BuyExactSolInIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_sol_in_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn buy_exact_sol_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
) -> ProgramResult {
    let keys: BuyExactSolInKeys = accounts.into();
    let ix = buy_exact_sol_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_sol_in_invoke(
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
) -> ProgramResult {
    buy_exact_sol_in_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_sol_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactSolInKeys = accounts.into();
    let ix = buy_exact_sol_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_sol_in_invoke_signed(
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_sol_in_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_sol_in_verify_account_keys(
    accounts: BuyExactSolInAccounts<'_, '_>,
    keys: BuyExactSolInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_sol_in_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactSolInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.creator_vault,
        accounts.user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_sol_in_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactSolInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_sol_in_verify_account_privileges<'me, 'info>(
    accounts: BuyExactSolInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_sol_in_verify_writable_privileges(accounts)?;
    buy_exact_sol_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_V2_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct BuyV2Accounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub associated_quote_fee_recipient: &'me AccountInfo<'info>,
    pub buyback_fee_recipient: &'me AccountInfo<'info>,
    pub associated_quote_buyback_fee_recipient: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_base_bonding_curve: &'me AccountInfo<'info>,
    pub associated_quote_bonding_curve: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_base_user: &'me AccountInfo<'info>,
    pub associated_quote_user: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub associated_creator_vault: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub associated_user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyV2Keys {
    pub global: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub fee_recipient: Pubkey,
    pub associated_quote_fee_recipient: Pubkey,
    pub buyback_fee_recipient: Pubkey,
    pub associated_quote_buyback_fee_recipient: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_base_bonding_curve: Pubkey,
    pub associated_quote_bonding_curve: Pubkey,
    pub user: Pubkey,
    pub associated_base_user: Pubkey,
    pub associated_quote_user: Pubkey,
    pub creator_vault: Pubkey,
    pub associated_creator_vault: Pubkey,
    pub sharing_config: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub associated_user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyV2Accounts<'_, '_>> for BuyV2Keys {
    fn from(accounts: BuyV2Accounts) -> Self {
        Self {
            global: *accounts.global.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            fee_recipient: *accounts.fee_recipient.key,
            associated_quote_fee_recipient: *accounts.associated_quote_fee_recipient.key,
            buyback_fee_recipient: *accounts.buyback_fee_recipient.key,
            associated_quote_buyback_fee_recipient: *accounts
                .associated_quote_buyback_fee_recipient
                .key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_base_bonding_curve: *accounts.associated_base_bonding_curve.key,
            associated_quote_bonding_curve: *accounts.associated_quote_bonding_curve.key,
            user: *accounts.user.key,
            associated_base_user: *accounts.associated_base_user.key,
            associated_quote_user: *accounts.associated_quote_user.key,
            creator_vault: *accounts.creator_vault.key,
            associated_creator_vault: *accounts.associated_creator_vault.key,
            sharing_config: *accounts.sharing_config.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            associated_user_volume_accumulator: *accounts
                .associated_user_volume_accumulator
                .key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyV2Keys> for [AccountMeta; BUY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyback_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_buyback_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
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
impl From<[Pubkey; BUY_V2_IX_ACCOUNTS_LEN]> for BuyV2Keys {
    fn from(pubkeys: [Pubkey; BUY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            base_token_program: pubkeys[3],
            quote_token_program: pubkeys[4],
            associated_token_program: pubkeys[5],
            fee_recipient: pubkeys[6],
            associated_quote_fee_recipient: pubkeys[7],
            buyback_fee_recipient: pubkeys[8],
            associated_quote_buyback_fee_recipient: pubkeys[9],
            bonding_curve: pubkeys[10],
            associated_base_bonding_curve: pubkeys[11],
            associated_quote_bonding_curve: pubkeys[12],
            user: pubkeys[13],
            associated_base_user: pubkeys[14],
            associated_quote_user: pubkeys[15],
            creator_vault: pubkeys[16],
            associated_creator_vault: pubkeys[17],
            sharing_config: pubkeys[18],
            global_volume_accumulator: pubkeys[19],
            user_volume_accumulator: pubkeys[20],
            associated_user_volume_accumulator: pubkeys[21],
            fee_config: pubkeys[22],
            fee_program: pubkeys[23],
            system_program: pubkeys[24],
            event_authority: pubkeys[25],
            program: pubkeys[26],
        }
    }
}
impl<'info> From<BuyV2Accounts<'_, 'info>>
for [AccountInfo<'info>; BUY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyV2Accounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.fee_recipient.clone(),
            accounts.associated_quote_fee_recipient.clone(),
            accounts.buyback_fee_recipient.clone(),
            accounts.associated_quote_buyback_fee_recipient.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_base_bonding_curve.clone(),
            accounts.associated_quote_bonding_curve.clone(),
            accounts.user.clone(),
            accounts.associated_base_user.clone(),
            accounts.associated_quote_user.clone(),
            accounts.creator_vault.clone(),
            accounts.associated_creator_vault.clone(),
            accounts.sharing_config.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.associated_user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_V2_IX_ACCOUNTS_LEN]>
for BuyV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            base_token_program: &arr[3],
            quote_token_program: &arr[4],
            associated_token_program: &arr[5],
            fee_recipient: &arr[6],
            associated_quote_fee_recipient: &arr[7],
            buyback_fee_recipient: &arr[8],
            associated_quote_buyback_fee_recipient: &arr[9],
            bonding_curve: &arr[10],
            associated_base_bonding_curve: &arr[11],
            associated_quote_bonding_curve: &arr[12],
            user: &arr[13],
            associated_base_user: &arr[14],
            associated_quote_user: &arr[15],
            creator_vault: &arr[16],
            associated_creator_vault: &arr[17],
            sharing_config: &arr[18],
            global_volume_accumulator: &arr[19],
            user_volume_accumulator: &arr[20],
            associated_user_volume_accumulator: &arr[21],
            fee_config: &arr[22],
            fee_program: &arr[23],
            system_program: &arr[24],
            event_authority: &arr[25],
            program: &arr[26],
        }
    }
}
pub const BUY_V2_IX_DISCM: [u8; 8usize] = [184, 23, 238, 97, 103, 197, 211, 61];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyV2IxArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyV2IxData(pub BuyV2IxArgs);
impl From<BuyV2IxArgs> for BuyV2IxData {
    fn from(args: BuyV2IxArgs) -> Self {
        Self(args)
    }
}
impl BuyV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_sol_cost: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyV2IxArgs {
                amount,
                max_sol_cost,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_sol_cost, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyV2Keys,
    args: BuyV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_v2_ix(keys: BuyV2Keys, args: BuyV2IxArgs) -> std::io::Result<Instruction> {
    buy_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn buy_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyV2Accounts<'_, '_>,
    args: BuyV2IxArgs,
) -> ProgramResult {
    let keys: BuyV2Keys = accounts.into();
    let ix = buy_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_v2_invoke(
    accounts: BuyV2Accounts<'_, '_>,
    args: BuyV2IxArgs,
) -> ProgramResult {
    buy_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn buy_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyV2Accounts<'_, '_>,
    args: BuyV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyV2Keys = accounts.into();
    let ix = buy_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_v2_invoke_signed(
    accounts: BuyV2Accounts<'_, '_>,
    args: BuyV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_v2_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_v2_verify_account_keys(
    accounts: BuyV2Accounts<'_, '_>,
    keys: BuyV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (
            *accounts.associated_quote_fee_recipient.key,
            keys.associated_quote_fee_recipient,
        ),
        (*accounts.buyback_fee_recipient.key, keys.buyback_fee_recipient),
        (
            *accounts.associated_quote_buyback_fee_recipient.key,
            keys.associated_quote_buyback_fee_recipient,
        ),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.associated_base_bonding_curve.key,
            keys.associated_base_bonding_curve,
        ),
        (
            *accounts.associated_quote_bonding_curve.key,
            keys.associated_quote_bonding_curve,
        ),
        (*accounts.user.key, keys.user),
        (*accounts.associated_base_user.key, keys.associated_base_user),
        (*accounts.associated_quote_user.key, keys.associated_quote_user),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.associated_creator_vault.key, keys.associated_creator_vault),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (
            *accounts.associated_user_volume_accumulator.key,
            keys.associated_user_volume_accumulator,
        ),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
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
pub fn buy_v2_verify_writable_privileges<'me, 'info>(
    accounts: BuyV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.associated_quote_fee_recipient,
        accounts.buyback_fee_recipient,
        accounts.associated_quote_buyback_fee_recipient,
        accounts.bonding_curve,
        accounts.associated_base_bonding_curve,
        accounts.associated_quote_bonding_curve,
        accounts.user,
        accounts.associated_base_user,
        accounts.associated_quote_user,
        accounts.creator_vault,
        accounts.associated_creator_vault,
        accounts.user_volume_accumulator,
        accounts.associated_user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_v2_verify_signer_privileges<'me, 'info>(
    accounts: BuyV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_v2_verify_account_privileges<'me, 'info>(
    accounts: BuyV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_v2_verify_writable_privileges(accounts)?;
    buy_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_CASHBACK_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCashbackAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCashbackKeys {
    pub user: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimCashbackAccounts<'_, '_>> for ClaimCashbackKeys {
    fn from(accounts: ClaimCashbackAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimCashbackKeys> for [AccountMeta; CLAIM_CASHBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCashbackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; CLAIM_CASHBACK_IX_ACCOUNTS_LEN]> for ClaimCashbackKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CASHBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_volume_accumulator: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ClaimCashbackAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CASHBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCashbackAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CASHBACK_IX_ACCOUNTS_LEN]>
for ClaimCashbackAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_CASHBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_volume_accumulator: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLAIM_CASHBACK_IX_DISCM: [u8; 8usize] = [37, 58, 35, 126, 190, 53, 228, 197];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCashbackIxData;
impl ClaimCashbackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CASHBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CASHBACK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_cashback_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCashbackKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CASHBACK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCashbackIxData.try_to_vec()?,
    })
}
pub fn claim_cashback_ix(keys: ClaimCashbackKeys) -> std::io::Result<Instruction> {
    claim_cashback_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn claim_cashback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCashbackAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCashbackKeys = accounts.into();
    let ix = claim_cashback_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_cashback_invoke(accounts: ClaimCashbackAccounts<'_, '_>) -> ProgramResult {
    claim_cashback_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn claim_cashback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCashbackAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCashbackKeys = accounts.into();
    let ix = claim_cashback_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_cashback_invoke_signed(
    accounts: ClaimCashbackAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_cashback_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn claim_cashback_verify_account_keys(
    accounts: ClaimCashbackAccounts<'_, '_>,
    keys: ClaimCashbackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
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
pub fn claim_cashback_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCashbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_cashback_verify_account_privileges<'me, 'info>(
    accounts: ClaimCashbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_cashback_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCashbackV2Accounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub associated_user_volume_accumulator: &'me AccountInfo<'info>,
    pub associated_quote_user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCashbackV2Keys {
    pub user: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub associated_user_volume_accumulator: Pubkey,
    pub associated_quote_user: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimCashbackV2Accounts<'_, '_>> for ClaimCashbackV2Keys {
    fn from(accounts: ClaimCashbackV2Accounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            associated_user_volume_accumulator: *accounts
                .associated_user_volume_accumulator
                .key,
            associated_quote_user: *accounts.associated_quote_user.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimCashbackV2Keys> for [AccountMeta; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCashbackV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_user,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN]> for ClaimCashbackV2Keys {
    fn from(pubkeys: [Pubkey; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_volume_accumulator: pubkeys[1],
            quote_mint: pubkeys[2],
            quote_token_program: pubkeys[3],
            associated_token_program: pubkeys[4],
            associated_user_volume_accumulator: pubkeys[5],
            associated_quote_user: pubkeys[6],
            system_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimCashbackV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCashbackV2Accounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.associated_user_volume_accumulator.clone(),
            accounts.associated_quote_user.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN]>
for ClaimCashbackV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_volume_accumulator: &arr[1],
            quote_mint: &arr[2],
            quote_token_program: &arr[3],
            associated_token_program: &arr[4],
            associated_user_volume_accumulator: &arr[5],
            associated_quote_user: &arr[6],
            system_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CLAIM_CASHBACK_V2_IX_DISCM: [u8; 8usize] = [
    122, 243, 204, 65, 94, 116, 29, 55,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCashbackV2IxData;
impl ClaimCashbackV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CASHBACK_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CASHBACK_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_cashback_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCashbackV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CASHBACK_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCashbackV2IxData.try_to_vec()?,
    })
}
pub fn claim_cashback_v2_ix(keys: ClaimCashbackV2Keys) -> std::io::Result<Instruction> {
    claim_cashback_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn claim_cashback_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCashbackV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCashbackV2Keys = accounts.into();
    let ix = claim_cashback_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_cashback_v2_invoke(
    accounts: ClaimCashbackV2Accounts<'_, '_>,
) -> ProgramResult {
    claim_cashback_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn claim_cashback_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCashbackV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCashbackV2Keys = accounts.into();
    let ix = claim_cashback_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_cashback_v2_invoke_signed(
    accounts: ClaimCashbackV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_cashback_v2_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn claim_cashback_v2_verify_account_keys(
    accounts: ClaimCashbackV2Accounts<'_, '_>,
    keys: ClaimCashbackV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (
            *accounts.associated_user_volume_accumulator.key,
            keys.associated_user_volume_accumulator,
        ),
        (*accounts.associated_quote_user.key, keys.associated_quote_user),
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
pub fn claim_cashback_v2_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCashbackV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_volume_accumulator,
        accounts.associated_user_volume_accumulator,
        accounts.associated_quote_user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_cashback_v2_verify_account_privileges<'me, 'info>(
    accounts: ClaimCashbackV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_cashback_v2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenIncentivesAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub global_incentive_token_account: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenIncentivesKeys {
    pub user: Pubkey,
    pub user_ata: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub global_incentive_token_account: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub payer: Pubkey,
}
impl From<ClaimTokenIncentivesAccounts<'_, '_>> for ClaimTokenIncentivesKeys {
    fn from(accounts: ClaimTokenIncentivesAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_ata: *accounts.user_ata.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            global_incentive_token_account: *accounts.global_incentive_token_account.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<ClaimTokenIncentivesKeys>
for [AccountMeta; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenIncentivesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_incentive_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
                pubkey: keys.associated_token_program,
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
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]>
for ClaimTokenIncentivesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_ata: pubkeys[1],
            global_volume_accumulator: pubkeys[2],
            global_incentive_token_account: pubkeys[3],
            user_volume_accumulator: pubkeys[4],
            mint: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
            payer: pubkeys[11],
        }
    }
}
impl<'info> From<ClaimTokenIncentivesAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTokenIncentivesAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_ata.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.global_incentive_token_account.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.payer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]>
for ClaimTokenIncentivesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_ata: &arr[1],
            global_volume_accumulator: &arr[2],
            global_incentive_token_account: &arr[3],
            user_volume_accumulator: &arr[4],
            mint: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
            payer: &arr[11],
        }
    }
}
pub const CLAIM_TOKEN_INCENTIVES_IX_DISCM: [u8; 8usize] = [
    16, 4, 71, 28, 204, 1, 40, 27,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIncentivesIxData;
impl ClaimTokenIncentivesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_INCENTIVES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_INCENTIVES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_token_incentives_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenIncentivesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimTokenIncentivesIxData.try_to_vec()?,
    })
}
pub fn claim_token_incentives_ix(
    keys: ClaimTokenIncentivesKeys,
) -> std::io::Result<Instruction> {
    claim_token_incentives_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn claim_token_incentives_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimTokenIncentivesKeys = accounts.into();
    let ix = claim_token_incentives_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_token_incentives_invoke(
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
) -> ProgramResult {
    claim_token_incentives_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn claim_token_incentives_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenIncentivesKeys = accounts.into();
    let ix = claim_token_incentives_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_token_incentives_invoke_signed(
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_incentives_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_token_incentives_verify_account_keys(
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
    keys: ClaimTokenIncentivesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (
            *accounts.global_incentive_token_account.key,
            keys.global_incentive_token_account,
        ),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_token_incentives_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_ata,
        accounts.global_incentive_token_account,
        accounts.user_volume_accumulator,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_token_incentives_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_token_incentives_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_incentives_verify_writable_privileges(accounts)?;
    claim_token_incentives_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CloseUserVolumeAccumulatorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseUserVolumeAccumulatorKeys {
    pub user: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseUserVolumeAccumulatorAccounts<'_, '_>>
for CloseUserVolumeAccumulatorKeys {
    fn from(accounts: CloseUserVolumeAccumulatorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseUserVolumeAccumulatorKeys>
for [AccountMeta; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseUserVolumeAccumulatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
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
impl From<[Pubkey; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]>
for CloseUserVolumeAccumulatorKeys {
    fn from(pubkeys: [Pubkey; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_volume_accumulator: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<CloseUserVolumeAccumulatorAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseUserVolumeAccumulatorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]>
for CloseUserVolumeAccumulatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            user_volume_accumulator: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM: [u8; 8usize] = [
    249, 69, 164, 218, 150, 103, 84, 138,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseUserVolumeAccumulatorIxData;
impl CloseUserVolumeAccumulatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_user_volume_accumulator_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseUserVolumeAccumulatorIxData.try_to_vec()?,
    })
}
pub fn close_user_volume_accumulator_ix(
    keys: CloseUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    close_user_volume_accumulator_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn close_user_volume_accumulator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseUserVolumeAccumulatorKeys = accounts.into();
    let ix = close_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_user_volume_accumulator_invoke(
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    close_user_volume_accumulator_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn close_user_volume_accumulator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseUserVolumeAccumulatorKeys = accounts.into();
    let ix = close_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_user_volume_accumulator_invoke_signed(
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_user_volume_accumulator_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_user_volume_accumulator_verify_account_keys(
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
    keys: CloseUserVolumeAccumulatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_user_volume_accumulator_verify_writable_privileges<'me, 'info>(
    accounts: CloseUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_user_volume_accumulator_verify_signer_privileges<'me, 'info>(
    accounts: CloseUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_user_volume_accumulator_verify_account_privileges<'me, 'info>(
    accounts: CloseUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_user_volume_accumulator_verify_writable_privileges(accounts)?;
    close_user_volume_accumulator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CollectCreatorFeeAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeKeys {
    pub creator: Pubkey,
    pub creator_vault: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CollectCreatorFeeAccounts<'_, '_>> for CollectCreatorFeeKeys {
    fn from(accounts: CollectCreatorFeeAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            creator_vault: *accounts.creator_vault.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CollectCreatorFeeKeys> for [AccountMeta; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]> for CollectCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            creator_vault: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CollectCreatorFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.creator_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for CollectCreatorFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            creator_vault: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const COLLECT_CREATOR_FEE_IX_DISCM: [u8; 8usize] = [
    20, 22, 86, 123, 198, 28, 219, 132,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeIxData;
impl CollectCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_CREATOR_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectCreatorFeeIxData.try_to_vec()?,
    })
}
pub fn collect_creator_fee_ix(
    keys: CollectCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    collect_creator_fee_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn collect_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectCreatorFeeKeys = accounts.into();
    let ix = collect_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_creator_fee_invoke(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    collect_creator_fee_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn collect_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectCreatorFeeKeys = accounts.into();
    let ix = collect_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_creator_fee_invoke_signed(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_creator_fee_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn collect_creator_fee_verify_account_keys(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    keys: CollectCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.creator_vault.key, keys.creator_vault),
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
pub fn collect_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.creator, accounts.creator_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_creator_fee_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CollectCreatorFeeV2Accounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub creator_token_account: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub creator_vault_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeV2Keys {
    pub creator: Pubkey,
    pub creator_token_account: Pubkey,
    pub creator_vault: Pubkey,
    pub creator_vault_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CollectCreatorFeeV2Accounts<'_, '_>> for CollectCreatorFeeV2Keys {
    fn from(accounts: CollectCreatorFeeV2Accounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            creator_token_account: *accounts.creator_token_account.key,
            creator_vault: *accounts.creator_vault.key,
            creator_vault_token_account: *accounts.creator_vault_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CollectCreatorFeeV2Keys>
for [AccountMeta; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectCreatorFeeV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN]> for CollectCreatorFeeV2Keys {
    fn from(pubkeys: [Pubkey; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            creator_token_account: pubkeys[1],
            creator_vault: pubkeys[2],
            creator_vault_token_account: pubkeys[3],
            quote_mint: pubkeys[4],
            quote_token_program: pubkeys[5],
            associated_token_program: pubkeys[6],
            system_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<CollectCreatorFeeV2Accounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectCreatorFeeV2Accounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.creator_token_account.clone(),
            accounts.creator_vault.clone(),
            accounts.creator_vault_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN]>
for CollectCreatorFeeV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            creator_token_account: &arr[1],
            creator_vault: &arr[2],
            creator_vault_token_account: &arr[3],
            quote_mint: &arr[4],
            quote_token_program: &arr[5],
            associated_token_program: &arr[6],
            system_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const COLLECT_CREATOR_FEE_V2_IX_DISCM: [u8; 8usize] = [
    207, 17, 138, 242, 4, 34, 19, 56,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeV2IxData;
impl CollectCreatorFeeV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_CREATOR_FEE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_CREATOR_FEE_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_creator_fee_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectCreatorFeeV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_CREATOR_FEE_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectCreatorFeeV2IxData.try_to_vec()?,
    })
}
pub fn collect_creator_fee_v2_ix(
    keys: CollectCreatorFeeV2Keys,
) -> std::io::Result<Instruction> {
    collect_creator_fee_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn collect_creator_fee_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectCreatorFeeV2Keys = accounts.into();
    let ix = collect_creator_fee_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_creator_fee_v2_invoke(
    accounts: CollectCreatorFeeV2Accounts<'_, '_>,
) -> ProgramResult {
    collect_creator_fee_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn collect_creator_fee_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectCreatorFeeV2Keys = accounts.into();
    let ix = collect_creator_fee_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_creator_fee_v2_invoke_signed(
    accounts: CollectCreatorFeeV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_creator_fee_v2_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_creator_fee_v2_verify_account_keys(
    accounts: CollectCreatorFeeV2Accounts<'_, '_>,
    keys: CollectCreatorFeeV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.creator_token_account.key, keys.creator_token_account),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.creator_vault_token_account.key, keys.creator_vault_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
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
pub fn collect_creator_fee_v2_verify_writable_privileges<'me, 'info>(
    accounts: CollectCreatorFeeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.creator_token_account,
        accounts.creator_vault,
        accounts.creator_vault_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_v2_verify_account_privileges<'me, 'info>(
    accounts: CollectCreatorFeeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_creator_fee_v2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CREATE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mpl_token_metadata: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub global: Pubkey,
    pub mpl_token_metadata: Pubkey,
    pub metadata: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            global: *accounts.global.key,
            mpl_token_metadata: *accounts.mpl_token_metadata.key,
            metadata: *accounts.metadata.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mpl_token_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            mint_authority: pubkeys[1],
            bonding_curve: pubkeys[2],
            associated_bonding_curve: pubkeys[3],
            global: pubkeys[4],
            mpl_token_metadata: pubkeys[5],
            metadata: pubkeys[6],
            user: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.global.clone(),
            accounts.mpl_token_metadata.clone(),
            accounts.metadata.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            mint_authority: &arr[1],
            bonding_curve: &arr[2],
            associated_bonding_curve: &arr[3],
            global: &arr[4],
            mpl_token_metadata: &arr[5],
            metadata: &arr[6],
            user: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CREATE_IX_DISCM: [u8; 8usize] = [24, 30, 200, 40, 5, 28, 7, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIxData(pub CreateIxArgs);
impl From<CreateIxArgs> for CreateIxData {
    fn from(args: CreateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateIxArgs {
                name,
                symbol,
                uri,
                creator,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateKeys,
    args: CreateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_ix(keys: CreateKeys, args: CreateIxArgs) -> std::io::Result<Instruction> {
    create_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn create_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_invoke(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    create_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn create_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_invoke_signed(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.global.key, keys.global),
        (*accounts.mpl_token_metadata.key, keys.mpl_token_metadata),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.mint,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.metadata,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.mint, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_verify_account_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_verify_writable_privileges(accounts)?;
    create_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_V2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CreateV2Accounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub mayhem_program_id: &'me AccountInfo<'info>,
    pub global_params: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub mayhem_state: &'me AccountInfo<'info>,
    pub mayhem_token_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateV2Keys {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub global: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub mayhem_program_id: Pubkey,
    pub global_params: Pubkey,
    pub sol_vault: Pubkey,
    pub mayhem_state: Pubkey,
    pub mayhem_token_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateV2Accounts<'_, '_>> for CreateV2Keys {
    fn from(accounts: CreateV2Accounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            global: *accounts.global.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            mayhem_program_id: *accounts.mayhem_program_id.key,
            global_params: *accounts.global_params.key,
            sol_vault: *accounts.sol_vault.key,
            mayhem_state: *accounts.mayhem_state.key,
            mayhem_token_vault: *accounts.mayhem_token_vault.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateV2Keys> for [AccountMeta; CREATE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
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
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mayhem_program_id,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_params,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mayhem_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mayhem_token_vault,
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
impl From<[Pubkey; CREATE_V2_IX_ACCOUNTS_LEN]> for CreateV2Keys {
    fn from(pubkeys: [Pubkey; CREATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            mint_authority: pubkeys[1],
            bonding_curve: pubkeys[2],
            associated_bonding_curve: pubkeys[3],
            global: pubkeys[4],
            user: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            mayhem_program_id: pubkeys[9],
            global_params: pubkeys[10],
            sol_vault: pubkeys[11],
            mayhem_state: pubkeys[12],
            mayhem_token_vault: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<CreateV2Accounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateV2Accounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.global.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.mayhem_program_id.clone(),
            accounts.global_params.clone(),
            accounts.sol_vault.clone(),
            accounts.mayhem_state.clone(),
            accounts.mayhem_token_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_V2_IX_ACCOUNTS_LEN]>
for CreateV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            mint_authority: &arr[1],
            bonding_curve: &arr[2],
            associated_bonding_curve: &arr[3],
            global: &arr[4],
            user: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            mayhem_program_id: &arr[9],
            global_params: &arr[10],
            sol_vault: &arr[11],
            mayhem_state: &arr[12],
            mayhem_token_vault: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const CREATE_V2_IX_DISCM: [u8; 8usize] = [214, 144, 76, 236, 95, 139, 49, 180];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateV2IxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub is_cashback_enabled: OptionBool,
    pub creator_fee_bps: OptionU64,
    pub is_holder_reward: OptionBool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateV2IxData(pub CreateV2IxArgs);
impl From<CreateV2IxArgs> for CreateV2IxData {
    fn from(args: CreateV2IxArgs) -> Self {
        Self(args)
    }
}
impl CreateV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_mayhem_mode: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_cashback_enabled = if reader.is_empty() {
            Default::default()
        } else {
            <OptionBool>::deserialize(&mut reader)?
        };
        let creator_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <OptionU64>::deserialize(&mut reader)?
        };
        let is_holder_reward = if reader.is_empty() {
            Default::default()
        } else {
            <OptionBool>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreateV2IxArgs {
                name,
                symbol,
                uri,
                creator,
                is_mayhem_mode,
                is_cashback_enabled,
                creator_fee_bps,
                is_holder_reward,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.symbol, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.uri, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_mayhem_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_cashback_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.is_holder_reward, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateV2Keys,
    args: CreateV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_v2_ix(
    keys: CreateV2Keys,
    args: CreateV2IxArgs,
) -> std::io::Result<Instruction> {
    create_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn create_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
) -> ProgramResult {
    let keys: CreateV2Keys = accounts.into();
    let ix = create_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_v2_invoke(
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
) -> ProgramResult {
    create_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn create_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateV2Keys = accounts.into();
    let ix = create_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_v2_invoke_signed(
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_v2_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_v2_verify_account_keys(
    accounts: CreateV2Accounts<'_, '_>,
    keys: CreateV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.global.key, keys.global),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.mayhem_program_id.key, keys.mayhem_program_id),
        (*accounts.global_params.key, keys.global_params),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.mayhem_state.key, keys.mayhem_state),
        (*accounts.mayhem_token_vault.key, keys.mayhem_token_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_v2_verify_writable_privileges<'me, 'info>(
    accounts: CreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.mint,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.user,
        accounts.mayhem_program_id,
        accounts.sol_vault,
        accounts.mayhem_state,
        accounts.mayhem_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_v2_verify_signer_privileges<'me, 'info>(
    accounts: CreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.mint, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_v2_verify_account_privileges<'me, 'info>(
    accounts: CreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_v2_verify_writable_privileges(accounts)?;
    create_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DistributeCreatorFeesAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub creator_vault: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DistributeCreatorFeesAccounts<'_, '_>> for DistributeCreatorFeesKeys {
    fn from(accounts: DistributeCreatorFeesAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            creator_vault: *accounts.creator_vault.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DistributeCreatorFeesKeys>
for [AccountMeta; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: DistributeCreatorFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]>
for DistributeCreatorFeesKeys {
    fn from(pubkeys: [Pubkey; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            sharing_config: pubkeys[2],
            creator_vault: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<DistributeCreatorFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: DistributeCreatorFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.creator_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]>
for DistributeCreatorFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            sharing_config: &arr[2],
            creator_vault: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const DISTRIBUTE_CREATOR_FEES_IX_DISCM: [u8; 8usize] = [
    165, 114, 103, 0, 121, 206, 247, 81,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesIxData;
impl DistributeCreatorFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISTRIBUTE_CREATOR_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISTRIBUTE_CREATOR_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn distribute_creator_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: DistributeCreatorFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DistributeCreatorFeesIxData.try_to_vec()?,
    })
}
pub fn distribute_creator_fees_ix(
    keys: DistributeCreatorFeesKeys,
) -> std::io::Result<Instruction> {
    distribute_creator_fees_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn distribute_creator_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DistributeCreatorFeesKeys = accounts.into();
    let ix = distribute_creator_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn distribute_creator_fees_invoke(
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
) -> ProgramResult {
    distribute_creator_fees_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn distribute_creator_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DistributeCreatorFeesKeys = accounts.into();
    let ix = distribute_creator_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn distribute_creator_fees_invoke_signed(
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    distribute_creator_fees_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn distribute_creator_fees_verify_account_keys(
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    keys: DistributeCreatorFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.creator_vault.key, keys.creator_vault),
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
pub fn distribute_creator_fees_verify_writable_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.creator_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn distribute_creator_fees_verify_account_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    distribute_creator_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DistributeCreatorFeesV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub creator_vault_quote_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesV2Keys {
    pub payer: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub creator_vault: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub creator_vault_quote_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<DistributeCreatorFeesV2Accounts<'_, '_>> for DistributeCreatorFeesV2Keys {
    fn from(accounts: DistributeCreatorFeesV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            creator_vault: *accounts.creator_vault.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            creator_vault_quote_token_account: *accounts
                .creator_vault_quote_token_account
                .key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<DistributeCreatorFeesV2Keys>
for [AccountMeta; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: DistributeCreatorFeesV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
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
            AccountMeta {
                pubkey: keys.creator_vault_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN]>
for DistributeCreatorFeesV2Keys {
    fn from(pubkeys: [Pubkey; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            mint: pubkeys[1],
            bonding_curve: pubkeys[2],
            sharing_config: pubkeys[3],
            creator_vault: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
            creator_vault_quote_token_account: pubkeys[8],
            quote_mint: pubkeys[9],
            quote_token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
        }
    }
}
impl<'info> From<DistributeCreatorFeesV2Accounts<'_, 'info>>
for [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: DistributeCreatorFeesV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.creator_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.creator_vault_quote_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN]>
for DistributeCreatorFeesV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            mint: &arr[1],
            bonding_curve: &arr[2],
            sharing_config: &arr[3],
            creator_vault: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
            creator_vault_quote_token_account: &arr[8],
            quote_mint: &arr[9],
            quote_token_program: &arr[10],
            associated_token_program: &arr[11],
        }
    }
}
pub const DISTRIBUTE_CREATOR_FEES_V2_IX_DISCM: [u8; 8usize] = [
    255, 203, 19, 79, 244, 68, 8, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DistributeCreatorFeesV2IxArgs {
    pub initialize_ata: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesV2IxData(pub DistributeCreatorFeesV2IxArgs);
impl From<DistributeCreatorFeesV2IxArgs> for DistributeCreatorFeesV2IxData {
    fn from(args: DistributeCreatorFeesV2IxArgs) -> Self {
        Self(args)
    }
}
impl DistributeCreatorFeesV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISTRIBUTE_CREATOR_FEES_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let initialize_ata: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DistributeCreatorFeesV2IxArgs {
                initialize_ata,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISTRIBUTE_CREATOR_FEES_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.initialize_ata, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn distribute_creator_fees_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: DistributeCreatorFeesV2Keys,
    args: DistributeCreatorFeesV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISTRIBUTE_CREATOR_FEES_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: DistributeCreatorFeesV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn distribute_creator_fees_v2_ix(
    keys: DistributeCreatorFeesV2Keys,
    args: DistributeCreatorFeesV2IxArgs,
) -> std::io::Result<Instruction> {
    distribute_creator_fees_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn distribute_creator_fees_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DistributeCreatorFeesV2Accounts<'_, '_>,
    args: DistributeCreatorFeesV2IxArgs,
) -> ProgramResult {
    let keys: DistributeCreatorFeesV2Keys = accounts.into();
    let ix = distribute_creator_fees_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn distribute_creator_fees_v2_invoke(
    accounts: DistributeCreatorFeesV2Accounts<'_, '_>,
    args: DistributeCreatorFeesV2IxArgs,
) -> ProgramResult {
    distribute_creator_fees_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn distribute_creator_fees_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DistributeCreatorFeesV2Accounts<'_, '_>,
    args: DistributeCreatorFeesV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DistributeCreatorFeesV2Keys = accounts.into();
    let ix = distribute_creator_fees_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn distribute_creator_fees_v2_invoke_signed(
    accounts: DistributeCreatorFeesV2Accounts<'_, '_>,
    args: DistributeCreatorFeesV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    distribute_creator_fees_v2_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn distribute_creator_fees_v2_verify_account_keys(
    accounts: DistributeCreatorFeesV2Accounts<'_, '_>,
    keys: DistributeCreatorFeesV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (
            *accounts.creator_vault_quote_token_account.key,
            keys.creator_vault_quote_token_account,
        ),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn distribute_creator_fees_v2_verify_writable_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.creator_vault,
        accounts.creator_vault_quote_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn distribute_creator_fees_v2_verify_signer_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn distribute_creator_fees_v2_verify_account_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    distribute_creator_fees_v2_verify_writable_privileges(accounts)?;
    distribute_creator_fees_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DistributeFeeToHoldersAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub holder_reward_claim_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub holder_rewards: &'me AccountInfo<'info>,
    pub holder_rewards_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DistributeFeeToHoldersKeys {
    pub global: Pubkey,
    pub holder_reward_claim_authority: Pubkey,
    pub mint: Pubkey,
    pub holder_rewards: Pubkey,
    pub holder_rewards_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DistributeFeeToHoldersAccounts<'_, '_>> for DistributeFeeToHoldersKeys {
    fn from(accounts: DistributeFeeToHoldersAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            holder_reward_claim_authority: *accounts.holder_reward_claim_authority.key,
            mint: *accounts.mint.key,
            holder_rewards: *accounts.holder_rewards.key,
            holder_rewards_token_account: *accounts.holder_rewards_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DistributeFeeToHoldersKeys>
for [AccountMeta; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: DistributeFeeToHoldersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.holder_reward_claim_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.holder_rewards,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.holder_rewards_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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
impl From<[Pubkey; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN]>
for DistributeFeeToHoldersKeys {
    fn from(pubkeys: [Pubkey; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            holder_reward_claim_authority: pubkeys[1],
            mint: pubkeys[2],
            holder_rewards: pubkeys[3],
            holder_rewards_token_account: pubkeys[4],
            quote_mint: pubkeys[5],
            quote_token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<DistributeFeeToHoldersAccounts<'_, 'info>>
for [AccountInfo<'info>; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: DistributeFeeToHoldersAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.holder_reward_claim_authority.clone(),
            accounts.mint.clone(),
            accounts.holder_rewards.clone(),
            accounts.holder_rewards_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN]>
for DistributeFeeToHoldersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            holder_reward_claim_authority: &arr[1],
            mint: &arr[2],
            holder_rewards: &arr[3],
            holder_rewards_token_account: &arr[4],
            quote_mint: &arr[5],
            quote_token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const DISTRIBUTE_FEE_TO_HOLDERS_IX_DISCM: [u8; 8usize] = [
    98, 54, 145, 97, 2, 70, 173, 43,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DistributeFeeToHoldersIxArgs {
    pub amounts: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeFeeToHoldersIxData(pub DistributeFeeToHoldersIxArgs);
impl From<DistributeFeeToHoldersIxArgs> for DistributeFeeToHoldersIxData {
    fn from(args: DistributeFeeToHoldersIxArgs) -> Self {
        Self(args)
    }
}
impl DistributeFeeToHoldersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISTRIBUTE_FEE_TO_HOLDERS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amounts: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(DistributeFeeToHoldersIxArgs {
                amounts,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISTRIBUTE_FEE_TO_HOLDERS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amounts, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn distribute_fee_to_holders_ix_with_program_id(
    program_id: Pubkey,
    keys: DistributeFeeToHoldersKeys,
    args: DistributeFeeToHoldersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISTRIBUTE_FEE_TO_HOLDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: DistributeFeeToHoldersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn distribute_fee_to_holders_ix(
    keys: DistributeFeeToHoldersKeys,
    args: DistributeFeeToHoldersIxArgs,
) -> std::io::Result<Instruction> {
    distribute_fee_to_holders_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn distribute_fee_to_holders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DistributeFeeToHoldersAccounts<'_, '_>,
    args: DistributeFeeToHoldersIxArgs,
) -> ProgramResult {
    let keys: DistributeFeeToHoldersKeys = accounts.into();
    let ix = distribute_fee_to_holders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn distribute_fee_to_holders_invoke(
    accounts: DistributeFeeToHoldersAccounts<'_, '_>,
    args: DistributeFeeToHoldersIxArgs,
) -> ProgramResult {
    distribute_fee_to_holders_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn distribute_fee_to_holders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DistributeFeeToHoldersAccounts<'_, '_>,
    args: DistributeFeeToHoldersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DistributeFeeToHoldersKeys = accounts.into();
    let ix = distribute_fee_to_holders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn distribute_fee_to_holders_invoke_signed(
    accounts: DistributeFeeToHoldersAccounts<'_, '_>,
    args: DistributeFeeToHoldersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    distribute_fee_to_holders_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn distribute_fee_to_holders_verify_account_keys(
    accounts: DistributeFeeToHoldersAccounts<'_, '_>,
    keys: DistributeFeeToHoldersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (
            *accounts.holder_reward_claim_authority.key,
            keys.holder_reward_claim_authority,
        ),
        (*accounts.mint.key, keys.mint),
        (*accounts.holder_rewards.key, keys.holder_rewards),
        (*accounts.holder_rewards_token_account.key, keys.holder_rewards_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.quote_token_program.key, keys.quote_token_program),
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
pub fn distribute_fee_to_holders_verify_writable_privileges<'me, 'info>(
    accounts: DistributeFeeToHoldersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.holder_reward_claim_authority,
        accounts.holder_rewards,
        accounts.holder_rewards_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn distribute_fee_to_holders_verify_signer_privileges<'me, 'info>(
    accounts: DistributeFeeToHoldersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.holder_reward_claim_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn distribute_fee_to_holders_verify_account_privileges<'me, 'info>(
    accounts: DistributeFeeToHoldersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    distribute_fee_to_holders_verify_writable_privileges(accounts)?;
    distribute_fee_to_holders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXTEND_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ExtendAccountAccounts<'me, 'info> {
    pub account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExtendAccountKeys {
    pub account: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ExtendAccountAccounts<'_, '_>> for ExtendAccountKeys {
    fn from(accounts: ExtendAccountAccounts) -> Self {
        Self {
            account: *accounts.account.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ExtendAccountKeys> for [AccountMeta; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExtendAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
impl From<[Pubkey; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]> for ExtendAccountKeys {
    fn from(pubkeys: [Pubkey; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            account: pubkeys[0],
            user: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ExtendAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExtendAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.account.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]>
for ExtendAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            account: &arr[0],
            user: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const EXTEND_ACCOUNT_IX_DISCM: [u8; 8usize] = [234, 102, 194, 203, 150, 72, 62, 229];
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendAccountIxData;
impl ExtendAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXTEND_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXTEND_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn extend_account_ix_with_program_id(
    program_id: Pubkey,
    keys: ExtendAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ExtendAccountIxData.try_to_vec()?,
    })
}
pub fn extend_account_ix(keys: ExtendAccountKeys) -> std::io::Result<Instruction> {
    extend_account_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn extend_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExtendAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ExtendAccountKeys = accounts.into();
    let ix = extend_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn extend_account_invoke(accounts: ExtendAccountAccounts<'_, '_>) -> ProgramResult {
    extend_account_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn extend_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExtendAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExtendAccountKeys = accounts.into();
    let ix = extend_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn extend_account_invoke_signed(
    accounts: ExtendAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    extend_account_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn extend_account_verify_account_keys(
    accounts: ExtendAccountAccounts<'_, '_>,
    keys: ExtendAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.account.key, keys.account),
        (*accounts.user.key, keys.user),
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
pub fn extend_account_verify_writable_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.account, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn extend_account_verify_signer_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn extend_account_verify_account_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    extend_account_verify_writable_privileges(accounts)?;
    extend_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct GetMinimumDistributableFeeAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetMinimumDistributableFeeKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub creator_vault: Pubkey,
}
impl From<GetMinimumDistributableFeeAccounts<'_, '_>>
for GetMinimumDistributableFeeKeys {
    fn from(accounts: GetMinimumDistributableFeeAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            creator_vault: *accounts.creator_vault.key,
        }
    }
}
impl From<GetMinimumDistributableFeeKeys>
for [AccountMeta; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: GetMinimumDistributableFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]>
for GetMinimumDistributableFeeKeys {
    fn from(pubkeys: [Pubkey; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            sharing_config: pubkeys[2],
            creator_vault: pubkeys[3],
        }
    }
}
impl<'info> From<GetMinimumDistributableFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetMinimumDistributableFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.creator_vault.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]>
for GetMinimumDistributableFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            sharing_config: &arr[2],
            creator_vault: &arr[3],
        }
    }
}
pub const GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM: [u8; 8usize] = [
    117, 225, 127, 202, 134, 95, 68, 35,
];
#[derive(Clone, Debug, PartialEq)]
pub struct GetMinimumDistributableFeeIxData;
impl GetMinimumDistributableFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_minimum_distributable_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: GetMinimumDistributableFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetMinimumDistributableFeeIxData.try_to_vec()?,
    })
}
pub fn get_minimum_distributable_fee_ix(
    keys: GetMinimumDistributableFeeKeys,
) -> std::io::Result<Instruction> {
    get_minimum_distributable_fee_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn get_minimum_distributable_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetMinimumDistributableFeeKeys = accounts.into();
    let ix = get_minimum_distributable_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_minimum_distributable_fee_invoke(
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
) -> ProgramResult {
    get_minimum_distributable_fee_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn get_minimum_distributable_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetMinimumDistributableFeeKeys = accounts.into();
    let ix = get_minimum_distributable_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_minimum_distributable_fee_invoke_signed(
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_minimum_distributable_fee_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn get_minimum_distributable_fee_verify_account_keys(
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    keys: GetMinimumDistributableFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.creator_vault.key, keys.creator_vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitUserVolumeAccumulatorAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitUserVolumeAccumulatorKeys {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitUserVolumeAccumulatorAccounts<'_, '_>> for InitUserVolumeAccumulatorKeys {
    fn from(accounts: InitUserVolumeAccumulatorAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitUserVolumeAccumulatorKeys>
for [AccountMeta; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitUserVolumeAccumulatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]>
for InitUserVolumeAccumulatorKeys {
    fn from(pubkeys: [Pubkey; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user: pubkeys[1],
            user_volume_accumulator: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<InitUserVolumeAccumulatorAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitUserVolumeAccumulatorAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]>
for InitUserVolumeAccumulatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            user: &arr[1],
            user_volume_accumulator: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM: [u8; 8usize] = [
    94, 6, 202, 115, 255, 96, 232, 183,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitUserVolumeAccumulatorIxData;
impl InitUserVolumeAccumulatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_user_volume_accumulator_ix_with_program_id(
    program_id: Pubkey,
    keys: InitUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitUserVolumeAccumulatorIxData.try_to_vec()?,
    })
}
pub fn init_user_volume_accumulator_ix(
    keys: InitUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    init_user_volume_accumulator_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn init_user_volume_accumulator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitUserVolumeAccumulatorKeys = accounts.into();
    let ix = init_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_user_volume_accumulator_invoke(
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    init_user_volume_accumulator_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn init_user_volume_accumulator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitUserVolumeAccumulatorKeys = accounts.into();
    let ix = init_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_user_volume_accumulator_invoke_signed(
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_user_volume_accumulator_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn init_user_volume_accumulator_verify_account_keys(
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
    keys: InitUserVolumeAccumulatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user.key, keys.user),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
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
pub fn init_user_volume_accumulator_verify_writable_privileges<'me, 'info>(
    accounts: InitUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_user_volume_accumulator_verify_signer_privileges<'me, 'info>(
    accounts: InitUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_user_volume_accumulator_verify_account_privileges<'me, 'info>(
    accounts: InitUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_user_volume_accumulator_verify_writable_privileges(accounts)?;
    init_user_volume_accumulator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub global: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            user: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [accounts.global.clone(), accounts.user.clone(), accounts.system_program.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            user: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData;
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeIxData.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(accounts: InitializeAccounts<'_, '_>) -> ProgramResult {
    initialize_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeQuoteControlAccounts<'me, 'info> {
    pub quote_control: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeQuoteControlKeys {
    pub quote_control: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeQuoteControlAccounts<'_, '_>> for InitializeQuoteControlKeys {
    fn from(accounts: InitializeQuoteControlAccounts) -> Self {
        Self {
            quote_control: *accounts.quote_control.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeQuoteControlKeys>
for [AccountMeta; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeQuoteControlKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.quote_control,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
impl From<[Pubkey; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN]>
for InitializeQuoteControlKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            quote_control: pubkeys[0],
            user: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeQuoteControlAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeQuoteControlAccounts<'_, 'info>) -> Self {
        [
            accounts.quote_control.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN]>
for InitializeQuoteControlAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            quote_control: &arr[0],
            user: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_QUOTE_CONTROL_IX_DISCM: [u8; 8usize] = [
    239, 73, 245, 173, 209, 177, 84, 66,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeQuoteControlIxData;
impl InitializeQuoteControlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_QUOTE_CONTROL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_QUOTE_CONTROL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_quote_control_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeQuoteControlKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_QUOTE_CONTROL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeQuoteControlIxData.try_to_vec()?,
    })
}
pub fn initialize_quote_control_ix(
    keys: InitializeQuoteControlKeys,
) -> std::io::Result<Instruction> {
    initialize_quote_control_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn initialize_quote_control_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeQuoteControlAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeQuoteControlKeys = accounts.into();
    let ix = initialize_quote_control_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_quote_control_invoke(
    accounts: InitializeQuoteControlAccounts<'_, '_>,
) -> ProgramResult {
    initialize_quote_control_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn initialize_quote_control_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeQuoteControlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeQuoteControlKeys = accounts.into();
    let ix = initialize_quote_control_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_quote_control_invoke_signed(
    accounts: InitializeQuoteControlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_quote_control_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn initialize_quote_control_verify_account_keys(
    accounts: InitializeQuoteControlAccounts<'_, '_>,
    keys: InitializeQuoteControlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.quote_control.key, keys.quote_control),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_quote_control_verify_writable_privileges<'me, 'info>(
    accounts: InitializeQuoteControlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.quote_control, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_quote_control_verify_signer_privileges<'me, 'info>(
    accounts: InitializeQuoteControlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_quote_control_verify_account_privileges<'me, 'info>(
    accounts: InitializeQuoteControlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_quote_control_verify_writable_privileges(accounts)?;
    initialize_quote_control_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct MigrateAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub pump_amm: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_authority_mint_account: &'me AccountInfo<'info>,
    pub pool_authority_wsol_account: &'me AccountInfo<'info>,
    pub amm_global_config: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub pump_amm_event_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateKeys {
    pub global: Pubkey,
    pub withdraw_authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub pump_amm: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_authority_mint_account: Pubkey,
    pub pool_authority_wsol_account: Pubkey,
    pub amm_global_config: Pubkey,
    pub wsol_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub pump_amm_event_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub rent: Pubkey,
}
impl From<MigrateAccounts<'_, '_>> for MigrateKeys {
    fn from(accounts: MigrateAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            pump_amm: *accounts.pump_amm.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            pool_authority_mint_account: *accounts.pool_authority_mint_account.key,
            pool_authority_wsol_account: *accounts.pool_authority_wsol_account.key,
            amm_global_config: *accounts.amm_global_config.key,
            wsol_mint: *accounts.wsol_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_token_account: *accounts.user_pool_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            token_2022_program: *accounts.token_2022_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            pump_amm_event_authority: *accounts.pump_amm_event_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<MigrateKeys> for [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
            AccountMeta {
                pubkey: keys.pump_amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority_mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pump_amm_event_authority,
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
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_IX_ACCOUNTS_LEN]> for MigrateKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            withdraw_authority: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            user: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            pump_amm: pubkeys[8],
            pool: pubkeys[9],
            pool_authority: pubkeys[10],
            pool_authority_mint_account: pubkeys[11],
            pool_authority_wsol_account: pubkeys[12],
            amm_global_config: pubkeys[13],
            wsol_mint: pubkeys[14],
            lp_mint: pubkeys[15],
            user_pool_token_account: pubkeys[16],
            pool_base_token_account: pubkeys[17],
            pool_quote_token_account: pubkeys[18],
            token_2022_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            pump_amm_event_authority: pubkeys[21],
            event_authority: pubkeys[22],
            program: pubkeys[23],
            rent: pubkeys[24],
        }
    }
}
impl<'info> From<MigrateAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.withdraw_authority.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.pump_amm.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_authority_mint_account.clone(),
            accounts.pool_authority_wsol_account.clone(),
            accounts.amm_global_config.clone(),
            accounts.wsol_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.token_2022_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.pump_amm_event_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]>
for MigrateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            withdraw_authority: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            user: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            pump_amm: &arr[8],
            pool: &arr[9],
            pool_authority: &arr[10],
            pool_authority_mint_account: &arr[11],
            pool_authority_wsol_account: &arr[12],
            amm_global_config: &arr[13],
            wsol_mint: &arr[14],
            lp_mint: &arr[15],
            user_pool_token_account: &arr[16],
            pool_base_token_account: &arr[17],
            pool_quote_token_account: &arr[18],
            token_2022_program: &arr[19],
            associated_token_program: &arr[20],
            pump_amm_event_authority: &arr[21],
            event_authority: &arr[22],
            program: &arr[23],
            rent: &arr[24],
        }
    }
}
pub const MIGRATE_IX_DISCM: [u8; 8usize] = [155, 234, 231, 146, 236, 158, 162, 30];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateIxData;
impl MigrateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateIxData.try_to_vec()?,
    })
}
pub fn migrate_ix(keys: MigrateKeys) -> std::io::Result<Instruction> {
    migrate_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn migrate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateKeys = accounts.into();
    let ix = migrate_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_invoke(accounts: MigrateAccounts<'_, '_>) -> ProgramResult {
    migrate_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn migrate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateKeys = accounts.into();
    let ix = migrate_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_invoke_signed(
    accounts: MigrateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn migrate_verify_account_keys(
    accounts: MigrateAccounts<'_, '_>,
    keys: MigrateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.pump_amm.key, keys.pump_amm),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_authority_mint_account.key, keys.pool_authority_mint_account),
        (*accounts.pool_authority_wsol_account.key, keys.pool_authority_wsol_account),
        (*accounts.amm_global_config.key, keys.amm_global_config),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_token_account.key, keys.user_pool_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.pump_amm_event_authority.key, keys.pump_amm_event_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_verify_writable_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_authority,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.user,
        accounts.pool,
        accounts.pool_authority,
        accounts.pool_authority_mint_account,
        accounts.pool_authority_wsol_account,
        accounts.lp_mint,
        accounts.user_pool_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_verify_signer_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_verify_account_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_verify_writable_privileges(accounts)?;
    migrate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct MigrateBondingCurveCreatorAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateBondingCurveCreatorKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigrateBondingCurveCreatorAccounts<'_, '_>>
for MigrateBondingCurveCreatorKeys {
    fn from(accounts: MigrateBondingCurveCreatorAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateBondingCurveCreatorKeys>
for [AccountMeta; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateBondingCurveCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
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
impl From<[Pubkey; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]>
for MigrateBondingCurveCreatorKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            sharing_config: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<MigrateBondingCurveCreatorAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateBondingCurveCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]>
for MigrateBondingCurveCreatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            sharing_config: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM: [u8; 8usize] = [
    87, 124, 52, 191, 52, 38, 214, 232,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateBondingCurveCreatorIxData;
impl MigrateBondingCurveCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_bonding_curve_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateBondingCurveCreatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateBondingCurveCreatorIxData.try_to_vec()?,
    })
}
pub fn migrate_bonding_curve_creator_ix(
    keys: MigrateBondingCurveCreatorKeys,
) -> std::io::Result<Instruction> {
    migrate_bonding_curve_creator_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn migrate_bonding_curve_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateBondingCurveCreatorKeys = accounts.into();
    let ix = migrate_bonding_curve_creator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_bonding_curve_creator_invoke(
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
) -> ProgramResult {
    migrate_bonding_curve_creator_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn migrate_bonding_curve_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateBondingCurveCreatorKeys = accounts.into();
    let ix = migrate_bonding_curve_creator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_bonding_curve_creator_invoke_signed(
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_bonding_curve_creator_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_bonding_curve_creator_verify_account_keys(
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    keys: MigrateBondingCurveCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_bonding_curve_creator_verify_writable_privileges<'me, 'info>(
    accounts: MigrateBondingCurveCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bonding_curve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_bonding_curve_creator_verify_account_privileges<'me, 'info>(
    accounts: MigrateBondingCurveCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_bonding_curve_creator_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_V2_IX_ACCOUNTS_LEN: usize = 27;
#[derive(Copy, Clone, Debug)]
pub struct MigrateV2Accounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_base_bonding_curve: &'me AccountInfo<'info>,
    pub associated_quote_bonding_curve: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub pump_amm: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_authority_mint_account: &'me AccountInfo<'info>,
    pub pool_authority_quote_account: &'me AccountInfo<'info>,
    pub amm_global_config: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub pump_amm_event_authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateV2Keys {
    pub global: Pubkey,
    pub withdraw_authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_base_bonding_curve: Pubkey,
    pub associated_quote_bonding_curve: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub pump_amm: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_authority_mint_account: Pubkey,
    pub pool_authority_quote_account: Pubkey,
    pub amm_global_config: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub pump_amm_event_authority: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigrateV2Accounts<'_, '_>> for MigrateV2Keys {
    fn from(accounts: MigrateV2Accounts) -> Self {
        Self {
            global: *accounts.global.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_base_bonding_curve: *accounts.associated_base_bonding_curve.key,
            associated_quote_bonding_curve: *accounts.associated_quote_bonding_curve.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            pump_amm: *accounts.pump_amm.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            pool_authority_mint_account: *accounts.pool_authority_mint_account.key,
            pool_authority_quote_account: *accounts.pool_authority_quote_account.key,
            amm_global_config: *accounts.amm_global_config.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_token_account: *accounts.user_pool_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            pump_amm_event_authority: *accounts.pump_amm_event_authority.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateV2Keys> for [AccountMeta; MIGRATE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pump_amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority_mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pump_amm_event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; MIGRATE_V2_IX_ACCOUNTS_LEN]> for MigrateV2Keys {
    fn from(pubkeys: [Pubkey; MIGRATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            withdraw_authority: pubkeys[1],
            base_mint: pubkeys[2],
            quote_mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            associated_base_bonding_curve: pubkeys[5],
            associated_quote_bonding_curve: pubkeys[6],
            user: pubkeys[7],
            system_program: pubkeys[8],
            pump_amm: pubkeys[9],
            pool: pubkeys[10],
            pool_authority: pubkeys[11],
            pool_authority_mint_account: pubkeys[12],
            pool_authority_quote_account: pubkeys[13],
            amm_global_config: pubkeys[14],
            lp_mint: pubkeys[15],
            user_pool_token_account: pubkeys[16],
            pool_base_token_account: pubkeys[17],
            pool_quote_token_account: pubkeys[18],
            base_token_program: pubkeys[19],
            quote_token_program: pubkeys[20],
            token_2022_program: pubkeys[21],
            associated_token_program: pubkeys[22],
            pump_amm_event_authority: pubkeys[23],
            rent: pubkeys[24],
            event_authority: pubkeys[25],
            program: pubkeys[26],
        }
    }
}
impl<'info> From<MigrateV2Accounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateV2Accounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.withdraw_authority.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_base_bonding_curve.clone(),
            accounts.associated_quote_bonding_curve.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.pump_amm.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_authority_mint_account.clone(),
            accounts.pool_authority_quote_account.clone(),
            accounts.amm_global_config.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.pump_amm_event_authority.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_V2_IX_ACCOUNTS_LEN]>
for MigrateV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            withdraw_authority: &arr[1],
            base_mint: &arr[2],
            quote_mint: &arr[3],
            bonding_curve: &arr[4],
            associated_base_bonding_curve: &arr[5],
            associated_quote_bonding_curve: &arr[6],
            user: &arr[7],
            system_program: &arr[8],
            pump_amm: &arr[9],
            pool: &arr[10],
            pool_authority: &arr[11],
            pool_authority_mint_account: &arr[12],
            pool_authority_quote_account: &arr[13],
            amm_global_config: &arr[14],
            lp_mint: &arr[15],
            user_pool_token_account: &arr[16],
            pool_base_token_account: &arr[17],
            pool_quote_token_account: &arr[18],
            base_token_program: &arr[19],
            quote_token_program: &arr[20],
            token_2022_program: &arr[21],
            associated_token_program: &arr[22],
            pump_amm_event_authority: &arr[23],
            rent: &arr[24],
            event_authority: &arr[25],
            program: &arr[26],
        }
    }
}
pub const MIGRATE_V2_IX_DISCM: [u8; 8usize] = [187, 203, 18, 31, 206, 237, 254, 41];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateV2IxData;
impl MigrateV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateV2IxData.try_to_vec()?,
    })
}
pub fn migrate_v2_ix(keys: MigrateV2Keys) -> std::io::Result<Instruction> {
    migrate_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn migrate_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateV2Keys = accounts.into();
    let ix = migrate_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_v2_invoke(accounts: MigrateV2Accounts<'_, '_>) -> ProgramResult {
    migrate_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn migrate_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateV2Keys = accounts.into();
    let ix = migrate_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_v2_invoke_signed(
    accounts: MigrateV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_v2_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn migrate_v2_verify_account_keys(
    accounts: MigrateV2Accounts<'_, '_>,
    keys: MigrateV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.associated_base_bonding_curve.key,
            keys.associated_base_bonding_curve,
        ),
        (
            *accounts.associated_quote_bonding_curve.key,
            keys.associated_quote_bonding_curve,
        ),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.pump_amm.key, keys.pump_amm),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_authority_mint_account.key, keys.pool_authority_mint_account),
        (*accounts.pool_authority_quote_account.key, keys.pool_authority_quote_account),
        (*accounts.amm_global_config.key, keys.amm_global_config),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_token_account.key, keys.user_pool_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.pump_amm_event_authority.key, keys.pump_amm_event_authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_v2_verify_writable_privileges<'me, 'info>(
    accounts: MigrateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_authority,
        accounts.bonding_curve,
        accounts.associated_base_bonding_curve,
        accounts.associated_quote_bonding_curve,
        accounts.user,
        accounts.pool,
        accounts.pool_authority,
        accounts.pool_authority_mint_account,
        accounts.pool_authority_quote_account,
        accounts.lp_mint,
        accounts.user_pool_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_v2_verify_signer_privileges<'me, 'info>(
    accounts: MigrateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_v2_verify_account_privileges<'me, 'info>(
    accounts: MigrateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_v2_verify_writable_privileges(accounts)?;
    migrate_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveQuoteControlMintAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub quote_control: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveQuoteControlMintKeys {
    pub authority: Pubkey,
    pub global: Pubkey,
    pub quote_control: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveQuoteControlMintAccounts<'_, '_>> for RemoveQuoteControlMintKeys {
    fn from(accounts: RemoveQuoteControlMintAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            global: *accounts.global.key,
            quote_control: *accounts.quote_control.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveQuoteControlMintKeys>
for [AccountMeta; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveQuoteControlMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_control,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN]>
for RemoveQuoteControlMintKeys {
    fn from(pubkeys: [Pubkey; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            global: pubkeys[1],
            quote_control: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveQuoteControlMintAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveQuoteControlMintAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.global.clone(),
            accounts.quote_control.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN]>
for RemoveQuoteControlMintAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            authority: &arr[0],
            global: &arr[1],
            quote_control: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const REMOVE_QUOTE_CONTROL_MINT_IX_DISCM: [u8; 8usize] = [
    223, 7, 253, 26, 81, 165, 218, 166,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveQuoteControlMintIxArgs {
    pub quote_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveQuoteControlMintIxData(pub RemoveQuoteControlMintIxArgs);
impl From<RemoveQuoteControlMintIxArgs> for RemoveQuoteControlMintIxData {
    fn from(args: RemoveQuoteControlMintIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveQuoteControlMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_QUOTE_CONTROL_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveQuoteControlMintIxArgs {
                quote_mint,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_QUOTE_CONTROL_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.quote_mint, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_quote_control_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveQuoteControlMintKeys,
    args: RemoveQuoteControlMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_QUOTE_CONTROL_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveQuoteControlMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_quote_control_mint_ix(
    keys: RemoveQuoteControlMintKeys,
    args: RemoveQuoteControlMintIxArgs,
) -> std::io::Result<Instruction> {
    remove_quote_control_mint_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn remove_quote_control_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveQuoteControlMintAccounts<'_, '_>,
    args: RemoveQuoteControlMintIxArgs,
) -> ProgramResult {
    let keys: RemoveQuoteControlMintKeys = accounts.into();
    let ix = remove_quote_control_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_quote_control_mint_invoke(
    accounts: RemoveQuoteControlMintAccounts<'_, '_>,
    args: RemoveQuoteControlMintIxArgs,
) -> ProgramResult {
    remove_quote_control_mint_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn remove_quote_control_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveQuoteControlMintAccounts<'_, '_>,
    args: RemoveQuoteControlMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveQuoteControlMintKeys = accounts.into();
    let ix = remove_quote_control_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_quote_control_mint_invoke_signed(
    accounts: RemoveQuoteControlMintAccounts<'_, '_>,
    args: RemoveQuoteControlMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_quote_control_mint_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_quote_control_mint_verify_account_keys(
    accounts: RemoveQuoteControlMintAccounts<'_, '_>,
    keys: RemoveQuoteControlMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.global.key, keys.global),
        (*accounts.quote_control.key, keys.quote_control),
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
pub fn remove_quote_control_mint_verify_writable_privileges<'me, 'info>(
    accounts: RemoveQuoteControlMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.quote_control] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_quote_control_mint_verify_signer_privileges<'me, 'info>(
    accounts: RemoveQuoteControlMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_quote_control_mint_verify_account_privileges<'me, 'info>(
    accounts: RemoveQuoteControlMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_quote_control_mint_verify_writable_privileges(accounts)?;
    remove_quote_control_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RemoveQuoteMintAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveQuoteMintKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveQuoteMintAccounts<'_, '_>> for RemoveQuoteMintKeys {
    fn from(accounts: RemoveQuoteMintAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveQuoteMintKeys> for [AccountMeta; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveQuoteMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN]> for RemoveQuoteMintKeys {
    fn from(pubkeys: [Pubkey; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<RemoveQuoteMintAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveQuoteMintAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN]>
for RemoveQuoteMintAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const REMOVE_QUOTE_MINT_IX_DISCM: [u8; 8usize] = [
    177, 65, 223, 38, 88, 209, 158, 155,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemoveQuoteMintIxArgs {
    pub quote_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveQuoteMintIxData(pub RemoveQuoteMintIxArgs);
impl From<RemoveQuoteMintIxArgs> for RemoveQuoteMintIxData {
    fn from(args: RemoveQuoteMintIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveQuoteMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_QUOTE_MINT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemoveQuoteMintIxArgs {
                quote_mint,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_QUOTE_MINT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.quote_mint, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_quote_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveQuoteMintKeys,
    args: RemoveQuoteMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_QUOTE_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveQuoteMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_quote_mint_ix(
    keys: RemoveQuoteMintKeys,
    args: RemoveQuoteMintIxArgs,
) -> std::io::Result<Instruction> {
    remove_quote_mint_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn remove_quote_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveQuoteMintAccounts<'_, '_>,
    args: RemoveQuoteMintIxArgs,
) -> ProgramResult {
    let keys: RemoveQuoteMintKeys = accounts.into();
    let ix = remove_quote_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_quote_mint_invoke(
    accounts: RemoveQuoteMintAccounts<'_, '_>,
    args: RemoveQuoteMintIxArgs,
) -> ProgramResult {
    remove_quote_mint_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn remove_quote_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveQuoteMintAccounts<'_, '_>,
    args: RemoveQuoteMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveQuoteMintKeys = accounts.into();
    let ix = remove_quote_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_quote_mint_invoke_signed(
    accounts: RemoveQuoteMintAccounts<'_, '_>,
    args: RemoveQuoteMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_quote_mint_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_quote_mint_verify_account_keys(
    accounts: RemoveQuoteMintAccounts<'_, '_>,
    keys: RemoveQuoteMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_quote_mint_verify_writable_privileges<'me, 'info>(
    accounts: RemoveQuoteMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_quote_mint_verify_signer_privileges<'me, 'info>(
    accounts: RemoveQuoteMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_quote_mint_verify_account_privileges<'me, 'info>(
    accounts: RemoveQuoteMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_quote_mint_verify_writable_privileges(accounts)?;
    remove_quote_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub creator_vault: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
}
impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            creator_vault: *accounts.creator_vault.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
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
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            fee_recipient: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            associated_user: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            creator_vault: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
            fee_config: pubkeys[12],
            fee_program: pubkeys[13],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.creator_vault.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            fee_recipient: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            associated_user: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            creator_vault: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
            fee_config: &arr[12],
            fee_program: &arr[13],
        }
    }
}
pub const SELL_IX_DISCM: [u8; 8usize] = [51, 230, 133, 164, 1, 127, 131, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellIxArgs {
    pub amount: u64,
    pub min_sol_output: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellIxData(pub SellIxArgs);
impl From<SellIxArgs> for SellIxData {
    fn from(args: SellIxArgs) -> Self {
        Self(args)
    }
}
impl SellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_sol_output: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellIxArgs {
                amount,
                min_sol_output,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_sol_output, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_ix_with_program_id(
    program_id: Pubkey,
    keys: SellKeys,
    args: SellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_ix(keys: SellKeys, args: SellIxArgs) -> std::io::Result<Instruction> {
    sell_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_invoke(accounts: SellAccounts<'_, '_>, args: SellIxArgs) -> ProgramResult {
    sell_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_invoke_signed(
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_verify_writable_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.creator_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_verify_signer_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_verify_account_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_verify_writable_privileges(accounts)?;
    sell_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_V2_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct SellV2Accounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub associated_quote_fee_recipient: &'me AccountInfo<'info>,
    pub buyback_fee_recipient: &'me AccountInfo<'info>,
    pub associated_quote_buyback_fee_recipient: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_base_bonding_curve: &'me AccountInfo<'info>,
    pub associated_quote_bonding_curve: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub associated_base_user: &'me AccountInfo<'info>,
    pub associated_quote_user: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub associated_creator_vault: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub associated_user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellV2Keys {
    pub global: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub fee_recipient: Pubkey,
    pub associated_quote_fee_recipient: Pubkey,
    pub buyback_fee_recipient: Pubkey,
    pub associated_quote_buyback_fee_recipient: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_base_bonding_curve: Pubkey,
    pub associated_quote_bonding_curve: Pubkey,
    pub user: Pubkey,
    pub associated_base_user: Pubkey,
    pub associated_quote_user: Pubkey,
    pub creator_vault: Pubkey,
    pub associated_creator_vault: Pubkey,
    pub sharing_config: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub associated_user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SellV2Accounts<'_, '_>> for SellV2Keys {
    fn from(accounts: SellV2Accounts) -> Self {
        Self {
            global: *accounts.global.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            fee_recipient: *accounts.fee_recipient.key,
            associated_quote_fee_recipient: *accounts.associated_quote_fee_recipient.key,
            buyback_fee_recipient: *accounts.buyback_fee_recipient.key,
            associated_quote_buyback_fee_recipient: *accounts
                .associated_quote_buyback_fee_recipient
                .key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_base_bonding_curve: *accounts.associated_base_bonding_curve.key,
            associated_quote_bonding_curve: *accounts.associated_quote_bonding_curve.key,
            user: *accounts.user.key,
            associated_base_user: *accounts.associated_base_user.key,
            associated_quote_user: *accounts.associated_quote_user.key,
            creator_vault: *accounts.creator_vault.key,
            associated_creator_vault: *accounts.associated_creator_vault.key,
            sharing_config: *accounts.sharing_config.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            associated_user_volume_accumulator: *accounts
                .associated_user_volume_accumulator
                .key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellV2Keys> for [AccountMeta; SELL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SellV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyback_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_buyback_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_base_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_quote_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
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
impl From<[Pubkey; SELL_V2_IX_ACCOUNTS_LEN]> for SellV2Keys {
    fn from(pubkeys: [Pubkey; SELL_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            base_token_program: pubkeys[3],
            quote_token_program: pubkeys[4],
            associated_token_program: pubkeys[5],
            fee_recipient: pubkeys[6],
            associated_quote_fee_recipient: pubkeys[7],
            buyback_fee_recipient: pubkeys[8],
            associated_quote_buyback_fee_recipient: pubkeys[9],
            bonding_curve: pubkeys[10],
            associated_base_bonding_curve: pubkeys[11],
            associated_quote_bonding_curve: pubkeys[12],
            user: pubkeys[13],
            associated_base_user: pubkeys[14],
            associated_quote_user: pubkeys[15],
            creator_vault: pubkeys[16],
            associated_creator_vault: pubkeys[17],
            sharing_config: pubkeys[18],
            user_volume_accumulator: pubkeys[19],
            associated_user_volume_accumulator: pubkeys[20],
            fee_config: pubkeys[21],
            fee_program: pubkeys[22],
            system_program: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<SellV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SELL_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellV2Accounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.fee_recipient.clone(),
            accounts.associated_quote_fee_recipient.clone(),
            accounts.buyback_fee_recipient.clone(),
            accounts.associated_quote_buyback_fee_recipient.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_base_bonding_curve.clone(),
            accounts.associated_quote_bonding_curve.clone(),
            accounts.user.clone(),
            accounts.associated_base_user.clone(),
            accounts.associated_quote_user.clone(),
            accounts.creator_vault.clone(),
            accounts.associated_creator_vault.clone(),
            accounts.sharing_config.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.associated_user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_V2_IX_ACCOUNTS_LEN]>
for SellV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            base_token_program: &arr[3],
            quote_token_program: &arr[4],
            associated_token_program: &arr[5],
            fee_recipient: &arr[6],
            associated_quote_fee_recipient: &arr[7],
            buyback_fee_recipient: &arr[8],
            associated_quote_buyback_fee_recipient: &arr[9],
            bonding_curve: &arr[10],
            associated_base_bonding_curve: &arr[11],
            associated_quote_bonding_curve: &arr[12],
            user: &arr[13],
            associated_base_user: &arr[14],
            associated_quote_user: &arr[15],
            creator_vault: &arr[16],
            associated_creator_vault: &arr[17],
            sharing_config: &arr[18],
            user_volume_accumulator: &arr[19],
            associated_user_volume_accumulator: &arr[20],
            fee_config: &arr[21],
            fee_program: &arr[22],
            system_program: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const SELL_V2_IX_DISCM: [u8; 8usize] = [93, 246, 130, 60, 231, 233, 64, 178];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellV2IxArgs {
    pub amount: u64,
    pub min_sol_output: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellV2IxData(pub SellV2IxArgs);
impl From<SellV2IxArgs> for SellV2IxData {
    fn from(args: SellV2IxArgs) -> Self {
        Self(args)
    }
}
impl SellV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_sol_output: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellV2IxArgs {
                amount,
                min_sol_output,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.min_sol_output, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SellV2Keys,
    args: SellV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_v2_ix(keys: SellV2Keys, args: SellV2IxArgs) -> std::io::Result<Instruction> {
    sell_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn sell_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellV2Accounts<'_, '_>,
    args: SellV2IxArgs,
) -> ProgramResult {
    let keys: SellV2Keys = accounts.into();
    let ix = sell_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_v2_invoke(
    accounts: SellV2Accounts<'_, '_>,
    args: SellV2IxArgs,
) -> ProgramResult {
    sell_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn sell_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellV2Accounts<'_, '_>,
    args: SellV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellV2Keys = accounts.into();
    let ix = sell_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_v2_invoke_signed(
    accounts: SellV2Accounts<'_, '_>,
    args: SellV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_v2_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_v2_verify_account_keys(
    accounts: SellV2Accounts<'_, '_>,
    keys: SellV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (
            *accounts.associated_quote_fee_recipient.key,
            keys.associated_quote_fee_recipient,
        ),
        (*accounts.buyback_fee_recipient.key, keys.buyback_fee_recipient),
        (
            *accounts.associated_quote_buyback_fee_recipient.key,
            keys.associated_quote_buyback_fee_recipient,
        ),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.associated_base_bonding_curve.key,
            keys.associated_base_bonding_curve,
        ),
        (
            *accounts.associated_quote_bonding_curve.key,
            keys.associated_quote_bonding_curve,
        ),
        (*accounts.user.key, keys.user),
        (*accounts.associated_base_user.key, keys.associated_base_user),
        (*accounts.associated_quote_user.key, keys.associated_quote_user),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.associated_creator_vault.key, keys.associated_creator_vault),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (
            *accounts.associated_user_volume_accumulator.key,
            keys.associated_user_volume_accumulator,
        ),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
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
pub fn sell_v2_verify_writable_privileges<'me, 'info>(
    accounts: SellV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.associated_quote_fee_recipient,
        accounts.buyback_fee_recipient,
        accounts.associated_quote_buyback_fee_recipient,
        accounts.bonding_curve,
        accounts.associated_base_bonding_curve,
        accounts.associated_quote_bonding_curve,
        accounts.user,
        accounts.associated_base_user,
        accounts.associated_quote_user,
        accounts.creator_vault,
        accounts.associated_creator_vault,
        accounts.user_volume_accumulator,
        accounts.associated_user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_v2_verify_signer_privileges<'me, 'info>(
    accounts: SellV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_v2_verify_account_privileges<'me, 'info>(
    accounts: SellV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_v2_verify_writable_privileges(accounts)?;
    sell_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_CREATOR_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct SetCreatorAccounts<'me, 'info> {
    pub set_creator_authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCreatorKeys {
    pub set_creator_authority: Pubkey,
    pub global: Pubkey,
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub bonding_curve: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetCreatorAccounts<'_, '_>> for SetCreatorKeys {
    fn from(accounts: SetCreatorAccounts) -> Self {
        Self {
            set_creator_authority: *accounts.set_creator_authority.key,
            global: *accounts.global.key,
            mint: *accounts.mint.key,
            metadata: *accounts.metadata.key,
            bonding_curve: *accounts.bonding_curve.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetCreatorKeys> for [AccountMeta; SET_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.set_creator_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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
impl From<[Pubkey; SET_CREATOR_IX_ACCOUNTS_LEN]> for SetCreatorKeys {
    fn from(pubkeys: [Pubkey; SET_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            set_creator_authority: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            metadata: pubkeys[3],
            bonding_curve: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<SetCreatorAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.set_creator_authority.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.metadata.clone(),
            accounts.bonding_curve.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_CREATOR_IX_ACCOUNTS_LEN]>
for SetCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            set_creator_authority: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            metadata: &arr[3],
            bonding_curve: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const SET_CREATOR_IX_DISCM: [u8; 8usize] = [254, 148, 255, 112, 207, 142, 170, 165];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetCreatorIxArgs {
    pub creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCreatorIxData(pub SetCreatorIxArgs);
impl From<SetCreatorIxArgs> for SetCreatorIxData {
    fn from(args: SetCreatorIxArgs) -> Self {
        Self(args)
    }
}
impl SetCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CREATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(SetCreatorIxArgs { creator }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CREATOR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.creator, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCreatorKeys,
    args: SetCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_creator_ix(
    keys: SetCreatorKeys,
    args: SetCreatorIxArgs,
) -> std::io::Result<Instruction> {
    set_creator_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn set_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
) -> ProgramResult {
    let keys: SetCreatorKeys = accounts.into();
    let ix = set_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_creator_invoke(
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
) -> ProgramResult {
    set_creator_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn set_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCreatorKeys = accounts.into();
    let ix = set_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_creator_invoke_signed(
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_creator_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_creator_verify_account_keys(
    accounts: SetCreatorAccounts<'_, '_>,
    keys: SetCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.set_creator_authority.key, keys.set_creator_authority),
        (*accounts.global.key, keys.global),
        (*accounts.mint.key, keys.mint),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_creator_verify_writable_privileges<'me, 'info>(
    accounts: SetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bonding_curve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_creator_verify_signer_privileges<'me, 'info>(
    accounts: SetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.set_creator_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_creator_verify_account_privileges<'me, 'info>(
    accounts: SetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_creator_verify_writable_privileges(accounts)?;
    set_creator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct SetMayhemVirtualParamsAccounts<'me, 'info> {
    pub sol_vault_authority: &'me AccountInfo<'info>,
    pub mayhem_token_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMayhemVirtualParamsKeys {
    pub sol_vault_authority: Pubkey,
    pub mayhem_token_vault: Pubkey,
    pub mint: Pubkey,
    pub global: Pubkey,
    pub bonding_curve: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetMayhemVirtualParamsAccounts<'_, '_>> for SetMayhemVirtualParamsKeys {
    fn from(accounts: SetMayhemVirtualParamsAccounts) -> Self {
        Self {
            sol_vault_authority: *accounts.sol_vault_authority.key,
            mayhem_token_vault: *accounts.mayhem_token_vault.key,
            mint: *accounts.mint.key,
            global: *accounts.global.key,
            bonding_curve: *accounts.bonding_curve.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetMayhemVirtualParamsKeys>
for [AccountMeta; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMayhemVirtualParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sol_vault_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mayhem_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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
impl From<[Pubkey; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]>
for SetMayhemVirtualParamsKeys {
    fn from(pubkeys: [Pubkey; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sol_vault_authority: pubkeys[0],
            mayhem_token_vault: pubkeys[1],
            mint: pubkeys[2],
            global: pubkeys[3],
            bonding_curve: pubkeys[4],
            token_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<SetMayhemVirtualParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMayhemVirtualParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.sol_vault_authority.clone(),
            accounts.mayhem_token_vault.clone(),
            accounts.mint.clone(),
            accounts.global.clone(),
            accounts.bonding_curve.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]>
for SetMayhemVirtualParamsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            sol_vault_authority: &arr[0],
            mayhem_token_vault: &arr[1],
            mint: &arr[2],
            global: &arr[3],
            bonding_curve: &arr[4],
            token_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM: [u8; 8usize] = [
    61, 169, 188, 191, 153, 149, 42, 97,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetMayhemVirtualParamsIxData;
impl SetMayhemVirtualParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_mayhem_virtual_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMayhemVirtualParamsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetMayhemVirtualParamsIxData.try_to_vec()?,
    })
}
pub fn set_mayhem_virtual_params_ix(
    keys: SetMayhemVirtualParamsKeys,
) -> std::io::Result<Instruction> {
    set_mayhem_virtual_params_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn set_mayhem_virtual_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetMayhemVirtualParamsKeys = accounts.into();
    let ix = set_mayhem_virtual_params_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_mayhem_virtual_params_invoke(
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
) -> ProgramResult {
    set_mayhem_virtual_params_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn set_mayhem_virtual_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMayhemVirtualParamsKeys = accounts.into();
    let ix = set_mayhem_virtual_params_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_mayhem_virtual_params_invoke_signed(
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_mayhem_virtual_params_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn set_mayhem_virtual_params_verify_account_keys(
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
    keys: SetMayhemVirtualParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.sol_vault_authority.key, keys.sol_vault_authority),
        (*accounts.mayhem_token_vault.key, keys.mayhem_token_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.global.key, keys.global),
        (*accounts.bonding_curve.key, keys.bonding_curve),
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
pub fn set_mayhem_virtual_params_verify_writable_privileges<'me, 'info>(
    accounts: SetMayhemVirtualParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.sol_vault_authority,
        accounts.mayhem_token_vault,
        accounts.bonding_curve,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_mayhem_virtual_params_verify_signer_privileges<'me, 'info>(
    accounts: SetMayhemVirtualParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sol_vault_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_mayhem_virtual_params_verify_account_privileges<'me, 'info>(
    accounts: SetMayhemVirtualParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_mayhem_virtual_params_verify_writable_privileges(accounts)?;
    set_mayhem_virtual_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetMetaplexCreatorAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMetaplexCreatorKeys {
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub bonding_curve: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetMetaplexCreatorAccounts<'_, '_>> for SetMetaplexCreatorKeys {
    fn from(accounts: SetMetaplexCreatorAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            metadata: *accounts.metadata.key,
            bonding_curve: *accounts.bonding_curve.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetMetaplexCreatorKeys>
for [AccountMeta; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMetaplexCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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
impl From<[Pubkey; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]> for SetMetaplexCreatorKeys {
    fn from(pubkeys: [Pubkey; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            metadata: pubkeys[1],
            bonding_curve: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<SetMetaplexCreatorAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMetaplexCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.metadata.clone(),
            accounts.bonding_curve.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]>
for SetMetaplexCreatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            mint: &arr[0],
            metadata: &arr[1],
            bonding_curve: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const SET_METAPLEX_CREATOR_IX_DISCM: [u8; 8usize] = [
    138, 96, 174, 217, 48, 85, 197, 246,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetMetaplexCreatorIxData;
impl SetMetaplexCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_METAPLEX_CREATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_METAPLEX_CREATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_metaplex_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMetaplexCreatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetMetaplexCreatorIxData.try_to_vec()?,
    })
}
pub fn set_metaplex_creator_ix(
    keys: SetMetaplexCreatorKeys,
) -> std::io::Result<Instruction> {
    set_metaplex_creator_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn set_metaplex_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetMetaplexCreatorKeys = accounts.into();
    let ix = set_metaplex_creator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_metaplex_creator_invoke(
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
) -> ProgramResult {
    set_metaplex_creator_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn set_metaplex_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetMetaplexCreatorKeys = accounts.into();
    let ix = set_metaplex_creator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_metaplex_creator_invoke_signed(
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_metaplex_creator_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, seeds)
}
pub fn set_metaplex_creator_verify_account_keys(
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
    keys: SetMetaplexCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_metaplex_creator_verify_writable_privileges<'me, 'info>(
    accounts: SetMetaplexCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bonding_curve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_metaplex_creator_verify_account_privileges<'me, 'info>(
    accounts: SetMetaplexCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_metaplex_creator_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SET_PARAMS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetParamsAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetParamsKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetParamsAccounts<'_, '_>> for SetParamsKeys {
    fn from(accounts: SetParamsAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetParamsKeys> for [AccountMeta; SET_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; SET_PARAMS_IX_ACCOUNTS_LEN]> for SetParamsKeys {
    fn from(pubkeys: [Pubkey; SET_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<SetParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN]>
for SetParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const SET_PARAMS_IX_DISCM: [u8; 8usize] = [27, 234, 178, 52, 147, 2, 187, 141];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetParamsIxArgs {
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub set_creator_authority: Pubkey,
    pub admin_set_creator_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetParamsIxData(pub SetParamsIxArgs);
impl From<SetParamsIxArgs> for SetParamsIxData {
    fn from(args: SetParamsIxArgs) -> Self {
        Self(args)
    }
}
impl SetParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PARAMS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let initial_virtual_token_reserves: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enable_migrate: bool = crate::borsh_de_or_default(&mut reader)?;
        let pool_migration_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let set_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_set_creator_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetParamsIxArgs {
                initial_virtual_token_reserves,
                initial_virtual_sol_reserves,
                initial_real_token_reserves,
                token_total_supply,
                fee_basis_points,
                withdraw_authority,
                enable_migrate,
                pool_migration_fee,
                creator_fee_basis_points,
                set_creator_authority,
                admin_set_creator_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PARAMS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_token_reserves,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_sol_reserves,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_real_token_reserves,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.0.token_total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.withdraw_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.enable_migrate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.pool_migration_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.set_creator_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.admin_set_creator_authority,
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
pub fn set_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetParamsKeys,
    args: SetParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_params_ix(
    keys: SetParamsKeys,
    args: SetParamsIxArgs,
) -> std::io::Result<Instruction> {
    set_params_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn set_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
) -> ProgramResult {
    let keys: SetParamsKeys = accounts.into();
    let ix = set_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_params_invoke(
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
) -> ProgramResult {
    set_params_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn set_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetParamsKeys = accounts.into();
    let ix = set_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_params_invoke_signed(
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_params_invoke_signed_with_program_id(PUMP_PROGRAM_ID, accounts, args, seeds)
}
pub fn set_params_verify_account_keys(
    accounts: SetParamsAccounts<'_, '_>,
    keys: SetParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_params_verify_writable_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_params_verify_signer_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_params_verify_account_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_params_verify_writable_privileges(accounts)?;
    set_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetQuoteControlAdminAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub quote_control: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetQuoteControlAdminKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub quote_control: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetQuoteControlAdminAccounts<'_, '_>> for SetQuoteControlAdminKeys {
    fn from(accounts: SetQuoteControlAdminAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            quote_control: *accounts.quote_control.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetQuoteControlAdminKeys>
for [AccountMeta; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: SetQuoteControlAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_control,
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
impl From<[Pubkey; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN]>
for SetQuoteControlAdminKeys {
    fn from(pubkeys: [Pubkey; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            quote_control: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<SetQuoteControlAdminAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetQuoteControlAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.quote_control.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN]>
for SetQuoteControlAdminAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            quote_control: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const SET_QUOTE_CONTROL_ADMIN_IX_DISCM: [u8; 8usize] = [
    62, 79, 161, 211, 165, 170, 214, 210,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetQuoteControlAdminIxArgs {
    pub new_admin: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetQuoteControlAdminIxData(pub SetQuoteControlAdminIxArgs);
impl From<SetQuoteControlAdminIxArgs> for SetQuoteControlAdminIxData {
    fn from(args: SetQuoteControlAdminIxArgs) -> Self {
        Self(args)
    }
}
impl SetQuoteControlAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_QUOTE_CONTROL_ADMIN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetQuoteControlAdminIxArgs {
                new_admin,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_QUOTE_CONTROL_ADMIN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_admin, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_quote_control_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: SetQuoteControlAdminKeys,
    args: SetQuoteControlAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_QUOTE_CONTROL_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetQuoteControlAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_quote_control_admin_ix(
    keys: SetQuoteControlAdminKeys,
    args: SetQuoteControlAdminIxArgs,
) -> std::io::Result<Instruction> {
    set_quote_control_admin_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn set_quote_control_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetQuoteControlAdminAccounts<'_, '_>,
    args: SetQuoteControlAdminIxArgs,
) -> ProgramResult {
    let keys: SetQuoteControlAdminKeys = accounts.into();
    let ix = set_quote_control_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_quote_control_admin_invoke(
    accounts: SetQuoteControlAdminAccounts<'_, '_>,
    args: SetQuoteControlAdminIxArgs,
) -> ProgramResult {
    set_quote_control_admin_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn set_quote_control_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetQuoteControlAdminAccounts<'_, '_>,
    args: SetQuoteControlAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetQuoteControlAdminKeys = accounts.into();
    let ix = set_quote_control_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_quote_control_admin_invoke_signed(
    accounts: SetQuoteControlAdminAccounts<'_, '_>,
    args: SetQuoteControlAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_quote_control_admin_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_quote_control_admin_verify_account_keys(
    accounts: SetQuoteControlAdminAccounts<'_, '_>,
    keys: SetQuoteControlAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.quote_control.key, keys.quote_control),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_quote_control_admin_verify_writable_privileges<'me, 'info>(
    accounts: SetQuoteControlAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.quote_control] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_quote_control_admin_verify_signer_privileges<'me, 'info>(
    accounts: SetQuoteControlAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_quote_control_admin_verify_account_privileges<'me, 'info>(
    accounts: SetQuoteControlAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_quote_control_admin_verify_writable_privileges(accounts)?;
    set_quote_control_admin_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetReservedFeeRecipientsAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetReservedFeeRecipientsKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetReservedFeeRecipientsAccounts<'_, '_>> for SetReservedFeeRecipientsKeys {
    fn from(accounts: SetReservedFeeRecipientsAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetReservedFeeRecipientsKeys>
for [AccountMeta; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetReservedFeeRecipientsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]>
for SetReservedFeeRecipientsKeys {
    fn from(pubkeys: [Pubkey; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<SetReservedFeeRecipientsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetReservedFeeRecipientsAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]>
for SetReservedFeeRecipientsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const SET_RESERVED_FEE_RECIPIENTS_IX_DISCM: [u8; 8usize] = [
    111, 172, 162, 232, 114, 89, 213, 142,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetReservedFeeRecipientsIxArgs {
    pub whitelist_pda: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetReservedFeeRecipientsIxData(pub SetReservedFeeRecipientsIxArgs);
impl From<SetReservedFeeRecipientsIxArgs> for SetReservedFeeRecipientsIxData {
    fn from(args: SetReservedFeeRecipientsIxArgs) -> Self {
        Self(args)
    }
}
impl SetReservedFeeRecipientsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_RESERVED_FEE_RECIPIENTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let whitelist_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetReservedFeeRecipientsIxArgs {
                whitelist_pda,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_RESERVED_FEE_RECIPIENTS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.whitelist_pda, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_reserved_fee_recipients_ix_with_program_id(
    program_id: Pubkey,
    keys: SetReservedFeeRecipientsKeys,
    args: SetReservedFeeRecipientsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetReservedFeeRecipientsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_reserved_fee_recipients_ix(
    keys: SetReservedFeeRecipientsKeys,
    args: SetReservedFeeRecipientsIxArgs,
) -> std::io::Result<Instruction> {
    set_reserved_fee_recipients_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn set_reserved_fee_recipients_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
) -> ProgramResult {
    let keys: SetReservedFeeRecipientsKeys = accounts.into();
    let ix = set_reserved_fee_recipients_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_reserved_fee_recipients_invoke(
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
) -> ProgramResult {
    set_reserved_fee_recipients_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn set_reserved_fee_recipients_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetReservedFeeRecipientsKeys = accounts.into();
    let ix = set_reserved_fee_recipients_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_reserved_fee_recipients_invoke_signed(
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reserved_fee_recipients_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_reserved_fee_recipients_verify_account_keys(
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    keys: SetReservedFeeRecipientsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_reserved_fee_recipients_verify_writable_privileges<'me, 'info>(
    accounts: SetReservedFeeRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_reserved_fee_recipients_verify_signer_privileges<'me, 'info>(
    accounts: SetReservedFeeRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_reserved_fee_recipients_verify_account_privileges<'me, 'info>(
    accounts: SetReservedFeeRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reserved_fee_recipients_verify_writable_privileges(accounts)?;
    set_reserved_fee_recipients_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetVirtualQuoteReservesAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetVirtualQuoteReservesKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetVirtualQuoteReservesAccounts<'_, '_>> for SetVirtualQuoteReservesKeys {
    fn from(accounts: SetVirtualQuoteReservesAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetVirtualQuoteReservesKeys>
for [AccountMeta; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN] {
    fn from(keys: SetVirtualQuoteReservesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN]>
for SetVirtualQuoteReservesKeys {
    fn from(pubkeys: [Pubkey; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<SetVirtualQuoteReservesAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetVirtualQuoteReservesAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN]>
for SetVirtualQuoteReservesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const SET_VIRTUAL_QUOTE_RESERVES_IX_DISCM: [u8; 8usize] = [
    101, 135, 191, 104, 9, 88, 20, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetVirtualQuoteReservesIxArgs {
    pub initial_virtual_quote_reserves: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVirtualQuoteReservesIxData(pub SetVirtualQuoteReservesIxArgs);
impl From<SetVirtualQuoteReservesIxArgs> for SetVirtualQuoteReservesIxData {
    fn from(args: SetVirtualQuoteReservesIxArgs) -> Self {
        Self(args)
    }
}
impl SetVirtualQuoteReservesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VIRTUAL_QUOTE_RESERVES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let initial_virtual_quote_reserves: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(SetVirtualQuoteReservesIxArgs {
                initial_virtual_quote_reserves,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VIRTUAL_QUOTE_RESERVES_IX_DISCM)?;
        borsh::BorshSerialize::serialize(
            &self.0.initial_virtual_quote_reserves,
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
pub fn set_virtual_quote_reserves_ix_with_program_id(
    program_id: Pubkey,
    keys: SetVirtualQuoteReservesKeys,
    args: SetVirtualQuoteReservesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_VIRTUAL_QUOTE_RESERVES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetVirtualQuoteReservesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_virtual_quote_reserves_ix(
    keys: SetVirtualQuoteReservesKeys,
    args: SetVirtualQuoteReservesIxArgs,
) -> std::io::Result<Instruction> {
    set_virtual_quote_reserves_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn set_virtual_quote_reserves_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetVirtualQuoteReservesAccounts<'_, '_>,
    args: SetVirtualQuoteReservesIxArgs,
) -> ProgramResult {
    let keys: SetVirtualQuoteReservesKeys = accounts.into();
    let ix = set_virtual_quote_reserves_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_virtual_quote_reserves_invoke(
    accounts: SetVirtualQuoteReservesAccounts<'_, '_>,
    args: SetVirtualQuoteReservesIxArgs,
) -> ProgramResult {
    set_virtual_quote_reserves_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn set_virtual_quote_reserves_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetVirtualQuoteReservesAccounts<'_, '_>,
    args: SetVirtualQuoteReservesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetVirtualQuoteReservesKeys = accounts.into();
    let ix = set_virtual_quote_reserves_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_virtual_quote_reserves_invoke_signed(
    accounts: SetVirtualQuoteReservesAccounts<'_, '_>,
    args: SetVirtualQuoteReservesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_virtual_quote_reserves_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_virtual_quote_reserves_verify_account_keys(
    accounts: SetVirtualQuoteReservesAccounts<'_, '_>,
    keys: SetVirtualQuoteReservesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_virtual_quote_reserves_verify_writable_privileges<'me, 'info>(
    accounts: SetVirtualQuoteReservesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_virtual_quote_reserves_verify_signer_privileges<'me, 'info>(
    accounts: SetVirtualQuoteReservesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_virtual_quote_reserves_verify_account_privileges<'me, 'info>(
    accounts: SetVirtualQuoteReservesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_virtual_quote_reserves_verify_writable_privileges(accounts)?;
    set_virtual_quote_reserves_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SyncUserVolumeAccumulatorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SyncUserVolumeAccumulatorKeys {
    pub user: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SyncUserVolumeAccumulatorAccounts<'_, '_>> for SyncUserVolumeAccumulatorKeys {
    fn from(accounts: SyncUserVolumeAccumulatorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SyncUserVolumeAccumulatorKeys>
for [AccountMeta; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SyncUserVolumeAccumulatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
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
impl From<[Pubkey; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]>
for SyncUserVolumeAccumulatorKeys {
    fn from(pubkeys: [Pubkey; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            global_volume_accumulator: pubkeys[1],
            user_volume_accumulator: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<SyncUserVolumeAccumulatorAccounts<'_, 'info>>
for [AccountInfo<'info>; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SyncUserVolumeAccumulatorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]>
for SyncUserVolumeAccumulatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            global_volume_accumulator: &arr[1],
            user_volume_accumulator: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM: [u8; 8usize] = [
    86, 31, 192, 87, 163, 87, 79, 238,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SyncUserVolumeAccumulatorIxData;
impl SyncUserVolumeAccumulatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sync_user_volume_accumulator_ix_with_program_id(
    program_id: Pubkey,
    keys: SyncUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SyncUserVolumeAccumulatorIxData.try_to_vec()?,
    })
}
pub fn sync_user_volume_accumulator_ix(
    keys: SyncUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    sync_user_volume_accumulator_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn sync_user_volume_accumulator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SyncUserVolumeAccumulatorKeys = accounts.into();
    let ix = sync_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sync_user_volume_accumulator_invoke(
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    sync_user_volume_accumulator_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn sync_user_volume_accumulator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SyncUserVolumeAccumulatorKeys = accounts.into();
    let ix = sync_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sync_user_volume_accumulator_invoke_signed(
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sync_user_volume_accumulator_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn sync_user_volume_accumulator_verify_account_keys(
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
    keys: SyncUserVolumeAccumulatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sync_user_volume_accumulator_verify_writable_privileges<'me, 'info>(
    accounts: SyncUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sync_user_volume_accumulator_verify_account_privileges<'me, 'info>(
    accounts: SyncUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sync_user_volume_accumulator_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ToggleCashbackEnabledAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleCashbackEnabledKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ToggleCashbackEnabledAccounts<'_, '_>> for ToggleCashbackEnabledKeys {
    fn from(accounts: ToggleCashbackEnabledAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ToggleCashbackEnabledKeys>
for [AccountMeta; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN] {
    fn from(keys: ToggleCashbackEnabledKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN]>
for ToggleCashbackEnabledKeys {
    fn from(pubkeys: [Pubkey; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<ToggleCashbackEnabledAccounts<'_, 'info>>
for [AccountInfo<'info>; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN] {
    fn from(accounts: ToggleCashbackEnabledAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN]>
for ToggleCashbackEnabledAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TOGGLE_CASHBACK_ENABLED_IX_DISCM: [u8; 8usize] = [
    115, 103, 224, 255, 189, 89, 86, 195,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToggleCashbackEnabledIxArgs {
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ToggleCashbackEnabledIxData(pub ToggleCashbackEnabledIxArgs);
impl From<ToggleCashbackEnabledIxArgs> for ToggleCashbackEnabledIxData {
    fn from(args: ToggleCashbackEnabledIxArgs) -> Self {
        Self(args)
    }
}
impl ToggleCashbackEnabledIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOGGLE_CASHBACK_ENABLED_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(ToggleCashbackEnabledIxArgs {
                enabled,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_CASHBACK_ENABLED_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn toggle_cashback_enabled_ix_with_program_id(
    program_id: Pubkey,
    keys: ToggleCashbackEnabledKeys,
    args: ToggleCashbackEnabledIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_CASHBACK_ENABLED_IX_ACCOUNTS_LEN] = keys.into();
    let data: ToggleCashbackEnabledIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn toggle_cashback_enabled_ix(
    keys: ToggleCashbackEnabledKeys,
    args: ToggleCashbackEnabledIxArgs,
) -> std::io::Result<Instruction> {
    toggle_cashback_enabled_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn toggle_cashback_enabled_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ToggleCashbackEnabledAccounts<'_, '_>,
    args: ToggleCashbackEnabledIxArgs,
) -> ProgramResult {
    let keys: ToggleCashbackEnabledKeys = accounts.into();
    let ix = toggle_cashback_enabled_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn toggle_cashback_enabled_invoke(
    accounts: ToggleCashbackEnabledAccounts<'_, '_>,
    args: ToggleCashbackEnabledIxArgs,
) -> ProgramResult {
    toggle_cashback_enabled_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn toggle_cashback_enabled_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ToggleCashbackEnabledAccounts<'_, '_>,
    args: ToggleCashbackEnabledIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ToggleCashbackEnabledKeys = accounts.into();
    let ix = toggle_cashback_enabled_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn toggle_cashback_enabled_invoke_signed(
    accounts: ToggleCashbackEnabledAccounts<'_, '_>,
    args: ToggleCashbackEnabledIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_cashback_enabled_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn toggle_cashback_enabled_verify_account_keys(
    accounts: ToggleCashbackEnabledAccounts<'_, '_>,
    keys: ToggleCashbackEnabledKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn toggle_cashback_enabled_verify_writable_privileges<'me, 'info>(
    accounts: ToggleCashbackEnabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn toggle_cashback_enabled_verify_signer_privileges<'me, 'info>(
    accounts: ToggleCashbackEnabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn toggle_cashback_enabled_verify_account_privileges<'me, 'info>(
    accounts: ToggleCashbackEnabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_cashback_enabled_verify_writable_privileges(accounts)?;
    toggle_cashback_enabled_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ToggleCreateV2Accounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleCreateV2Keys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ToggleCreateV2Accounts<'_, '_>> for ToggleCreateV2Keys {
    fn from(accounts: ToggleCreateV2Accounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ToggleCreateV2Keys> for [AccountMeta; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: ToggleCreateV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN]> for ToggleCreateV2Keys {
    fn from(pubkeys: [Pubkey; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<ToggleCreateV2Accounts<'_, 'info>>
for [AccountInfo<'info>; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ToggleCreateV2Accounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN]>
for ToggleCreateV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TOGGLE_CREATE_V2_IX_DISCM: [u8; 8usize] = [
    28, 255, 230, 240, 172, 107, 203, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToggleCreateV2IxArgs {
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ToggleCreateV2IxData(pub ToggleCreateV2IxArgs);
impl From<ToggleCreateV2IxArgs> for ToggleCreateV2IxData {
    fn from(args: ToggleCreateV2IxArgs) -> Self {
        Self(args)
    }
}
impl ToggleCreateV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOGGLE_CREATE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ToggleCreateV2IxArgs { enabled }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_CREATE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn toggle_create_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: ToggleCreateV2Keys,
    args: ToggleCreateV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_CREATE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: ToggleCreateV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn toggle_create_v2_ix(
    keys: ToggleCreateV2Keys,
    args: ToggleCreateV2IxArgs,
) -> std::io::Result<Instruction> {
    toggle_create_v2_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn toggle_create_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
) -> ProgramResult {
    let keys: ToggleCreateV2Keys = accounts.into();
    let ix = toggle_create_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn toggle_create_v2_invoke(
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
) -> ProgramResult {
    toggle_create_v2_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn toggle_create_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ToggleCreateV2Keys = accounts.into();
    let ix = toggle_create_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn toggle_create_v2_invoke_signed(
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_create_v2_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn toggle_create_v2_verify_account_keys(
    accounts: ToggleCreateV2Accounts<'_, '_>,
    keys: ToggleCreateV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn toggle_create_v2_verify_writable_privileges<'me, 'info>(
    accounts: ToggleCreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn toggle_create_v2_verify_signer_privileges<'me, 'info>(
    accounts: ToggleCreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn toggle_create_v2_verify_account_privileges<'me, 'info>(
    accounts: ToggleCreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_create_v2_verify_writable_privileges(accounts)?;
    toggle_create_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ToggleMayhemModeAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleMayhemModeKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ToggleMayhemModeAccounts<'_, '_>> for ToggleMayhemModeKeys {
    fn from(accounts: ToggleMayhemModeAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ToggleMayhemModeKeys> for [AccountMeta; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN] {
    fn from(keys: ToggleMayhemModeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]> for ToggleMayhemModeKeys {
    fn from(pubkeys: [Pubkey; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<ToggleMayhemModeAccounts<'_, 'info>>
for [AccountInfo<'info>; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ToggleMayhemModeAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]>
for ToggleMayhemModeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TOGGLE_MAYHEM_MODE_IX_DISCM: [u8; 8usize] = [
    1, 9, 111, 208, 100, 31, 255, 163,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToggleMayhemModeIxArgs {
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ToggleMayhemModeIxData(pub ToggleMayhemModeIxArgs);
impl From<ToggleMayhemModeIxArgs> for ToggleMayhemModeIxData {
    fn from(args: ToggleMayhemModeIxArgs) -> Self {
        Self(args)
    }
}
impl ToggleMayhemModeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOGGLE_MAYHEM_MODE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(ToggleMayhemModeIxArgs { enabled }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_MAYHEM_MODE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.enabled, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn toggle_mayhem_mode_ix_with_program_id(
    program_id: Pubkey,
    keys: ToggleMayhemModeKeys,
    args: ToggleMayhemModeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ToggleMayhemModeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn toggle_mayhem_mode_ix(
    keys: ToggleMayhemModeKeys,
    args: ToggleMayhemModeIxArgs,
) -> std::io::Result<Instruction> {
    toggle_mayhem_mode_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn toggle_mayhem_mode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
) -> ProgramResult {
    let keys: ToggleMayhemModeKeys = accounts.into();
    let ix = toggle_mayhem_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn toggle_mayhem_mode_invoke(
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
) -> ProgramResult {
    toggle_mayhem_mode_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn toggle_mayhem_mode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ToggleMayhemModeKeys = accounts.into();
    let ix = toggle_mayhem_mode_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn toggle_mayhem_mode_invoke_signed(
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_mayhem_mode_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn toggle_mayhem_mode_verify_account_keys(
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    keys: ToggleMayhemModeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn toggle_mayhem_mode_verify_writable_privileges<'me, 'info>(
    accounts: ToggleMayhemModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn toggle_mayhem_mode_verify_signer_privileges<'me, 'info>(
    accounts: ToggleMayhemModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn toggle_mayhem_mode_verify_account_privileges<'me, 'info>(
    accounts: ToggleMayhemModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_mayhem_mode_verify_writable_privileges(accounts)?;
    toggle_mayhem_mode_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateBuybackConfigAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateBuybackConfigKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateBuybackConfigAccounts<'_, '_>> for UpdateBuybackConfigKeys {
    fn from(accounts: UpdateBuybackConfigAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateBuybackConfigKeys>
for [AccountMeta; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateBuybackConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN]> for UpdateBuybackConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateBuybackConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateBuybackConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateBuybackConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_BUYBACK_CONFIG_IX_DISCM: [u8; 8usize] = [
    251, 224, 171, 146, 160, 26, 113, 233,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateBuybackConfigIxArgs {
    pub buyback_basis_points: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateBuybackConfigIxData(pub UpdateBuybackConfigIxArgs);
impl From<UpdateBuybackConfigIxArgs> for UpdateBuybackConfigIxData {
    fn from(args: UpdateBuybackConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateBuybackConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_BUYBACK_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let buyback_basis_points: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdateBuybackConfigIxArgs {
                buyback_basis_points,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_BUYBACK_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.buyback_basis_points, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_buyback_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateBuybackConfigKeys,
    args: UpdateBuybackConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_BUYBACK_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateBuybackConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_buyback_config_ix(
    keys: UpdateBuybackConfigKeys,
    args: UpdateBuybackConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_buyback_config_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn update_buyback_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateBuybackConfigAccounts<'_, '_>,
    args: UpdateBuybackConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateBuybackConfigKeys = accounts.into();
    let ix = update_buyback_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_buyback_config_invoke(
    accounts: UpdateBuybackConfigAccounts<'_, '_>,
    args: UpdateBuybackConfigIxArgs,
) -> ProgramResult {
    update_buyback_config_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn update_buyback_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateBuybackConfigAccounts<'_, '_>,
    args: UpdateBuybackConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateBuybackConfigKeys = accounts.into();
    let ix = update_buyback_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_buyback_config_invoke_signed(
    accounts: UpdateBuybackConfigAccounts<'_, '_>,
    args: UpdateBuybackConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_buyback_config_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_buyback_config_verify_account_keys(
    accounts: UpdateBuybackConfigAccounts<'_, '_>,
    keys: UpdateBuybackConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_buyback_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateBuybackConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_buyback_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateBuybackConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_buyback_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateBuybackConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_buyback_config_verify_writable_privileges(accounts)?;
    update_buyback_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateCreatorFeeConfigAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateCreatorFeeConfigKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateCreatorFeeConfigAccounts<'_, '_>> for UpdateCreatorFeeConfigKeys {
    fn from(accounts: UpdateCreatorFeeConfigAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateCreatorFeeConfigKeys>
for [AccountMeta; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateCreatorFeeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateCreatorFeeConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateCreatorFeeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateCreatorFeeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateCreatorFeeConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_CREATOR_FEE_CONFIG_IX_DISCM: [u8; 8usize] = [
    61, 175, 160, 249, 66, 66, 136, 175,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateCreatorFeeConfigIxArgs {
    pub creator_fee_configurable: bool,
    pub max_configurable_creator_fee_bps: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCreatorFeeConfigIxData(pub UpdateCreatorFeeConfigIxArgs);
impl From<UpdateCreatorFeeConfigIxArgs> for UpdateCreatorFeeConfigIxData {
    fn from(args: UpdateCreatorFeeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateCreatorFeeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CREATOR_FEE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let creator_fee_configurable: bool = crate::borsh_de_or_default(&mut reader)?;
        let max_configurable_creator_fee_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateCreatorFeeConfigIxArgs {
                creator_fee_configurable,
                max_configurable_creator_fee_bps,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CREATOR_FEE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.creator_fee_configurable, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.max_configurable_creator_fee_bps,
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
pub fn update_creator_fee_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateCreatorFeeConfigKeys,
    args: UpdateCreatorFeeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CREATOR_FEE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateCreatorFeeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_creator_fee_config_ix(
    keys: UpdateCreatorFeeConfigKeys,
    args: UpdateCreatorFeeConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_creator_fee_config_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn update_creator_fee_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCreatorFeeConfigAccounts<'_, '_>,
    args: UpdateCreatorFeeConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateCreatorFeeConfigKeys = accounts.into();
    let ix = update_creator_fee_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_creator_fee_config_invoke(
    accounts: UpdateCreatorFeeConfigAccounts<'_, '_>,
    args: UpdateCreatorFeeConfigIxArgs,
) -> ProgramResult {
    update_creator_fee_config_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn update_creator_fee_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateCreatorFeeConfigAccounts<'_, '_>,
    args: UpdateCreatorFeeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateCreatorFeeConfigKeys = accounts.into();
    let ix = update_creator_fee_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_creator_fee_config_invoke_signed(
    accounts: UpdateCreatorFeeConfigAccounts<'_, '_>,
    args: UpdateCreatorFeeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_creator_fee_config_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_creator_fee_config_verify_account_keys(
    accounts: UpdateCreatorFeeConfigAccounts<'_, '_>,
    keys: UpdateCreatorFeeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_creator_fee_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateCreatorFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_creator_fee_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateCreatorFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_creator_fee_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateCreatorFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_creator_fee_config_verify_writable_privileges(accounts)?;
    update_creator_fee_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalAuthorityAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalAuthorityKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub new_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateGlobalAuthorityAccounts<'_, '_>> for UpdateGlobalAuthorityKeys {
    fn from(accounts: UpdateGlobalAuthorityAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            new_authority: *accounts.new_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateGlobalAuthorityKeys>
for [AccountMeta; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_authority,
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
impl From<[Pubkey; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateGlobalAuthorityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            new_authority: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateGlobalAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.new_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]>
for UpdateGlobalAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            new_authority: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_GLOBAL_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    227, 181, 74, 196, 208, 21, 97, 213,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalAuthorityIxData;
impl UpdateGlobalAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_GLOBAL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_global_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateGlobalAuthorityIxData.try_to_vec()?,
    })
}
pub fn update_global_authority_ix(
    keys: UpdateGlobalAuthorityKeys,
) -> std::io::Result<Instruction> {
    update_global_authority_ix_with_program_id(PUMP_PROGRAM_ID, keys)
}
pub fn update_global_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateGlobalAuthorityKeys = accounts.into();
    let ix = update_global_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_global_authority_invoke(
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    update_global_authority_invoke_with_program_id(PUMP_PROGRAM_ID, accounts)
}
pub fn update_global_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateGlobalAuthorityKeys = accounts.into();
    let ix = update_global_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_global_authority_invoke_signed(
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_authority_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_global_authority_verify_account_keys(
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
    keys: UpdateGlobalAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_global_authority_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_global_authority_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_global_authority_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_authority_verify_writable_privileges(accounts)?;
    update_global_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateHolderRewardConfigAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateHolderRewardConfigKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateHolderRewardConfigAccounts<'_, '_>> for UpdateHolderRewardConfigKeys {
    fn from(accounts: UpdateHolderRewardConfigAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateHolderRewardConfigKeys>
for [AccountMeta; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateHolderRewardConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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
impl From<[Pubkey; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateHolderRewardConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateHolderRewardConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateHolderRewardConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateHolderRewardConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_HOLDER_REWARD_CONFIG_IX_DISCM: [u8; 8usize] = [
    225, 252, 66, 4, 199, 35, 236, 16,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateHolderRewardConfigIxArgs {
    pub is_holder_reward_enabled: bool,
    pub holder_reward_claim_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateHolderRewardConfigIxData(pub UpdateHolderRewardConfigIxArgs);
impl From<UpdateHolderRewardConfigIxArgs> for UpdateHolderRewardConfigIxData {
    fn from(args: UpdateHolderRewardConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateHolderRewardConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_HOLDER_REWARD_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let is_holder_reward_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let holder_reward_claim_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(UpdateHolderRewardConfigIxArgs {
                is_holder_reward_enabled,
                holder_reward_claim_authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_HOLDER_REWARD_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.is_holder_reward_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.holder_reward_claim_authority,
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
pub fn update_holder_reward_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateHolderRewardConfigKeys,
    args: UpdateHolderRewardConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_HOLDER_REWARD_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateHolderRewardConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_holder_reward_config_ix(
    keys: UpdateHolderRewardConfigKeys,
    args: UpdateHolderRewardConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_holder_reward_config_ix_with_program_id(PUMP_PROGRAM_ID, keys, args)
}
pub fn update_holder_reward_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateHolderRewardConfigAccounts<'_, '_>,
    args: UpdateHolderRewardConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateHolderRewardConfigKeys = accounts.into();
    let ix = update_holder_reward_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_holder_reward_config_invoke(
    accounts: UpdateHolderRewardConfigAccounts<'_, '_>,
    args: UpdateHolderRewardConfigIxArgs,
) -> ProgramResult {
    update_holder_reward_config_invoke_with_program_id(PUMP_PROGRAM_ID, accounts, args)
}
pub fn update_holder_reward_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateHolderRewardConfigAccounts<'_, '_>,
    args: UpdateHolderRewardConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateHolderRewardConfigKeys = accounts.into();
    let ix = update_holder_reward_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_holder_reward_config_invoke_signed(
    accounts: UpdateHolderRewardConfigAccounts<'_, '_>,
    args: UpdateHolderRewardConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_holder_reward_config_invoke_signed_with_program_id(
        PUMP_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_holder_reward_config_verify_account_keys(
    accounts: UpdateHolderRewardConfigAccounts<'_, '_>,
    keys: UpdateHolderRewardConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_holder_reward_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateHolderRewardConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_holder_reward_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateHolderRewardConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_holder_reward_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateHolderRewardConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_holder_reward_config_verify_writable_privileges(accounts)?;
    update_holder_reward_config_verify_signer_privileges(accounts)?;
    Ok(())
}
