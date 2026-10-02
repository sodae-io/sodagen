use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum RaydiumLaunchpadProgramIx {
    BuyExactIn(BuyExactInIxArgs),
    BuyExactOut(BuyExactOutIxArgs),
    ClaimCreatorFee,
    ClaimPlatformFee,
    ClaimPlatformFeeFromVault,
    ClaimVestedToken,
    ClosePlatformAllowConfig,
    ClosePlatformCurveRule,
    CollectExcessLamports,
    CollectFee,
    CollectMigrateFee,
    CreateConfig(CreateConfigIxArgs),
    CreatePlatformAllowConfig,
    CreatePlatformConfig(CreatePlatformConfigIxArgs),
    CreatePlatformCurveRule,
    CreatePlatformVestingAccount,
    CreateVestingAccount(CreateVestingAccountIxArgs),
    Initialize(InitializeIxArgs),
    InitializeV2(InitializeV2IxArgs),
    InitializeWithToken2022(InitializeWithToken2022IxArgs),
    MigrateToAmm,
    MigrateToCpswap,
    RemovePlatformCurveRule(RemovePlatformCurveRuleIxArgs),
    SellExactIn(SellExactInIxArgs),
    SellExactOut(SellExactOutIxArgs),
    UpdateConfig(UpdateConfigIxArgs),
    UpdatePlatformConfig(UpdatePlatformConfigIxArgs),
    UpdatePlatformCurveRule(UpdatePlatformCurveRuleIxArgs),
}
impl RaydiumLaunchpadProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BUY_EXACT_IN_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_IN_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyExactIn(BuyExactInIxArgs {
                    amount_in,
                    minimum_amount_out,
                    share_fee_rate,
                }),
            );
        }
        if buf.starts_with(&BUY_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_OUT_IX_DISCM.len()..];
            let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::BuyExactOut(BuyExactOutIxArgs {
                    amount_out,
                    maximum_amount_in,
                    share_fee_rate,
                }),
            );
        }
        if buf.starts_with(&CLAIM_CREATOR_FEE_IX_DISCM) {
            return Ok(Self::ClaimCreatorFee);
        }
        if buf.starts_with(&CLAIM_PLATFORM_FEE_IX_DISCM) {
            return Ok(Self::ClaimPlatformFee);
        }
        if buf.starts_with(&CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM) {
            return Ok(Self::ClaimPlatformFeeFromVault);
        }
        if buf.starts_with(&CLAIM_VESTED_TOKEN_IX_DISCM) {
            return Ok(Self::ClaimVestedToken);
        }
        if buf.starts_with(&CLOSE_PLATFORM_ALLOW_CONFIG_IX_DISCM) {
            return Ok(Self::ClosePlatformAllowConfig);
        }
        if buf.starts_with(&CLOSE_PLATFORM_CURVE_RULE_IX_DISCM) {
            return Ok(Self::ClosePlatformCurveRule);
        }
        if buf.starts_with(&COLLECT_EXCESS_LAMPORTS_IX_DISCM) {
            return Ok(Self::CollectExcessLamports);
        }
        if buf.starts_with(&COLLECT_FEE_IX_DISCM) {
            return Ok(Self::CollectFee);
        }
        if buf.starts_with(&COLLECT_MIGRATE_FEE_IX_DISCM) {
            return Ok(Self::CollectMigrateFee);
        }
        if buf.starts_with(&CREATE_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_CONFIG_IX_DISCM.len()..];
            let curve_type: u8 = crate::borsh_de_or_default(&mut reader)?;
            let index: u16 = crate::borsh_de_or_default(&mut reader)?;
            let migrate_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
            let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateConfig(CreateConfigIxArgs {
                    curve_type,
                    index,
                    migrate_fee,
                    trade_fee_rate,
                }),
            );
        }
        if buf.starts_with(&CREATE_PLATFORM_ALLOW_CONFIG_IX_DISCM) {
            return Ok(Self::CreatePlatformAllowConfig);
        }
        if buf.starts_with(&CREATE_PLATFORM_CONFIG_IX_DISCM) {
            let mut reader = &buf[CREATE_PLATFORM_CONFIG_IX_DISCM.len()..];
            let platform_params = if reader.is_empty() {
                Default::default()
            } else {
                <PlatformParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::CreatePlatformConfig(CreatePlatformConfigIxArgs {
                    platform_params,
                }),
            );
        }
        if buf.starts_with(&CREATE_PLATFORM_CURVE_RULE_IX_DISCM) {
            return Ok(Self::CreatePlatformCurveRule);
        }
        if buf.starts_with(&CREATE_PLATFORM_VESTING_ACCOUNT_IX_DISCM) {
            return Ok(Self::CreatePlatformVestingAccount);
        }
        if buf.starts_with(&CREATE_VESTING_ACCOUNT_IX_DISCM) {
            let mut reader = &buf[CREATE_VESTING_ACCOUNT_IX_DISCM.len()..];
            let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateVestingAccount(CreateVestingAccountIxArgs {
                    share_amount,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_IX_DISCM.len()..];
            let base_mint_param = if reader.is_empty() {
                Default::default()
            } else {
                <MintParams>::deserialize(&mut reader)?
            };
            let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            let vesting_param = if reader.is_empty() {
                Default::default()
            } else {
                <VestingParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::Initialize(InitializeIxArgs {
                    base_mint_param,
                    curve_param,
                    vesting_param,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_V2_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_V2_IX_DISCM.len()..];
            let base_mint_param = if reader.is_empty() {
                Default::default()
            } else {
                <MintParams>::deserialize(&mut reader)?
            };
            let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            let vesting_param = if reader.is_empty() {
                Default::default()
            } else {
                <VestingParams>::deserialize(&mut reader)?
            };
            let amm_fee_on: AmmCreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::InitializeV2(InitializeV2IxArgs {
                    base_mint_param,
                    curve_param,
                    vesting_param,
                    amm_fee_on,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_WITH_TOKEN_2022_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_WITH_TOKEN_2022_IX_DISCM.len()..];
            let base_mint_param = if reader.is_empty() {
                Default::default()
            } else {
                <MintParams>::deserialize(&mut reader)?
            };
            let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            let vesting_param = if reader.is_empty() {
                Default::default()
            } else {
                <VestingParams>::deserialize(&mut reader)?
            };
            let amm_fee_on: AmmCreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
            let transfer_fee_extension_param: Option<TransferFeeExtensionParams> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::InitializeWithToken2022(InitializeWithToken2022IxArgs {
                    base_mint_param,
                    curve_param,
                    vesting_param,
                    amm_fee_on,
                    transfer_fee_extension_param,
                }),
            );
        }
        if buf.starts_with(&MIGRATE_TO_AMM_IX_DISCM) {
            return Ok(Self::MigrateToAmm);
        }
        if buf.starts_with(&MIGRATE_TO_CPSWAP_IX_DISCM) {
            return Ok(Self::MigrateToCpswap);
        }
        if buf.starts_with(&REMOVE_PLATFORM_CURVE_RULE_IX_DISCM) {
            let mut reader = &buf[REMOVE_PLATFORM_CURVE_RULE_IX_DISCM.len()..];
            let group_id: u16 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::RemovePlatformCurveRule(RemovePlatformCurveRuleIxArgs {
                    group_id,
                }),
            );
        }
        if buf.starts_with(&SELL_EXACT_IN_IX_DISCM) {
            let mut reader = &buf[SELL_EXACT_IN_IX_DISCM.len()..];
            let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellExactIn(SellExactInIxArgs {
                    amount_in,
                    minimum_amount_out,
                    share_fee_rate,
                }),
            );
        }
        if buf.starts_with(&SELL_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[SELL_EXACT_OUT_IX_DISCM.len()..];
            let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
            let maximum_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
            let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::SellExactOut(SellExactOutIxArgs {
                    amount_out,
                    maximum_amount_in,
                    share_fee_rate,
                }),
            );
        }
        if buf.starts_with(&UPDATE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_CONFIG_IX_DISCM.len()..];
            let param: u8 = crate::borsh_de_or_default(&mut reader)?;
            let value: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(Self::UpdateConfig(UpdateConfigIxArgs { param, value }));
        }
        if buf.starts_with(&UPDATE_PLATFORM_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_PLATFORM_CONFIG_IX_DISCM.len()..];
            let param = <PlatformConfigParam as borsh::BorshDeserialize>::deserialize_reader(
                &mut reader,
            )?;
            return Ok(
                Self::UpdatePlatformConfig(UpdatePlatformConfigIxArgs {
                    param,
                }),
            );
        }
        if buf.starts_with(&UPDATE_PLATFORM_CURVE_RULE_IX_DISCM) {
            let mut reader = &buf[UPDATE_PLATFORM_CURVE_RULE_IX_DISCM.len()..];
            let group_id: u16 = crate::borsh_de_or_default(&mut reader)?;
            let constraints: Vec<ParamConstraint> = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::UpdatePlatformCurveRule(UpdatePlatformCurveRuleIxArgs {
                    group_id,
                    constraints,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BuyExactIn(args) => {
                writer.write_all(&BUY_EXACT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::BuyExactOut(args) => {
                writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.maximum_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::ClaimCreatorFee => writer.write_all(&CLAIM_CREATOR_FEE_IX_DISCM),
            Self::ClaimPlatformFee => writer.write_all(&CLAIM_PLATFORM_FEE_IX_DISCM),
            Self::ClaimPlatformFeeFromVault => {
                writer.write_all(&CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM)
            }
            Self::ClaimVestedToken => writer.write_all(&CLAIM_VESTED_TOKEN_IX_DISCM),
            Self::ClosePlatformAllowConfig => {
                writer.write_all(&CLOSE_PLATFORM_ALLOW_CONFIG_IX_DISCM)
            }
            Self::ClosePlatformCurveRule => {
                writer.write_all(&CLOSE_PLATFORM_CURVE_RULE_IX_DISCM)
            }
            Self::CollectExcessLamports => {
                writer.write_all(&COLLECT_EXCESS_LAMPORTS_IX_DISCM)
            }
            Self::CollectFee => writer.write_all(&COLLECT_FEE_IX_DISCM),
            Self::CollectMigrateFee => writer.write_all(&COLLECT_MIGRATE_FEE_IX_DISCM),
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.curve_type, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.index, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.migrate_fee, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.trade_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::CreatePlatformAllowConfig => {
                writer.write_all(&CREATE_PLATFORM_ALLOW_CONFIG_IX_DISCM)
            }
            Self::CreatePlatformConfig(args) => {
                writer.write_all(&CREATE_PLATFORM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.platform_params, &mut writer)?;
                Ok(())
            }
            Self::CreatePlatformCurveRule => {
                writer.write_all(&CREATE_PLATFORM_CURVE_RULE_IX_DISCM)
            }
            Self::CreatePlatformVestingAccount => {
                writer.write_all(&CREATE_PLATFORM_VESTING_ACCOUNT_IX_DISCM)
            }
            Self::CreateVestingAccount(args) => {
                writer.write_all(&CREATE_VESTING_ACCOUNT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.share_amount, &mut writer)?;
                Ok(())
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_mint_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.curve_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.vesting_param, &mut writer)?;
                Ok(())
            }
            Self::InitializeV2(args) => {
                writer.write_all(&INITIALIZE_V2_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_mint_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.curve_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.vesting_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amm_fee_on, &mut writer)?;
                Ok(())
            }
            Self::InitializeWithToken2022(args) => {
                writer.write_all(&INITIALIZE_WITH_TOKEN_2022_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.base_mint_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.curve_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.vesting_param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.amm_fee_on, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.transfer_fee_extension_param,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::MigrateToAmm => writer.write_all(&MIGRATE_TO_AMM_IX_DISCM),
            Self::MigrateToCpswap => writer.write_all(&MIGRATE_TO_CPSWAP_IX_DISCM),
            Self::RemovePlatformCurveRule(args) => {
                writer.write_all(&REMOVE_PLATFORM_CURVE_RULE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.group_id, &mut writer)?;
                Ok(())
            }
            Self::SellExactIn(args) => {
                writer.write_all(&SELL_EXACT_IN_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.minimum_amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::SellExactOut(args) => {
                writer.write_all(&SELL_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.amount_out, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.maximum_amount_in, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.share_fee_rate, &mut writer)?;
                Ok(())
            }
            Self::UpdateConfig(args) => {
                writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.value, &mut writer)?;
                Ok(())
            }
            Self::UpdatePlatformConfig(args) => {
                writer.write_all(&UPDATE_PLATFORM_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.param, &mut writer)?;
                Ok(())
            }
            Self::UpdatePlatformCurveRule(args) => {
                writer.write_all(&UPDATE_PLATFORM_CURVE_RULE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.group_id, &mut writer)?;
                borsh::BorshSerialize::serialize(&args.constraints, &mut writer)?;
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
pub const BUY_EXACT_IN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactInAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactInKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyExactInAccounts<'_, '_>> for BuyExactInKeys {
    fn from(accounts: BuyExactInAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyExactInKeys> for [AccountMeta; BUY_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
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
impl From<[Pubkey; BUY_EXACT_IN_IX_ACCOUNTS_LEN]> for BuyExactInKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<BuyExactInAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN]>
for BuyExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const BUY_EXACT_IN_IX_DISCM: [u8; 8usize] = [250, 234, 13, 123, 213, 156, 19, 236];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactInIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactInIxData(pub BuyExactInIxArgs);
impl From<BuyExactInIxArgs> for BuyExactInIxData {
    fn from(args: BuyExactInIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyExactInIxArgs {
                amount_in,
                minimum_amount_out,
                share_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactInKeys,
    args: BuyExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_in_ix(
    keys: BuyExactInKeys,
    args: BuyExactInIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_in_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn buy_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
) -> ProgramResult {
    let keys: BuyExactInKeys = accounts.into();
    let ix = buy_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_in_invoke(
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
) -> ProgramResult {
    buy_exact_in_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactInKeys = accounts.into();
    let ix = buy_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_in_invoke_signed(
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_in_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_in_verify_account_keys(
    accounts: BuyExactInAccounts<'_, '_>,
    keys: BuyExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_in_verify_account_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_in_verify_writable_privileges(accounts)?;
    buy_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactOutAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactOutKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyExactOutAccounts<'_, '_>> for BuyExactOutKeys {
    fn from(accounts: BuyExactOutAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyExactOutKeys> for [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
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
impl From<[Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]> for BuyExactOutKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<BuyExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]>
for BuyExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const BUY_EXACT_OUT_IX_DISCM: [u8; 8usize] = [24, 211, 116, 40, 105, 3, 153, 56];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactOutIxArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactOutIxData(pub BuyExactOutIxArgs);
impl From<BuyExactOutIxArgs> for BuyExactOutIxData {
    fn from(args: BuyExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(BuyExactOutIxArgs {
                amount_out,
                maximum_amount_in,
                share_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactOutKeys,
    args: BuyExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_out_ix(
    keys: BuyExactOutKeys,
    args: BuyExactOutIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_out_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn buy_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
) -> ProgramResult {
    let keys: BuyExactOutKeys = accounts.into();
    let ix = buy_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_out_invoke(
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
) -> ProgramResult {
    buy_exact_out_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn buy_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactOutKeys = accounts.into();
    let ix = buy_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_out_invoke_signed(
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_out_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn buy_exact_out_verify_account_keys(
    accounts: BuyExactOutAccounts<'_, '_>,
    keys: BuyExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_out_verify_account_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_out_verify_writable_privileges(accounts)?;
    buy_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorFeeAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub creator_fee_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeKeys {
    pub creator: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub creator_fee_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<ClaimCreatorFeeAccounts<'_, '_>> for ClaimCreatorFeeKeys {
    fn from(accounts: ClaimCreatorFeeAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            creator_fee_vault: *accounts.creator_fee_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<ClaimCreatorFeeKeys> for [AccountMeta; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
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
        ]
    }
}
impl From<[Pubkey; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]> for ClaimCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            creator_fee_vault: pubkeys[2],
            recipient_token_account: pubkeys[3],
            quote_mint: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<ClaimCreatorFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.creator_fee_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for ClaimCreatorFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            fee_vault_authority: &arr[1],
            creator_fee_vault: &arr[2],
            recipient_token_account: &arr[3],
            quote_mint: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}
pub const CLAIM_CREATOR_FEE_IX_DISCM: [u8; 8usize] = [
    26, 97, 138, 203, 132, 171, 141, 252,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeIxData;
impl ClaimCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCreatorFeeIxData.try_to_vec()?,
    })
}
pub fn claim_creator_fee_ix(keys: ClaimCreatorFeeKeys) -> std::io::Result<Instruction> {
    claim_creator_fee_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn claim_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCreatorFeeKeys = accounts.into();
    let ix = claim_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_creator_fee_invoke(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    claim_creator_fee_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn claim_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorFeeKeys = accounts.into();
    let ix = claim_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_creator_fee_invoke_signed(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_fee_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_creator_fee_verify_account_keys(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    keys: ClaimCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.creator_fee_vault.key, keys.creator_fee_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.creator_fee_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_creator_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_fee_verify_writable_privileges(accounts)?;
    claim_creator_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPlatformFeeAccounts<'me, 'info> {
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeKeys {
    pub platform_fee_wallet: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub platform_config: Pubkey,
    pub quote_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<ClaimPlatformFeeAccounts<'_, '_>> for ClaimPlatformFeeKeys {
    fn from(accounts: ClaimPlatformFeeAccounts) -> Self {
        Self {
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            platform_config: *accounts.platform_config.key,
            quote_vault: *accounts.quote_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<ClaimPlatformFeeKeys> for [AccountMeta; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPlatformFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
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
        ]
    }
}
impl From<[Pubkey; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]> for ClaimPlatformFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            platform_config: pubkeys[3],
            quote_vault: pubkeys[4],
            recipient_token_account: pubkeys[5],
            quote_mint: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimPlatformFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPlatformFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_fee_wallet.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.platform_config.clone(),
            accounts.quote_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]>
for ClaimPlatformFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            platform_config: &arr[3],
            quote_vault: &arr[4],
            recipient_token_account: &arr[5],
            quote_mint: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const CLAIM_PLATFORM_FEE_IX_DISCM: [u8; 8usize] = [
    156, 39, 208, 135, 76, 237, 61, 72,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeIxData;
impl ClaimPlatformFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PLATFORM_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PLATFORM_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_platform_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPlatformFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimPlatformFeeIxData.try_to_vec()?,
    })
}
pub fn claim_platform_fee_ix(
    keys: ClaimPlatformFeeKeys,
) -> std::io::Result<Instruction> {
    claim_platform_fee_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn claim_platform_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimPlatformFeeKeys = accounts.into();
    let ix = claim_platform_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_platform_fee_invoke(
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
) -> ProgramResult {
    claim_platform_fee_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn claim_platform_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPlatformFeeKeys = accounts.into();
    let ix = claim_platform_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_platform_fee_invoke_signed(
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_platform_fee_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_platform_fee_verify_account_keys(
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    keys: ClaimPlatformFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_platform_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.platform_fee_wallet,
        accounts.pool_state,
        accounts.quote_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_platform_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_fee_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_platform_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_platform_fee_verify_writable_privileges(accounts)?;
    claim_platform_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPlatformFeeFromVaultAccounts<'me, 'info> {
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub platform_fee_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeFromVaultKeys {
    pub platform_fee_wallet: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub platform_config: Pubkey,
    pub platform_fee_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<ClaimPlatformFeeFromVaultAccounts<'_, '_>> for ClaimPlatformFeeFromVaultKeys {
    fn from(accounts: ClaimPlatformFeeFromVaultAccounts) -> Self {
        Self {
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            platform_config: *accounts.platform_config.key,
            platform_fee_vault: *accounts.platform_fee_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<ClaimPlatformFeeFromVaultKeys>
for [AccountMeta; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPlatformFeeFromVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
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
        ]
    }
}
impl From<[Pubkey; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]>
for ClaimPlatformFeeFromVaultKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            platform_config: pubkeys[2],
            platform_fee_vault: pubkeys[3],
            recipient_token_account: pubkeys[4],
            quote_mint: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<ClaimPlatformFeeFromVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPlatformFeeFromVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_fee_wallet.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.platform_config.clone(),
            accounts.platform_fee_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]>
for ClaimPlatformFeeFromVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            platform_fee_wallet: &arr[0],
            fee_vault_authority: &arr[1],
            platform_config: &arr[2],
            platform_fee_vault: &arr[3],
            recipient_token_account: &arr[4],
            quote_mint: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}
pub const CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM: [u8; 8usize] = [
    117, 241, 198, 168, 248, 218, 80, 29,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeFromVaultIxData;
impl ClaimPlatformFeeFromVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_platform_fee_from_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPlatformFeeFromVaultKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimPlatformFeeFromVaultIxData.try_to_vec()?,
    })
}
pub fn claim_platform_fee_from_vault_ix(
    keys: ClaimPlatformFeeFromVaultKeys,
) -> std::io::Result<Instruction> {
    claim_platform_fee_from_vault_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn claim_platform_fee_from_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimPlatformFeeFromVaultKeys = accounts.into();
    let ix = claim_platform_fee_from_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_platform_fee_from_vault_invoke(
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
) -> ProgramResult {
    claim_platform_fee_from_vault_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn claim_platform_fee_from_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPlatformFeeFromVaultKeys = accounts.into();
    let ix = claim_platform_fee_from_vault_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_platform_fee_from_vault_invoke_signed(
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_platform_fee_from_vault_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_platform_fee_from_vault_verify_account_keys(
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    keys: ClaimPlatformFeeFromVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.platform_fee_vault.key, keys.platform_fee_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_platform_fee_from_vault_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeFromVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.platform_fee_wallet,
        accounts.platform_fee_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_platform_fee_from_vault_verify_signer_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeFromVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_fee_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_platform_fee_from_vault_verify_account_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeFromVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_platform_fee_from_vault_verify_writable_privileges(accounts)?;
    claim_platform_fee_from_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimVestedTokenAccounts<'me, 'info> {
    pub beneficiary: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub vesting_record: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimVestedTokenKeys {
    pub beneficiary: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub vesting_record: Pubkey,
    pub base_vault: Pubkey,
    pub user_base_token: Pubkey,
    pub base_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<ClaimVestedTokenAccounts<'_, '_>> for ClaimVestedTokenKeys {
    fn from(accounts: ClaimVestedTokenAccounts) -> Self {
        Self {
            beneficiary: *accounts.beneficiary.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            vesting_record: *accounts.vesting_record.key,
            base_vault: *accounts.base_vault.key,
            user_base_token: *accounts.user_base_token.key,
            base_token_mint: *accounts.base_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<ClaimVestedTokenKeys> for [AccountMeta; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimVestedTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.beneficiary,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
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
        ]
    }
}
impl From<[Pubkey; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]> for ClaimVestedTokenKeys {
    fn from(pubkeys: [Pubkey; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            beneficiary: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            vesting_record: pubkeys[3],
            base_vault: pubkeys[4],
            user_base_token: pubkeys[5],
            base_token_mint: pubkeys[6],
            base_token_program: pubkeys[7],
            system_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<ClaimVestedTokenAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimVestedTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.beneficiary.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.vesting_record.clone(),
            accounts.base_vault.clone(),
            accounts.user_base_token.clone(),
            accounts.base_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]>
for ClaimVestedTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            beneficiary: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            vesting_record: &arr[3],
            base_vault: &arr[4],
            user_base_token: &arr[5],
            base_token_mint: &arr[6],
            base_token_program: &arr[7],
            system_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const CLAIM_VESTED_TOKEN_IX_DISCM: [u8; 8usize] = [
    49, 33, 104, 30, 189, 157, 79, 35,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimVestedTokenIxData;
impl ClaimVestedTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_VESTED_TOKEN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_VESTED_TOKEN_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_vested_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimVestedTokenKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimVestedTokenIxData.try_to_vec()?,
    })
}
pub fn claim_vested_token_ix(
    keys: ClaimVestedTokenKeys,
) -> std::io::Result<Instruction> {
    claim_vested_token_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn claim_vested_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimVestedTokenAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimVestedTokenKeys = accounts.into();
    let ix = claim_vested_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_vested_token_invoke(
    accounts: ClaimVestedTokenAccounts<'_, '_>,
) -> ProgramResult {
    claim_vested_token_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn claim_vested_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimVestedTokenKeys = accounts.into();
    let ix = claim_vested_token_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_vested_token_invoke_signed(
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_vested_token_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_vested_token_verify_account_keys(
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    keys: ClaimVestedTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.beneficiary.key, keys.beneficiary),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.vesting_record.key, keys.vesting_record),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_vested_token_verify_writable_privileges<'me, 'info>(
    accounts: ClaimVestedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.beneficiary,
        accounts.pool_state,
        accounts.vesting_record,
        accounts.base_vault,
        accounts.user_base_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_vested_token_verify_signer_privileges<'me, 'info>(
    accounts: ClaimVestedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.beneficiary] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_vested_token_verify_account_privileges<'me, 'info>(
    accounts: ClaimVestedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_vested_token_verify_writable_privileges(accounts)?;
    claim_vested_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ClosePlatformAllowConfigAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_allow_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePlatformAllowConfigKeys {
    pub platform_admin: Pubkey,
    pub platform_config: Pubkey,
    pub global_config: Pubkey,
    pub platform_allow_config: Pubkey,
}
impl From<ClosePlatformAllowConfigAccounts<'_, '_>> for ClosePlatformAllowConfigKeys {
    fn from(accounts: ClosePlatformAllowConfigAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_config: *accounts.platform_config.key,
            global_config: *accounts.global_config.key,
            platform_allow_config: *accounts.platform_allow_config.key,
        }
    }
}
impl From<ClosePlatformAllowConfigKeys>
for [AccountMeta; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePlatformAllowConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_allow_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN]>
for ClosePlatformAllowConfigKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_config: pubkeys[1],
            global_config: pubkeys[2],
            platform_allow_config: pubkeys[3],
        }
    }
}
impl<'info> From<ClosePlatformAllowConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePlatformAllowConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_admin.clone(),
            accounts.platform_config.clone(),
            accounts.global_config.clone(),
            accounts.platform_allow_config.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN]>
for ClosePlatformAllowConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_config: &arr[1],
            global_config: &arr[2],
            platform_allow_config: &arr[3],
        }
    }
}
pub const CLOSE_PLATFORM_ALLOW_CONFIG_IX_DISCM: [u8; 8usize] = [
    82, 205, 36, 216, 16, 6, 240, 215,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePlatformAllowConfigIxData;
impl ClosePlatformAllowConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PLATFORM_ALLOW_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PLATFORM_ALLOW_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_platform_allow_config_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePlatformAllowConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePlatformAllowConfigIxData.try_to_vec()?,
    })
}
pub fn close_platform_allow_config_ix(
    keys: ClosePlatformAllowConfigKeys,
) -> std::io::Result<Instruction> {
    close_platform_allow_config_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn close_platform_allow_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePlatformAllowConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePlatformAllowConfigKeys = accounts.into();
    let ix = close_platform_allow_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_platform_allow_config_invoke(
    accounts: ClosePlatformAllowConfigAccounts<'_, '_>,
) -> ProgramResult {
    close_platform_allow_config_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn close_platform_allow_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePlatformAllowConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePlatformAllowConfigKeys = accounts.into();
    let ix = close_platform_allow_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_platform_allow_config_invoke_signed(
    accounts: ClosePlatformAllowConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_platform_allow_config_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_platform_allow_config_verify_account_keys(
    accounts: ClosePlatformAllowConfigAccounts<'_, '_>,
    keys: ClosePlatformAllowConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_allow_config.key, keys.platform_allow_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_platform_allow_config_verify_writable_privileges<'me, 'info>(
    accounts: ClosePlatformAllowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_admin, accounts.platform_allow_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_platform_allow_config_verify_signer_privileges<'me, 'info>(
    accounts: ClosePlatformAllowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_platform_allow_config_verify_account_privileges<'me, 'info>(
    accounts: ClosePlatformAllowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_platform_allow_config_verify_writable_privileges(accounts)?;
    close_platform_allow_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ClosePlatformCurveRuleAccounts<'me, 'info> {
    pub curve_rule_authority: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_curve_rule: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePlatformCurveRuleKeys {
    pub curve_rule_authority: Pubkey,
    pub platform_config: Pubkey,
    pub global_config: Pubkey,
    pub platform_curve_rule: Pubkey,
}
impl From<ClosePlatformCurveRuleAccounts<'_, '_>> for ClosePlatformCurveRuleKeys {
    fn from(accounts: ClosePlatformCurveRuleAccounts) -> Self {
        Self {
            curve_rule_authority: *accounts.curve_rule_authority.key,
            platform_config: *accounts.platform_config.key,
            global_config: *accounts.global_config.key,
            platform_curve_rule: *accounts.platform_curve_rule.key,
        }
    }
}
impl From<ClosePlatformCurveRuleKeys>
for [AccountMeta; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePlatformCurveRuleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curve_rule_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_curve_rule,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for ClosePlatformCurveRuleKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curve_rule_authority: pubkeys[0],
            platform_config: pubkeys[1],
            global_config: pubkeys[2],
            platform_curve_rule: pubkeys[3],
        }
    }
}
impl<'info> From<ClosePlatformCurveRuleAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePlatformCurveRuleAccounts<'_, 'info>) -> Self {
        [
            accounts.curve_rule_authority.clone(),
            accounts.platform_config.clone(),
            accounts.global_config.clone(),
            accounts.platform_curve_rule.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for ClosePlatformCurveRuleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curve_rule_authority: &arr[0],
            platform_config: &arr[1],
            global_config: &arr[2],
            platform_curve_rule: &arr[3],
        }
    }
}
pub const CLOSE_PLATFORM_CURVE_RULE_IX_DISCM: [u8; 8usize] = [
    194, 114, 82, 45, 136, 88, 169, 159,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePlatformCurveRuleIxData;
impl ClosePlatformCurveRuleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PLATFORM_CURVE_RULE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PLATFORM_CURVE_RULE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_platform_curve_rule_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePlatformCurveRuleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePlatformCurveRuleIxData.try_to_vec()?,
    })
}
pub fn close_platform_curve_rule_ix(
    keys: ClosePlatformCurveRuleKeys,
) -> std::io::Result<Instruction> {
    close_platform_curve_rule_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn close_platform_curve_rule_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePlatformCurveRuleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePlatformCurveRuleKeys = accounts.into();
    let ix = close_platform_curve_rule_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_platform_curve_rule_invoke(
    accounts: ClosePlatformCurveRuleAccounts<'_, '_>,
) -> ProgramResult {
    close_platform_curve_rule_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn close_platform_curve_rule_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePlatformCurveRuleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePlatformCurveRuleKeys = accounts.into();
    let ix = close_platform_curve_rule_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_platform_curve_rule_invoke_signed(
    accounts: ClosePlatformCurveRuleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_platform_curve_rule_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn close_platform_curve_rule_verify_account_keys(
    accounts: ClosePlatformCurveRuleAccounts<'_, '_>,
    keys: ClosePlatformCurveRuleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curve_rule_authority.key, keys.curve_rule_authority),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_curve_rule.key, keys.platform_curve_rule),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_platform_curve_rule_verify_writable_privileges<'me, 'info>(
    accounts: ClosePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curve_rule_authority,
        accounts.platform_curve_rule,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_platform_curve_rule_verify_signer_privileges<'me, 'info>(
    accounts: ClosePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curve_rule_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_platform_curve_rule_verify_account_privileges<'me, 'info>(
    accounts: ClosePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_platform_curve_rule_verify_writable_privileges(accounts)?;
    close_platform_curve_rule_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CollectExcessLamportsAccounts<'me, 'info> {
    pub collect_lamports_wallet: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectExcessLamportsKeys {
    pub collect_lamports_wallet: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
}
impl From<CollectExcessLamportsAccounts<'_, '_>> for CollectExcessLamportsKeys {
    fn from(accounts: CollectExcessLamportsAccounts) -> Self {
        Self {
            collect_lamports_wallet: *accounts.collect_lamports_wallet.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
        }
    }
}
impl From<CollectExcessLamportsKeys>
for [AccountMeta; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectExcessLamportsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.collect_lamports_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN]>
for CollectExcessLamportsKeys {
    fn from(pubkeys: [Pubkey; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collect_lamports_wallet: pubkeys[0],
            authority: pubkeys[1],
            token_program: pubkeys[2],
            token_program_2022: pubkeys[3],
        }
    }
}
impl<'info> From<CollectExcessLamportsAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectExcessLamportsAccounts<'_, 'info>) -> Self {
        [
            accounts.collect_lamports_wallet.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN]>
for CollectExcessLamportsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            collect_lamports_wallet: &arr[0],
            authority: &arr[1],
            token_program: &arr[2],
            token_program_2022: &arr[3],
        }
    }
}
pub const COLLECT_EXCESS_LAMPORTS_IX_DISCM: [u8; 8usize] = [
    28, 189, 19, 40, 164, 176, 118, 17,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectExcessLamportsIxData;
impl CollectExcessLamportsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_EXCESS_LAMPORTS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_EXCESS_LAMPORTS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_excess_lamports_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectExcessLamportsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_EXCESS_LAMPORTS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectExcessLamportsIxData.try_to_vec()?,
    })
}
pub fn collect_excess_lamports_ix(
    keys: CollectExcessLamportsKeys,
) -> std::io::Result<Instruction> {
    collect_excess_lamports_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn collect_excess_lamports_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectExcessLamportsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectExcessLamportsKeys = accounts.into();
    let ix = collect_excess_lamports_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_excess_lamports_invoke(
    accounts: CollectExcessLamportsAccounts<'_, '_>,
) -> ProgramResult {
    collect_excess_lamports_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn collect_excess_lamports_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectExcessLamportsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectExcessLamportsKeys = accounts.into();
    let ix = collect_excess_lamports_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_excess_lamports_invoke_signed(
    accounts: CollectExcessLamportsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_excess_lamports_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_excess_lamports_verify_account_keys(
    accounts: CollectExcessLamportsAccounts<'_, '_>,
    keys: CollectExcessLamportsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.collect_lamports_wallet.key, keys.collect_lamports_wallet),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_excess_lamports_verify_writable_privileges<'me, 'info>(
    accounts: CollectExcessLamportsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.collect_lamports_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_excess_lamports_verify_signer_privileges<'me, 'info>(
    accounts: CollectExcessLamportsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.collect_lamports_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_excess_lamports_verify_account_privileges<'me, 'info>(
    accounts: CollectExcessLamportsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_excess_lamports_verify_writable_privileges(accounts)?;
    collect_excess_lamports_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeeKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectFeeAccounts<'_, '_>> for CollectFeeKeys {
    fn from(accounts: CollectFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectFeeKeys> for [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
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
impl From<[Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]> for CollectFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            global_config: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            recipient_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.recipient_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]>
for CollectFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            global_config: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            recipient_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const COLLECT_FEE_IX_DISCM: [u8; 8usize] = [60, 173, 247, 103, 4, 93, 130, 48];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeeIxData;
impl CollectFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectFeeIxData.try_to_vec()?,
    })
}
pub fn collect_fee_ix(keys: CollectFeeKeys) -> std::io::Result<Instruction> {
    collect_fee_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn collect_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fee_invoke(accounts: CollectFeeAccounts<'_, '_>) -> ProgramResult {
    collect_fee_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn collect_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fee_invoke_signed(
    accounts: CollectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fee_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_fee_verify_account_keys(
    accounts: CollectFeeAccounts<'_, '_>,
    keys: CollectFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.quote_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fee_verify_writable_privileges(accounts)?;
    collect_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectMigrateFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectMigrateFeeKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<CollectMigrateFeeAccounts<'_, '_>> for CollectMigrateFeeKeys {
    fn from(accounts: CollectMigrateFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CollectMigrateFeeKeys> for [AccountMeta; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectMigrateFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
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
impl From<[Pubkey; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]> for CollectMigrateFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            global_config: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            recipient_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectMigrateFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectMigrateFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.recipient_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]>
for CollectMigrateFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            global_config: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            recipient_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const COLLECT_MIGRATE_FEE_IX_DISCM: [u8; 8usize] = [
    255, 186, 150, 223, 235, 118, 201, 186,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectMigrateFeeIxData;
impl CollectMigrateFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_MIGRATE_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_MIGRATE_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_migrate_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectMigrateFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectMigrateFeeIxData.try_to_vec()?,
    })
}
pub fn collect_migrate_fee_ix(
    keys: CollectMigrateFeeKeys,
) -> std::io::Result<Instruction> {
    collect_migrate_fee_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn collect_migrate_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectMigrateFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectMigrateFeeKeys = accounts.into();
    let ix = collect_migrate_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_migrate_fee_invoke(
    accounts: CollectMigrateFeeAccounts<'_, '_>,
) -> ProgramResult {
    collect_migrate_fee_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn collect_migrate_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectMigrateFeeKeys = accounts.into();
    let ix = collect_migrate_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_migrate_fee_invoke_signed(
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_migrate_fee_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn collect_migrate_fee_verify_account_keys(
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    keys: CollectMigrateFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_migrate_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectMigrateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.quote_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_migrate_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectMigrateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_migrate_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectMigrateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_migrate_fee_verify_writable_privileges(accounts)?;
    collect_migrate_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub protocol_fee_owner: &'me AccountInfo<'info>,
    pub migrate_fee_owner: &'me AccountInfo<'info>,
    pub migrate_to_amm_wallet: &'me AccountInfo<'info>,
    pub migrate_to_cpswap_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub owner: Pubkey,
    pub global_config: Pubkey,
    pub quote_token_mint: Pubkey,
    pub protocol_fee_owner: Pubkey,
    pub migrate_fee_owner: Pubkey,
    pub migrate_to_amm_wallet: Pubkey,
    pub migrate_to_cpswap_wallet: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            global_config: *accounts.global_config.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            protocol_fee_owner: *accounts.protocol_fee_owner.key,
            migrate_fee_owner: *accounts.migrate_fee_owner.key,
            migrate_to_amm_wallet: *accounts.migrate_to_amm_wallet.key,
            migrate_to_cpswap_wallet: *accounts.migrate_to_cpswap_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migrate_fee_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migrate_to_amm_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migrate_to_cpswap_wallet,
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
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            global_config: pubkeys[1],
            quote_token_mint: pubkeys[2],
            protocol_fee_owner: pubkeys[3],
            migrate_fee_owner: pubkeys[4],
            migrate_to_amm_wallet: pubkeys[5],
            migrate_to_cpswap_wallet: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.global_config.clone(),
            accounts.quote_token_mint.clone(),
            accounts.protocol_fee_owner.clone(),
            accounts.migrate_fee_owner.clone(),
            accounts.migrate_to_amm_wallet.clone(),
            accounts.migrate_to_cpswap_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            global_config: &arr[1],
            quote_token_mint: &arr[2],
            protocol_fee_owner: &arr[3],
            migrate_fee_owner: &arr[4],
            migrate_to_amm_wallet: &arr[5],
            migrate_to_cpswap_wallet: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const CREATE_CONFIG_IX_DISCM: [u8; 8usize] = [201, 207, 243, 114, 75, 111, 47, 189];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigIxArgs {
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigIxData(pub CreateConfigIxArgs);
impl From<CreateConfigIxArgs> for CreateConfigIxData {
    fn from(args: CreateConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let curve_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateConfigIxArgs {
                curve_type,
                index,
                migrate_fee,
                trade_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.curve_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.migrate_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.trade_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_config_ix(
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_config_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn create_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_config_invoke(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    create_config_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn create_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_config_invoke_signed(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_config_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_config_verify_account_keys(
    accounts: CreateConfigAccounts<'_, '_>,
    keys: CreateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.protocol_fee_owner.key, keys.protocol_fee_owner),
        (*accounts.migrate_fee_owner.key, keys.migrate_fee_owner),
        (*accounts.migrate_to_amm_wallet.key, keys.migrate_to_amm_wallet),
        (*accounts.migrate_to_cpswap_wallet.key, keys.migrate_to_cpswap_wallet),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_config_verify_account_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_config_verify_writable_privileges(accounts)?;
    create_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreatePlatformAllowConfigAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_allow_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePlatformAllowConfigKeys {
    pub platform_admin: Pubkey,
    pub platform_config: Pubkey,
    pub global_config: Pubkey,
    pub platform_allow_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePlatformAllowConfigAccounts<'_, '_>> for CreatePlatformAllowConfigKeys {
    fn from(accounts: CreatePlatformAllowConfigAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_config: *accounts.platform_config.key,
            global_config: *accounts.global_config.key,
            platform_allow_config: *accounts.platform_allow_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePlatformAllowConfigKeys>
for [AccountMeta; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePlatformAllowConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_allow_config,
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
impl From<[Pubkey; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN]>
for CreatePlatformAllowConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_config: pubkeys[1],
            global_config: pubkeys[2],
            platform_allow_config: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreatePlatformAllowConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePlatformAllowConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_admin.clone(),
            accounts.platform_config.clone(),
            accounts.global_config.clone(),
            accounts.platform_allow_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN]>
for CreatePlatformAllowConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_config: &arr[1],
            global_config: &arr[2],
            platform_allow_config: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_PLATFORM_ALLOW_CONFIG_IX_DISCM: [u8; 8usize] = [
    69, 71, 168, 7, 214, 250, 107, 102,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePlatformAllowConfigIxData;
impl CreatePlatformAllowConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PLATFORM_ALLOW_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PLATFORM_ALLOW_CONFIG_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_platform_allow_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePlatformAllowConfigKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PLATFORM_ALLOW_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePlatformAllowConfigIxData.try_to_vec()?,
    })
}
pub fn create_platform_allow_config_ix(
    keys: CreatePlatformAllowConfigKeys,
) -> std::io::Result<Instruction> {
    create_platform_allow_config_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn create_platform_allow_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformAllowConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePlatformAllowConfigKeys = accounts.into();
    let ix = create_platform_allow_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_platform_allow_config_invoke(
    accounts: CreatePlatformAllowConfigAccounts<'_, '_>,
) -> ProgramResult {
    create_platform_allow_config_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn create_platform_allow_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformAllowConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePlatformAllowConfigKeys = accounts.into();
    let ix = create_platform_allow_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_platform_allow_config_invoke_signed(
    accounts: CreatePlatformAllowConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_platform_allow_config_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_platform_allow_config_verify_account_keys(
    accounts: CreatePlatformAllowConfigAccounts<'_, '_>,
    keys: CreatePlatformAllowConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_allow_config.key, keys.platform_allow_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_platform_allow_config_verify_writable_privileges<'me, 'info>(
    accounts: CreatePlatformAllowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_admin, accounts.platform_allow_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_platform_allow_config_verify_signer_privileges<'me, 'info>(
    accounts: CreatePlatformAllowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_platform_allow_config_verify_account_privileges<'me, 'info>(
    accounts: CreatePlatformAllowConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_platform_allow_config_verify_writable_privileges(accounts)?;
    create_platform_allow_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreatePlatformConfigAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub platform_nft_wallet: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub cpswap_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub transfer_fee_extension_authority: &'me AccountInfo<'info>,
    pub platform_vesting_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePlatformConfigKeys {
    pub platform_admin: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub platform_nft_wallet: Pubkey,
    pub platform_config: Pubkey,
    pub cpswap_config: Pubkey,
    pub system_program: Pubkey,
    pub transfer_fee_extension_authority: Pubkey,
    pub platform_vesting_wallet: Pubkey,
}
impl From<CreatePlatformConfigAccounts<'_, '_>> for CreatePlatformConfigKeys {
    fn from(accounts: CreatePlatformConfigAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            platform_nft_wallet: *accounts.platform_nft_wallet.key,
            platform_config: *accounts.platform_config.key,
            cpswap_config: *accounts.cpswap_config.key,
            system_program: *accounts.system_program.key,
            transfer_fee_extension_authority: *accounts
                .transfer_fee_extension_authority
                .key,
            platform_vesting_wallet: *accounts.platform_vesting_wallet.key,
        }
    }
}
impl From<CreatePlatformConfigKeys>
for [AccountMeta; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePlatformConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_nft_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_fee_extension_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_vesting_wallet,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]>
for CreatePlatformConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_fee_wallet: pubkeys[1],
            platform_nft_wallet: pubkeys[2],
            platform_config: pubkeys[3],
            cpswap_config: pubkeys[4],
            system_program: pubkeys[5],
            transfer_fee_extension_authority: pubkeys[6],
            platform_vesting_wallet: pubkeys[7],
        }
    }
}
impl<'info> From<CreatePlatformConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePlatformConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_admin.clone(),
            accounts.platform_fee_wallet.clone(),
            accounts.platform_nft_wallet.clone(),
            accounts.platform_config.clone(),
            accounts.cpswap_config.clone(),
            accounts.system_program.clone(),
            accounts.transfer_fee_extension_authority.clone(),
            accounts.platform_vesting_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]>
for CreatePlatformConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_fee_wallet: &arr[1],
            platform_nft_wallet: &arr[2],
            platform_config: &arr[3],
            cpswap_config: &arr[4],
            system_program: &arr[5],
            transfer_fee_extension_authority: &arr[6],
            platform_vesting_wallet: &arr[7],
        }
    }
}
pub const CREATE_PLATFORM_CONFIG_IX_DISCM: [u8; 8usize] = [
    176, 90, 196, 175, 253, 113, 220, 20,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePlatformConfigIxArgs {
    pub platform_params: PlatformParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePlatformConfigIxData(pub CreatePlatformConfigIxArgs);
impl From<CreatePlatformConfigIxArgs> for CreatePlatformConfigIxData {
    fn from(args: CreatePlatformConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePlatformConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PLATFORM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let platform_params = if reader.is_empty() {
            Default::default()
        } else {
            <PlatformParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(CreatePlatformConfigIxArgs {
                platform_params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PLATFORM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.platform_params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_platform_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePlatformConfigKeys,
    args: CreatePlatformConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePlatformConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_platform_config_ix(
    keys: CreatePlatformConfigKeys,
    args: CreatePlatformConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_platform_config_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn create_platform_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
) -> ProgramResult {
    let keys: CreatePlatformConfigKeys = accounts.into();
    let ix = create_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_platform_config_invoke(
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
) -> ProgramResult {
    create_platform_config_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_platform_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePlatformConfigKeys = accounts.into();
    let ix = create_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_platform_config_invoke_signed(
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_platform_config_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_platform_config_verify_account_keys(
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    keys: CreatePlatformConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.platform_nft_wallet.key, keys.platform_nft_wallet),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.cpswap_config.key, keys.cpswap_config),
        (*accounts.system_program.key, keys.system_program),
        (
            *accounts.transfer_fee_extension_authority.key,
            keys.transfer_fee_extension_authority,
        ),
        (*accounts.platform_vesting_wallet.key, keys.platform_vesting_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_platform_config_verify_writable_privileges<'me, 'info>(
    accounts: CreatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_admin, accounts.platform_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_platform_config_verify_signer_privileges<'me, 'info>(
    accounts: CreatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_platform_config_verify_account_privileges<'me, 'info>(
    accounts: CreatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_platform_config_verify_writable_privileges(accounts)?;
    create_platform_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreatePlatformCurveRuleAccounts<'me, 'info> {
    pub curve_rule_authority: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_curve_rule: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePlatformCurveRuleKeys {
    pub curve_rule_authority: Pubkey,
    pub platform_config: Pubkey,
    pub global_config: Pubkey,
    pub platform_curve_rule: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePlatformCurveRuleAccounts<'_, '_>> for CreatePlatformCurveRuleKeys {
    fn from(accounts: CreatePlatformCurveRuleAccounts) -> Self {
        Self {
            curve_rule_authority: *accounts.curve_rule_authority.key,
            platform_config: *accounts.platform_config.key,
            global_config: *accounts.global_config.key,
            platform_curve_rule: *accounts.platform_curve_rule.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePlatformCurveRuleKeys>
for [AccountMeta; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePlatformCurveRuleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curve_rule_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_curve_rule,
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
impl From<[Pubkey; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for CreatePlatformCurveRuleKeys {
    fn from(pubkeys: [Pubkey; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curve_rule_authority: pubkeys[0],
            platform_config: pubkeys[1],
            global_config: pubkeys[2],
            platform_curve_rule: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreatePlatformCurveRuleAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePlatformCurveRuleAccounts<'_, 'info>) -> Self {
        [
            accounts.curve_rule_authority.clone(),
            accounts.platform_config.clone(),
            accounts.global_config.clone(),
            accounts.platform_curve_rule.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for CreatePlatformCurveRuleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curve_rule_authority: &arr[0],
            platform_config: &arr[1],
            global_config: &arr[2],
            platform_curve_rule: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_PLATFORM_CURVE_RULE_IX_DISCM: [u8; 8usize] = [
    148, 71, 180, 136, 122, 144, 59, 21,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePlatformCurveRuleIxData;
impl CreatePlatformCurveRuleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PLATFORM_CURVE_RULE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PLATFORM_CURVE_RULE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_platform_curve_rule_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePlatformCurveRuleKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePlatformCurveRuleIxData.try_to_vec()?,
    })
}
pub fn create_platform_curve_rule_ix(
    keys: CreatePlatformCurveRuleKeys,
) -> std::io::Result<Instruction> {
    create_platform_curve_rule_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn create_platform_curve_rule_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformCurveRuleAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePlatformCurveRuleKeys = accounts.into();
    let ix = create_platform_curve_rule_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_platform_curve_rule_invoke(
    accounts: CreatePlatformCurveRuleAccounts<'_, '_>,
) -> ProgramResult {
    create_platform_curve_rule_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn create_platform_curve_rule_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformCurveRuleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePlatformCurveRuleKeys = accounts.into();
    let ix = create_platform_curve_rule_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_platform_curve_rule_invoke_signed(
    accounts: CreatePlatformCurveRuleAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_platform_curve_rule_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_platform_curve_rule_verify_account_keys(
    accounts: CreatePlatformCurveRuleAccounts<'_, '_>,
    keys: CreatePlatformCurveRuleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curve_rule_authority.key, keys.curve_rule_authority),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_curve_rule.key, keys.platform_curve_rule),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_platform_curve_rule_verify_writable_privileges<'me, 'info>(
    accounts: CreatePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curve_rule_authority,
        accounts.platform_curve_rule,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_platform_curve_rule_verify_signer_privileges<'me, 'info>(
    accounts: CreatePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curve_rule_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_platform_curve_rule_verify_account_privileges<'me, 'info>(
    accounts: CreatePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_platform_curve_rule_verify_writable_privileges(accounts)?;
    create_platform_curve_rule_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreatePlatformVestingAccountAccounts<'me, 'info> {
    pub platform_vesting_wallet: &'me AccountInfo<'info>,
    pub beneficiary: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub platform_vesting_record: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePlatformVestingAccountKeys {
    pub platform_vesting_wallet: Pubkey,
    pub beneficiary: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub platform_vesting_record: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreatePlatformVestingAccountAccounts<'_, '_>>
for CreatePlatformVestingAccountKeys {
    fn from(accounts: CreatePlatformVestingAccountAccounts) -> Self {
        Self {
            platform_vesting_wallet: *accounts.platform_vesting_wallet.key,
            beneficiary: *accounts.beneficiary.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            platform_vesting_record: *accounts.platform_vesting_record.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreatePlatformVestingAccountKeys>
for [AccountMeta; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePlatformVestingAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_vesting_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_vesting_record,
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
impl From<[Pubkey; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreatePlatformVestingAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_vesting_wallet: pubkeys[0],
            beneficiary: pubkeys[1],
            platform_config: pubkeys[2],
            pool_state: pubkeys[3],
            platform_vesting_record: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreatePlatformVestingAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePlatformVestingAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_vesting_wallet.clone(),
            accounts.beneficiary.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.platform_vesting_record.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreatePlatformVestingAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            platform_vesting_wallet: &arr[0],
            beneficiary: &arr[1],
            platform_config: &arr[2],
            pool_state: &arr[3],
            platform_vesting_record: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const CREATE_PLATFORM_VESTING_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    146, 71, 173, 69, 98, 19, 15, 106,
];
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePlatformVestingAccountIxData;
impl CreatePlatformVestingAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PLATFORM_VESTING_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PLATFORM_VESTING_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_platform_vesting_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePlatformVestingAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PLATFORM_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreatePlatformVestingAccountIxData.try_to_vec()?,
    })
}
pub fn create_platform_vesting_account_ix(
    keys: CreatePlatformVestingAccountKeys,
) -> std::io::Result<Instruction> {
    create_platform_vesting_account_ix_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        keys,
    )
}
pub fn create_platform_vesting_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformVestingAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreatePlatformVestingAccountKeys = accounts.into();
    let ix = create_platform_vesting_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_platform_vesting_account_invoke(
    accounts: CreatePlatformVestingAccountAccounts<'_, '_>,
) -> ProgramResult {
    create_platform_vesting_account_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
    )
}
pub fn create_platform_vesting_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformVestingAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePlatformVestingAccountKeys = accounts.into();
    let ix = create_platform_vesting_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_platform_vesting_account_invoke_signed(
    accounts: CreatePlatformVestingAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_platform_vesting_account_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn create_platform_vesting_account_verify_account_keys(
    accounts: CreatePlatformVestingAccountAccounts<'_, '_>,
    keys: CreatePlatformVestingAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_vesting_wallet.key, keys.platform_vesting_wallet),
        (*accounts.beneficiary.key, keys.beneficiary),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.platform_vesting_record.key, keys.platform_vesting_record),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_platform_vesting_account_verify_writable_privileges<'me, 'info>(
    accounts: CreatePlatformVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.platform_vesting_wallet,
        accounts.beneficiary,
        accounts.pool_state,
        accounts.platform_vesting_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_platform_vesting_account_verify_signer_privileges<'me, 'info>(
    accounts: CreatePlatformVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_vesting_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_platform_vesting_account_verify_account_privileges<'me, 'info>(
    accounts: CreatePlatformVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_platform_vesting_account_verify_writable_privileges(accounts)?;
    create_platform_vesting_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateVestingAccountAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub beneficiary: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub vesting_record: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateVestingAccountKeys {
    pub creator: Pubkey,
    pub beneficiary: Pubkey,
    pub pool_state: Pubkey,
    pub vesting_record: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateVestingAccountAccounts<'_, '_>> for CreateVestingAccountKeys {
    fn from(accounts: CreateVestingAccountAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            beneficiary: *accounts.beneficiary.key,
            pool_state: *accounts.pool_state.key,
            vesting_record: *accounts.vesting_record.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateVestingAccountKeys>
for [AccountMeta; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateVestingAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting_record,
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
impl From<[Pubkey; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateVestingAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            beneficiary: pubkeys[1],
            pool_state: pubkeys[2],
            vesting_record: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateVestingAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateVestingAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.beneficiary.clone(),
            accounts.pool_state.clone(),
            accounts.vesting_record.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateVestingAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            beneficiary: &arr[1],
            pool_state: &arr[2],
            vesting_record: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_VESTING_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    129, 178, 2, 13, 217, 172, 230, 218,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateVestingAccountIxArgs {
    pub share_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateVestingAccountIxData(pub CreateVestingAccountIxArgs);
impl From<CreateVestingAccountIxArgs> for CreateVestingAccountIxData {
    fn from(args: CreateVestingAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateVestingAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_VESTING_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateVestingAccountIxArgs {
                share_amount,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_VESTING_ACCOUNT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.share_amount, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_vesting_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateVestingAccountKeys,
    args: CreateVestingAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateVestingAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_vesting_account_ix(
    keys: CreateVestingAccountKeys,
    args: CreateVestingAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_vesting_account_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn create_vesting_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
) -> ProgramResult {
    let keys: CreateVestingAccountKeys = accounts.into();
    let ix = create_vesting_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_vesting_account_invoke(
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
) -> ProgramResult {
    create_vesting_account_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn create_vesting_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateVestingAccountKeys = accounts.into();
    let ix = create_vesting_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_vesting_account_invoke_signed(
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_vesting_account_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_vesting_account_verify_account_keys(
    accounts: CreateVestingAccountAccounts<'_, '_>,
    keys: CreateVestingAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.beneficiary.key, keys.beneficiary),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.vesting_record.key, keys.vesting_record),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_vesting_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.beneficiary,
        accounts.pool_state,
        accounts.vesting_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_vesting_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_vesting_account_verify_account_privileges<'me, 'info>(
    accounts: CreateVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_vesting_account_verify_writable_privileges(accounts)?;
    create_vesting_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub metadata_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            metadata_account: *accounts.metadata_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
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
                pubkey: keys.rent_program,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            authority: pubkeys[4],
            pool_state: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            metadata_account: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            metadata_program: pubkeys[13],
            system_program: pubkeys[14],
            rent_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.metadata_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            authority: &arr[4],
            pool_state: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            metadata_account: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            metadata_program: &arr[13],
            system_program: &arr[14],
            rent_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8usize] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeIxArgs {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_mint_param = if reader.is_empty() {
            Default::default()
        } else {
            <MintParams>::deserialize(&mut reader)?
        };
        let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let vesting_param = if reader.is_empty() {
            Default::default()
        } else {
            <VestingParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeIxArgs {
                base_mint_param,
                curve_param,
                vesting_param,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_mint_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.curve_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.vesting_param, &mut writer)?;
        Ok(())
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
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base_mint] {
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
pub const INITIALIZE_V2_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct InitializeV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeV2Keys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub metadata_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeV2Accounts<'_, '_>> for InitializeV2Keys {
    fn from(accounts: InitializeV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            metadata_account: *accounts.metadata_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeV2Keys> for [AccountMeta; INITIALIZE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
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
                pubkey: keys.rent_program,
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
impl From<[Pubkey; INITIALIZE_V2_IX_ACCOUNTS_LEN]> for InitializeV2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            authority: pubkeys[4],
            pool_state: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            metadata_account: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            metadata_program: pubkeys[13],
            system_program: pubkeys[14],
            rent_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<InitializeV2Accounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.metadata_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_V2_IX_ACCOUNTS_LEN]>
for InitializeV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            authority: &arr[4],
            pool_state: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            metadata_account: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            metadata_program: &arr[13],
            system_program: &arr[14],
            rent_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const INITIALIZE_V2_IX_DISCM: [u8; 8usize] = [67, 153, 175, 39, 218, 16, 38, 32];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeV2IxArgs {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeV2IxData(pub InitializeV2IxArgs);
impl From<InitializeV2IxArgs> for InitializeV2IxData {
    fn from(args: InitializeV2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_V2_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_mint_param = if reader.is_empty() {
            Default::default()
        } else {
            <MintParams>::deserialize(&mut reader)?
        };
        let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let vesting_param = if reader.is_empty() {
            Default::default()
        } else {
            <VestingParams>::deserialize(&mut reader)?
        };
        let amm_fee_on: AmmCreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(InitializeV2IxArgs {
                base_mint_param,
                curve_param,
                vesting_param,
                amm_fee_on,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_V2_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_mint_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.curve_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.vesting_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amm_fee_on, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeV2Keys,
    args: InitializeV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_v2_ix(
    keys: InitializeV2Keys,
    args: InitializeV2IxArgs,
) -> std::io::Result<Instruction> {
    initialize_v2_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn initialize_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
) -> ProgramResult {
    let keys: InitializeV2Keys = accounts.into();
    let ix = initialize_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_v2_invoke(
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
) -> ProgramResult {
    initialize_v2_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn initialize_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeV2Keys = accounts.into();
    let ix = initialize_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_v2_invoke_signed(
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_v2_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_v2_verify_account_keys(
    accounts: InitializeV2Accounts<'_, '_>,
    keys: InitializeV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_v2_verify_writable_privileges<'me, 'info>(
    accounts: InitializeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_v2_verify_signer_privileges<'me, 'info>(
    accounts: InitializeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_v2_verify_account_privileges<'me, 'info>(
    accounts: InitializeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_v2_verify_writable_privileges(accounts)?;
    initialize_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct InitializeWithToken2022Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeWithToken2022Keys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeWithToken2022Accounts<'_, '_>> for InitializeWithToken2022Keys {
    fn from(accounts: InitializeWithToken2022Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeWithToken2022Keys>
for [AccountMeta; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeWithToken2022Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
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
impl From<[Pubkey; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]>
for InitializeWithToken2022Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            authority: pubkeys[4],
            pool_state: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            base_token_program: pubkeys[10],
            quote_token_program: pubkeys[11],
            system_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<InitializeWithToken2022Accounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeWithToken2022Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]>
for InitializeWithToken2022Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            authority: &arr[4],
            pool_state: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            base_token_program: &arr[10],
            quote_token_program: &arr[11],
            system_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const INITIALIZE_WITH_TOKEN_2022_IX_DISCM: [u8; 8usize] = [
    37, 190, 126, 222, 44, 154, 171, 17,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeWithToken2022IxArgs {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
    pub transfer_fee_extension_param: Option<TransferFeeExtensionParams>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeWithToken2022IxData(pub InitializeWithToken2022IxArgs);
impl From<InitializeWithToken2022IxArgs> for InitializeWithToken2022IxData {
    fn from(args: InitializeWithToken2022IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeWithToken2022IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_WITH_TOKEN_2022_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let base_mint_param = if reader.is_empty() {
            Default::default()
        } else {
            <MintParams>::deserialize(&mut reader)?
        };
        let curve_param = <CurveParams as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let vesting_param = if reader.is_empty() {
            Default::default()
        } else {
            <VestingParams>::deserialize(&mut reader)?
        };
        let amm_fee_on: AmmCreatorFeeOn = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_extension_param: Option<TransferFeeExtensionParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        Ok(
            Self(InitializeWithToken2022IxArgs {
                base_mint_param,
                curve_param,
                vesting_param,
                amm_fee_on,
                transfer_fee_extension_param,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_WITH_TOKEN_2022_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.base_mint_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.curve_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.vesting_param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.amm_fee_on, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.transfer_fee_extension_param,
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
pub fn initialize_with_token_2022_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeWithToken2022Keys,
    args: InitializeWithToken2022IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeWithToken2022IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_with_token_2022_ix(
    keys: InitializeWithToken2022Keys,
    args: InitializeWithToken2022IxArgs,
) -> std::io::Result<Instruction> {
    initialize_with_token_2022_ix_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn initialize_with_token_2022_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
) -> ProgramResult {
    let keys: InitializeWithToken2022Keys = accounts.into();
    let ix = initialize_with_token_2022_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_with_token_2022_invoke(
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
) -> ProgramResult {
    initialize_with_token_2022_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn initialize_with_token_2022_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeWithToken2022Keys = accounts.into();
    let ix = initialize_with_token_2022_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_with_token_2022_invoke_signed(
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_with_token_2022_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_with_token_2022_verify_account_keys(
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    keys: InitializeWithToken2022Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
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
pub fn initialize_with_token_2022_verify_writable_privileges<'me, 'info>(
    accounts: InitializeWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_with_token_2022_verify_signer_privileges<'me, 'info>(
    accounts: InitializeWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_with_token_2022_verify_account_privileges<'me, 'info>(
    accounts: InitializeWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_with_token_2022_verify_writable_privileges(accounts)?;
    initialize_with_token_2022_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_TO_AMM_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct MigrateToAmmAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub amm_pool: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_lp_mint: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub amm_create_fee_destination: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub pool_lp_token: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateToAmmKeys {
    pub payer: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub market: Pubkey,
    pub amm_program: Pubkey,
    pub amm_pool: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_lp_mint: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub amm_target_orders: Pubkey,
    pub amm_config: Pubkey,
    pub amm_create_fee_destination: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub pool_lp_token: Pubkey,
    pub spl_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
}
impl From<MigrateToAmmAccounts<'_, '_>> for MigrateToAmmKeys {
    fn from(accounts: MigrateToAmmAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            market: *accounts.market.key,
            amm_program: *accounts.amm_program.key,
            amm_pool: *accounts.amm_pool.key,
            amm_authority: *accounts.amm_authority.key,
            amm_lp_mint: *accounts.amm_lp_mint.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            amm_config: *accounts.amm_config.key,
            amm_create_fee_destination: *accounts.amm_create_fee_destination.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            pool_lp_token: *accounts.pool_lp_token.key,
            spl_token_program: *accounts.spl_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
        }
    }
}
impl From<MigrateToAmmKeys> for [AccountMeta; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateToAmmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_create_fee_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
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
                pubkey: keys.rent_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]> for MigrateToAmmKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            market: pubkeys[3],
            amm_program: pubkeys[4],
            amm_pool: pubkeys[5],
            amm_authority: pubkeys[6],
            amm_lp_mint: pubkeys[7],
            amm_base_vault: pubkeys[8],
            amm_quote_vault: pubkeys[9],
            amm_target_orders: pubkeys[10],
            amm_config: pubkeys[11],
            amm_create_fee_destination: pubkeys[12],
            authority: pubkeys[13],
            pool_state: pubkeys[14],
            global_config: pubkeys[15],
            base_vault: pubkeys[16],
            quote_vault: pubkeys[17],
            pool_lp_token: pubkeys[18],
            spl_token_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            system_program: pubkeys[21],
            rent_program: pubkeys[22],
        }
    }
}
impl<'info> From<MigrateToAmmAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateToAmmAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.market.clone(),
            accounts.amm_program.clone(),
            accounts.amm_pool.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_lp_mint.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.amm_target_orders.clone(),
            accounts.amm_config.clone(),
            accounts.amm_create_fee_destination.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.pool_lp_token.clone(),
            accounts.spl_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]>
for MigrateToAmmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            market: &arr[3],
            amm_program: &arr[4],
            amm_pool: &arr[5],
            amm_authority: &arr[6],
            amm_lp_mint: &arr[7],
            amm_base_vault: &arr[8],
            amm_quote_vault: &arr[9],
            amm_target_orders: &arr[10],
            amm_config: &arr[11],
            amm_create_fee_destination: &arr[12],
            authority: &arr[13],
            pool_state: &arr[14],
            global_config: &arr[15],
            base_vault: &arr[16],
            quote_vault: &arr[17],
            pool_lp_token: &arr[18],
            spl_token_program: &arr[19],
            associated_token_program: &arr[20],
            system_program: &arr[21],
            rent_program: &arr[22],
        }
    }
}
pub const MIGRATE_TO_AMM_IX_DISCM: [u8; 8usize] = [
    207, 82, 192, 145, 254, 207, 145, 223,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateToAmmIxData;
impl MigrateToAmmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_TO_AMM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_TO_AMM_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_to_amm_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateToAmmKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateToAmmIxData.try_to_vec()?,
    })
}
pub fn migrate_to_amm_ix(keys: MigrateToAmmKeys) -> std::io::Result<Instruction> {
    migrate_to_amm_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn migrate_to_amm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToAmmAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateToAmmKeys = accounts.into();
    let ix = migrate_to_amm_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_to_amm_invoke(accounts: MigrateToAmmAccounts<'_, '_>) -> ProgramResult {
    migrate_to_amm_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn migrate_to_amm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToAmmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateToAmmKeys = accounts.into();
    let ix = migrate_to_amm_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_to_amm_invoke_signed(
    accounts: MigrateToAmmAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_to_amm_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_to_amm_verify_account_keys(
    accounts: MigrateToAmmAccounts<'_, '_>,
    keys: MigrateToAmmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.market.key, keys.market),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.amm_pool.key, keys.amm_pool),
        (*accounts.amm_authority.key, keys.amm_authority),
        (*accounts.amm_lp_mint.key, keys.amm_lp_mint),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.amm_target_orders.key, keys.amm_target_orders),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.amm_create_fee_destination.key, keys.amm_create_fee_destination),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.pool_lp_token.key, keys.pool_lp_token),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_to_amm_verify_writable_privileges<'me, 'info>(
    accounts: MigrateToAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.amm_pool,
        accounts.amm_lp_mint,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.amm_target_orders,
        accounts.amm_create_fee_destination,
        accounts.authority,
        accounts.pool_state,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.pool_lp_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_to_amm_verify_signer_privileges<'me, 'info>(
    accounts: MigrateToAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_to_amm_verify_account_privileges<'me, 'info>(
    accounts: MigrateToAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_to_amm_verify_writable_privileges(accounts)?;
    migrate_to_amm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN: usize = 28;
#[derive(Copy, Clone, Debug)]
pub struct MigrateToCpswapAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub cpswap_program: &'me AccountInfo<'info>,
    pub cpswap_pool: &'me AccountInfo<'info>,
    pub cpswap_authority: &'me AccountInfo<'info>,
    pub cpswap_lp_mint: &'me AccountInfo<'info>,
    pub cpswap_base_vault: &'me AccountInfo<'info>,
    pub cpswap_quote_vault: &'me AccountInfo<'info>,
    pub cpswap_config: &'me AccountInfo<'info>,
    pub cpswap_create_pool_fee: &'me AccountInfo<'info>,
    pub cpswap_observation: &'me AccountInfo<'info>,
    pub lock_program: &'me AccountInfo<'info>,
    pub lock_authority: &'me AccountInfo<'info>,
    pub lock_lp_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub pool_lp_token: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateToCpswapKeys {
    pub payer: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub platform_config: Pubkey,
    pub cpswap_program: Pubkey,
    pub cpswap_pool: Pubkey,
    pub cpswap_authority: Pubkey,
    pub cpswap_lp_mint: Pubkey,
    pub cpswap_base_vault: Pubkey,
    pub cpswap_quote_vault: Pubkey,
    pub cpswap_config: Pubkey,
    pub cpswap_create_pool_fee: Pubkey,
    pub cpswap_observation: Pubkey,
    pub lock_program: Pubkey,
    pub lock_authority: Pubkey,
    pub lock_lp_vault: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub pool_lp_token: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
    pub metadata_program: Pubkey,
}
impl From<MigrateToCpswapAccounts<'_, '_>> for MigrateToCpswapKeys {
    fn from(accounts: MigrateToCpswapAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            platform_config: *accounts.platform_config.key,
            cpswap_program: *accounts.cpswap_program.key,
            cpswap_pool: *accounts.cpswap_pool.key,
            cpswap_authority: *accounts.cpswap_authority.key,
            cpswap_lp_mint: *accounts.cpswap_lp_mint.key,
            cpswap_base_vault: *accounts.cpswap_base_vault.key,
            cpswap_quote_vault: *accounts.cpswap_quote_vault.key,
            cpswap_config: *accounts.cpswap_config.key,
            cpswap_create_pool_fee: *accounts.cpswap_create_pool_fee.key,
            cpswap_observation: *accounts.cpswap_observation.key,
            lock_program: *accounts.lock_program.key,
            lock_authority: *accounts.lock_authority.key,
            lock_lp_vault: *accounts.lock_lp_vault.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            pool_lp_token: *accounts.pool_lp_token.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}
impl From<MigrateToCpswapKeys> for [AccountMeta; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateToCpswapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_create_pool_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_observation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_lp_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
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
                pubkey: keys.rent_program,
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
impl From<[Pubkey; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]> for MigrateToCpswapKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            platform_config: pubkeys[3],
            cpswap_program: pubkeys[4],
            cpswap_pool: pubkeys[5],
            cpswap_authority: pubkeys[6],
            cpswap_lp_mint: pubkeys[7],
            cpswap_base_vault: pubkeys[8],
            cpswap_quote_vault: pubkeys[9],
            cpswap_config: pubkeys[10],
            cpswap_create_pool_fee: pubkeys[11],
            cpswap_observation: pubkeys[12],
            lock_program: pubkeys[13],
            lock_authority: pubkeys[14],
            lock_lp_vault: pubkeys[15],
            authority: pubkeys[16],
            pool_state: pubkeys[17],
            global_config: pubkeys[18],
            base_vault: pubkeys[19],
            quote_vault: pubkeys[20],
            pool_lp_token: pubkeys[21],
            token_program: pubkeys[22],
            token_program_2022: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
            rent_program: pubkeys[26],
            metadata_program: pubkeys[27],
        }
    }
}
impl<'info> From<MigrateToCpswapAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateToCpswapAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.platform_config.clone(),
            accounts.cpswap_program.clone(),
            accounts.cpswap_pool.clone(),
            accounts.cpswap_authority.clone(),
            accounts.cpswap_lp_mint.clone(),
            accounts.cpswap_base_vault.clone(),
            accounts.cpswap_quote_vault.clone(),
            accounts.cpswap_config.clone(),
            accounts.cpswap_create_pool_fee.clone(),
            accounts.cpswap_observation.clone(),
            accounts.lock_program.clone(),
            accounts.lock_authority.clone(),
            accounts.lock_lp_vault.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.pool_lp_token.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]>
for MigrateToCpswapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            platform_config: &arr[3],
            cpswap_program: &arr[4],
            cpswap_pool: &arr[5],
            cpswap_authority: &arr[6],
            cpswap_lp_mint: &arr[7],
            cpswap_base_vault: &arr[8],
            cpswap_quote_vault: &arr[9],
            cpswap_config: &arr[10],
            cpswap_create_pool_fee: &arr[11],
            cpswap_observation: &arr[12],
            lock_program: &arr[13],
            lock_authority: &arr[14],
            lock_lp_vault: &arr[15],
            authority: &arr[16],
            pool_state: &arr[17],
            global_config: &arr[18],
            base_vault: &arr[19],
            quote_vault: &arr[20],
            pool_lp_token: &arr[21],
            token_program: &arr[22],
            token_program_2022: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
            rent_program: &arr[26],
            metadata_program: &arr[27],
        }
    }
}
pub const MIGRATE_TO_CPSWAP_IX_DISCM: [u8; 8usize] = [
    136, 92, 200, 103, 28, 218, 144, 140,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateToCpswapIxData;
impl MigrateToCpswapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_TO_CPSWAP_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_TO_CPSWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_to_cpswap_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateToCpswapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateToCpswapIxData.try_to_vec()?,
    })
}
pub fn migrate_to_cpswap_ix(keys: MigrateToCpswapKeys) -> std::io::Result<Instruction> {
    migrate_to_cpswap_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys)
}
pub fn migrate_to_cpswap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToCpswapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateToCpswapKeys = accounts.into();
    let ix = migrate_to_cpswap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_to_cpswap_invoke(
    accounts: MigrateToCpswapAccounts<'_, '_>,
) -> ProgramResult {
    migrate_to_cpswap_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts)
}
pub fn migrate_to_cpswap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToCpswapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateToCpswapKeys = accounts.into();
    let ix = migrate_to_cpswap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_to_cpswap_invoke_signed(
    accounts: MigrateToCpswapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_to_cpswap_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_to_cpswap_verify_account_keys(
    accounts: MigrateToCpswapAccounts<'_, '_>,
    keys: MigrateToCpswapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.cpswap_program.key, keys.cpswap_program),
        (*accounts.cpswap_pool.key, keys.cpswap_pool),
        (*accounts.cpswap_authority.key, keys.cpswap_authority),
        (*accounts.cpswap_lp_mint.key, keys.cpswap_lp_mint),
        (*accounts.cpswap_base_vault.key, keys.cpswap_base_vault),
        (*accounts.cpswap_quote_vault.key, keys.cpswap_quote_vault),
        (*accounts.cpswap_config.key, keys.cpswap_config),
        (*accounts.cpswap_create_pool_fee.key, keys.cpswap_create_pool_fee),
        (*accounts.cpswap_observation.key, keys.cpswap_observation),
        (*accounts.lock_program.key, keys.lock_program),
        (*accounts.lock_authority.key, keys.lock_authority),
        (*accounts.lock_lp_vault.key, keys.lock_lp_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.pool_lp_token.key, keys.pool_lp_token),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_to_cpswap_verify_writable_privileges<'me, 'info>(
    accounts: MigrateToCpswapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.base_mint,
        accounts.cpswap_pool,
        accounts.cpswap_lp_mint,
        accounts.cpswap_base_vault,
        accounts.cpswap_quote_vault,
        accounts.cpswap_create_pool_fee,
        accounts.cpswap_observation,
        accounts.lock_lp_vault,
        accounts.authority,
        accounts.pool_state,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.pool_lp_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_to_cpswap_verify_signer_privileges<'me, 'info>(
    accounts: MigrateToCpswapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_to_cpswap_verify_account_privileges<'me, 'info>(
    accounts: MigrateToCpswapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_to_cpswap_verify_writable_privileges(accounts)?;
    migrate_to_cpswap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemovePlatformCurveRuleAccounts<'me, 'info> {
    pub curve_rule_authority: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_curve_rule: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePlatformCurveRuleKeys {
    pub curve_rule_authority: Pubkey,
    pub platform_config: Pubkey,
    pub global_config: Pubkey,
    pub platform_curve_rule: Pubkey,
    pub system_program: Pubkey,
}
impl From<RemovePlatformCurveRuleAccounts<'_, '_>> for RemovePlatformCurveRuleKeys {
    fn from(accounts: RemovePlatformCurveRuleAccounts) -> Self {
        Self {
            curve_rule_authority: *accounts.curve_rule_authority.key,
            platform_config: *accounts.platform_config.key,
            global_config: *accounts.global_config.key,
            platform_curve_rule: *accounts.platform_curve_rule.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RemovePlatformCurveRuleKeys>
for [AccountMeta; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePlatformCurveRuleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curve_rule_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_curve_rule,
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
impl From<[Pubkey; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for RemovePlatformCurveRuleKeys {
    fn from(pubkeys: [Pubkey; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curve_rule_authority: pubkeys[0],
            platform_config: pubkeys[1],
            global_config: pubkeys[2],
            platform_curve_rule: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<RemovePlatformCurveRuleAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePlatformCurveRuleAccounts<'_, 'info>) -> Self {
        [
            accounts.curve_rule_authority.clone(),
            accounts.platform_config.clone(),
            accounts.global_config.clone(),
            accounts.platform_curve_rule.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for RemovePlatformCurveRuleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curve_rule_authority: &arr[0],
            platform_config: &arr[1],
            global_config: &arr[2],
            platform_curve_rule: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const REMOVE_PLATFORM_CURVE_RULE_IX_DISCM: [u8; 8usize] = [
    236, 77, 154, 138, 32, 145, 127, 219,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemovePlatformCurveRuleIxArgs {
    pub group_id: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePlatformCurveRuleIxData(pub RemovePlatformCurveRuleIxArgs);
impl From<RemovePlatformCurveRuleIxArgs> for RemovePlatformCurveRuleIxData {
    fn from(args: RemovePlatformCurveRuleIxArgs) -> Self {
        Self(args)
    }
}
impl RemovePlatformCurveRuleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_PLATFORM_CURVE_RULE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let group_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(RemovePlatformCurveRuleIxArgs {
                group_id,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PLATFORM_CURVE_RULE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.group_id, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_platform_curve_rule_ix_with_program_id(
    program_id: Pubkey,
    keys: RemovePlatformCurveRuleKeys,
    args: RemovePlatformCurveRuleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemovePlatformCurveRuleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_platform_curve_rule_ix(
    keys: RemovePlatformCurveRuleKeys,
    args: RemovePlatformCurveRuleIxArgs,
) -> std::io::Result<Instruction> {
    remove_platform_curve_rule_ix_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn remove_platform_curve_rule_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePlatformCurveRuleAccounts<'_, '_>,
    args: RemovePlatformCurveRuleIxArgs,
) -> ProgramResult {
    let keys: RemovePlatformCurveRuleKeys = accounts.into();
    let ix = remove_platform_curve_rule_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_platform_curve_rule_invoke(
    accounts: RemovePlatformCurveRuleAccounts<'_, '_>,
    args: RemovePlatformCurveRuleIxArgs,
) -> ProgramResult {
    remove_platform_curve_rule_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn remove_platform_curve_rule_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePlatformCurveRuleAccounts<'_, '_>,
    args: RemovePlatformCurveRuleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePlatformCurveRuleKeys = accounts.into();
    let ix = remove_platform_curve_rule_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_platform_curve_rule_invoke_signed(
    accounts: RemovePlatformCurveRuleAccounts<'_, '_>,
    args: RemovePlatformCurveRuleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_platform_curve_rule_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_platform_curve_rule_verify_account_keys(
    accounts: RemovePlatformCurveRuleAccounts<'_, '_>,
    keys: RemovePlatformCurveRuleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curve_rule_authority.key, keys.curve_rule_authority),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_curve_rule.key, keys.platform_curve_rule),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_platform_curve_rule_verify_writable_privileges<'me, 'info>(
    accounts: RemovePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curve_rule_authority,
        accounts.platform_curve_rule,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_platform_curve_rule_verify_signer_privileges<'me, 'info>(
    accounts: RemovePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curve_rule_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_platform_curve_rule_verify_account_privileges<'me, 'info>(
    accounts: RemovePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_platform_curve_rule_verify_writable_privileges(accounts)?;
    remove_platform_curve_rule_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_EXACT_IN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SellExactInAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactInKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SellExactInAccounts<'_, '_>> for SellExactInKeys {
    fn from(accounts: SellExactInAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellExactInKeys> for [AccountMeta; SELL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
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
impl From<[Pubkey; SELL_EXACT_IN_IX_ACCOUNTS_LEN]> for SellExactInKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SellExactInAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN]>
for SellExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SELL_EXACT_IN_IX_DISCM: [u8; 8usize] = [149, 39, 222, 155, 211, 124, 152, 26];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellExactInIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellExactInIxData(pub SellExactInIxArgs);
impl From<SellExactInIxArgs> for SellExactInIxData {
    fn from(args: SellExactInIxArgs) -> Self {
        Self(args)
    }
}
impl SellExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellExactInIxArgs {
                amount_in,
                minimum_amount_out,
                share_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_IN_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.minimum_amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactInKeys,
    args: SellExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_exact_in_ix(
    keys: SellExactInKeys,
    args: SellExactInIxArgs,
) -> std::io::Result<Instruction> {
    sell_exact_in_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn sell_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
) -> ProgramResult {
    let keys: SellExactInKeys = accounts.into();
    let ix = sell_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_exact_in_invoke(
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
) -> ProgramResult {
    sell_exact_in_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn sell_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactInKeys = accounts.into();
    let ix = sell_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_exact_in_invoke_signed(
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_in_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sell_exact_in_verify_account_keys(
    accounts: SellExactInAccounts<'_, '_>,
    keys: SellExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_exact_in_verify_account_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_in_verify_writable_privileges(accounts)?;
    sell_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SellExactOutAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactOutKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SellExactOutAccounts<'_, '_>> for SellExactOutKeys {
    fn from(accounts: SellExactOutAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellExactOutKeys> for [AccountMeta; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
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
impl From<[Pubkey; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]> for SellExactOutKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SellExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]>
for SellExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SELL_EXACT_OUT_IX_DISCM: [u8; 8usize] = [95, 200, 71, 34, 8, 9, 11, 166];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellExactOutIxArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellExactOutIxData(pub SellExactOutIxArgs);
impl From<SellExactOutIxArgs> for SellExactOutIxData {
    fn from(args: SellExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl SellExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maximum_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SellExactOutIxArgs {
                amount_out,
                maximum_amount_in,
                share_fee_rate,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.amount_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.maximum_amount_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.share_fee_rate, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactOutKeys,
    args: SellExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_exact_out_ix(
    keys: SellExactOutKeys,
    args: SellExactOutIxArgs,
) -> std::io::Result<Instruction> {
    sell_exact_out_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn sell_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
) -> ProgramResult {
    let keys: SellExactOutKeys = accounts.into();
    let ix = sell_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_exact_out_invoke(
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
) -> ProgramResult {
    sell_exact_out_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn sell_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactOutKeys = accounts.into();
    let ix = sell_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_exact_out_invoke_signed(
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_out_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn sell_exact_out_verify_account_keys(
    accounts: SellExactOutAccounts<'_, '_>,
    keys: SellExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_out_verify_writable_privileges(accounts)?;
    sell_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConfigKeys {
    pub owner: Pubkey,
    pub global_config: Pubkey,
}
impl From<UpdateConfigAccounts<'_, '_>> for UpdateConfigKeys {
    fn from(accounts: UpdateConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            global_config: *accounts.global_config.key,
        }
    }
}
impl From<UpdateConfigKeys> for [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConfigAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.global_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            global_config: &arr[1],
        }
    }
}
pub const UPDATE_CONFIG_IX_DISCM: [u8; 8usize] = [29, 158, 252, 191, 10, 83, 219, 99];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigIxArgs {
    pub param: u8,
    pub value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfigIxData(pub UpdateConfigIxArgs);
impl From<UpdateConfigIxArgs> for UpdateConfigIxData {
    fn from(args: UpdateConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let param: u8 = crate::borsh_de_or_default(&mut reader)?;
        let value: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(Self(UpdateConfigIxArgs { param, value }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.param, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.value, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConfigKeys,
    args: UpdateConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_config_ix(
    keys: UpdateConfigKeys,
    args: UpdateConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_config_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn update_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateConfigKeys = accounts.into();
    let ix = update_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_config_invoke(
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
) -> ProgramResult {
    update_config_invoke_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, accounts, args)
}
pub fn update_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConfigKeys = accounts.into();
    let ix = update_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_config_invoke_signed(
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_config_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_config_verify_account_keys(
    accounts: UpdateConfigAccounts<'_, '_>,
    keys: UpdateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_config_verify_writable_privileges(accounts)?;
    update_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePlatformConfigAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePlatformConfigKeys {
    pub platform_admin: Pubkey,
    pub platform_config: Pubkey,
}
impl From<UpdatePlatformConfigAccounts<'_, '_>> for UpdatePlatformConfigKeys {
    fn from(accounts: UpdatePlatformConfigAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_config: *accounts.platform_config.key,
        }
    }
}
impl From<UpdatePlatformConfigKeys>
for [AccountMeta; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePlatformConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]>
for UpdatePlatformConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdatePlatformConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePlatformConfigAccounts<'_, 'info>) -> Self {
        [accounts.platform_admin.clone(), accounts.platform_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]>
for UpdatePlatformConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_config: &arr[1],
        }
    }
}
pub const UPDATE_PLATFORM_CONFIG_IX_DISCM: [u8; 8usize] = [
    195, 60, 76, 129, 146, 45, 67, 143,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePlatformConfigIxArgs {
    pub param: PlatformConfigParam,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePlatformConfigIxData(pub UpdatePlatformConfigIxArgs);
impl From<UpdatePlatformConfigIxArgs> for UpdatePlatformConfigIxData {
    fn from(args: UpdatePlatformConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePlatformConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PLATFORM_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let param = <PlatformConfigParam as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        Ok(
            Self(UpdatePlatformConfigIxArgs {
                param,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PLATFORM_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.param, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_platform_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePlatformConfigKeys,
    args: UpdatePlatformConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePlatformConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_platform_config_ix(
    keys: UpdatePlatformConfigKeys,
    args: UpdatePlatformConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_platform_config_ix_with_program_id(RAYDIUM_LAUNCHPAD_PROGRAM_ID, keys, args)
}
pub fn update_platform_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
) -> ProgramResult {
    let keys: UpdatePlatformConfigKeys = accounts.into();
    let ix = update_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_platform_config_invoke(
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
) -> ProgramResult {
    update_platform_config_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_platform_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePlatformConfigKeys = accounts.into();
    let ix = update_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_platform_config_invoke_signed(
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_platform_config_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_platform_config_verify_account_keys(
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    keys: UpdatePlatformConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_config.key, keys.platform_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_platform_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_platform_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_platform_config_verify_account_privileges<'me, 'info>(
    accounts: UpdatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_platform_config_verify_writable_privileges(accounts)?;
    update_platform_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePlatformCurveRuleAccounts<'me, 'info> {
    pub curve_rule_authority: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_curve_rule: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePlatformCurveRuleKeys {
    pub curve_rule_authority: Pubkey,
    pub platform_config: Pubkey,
    pub global_config: Pubkey,
    pub platform_curve_rule: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdatePlatformCurveRuleAccounts<'_, '_>> for UpdatePlatformCurveRuleKeys {
    fn from(accounts: UpdatePlatformCurveRuleAccounts) -> Self {
        Self {
            curve_rule_authority: *accounts.curve_rule_authority.key,
            platform_config: *accounts.platform_config.key,
            global_config: *accounts.global_config.key,
            platform_curve_rule: *accounts.platform_curve_rule.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdatePlatformCurveRuleKeys>
for [AccountMeta; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePlatformCurveRuleKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curve_rule_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_curve_rule,
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
impl From<[Pubkey; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for UpdatePlatformCurveRuleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curve_rule_authority: pubkeys[0],
            platform_config: pubkeys[1],
            global_config: pubkeys[2],
            platform_curve_rule: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdatePlatformCurveRuleAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePlatformCurveRuleAccounts<'_, 'info>) -> Self {
        [
            accounts.curve_rule_authority.clone(),
            accounts.platform_config.clone(),
            accounts.global_config.clone(),
            accounts.platform_curve_rule.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN]>
for UpdatePlatformCurveRuleAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            curve_rule_authority: &arr[0],
            platform_config: &arr[1],
            global_config: &arr[2],
            platform_curve_rule: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const UPDATE_PLATFORM_CURVE_RULE_IX_DISCM: [u8; 8usize] = [
    90, 100, 206, 196, 54, 117, 120, 139,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdatePlatformCurveRuleIxArgs {
    pub group_id: u16,
    pub constraints: Vec<ParamConstraint>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePlatformCurveRuleIxData(pub UpdatePlatformCurveRuleIxArgs);
impl From<UpdatePlatformCurveRuleIxArgs> for UpdatePlatformCurveRuleIxData {
    fn from(args: UpdatePlatformCurveRuleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePlatformCurveRuleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PLATFORM_CURVE_RULE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let group_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let constraints: Vec<ParamConstraint> = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UpdatePlatformCurveRuleIxArgs {
                group_id,
                constraints,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PLATFORM_CURVE_RULE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.group_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.constraints, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_platform_curve_rule_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePlatformCurveRuleKeys,
    args: UpdatePlatformCurveRuleIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_PLATFORM_CURVE_RULE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePlatformCurveRuleIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_platform_curve_rule_ix(
    keys: UpdatePlatformCurveRuleKeys,
    args: UpdatePlatformCurveRuleIxArgs,
) -> std::io::Result<Instruction> {
    update_platform_curve_rule_ix_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn update_platform_curve_rule_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePlatformCurveRuleAccounts<'_, '_>,
    args: UpdatePlatformCurveRuleIxArgs,
) -> ProgramResult {
    let keys: UpdatePlatformCurveRuleKeys = accounts.into();
    let ix = update_platform_curve_rule_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_platform_curve_rule_invoke(
    accounts: UpdatePlatformCurveRuleAccounts<'_, '_>,
    args: UpdatePlatformCurveRuleIxArgs,
) -> ProgramResult {
    update_platform_curve_rule_invoke_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn update_platform_curve_rule_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePlatformCurveRuleAccounts<'_, '_>,
    args: UpdatePlatformCurveRuleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePlatformCurveRuleKeys = accounts.into();
    let ix = update_platform_curve_rule_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_platform_curve_rule_invoke_signed(
    accounts: UpdatePlatformCurveRuleAccounts<'_, '_>,
    args: UpdatePlatformCurveRuleIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_platform_curve_rule_invoke_signed_with_program_id(
        RAYDIUM_LAUNCHPAD_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_platform_curve_rule_verify_account_keys(
    accounts: UpdatePlatformCurveRuleAccounts<'_, '_>,
    keys: UpdatePlatformCurveRuleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curve_rule_authority.key, keys.curve_rule_authority),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_curve_rule.key, keys.platform_curve_rule),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_platform_curve_rule_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curve_rule_authority,
        accounts.platform_curve_rule,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_platform_curve_rule_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.curve_rule_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_platform_curve_rule_verify_account_privileges<'me, 'info>(
    accounts: UpdatePlatformCurveRuleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_platform_curve_rule_verify_writable_privileges(accounts)?;
    update_platform_curve_rule_verify_signer_privileges(accounts)?;
    Ok(())
}
