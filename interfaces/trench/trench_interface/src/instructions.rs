use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum TrenchProgramIx {
    Buy(BuyIxArgs),
    BuyExactOut(BuyExactOutIxArgs),
    ClaimCreatorRewards,
    ClaimReferralRewards,
    ClaimTraderCashback,
    Create(CreateIxArgs),
    CreateWithBypass(CreateWithBypassIxArgs),
    InitializeConfig(InitializeConfigIxArgs),
    InitializeMigrationConfig(InitializeMigrationConfigIxArgs),
    InitializeReferrals(InitializeReferralsIxArgs),
    InitializeRevenueConfig(InitializeRevenueConfigIxArgs),
    InitializeTraderReferral(InitializeTraderReferralIxArgs),
    Migrate,
    MigrateTradeAuthorityWhitelist,
    PayoutCreatorRevenue(PayoutCreatorRevenueIxArgs),
    PayoutProtocolRevenue(PayoutProtocolRevenueIxArgs),
    Sell(SellIxArgs),
    SetCreatorRevenueRecipient(SetCreatorRevenueRecipientIxArgs),
    SetPartnerStatus(SetPartnerStatusIxArgs),
    SettleCreatorRevenue(SettleCreatorRevenueIxArgs),
    SetupCreatorRevenue,
    UnwhitelistLaunchPair(UnwhitelistLaunchPairIxArgs),
    UnwhitelistTradeAuthority(UnwhitelistTradeAuthorityIxArgs),
    UpdateMigrationConfig(UpdateMigrationConfigIxArgs),
    UpdateRevenueConfig(UpdateRevenueConfigIxArgs),
    ValidateLaunchLimitTracker,
    WhitelistLaunchPair(WhitelistLaunchPairIxArgs),
    WhitelistTradeAuthority(WhitelistTradeAuthorityIxArgs),
}
impl TrenchProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&BUY_IX_DISCM) {
            let mut reader = &buf[BUY_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <BuyParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Buy(BuyIxArgs { params }));
        }
        if buf.starts_with(&BUY_EXACT_OUT_IX_DISCM) {
            let mut reader = &buf[BUY_EXACT_OUT_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <BuyExactOutParams>::deserialize(&mut reader)?
            };
            return Ok(Self::BuyExactOut(BuyExactOutIxArgs { params }));
        }
        if buf.starts_with(&CLAIM_CREATOR_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimCreatorRewards);
        }
        if buf.starts_with(&CLAIM_REFERRAL_REWARDS_IX_DISCM) {
            return Ok(Self::ClaimReferralRewards);
        }
        if buf.starts_with(&CLAIM_TRADER_CASHBACK_IX_DISCM) {
            return Ok(Self::ClaimTraderCashback);
        }
        if buf.starts_with(&CREATE_IX_DISCM) {
            let mut reader = &buf[CREATE_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Create(CreateIxArgs { params }));
        }
        if buf.starts_with(&CREATE_WITH_BYPASS_IX_DISCM) {
            let mut reader = &buf[CREATE_WITH_BYPASS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <CreateParams>::deserialize(&mut reader)?
            };
            let max_bypass_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::CreateWithBypass(CreateWithBypassIxArgs {
                    params,
                    max_bypass_fee_lamports,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeConfigParams>::deserialize(&mut reader)?
            };
            return Ok(Self::InitializeConfig(InitializeConfigIxArgs { params }));
        }
        if buf.starts_with(&INITIALIZE_MIGRATION_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_MIGRATION_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeMigrationConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeMigrationConfig(InitializeMigrationConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_REFERRALS_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_REFERRALS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeReferralsParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeReferrals(InitializeReferralsIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_REVENUE_CONFIG_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_REVENUE_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <RevenueConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeRevenueConfig(InitializeRevenueConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&INITIALIZE_TRADER_REFERRAL_IX_DISCM) {
            let mut reader = &buf[INITIALIZE_TRADER_REFERRAL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <InitializeTraderReferralParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::InitializeTraderReferral(InitializeTraderReferralIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&MIGRATE_IX_DISCM) {
            return Ok(Self::Migrate);
        }
        if buf.starts_with(&MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_DISCM) {
            return Ok(Self::MigrateTradeAuthorityWhitelist);
        }
        if buf.starts_with(&PAYOUT_CREATOR_REVENUE_IX_DISCM) {
            let mut reader = &buf[PAYOUT_CREATOR_REVENUE_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <PayoutCreatorRevenueParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::PayoutCreatorRevenue(PayoutCreatorRevenueIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&PAYOUT_PROTOCOL_REVENUE_IX_DISCM) {
            let mut reader = &buf[PAYOUT_PROTOCOL_REVENUE_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <PayoutProtocolRevenueParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::PayoutProtocolRevenue(PayoutProtocolRevenueIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&SELL_IX_DISCM) {
            let mut reader = &buf[SELL_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SellParams>::deserialize(&mut reader)?
            };
            return Ok(Self::Sell(SellIxArgs { params }));
        }
        if buf.starts_with(&SET_CREATOR_REVENUE_RECIPIENT_IX_DISCM) {
            let mut reader = &buf[SET_CREATOR_REVENUE_RECIPIENT_IX_DISCM.len()..];
            let new_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            let expected_recipient_revision: u64 = crate::borsh_de_or_default(
                &mut reader,
            )?;
            return Ok(
                Self::SetCreatorRevenueRecipient(SetCreatorRevenueRecipientIxArgs {
                    new_recipient,
                    expected_recipient_revision,
                }),
            );
        }
        if buf.starts_with(&SET_PARTNER_STATUS_IX_DISCM) {
            let mut reader = &buf[SET_PARTNER_STATUS_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SetPartnerStatusParams>::deserialize(&mut reader)?
            };
            return Ok(Self::SetPartnerStatus(SetPartnerStatusIxArgs { params }));
        }
        if buf.starts_with(&SETTLE_CREATOR_REVENUE_IX_DISCM) {
            let mut reader = &buf[SETTLE_CREATOR_REVENUE_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <SettleCreatorRevenueParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SettleCreatorRevenue(SettleCreatorRevenueIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&SETUP_CREATOR_REVENUE_IX_DISCM) {
            return Ok(Self::SetupCreatorRevenue);
        }
        if buf.starts_with(&UNWHITELIST_LAUNCH_PAIR_IX_DISCM) {
            let mut reader = &buf[UNWHITELIST_LAUNCH_PAIR_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <WhitelistLaunchPairParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UnwhitelistLaunchPair(UnwhitelistLaunchPairIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UNWHITELIST_TRADE_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[UNWHITELIST_TRADE_AUTHORITY_IX_DISCM.len()..];
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::UnwhitelistTradeAuthority(UnwhitelistTradeAuthorityIxArgs {
                    authority,
                }),
            );
        }
        if buf.starts_with(&UPDATE_MIGRATION_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_MIGRATION_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateMigrationConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateMigrationConfig(UpdateMigrationConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&UPDATE_REVENUE_CONFIG_IX_DISCM) {
            let mut reader = &buf[UPDATE_REVENUE_CONFIG_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <RevenueConfigParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::UpdateRevenueConfig(UpdateRevenueConfigIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&VALIDATE_LAUNCH_LIMIT_TRACKER_IX_DISCM) {
            return Ok(Self::ValidateLaunchLimitTracker);
        }
        if buf.starts_with(&WHITELIST_LAUNCH_PAIR_IX_DISCM) {
            let mut reader = &buf[WHITELIST_LAUNCH_PAIR_IX_DISCM.len()..];
            let params = if reader.is_empty() {
                Default::default()
            } else {
                <WhitelistLaunchPairParams>::deserialize(&mut reader)?
            };
            return Ok(
                Self::WhitelistLaunchPair(WhitelistLaunchPairIxArgs {
                    params,
                }),
            );
        }
        if buf.starts_with(&WHITELIST_TRADE_AUTHORITY_IX_DISCM) {
            let mut reader = &buf[WHITELIST_TRADE_AUTHORITY_IX_DISCM.len()..];
            let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
            return Ok(
                Self::WhitelistTradeAuthority(WhitelistTradeAuthorityIxArgs {
                    authority,
                }),
            );
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::BuyExactOut(args) => {
                writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::ClaimCreatorRewards => {
                writer.write_all(&CLAIM_CREATOR_REWARDS_IX_DISCM)
            }
            Self::ClaimReferralRewards => {
                writer.write_all(&CLAIM_REFERRAL_REWARDS_IX_DISCM)
            }
            Self::ClaimTraderCashback => {
                writer.write_all(&CLAIM_TRADER_CASHBACK_IX_DISCM)
            }
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::CreateWithBypass(args) => {
                writer.write_all(&CREATE_WITH_BYPASS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.max_bypass_fee_lamports,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::InitializeConfig(args) => {
                writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeMigrationConfig(args) => {
                writer.write_all(&INITIALIZE_MIGRATION_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeReferrals(args) => {
                writer.write_all(&INITIALIZE_REFERRALS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeRevenueConfig(args) => {
                writer.write_all(&INITIALIZE_REVENUE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::InitializeTraderReferral(args) => {
                writer.write_all(&INITIALIZE_TRADER_REFERRAL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Migrate => writer.write_all(&MIGRATE_IX_DISCM),
            Self::MigrateTradeAuthorityWhitelist => {
                writer.write_all(&MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_DISCM)
            }
            Self::PayoutCreatorRevenue(args) => {
                writer.write_all(&PAYOUT_CREATOR_REVENUE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::PayoutProtocolRevenue(args) => {
                writer.write_all(&PAYOUT_PROTOCOL_REVENUE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetCreatorRevenueRecipient(args) => {
                writer.write_all(&SET_CREATOR_REVENUE_RECIPIENT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.new_recipient, &mut writer)?;
                borsh::BorshSerialize::serialize(
                    &args.expected_recipient_revision,
                    &mut writer,
                )?;
                Ok(())
            }
            Self::SetPartnerStatus(args) => {
                writer.write_all(&SET_PARTNER_STATUS_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SettleCreatorRevenue(args) => {
                writer.write_all(&SETTLE_CREATOR_REVENUE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::SetupCreatorRevenue => {
                writer.write_all(&SETUP_CREATOR_REVENUE_IX_DISCM)
            }
            Self::UnwhitelistLaunchPair(args) => {
                writer.write_all(&UNWHITELIST_LAUNCH_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UnwhitelistTradeAuthority(args) => {
                writer.write_all(&UNWHITELIST_TRADE_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
                Ok(())
            }
            Self::UpdateMigrationConfig(args) => {
                writer.write_all(&UPDATE_MIGRATION_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::UpdateRevenueConfig(args) => {
                writer.write_all(&UPDATE_REVENUE_CONFIG_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::ValidateLaunchLimitTracker => {
                writer.write_all(&VALIDATE_LAUNCH_LIMIT_TRACKER_IX_DISCM)
            }
            Self::WhitelistLaunchPair(args) => {
                writer.write_all(&WHITELIST_LAUNCH_PAIR_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.params, &mut writer)?;
                Ok(())
            }
            Self::WhitelistTradeAuthority(args) => {
                writer.write_all(&WHITELIST_TRADE_AUTHORITY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.authority, &mut writer)?;
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
pub const BUY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub buyer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub curve_vault: &'me AccountInfo<'info>,
    pub buyer_token_account: &'me AccountInfo<'info>,
    pub buyer_limit: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub trader_cashback_vault: &'me AccountInfo<'info>,
    pub trader_referral: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub buyer: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub curve_vault: Pubkey,
    pub buyer_token_account: Pubkey,
    pub buyer_limit: Pubkey,
    pub creator_vault: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub trader_referral: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            buyer: *accounts.buyer.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            curve_vault: *accounts.curve_vault.key,
            buyer_token_account: *accounts.buyer_token_account.key,
            buyer_limit: *accounts.buyer_limit.key,
            creator_vault: *accounts.creator_vault.key,
            trader_cashback_vault: *accounts.trader_cashback_vault.key,
            trader_referral: *accounts.trader_referral.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_limit,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_cashback_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_referral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
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
        ]
    }
}
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            curve_vault: pubkeys[5],
            buyer_token_account: pubkeys[6],
            buyer_limit: pubkeys[7],
            creator_vault: pubkeys[8],
            trader_cashback_vault: pubkeys[9],
            trader_referral: pubkeys[10],
            protocol_fee_recipient: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.buyer.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.curve_vault.clone(),
            accounts.buyer_token_account.clone(),
            accounts.buyer_limit.clone(),
            accounts.creator_vault.clone(),
            accounts.trader_cashback_vault.clone(),
            accounts.trader_referral.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]>
for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            mint: &arr[3],
            bonding_curve: &arr[4],
            curve_vault: &arr[5],
            buyer_token_account: &arr[6],
            buyer_limit: &arr[7],
            creator_vault: &arr[8],
            trader_cashback_vault: &arr[9],
            trader_referral: &arr[10],
            protocol_fee_recipient: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const BUY_IX_DISCM: [u8; 8usize] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyIxArgs {
    pub params: BuyParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <BuyParams>::deserialize(&mut reader)?
        };
        Ok(Self(BuyIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    buy_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
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
    buy_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
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
    buy_invoke_signed_with_program_id(TRENCH_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.buyer.key, keys.buyer),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.curve_vault.key, keys.curve_vault),
        (*accounts.buyer_token_account.key, keys.buyer_token_account),
        (*accounts.buyer_limit.key, keys.buyer_limit),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.trader_cashback_vault.key, keys.trader_cashback_vault),
        (*accounts.trader_referral.key, keys.trader_referral),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.buyer,
        accounts.bonding_curve,
        accounts.curve_vault,
        accounts.buyer_token_account,
        accounts.buyer_limit,
        accounts.creator_vault,
        accounts.trader_cashback_vault,
        accounts.trader_referral,
        accounts.protocol_fee_recipient,
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
    for should_be_signer in [accounts.buyer, accounts.authority] {
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
pub const BUY_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactOutAccounts<'me, 'info> {
    pub buyer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub curve_vault: &'me AccountInfo<'info>,
    pub buyer_token_account: &'me AccountInfo<'info>,
    pub buyer_limit: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub trader_cashback_vault: &'me AccountInfo<'info>,
    pub trader_referral: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactOutKeys {
    pub buyer: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub curve_vault: Pubkey,
    pub buyer_token_account: Pubkey,
    pub buyer_limit: Pubkey,
    pub creator_vault: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub trader_referral: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<BuyExactOutAccounts<'_, '_>> for BuyExactOutKeys {
    fn from(accounts: BuyExactOutAccounts) -> Self {
        Self {
            buyer: *accounts.buyer.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            curve_vault: *accounts.curve_vault.key,
            buyer_token_account: *accounts.buyer_token_account.key,
            buyer_limit: *accounts.buyer_limit.key,
            creator_vault: *accounts.creator_vault.key,
            trader_cashback_vault: *accounts.trader_cashback_vault.key,
            trader_referral: *accounts.trader_referral.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<BuyExactOutKeys> for [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_limit,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_cashback_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_referral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
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
        ]
    }
}
impl From<[Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]> for BuyExactOutKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            curve_vault: pubkeys[5],
            buyer_token_account: pubkeys[6],
            buyer_limit: pubkeys[7],
            creator_vault: pubkeys[8],
            trader_cashback_vault: pubkeys[9],
            trader_referral: pubkeys[10],
            protocol_fee_recipient: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<BuyExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.buyer.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.curve_vault.clone(),
            accounts.buyer_token_account.clone(),
            accounts.buyer_limit.clone(),
            accounts.creator_vault.clone(),
            accounts.trader_cashback_vault.clone(),
            accounts.trader_referral.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]>
for BuyExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            buyer: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            mint: &arr[3],
            bonding_curve: &arr[4],
            curve_vault: &arr[5],
            buyer_token_account: &arr[6],
            buyer_limit: &arr[7],
            creator_vault: &arr[8],
            trader_cashback_vault: &arr[9],
            trader_referral: &arr[10],
            protocol_fee_recipient: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const BUY_EXACT_OUT_IX_DISCM: [u8; 8usize] = [24, 211, 116, 40, 105, 3, 153, 56];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyExactOutIxArgs {
    pub params: BuyExactOutParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <BuyExactOutParams>::deserialize(&mut reader)?
        };
        Ok(Self(BuyExactOutIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    buy_exact_out_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
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
    buy_exact_out_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
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
    buy_exact_out_invoke_signed_with_program_id(TRENCH_PROGRAM_ID, accounts, args, seeds)
}
pub fn buy_exact_out_verify_account_keys(
    accounts: BuyExactOutAccounts<'_, '_>,
    keys: BuyExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.buyer.key, keys.buyer),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.curve_vault.key, keys.curve_vault),
        (*accounts.buyer_token_account.key, keys.buyer_token_account),
        (*accounts.buyer_limit.key, keys.buyer_limit),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.trader_cashback_vault.key, keys.trader_cashback_vault),
        (*accounts.trader_referral.key, keys.trader_referral),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
        accounts.buyer,
        accounts.bonding_curve,
        accounts.curve_vault,
        accounts.buyer_token_account,
        accounts.buyer_limit,
        accounts.creator_vault,
        accounts.trader_cashback_vault,
        accounts.trader_referral,
        accounts.protocol_fee_recipient,
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
    for should_be_signer in [accounts.buyer, accounts.authority] {
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
pub const CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorRewardsAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorRewardsKeys {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub creator_vault: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimCreatorRewardsAccounts<'_, '_>> for ClaimCreatorRewardsKeys {
    fn from(accounts: ClaimCreatorRewardsAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            mint: *accounts.mint.key,
            creator_vault: *accounts.creator_vault.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimCreatorRewardsKeys>
for [AccountMeta; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCreatorRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
impl From<[Pubkey; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN]> for ClaimCreatorRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            mint: pubkeys[1],
            creator_vault: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<ClaimCreatorRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCreatorRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.mint.clone(),
            accounts.creator_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimCreatorRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            creator: &arr[0],
            mint: &arr[1],
            creator_vault: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CLAIM_CREATOR_REWARDS_IX_DISCM: [u8; 8usize] = [
    14, 215, 177, 181, 221, 193, 125, 85,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorRewardsIxData;
impl ClaimCreatorRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_REWARDS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_creator_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimCreatorRewardsIxData.try_to_vec()?,
    })
}
pub fn claim_creator_rewards_ix(
    keys: ClaimCreatorRewardsKeys,
) -> std::io::Result<Instruction> {
    claim_creator_rewards_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
}
pub fn claim_creator_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimCreatorRewardsKeys = accounts.into();
    let ix = claim_creator_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_creator_rewards_invoke(
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
) -> ProgramResult {
    claim_creator_rewards_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
}
pub fn claim_creator_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorRewardsKeys = accounts.into();
    let ix = claim_creator_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_creator_rewards_invoke_signed(
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_rewards_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_creator_rewards_verify_account_keys(
    accounts: ClaimCreatorRewardsAccounts<'_, '_>,
    keys: ClaimCreatorRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.mint.key, keys.mint),
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
pub fn claim_creator_rewards_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.creator, accounts.creator_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_creator_rewards_verify_signer_privileges<'me, 'info>(
    accounts: ClaimCreatorRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_creator_rewards_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_rewards_verify_writable_privileges(accounts)?;
    claim_creator_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ClaimReferralRewardsAccounts<'me, 'info> {
    pub payout_wallet: &'me AccountInfo<'info>,
    pub trader_referral: &'me AccountInfo<'info>,
    pub referrer_profile: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimReferralRewardsKeys {
    pub payout_wallet: Pubkey,
    pub trader_referral: Pubkey,
    pub referrer_profile: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimReferralRewardsAccounts<'_, '_>> for ClaimReferralRewardsKeys {
    fn from(accounts: ClaimReferralRewardsAccounts) -> Self {
        Self {
            payout_wallet: *accounts.payout_wallet.key,
            trader_referral: *accounts.trader_referral.key,
            referrer_profile: *accounts.referrer_profile.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimReferralRewardsKeys>
for [AccountMeta; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimReferralRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payout_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_referral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_profile,
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
impl From<[Pubkey; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimReferralRewardsKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payout_wallet: pubkeys[0],
            trader_referral: pubkeys[1],
            referrer_profile: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ClaimReferralRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimReferralRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.payout_wallet.clone(),
            accounts.trader_referral.clone(),
            accounts.referrer_profile.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN]>
for ClaimReferralRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_REFERRAL_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payout_wallet: &arr[0],
            trader_referral: &arr[1],
            referrer_profile: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
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
    claim_referral_rewards_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
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
    claim_referral_rewards_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
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
        TRENCH_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_referral_rewards_verify_account_keys(
    accounts: ClaimReferralRewardsAccounts<'_, '_>,
    keys: ClaimReferralRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payout_wallet.key, keys.payout_wallet),
        (*accounts.trader_referral.key, keys.trader_referral),
        (*accounts.referrer_profile.key, keys.referrer_profile),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
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
        accounts.payout_wallet,
        accounts.trader_referral,
        accounts.referrer_profile,
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
    for should_be_signer in [accounts.payout_wallet] {
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
pub const CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTraderCashbackAccounts<'me, 'info> {
    pub trader: &'me AccountInfo<'info>,
    pub trader_cashback_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTraderCashbackKeys {
    pub trader: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimTraderCashbackAccounts<'_, '_>> for ClaimTraderCashbackKeys {
    fn from(accounts: ClaimTraderCashbackAccounts) -> Self {
        Self {
            trader: *accounts.trader.key,
            trader_cashback_vault: *accounts.trader_cashback_vault.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimTraderCashbackKeys>
for [AccountMeta; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTraderCashbackKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_cashback_vault,
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
impl From<[Pubkey; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN]> for ClaimTraderCashbackKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            trader: pubkeys[0],
            trader_cashback_vault: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<ClaimTraderCashbackAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTraderCashbackAccounts<'_, 'info>) -> Self {
        [
            accounts.trader.clone(),
            accounts.trader_cashback_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN]>
for ClaimTraderCashbackAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            trader: &arr[0],
            trader_cashback_vault: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLAIM_TRADER_CASHBACK_IX_DISCM: [u8; 8usize] = [
    31, 144, 105, 250, 7, 98, 206, 29,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTraderCashbackIxData;
impl ClaimTraderCashbackIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TRADER_CASHBACK_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TRADER_CASHBACK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_trader_cashback_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTraderCashbackKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TRADER_CASHBACK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimTraderCashbackIxData.try_to_vec()?,
    })
}
pub fn claim_trader_cashback_ix(
    keys: ClaimTraderCashbackKeys,
) -> std::io::Result<Instruction> {
    claim_trader_cashback_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
}
pub fn claim_trader_cashback_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTraderCashbackAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimTraderCashbackKeys = accounts.into();
    let ix = claim_trader_cashback_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_trader_cashback_invoke(
    accounts: ClaimTraderCashbackAccounts<'_, '_>,
) -> ProgramResult {
    claim_trader_cashback_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
}
pub fn claim_trader_cashback_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTraderCashbackAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTraderCashbackKeys = accounts.into();
    let ix = claim_trader_cashback_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_trader_cashback_invoke_signed(
    accounts: ClaimTraderCashbackAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_trader_cashback_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn claim_trader_cashback_verify_account_keys(
    accounts: ClaimTraderCashbackAccounts<'_, '_>,
    keys: ClaimTraderCashbackKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.trader.key, keys.trader),
        (*accounts.trader_cashback_vault.key, keys.trader_cashback_vault),
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
pub fn claim_trader_cashback_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTraderCashbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.trader, accounts.trader_cashback_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_trader_cashback_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTraderCashbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_trader_cashback_verify_account_privileges<'me, 'info>(
    accounts: ClaimTraderCashbackAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_trader_cashback_verify_writable_privileges(accounts)?;
    claim_trader_cashback_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub creator_cooldown: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub curve_vault: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub governance_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub launch_name_tracker: &'me AccountInfo<'info>,
    pub launch_symbol_tracker: &'me AccountInfo<'info>,
    pub launch_pair_whitelist: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub creator: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub creator_cooldown: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub creator_vault: Pubkey,
    pub curve_vault: Pubkey,
    pub metadata: Pubkey,
    pub governance_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_metadata_program: Pubkey,
    pub rent: Pubkey,
    pub launch_name_tracker: Pubkey,
    pub launch_symbol_tracker: Pubkey,
    pub launch_pair_whitelist: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            creator_cooldown: *accounts.creator_cooldown.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            creator_vault: *accounts.creator_vault.key,
            curve_vault: *accounts.curve_vault.key,
            metadata: *accounts.metadata.key,
            governance_authority: *accounts.governance_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            rent: *accounts.rent.key,
            launch_name_tracker: *accounts.launch_name_tracker.key,
            launch_symbol_tracker: *accounts.launch_symbol_tracker.key,
            launch_pair_whitelist: *accounts.launch_pair_whitelist.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_cooldown,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.governance_authority,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.launch_name_tracker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.launch_symbol_tracker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.launch_pair_whitelist,
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
            creator: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            creator_cooldown: pubkeys[3],
            mint: pubkeys[4],
            bonding_curve: pubkeys[5],
            creator_vault: pubkeys[6],
            curve_vault: pubkeys[7],
            metadata: pubkeys[8],
            governance_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            token_metadata_program: pubkeys[13],
            rent: pubkeys[14],
            launch_name_tracker: pubkeys[15],
            launch_symbol_tracker: pubkeys[16],
            launch_pair_whitelist: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.creator_cooldown.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.creator_vault.clone(),
            accounts.curve_vault.clone(),
            accounts.metadata.clone(),
            accounts.governance_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_metadata_program.clone(),
            accounts.rent.clone(),
            accounts.launch_name_tracker.clone(),
            accounts.launch_symbol_tracker.clone(),
            accounts.launch_pair_whitelist.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            creator_cooldown: &arr[3],
            mint: &arr[4],
            bonding_curve: &arr[5],
            creator_vault: &arr[6],
            curve_vault: &arr[7],
            metadata: &arr[8],
            governance_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            token_metadata_program: &arr[13],
            rent: &arr[14],
            launch_name_tracker: &arr[15],
            launch_symbol_tracker: &arr[16],
            launch_pair_whitelist: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const CREATE_IX_DISCM: [u8; 8usize] = [24, 30, 200, 40, 5, 28, 7, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIxArgs {
    pub params: CreateParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateParams>::deserialize(&mut reader)?
        };
        Ok(Self(CreateIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    create_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
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
    create_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
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
    create_invoke_signed_with_program_id(TRENCH_PROGRAM_ID, accounts, args, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.creator_cooldown.key, keys.creator_cooldown),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.curve_vault.key, keys.curve_vault),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.governance_authority.key, keys.governance_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.launch_name_tracker.key, keys.launch_name_tracker),
        (*accounts.launch_symbol_tracker.key, keys.launch_symbol_tracker),
        (*accounts.launch_pair_whitelist.key, keys.launch_pair_whitelist),
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
        accounts.creator,
        accounts.creator_cooldown,
        accounts.mint,
        accounts.bonding_curve,
        accounts.creator_vault,
        accounts.curve_vault,
        accounts.metadata,
        accounts.launch_name_tracker,
        accounts.launch_symbol_tracker,
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
    for should_be_signer in [accounts.creator, accounts.authority, accounts.mint] {
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
pub const CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct CreateWithBypassAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub creator_cooldown: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub curve_vault: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub governance_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub launch_name_tracker: &'me AccountInfo<'info>,
    pub launch_symbol_tracker: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub launch_pair_whitelist: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateWithBypassKeys {
    pub creator: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub creator_cooldown: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub creator_vault: Pubkey,
    pub curve_vault: Pubkey,
    pub metadata: Pubkey,
    pub governance_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_metadata_program: Pubkey,
    pub rent: Pubkey,
    pub launch_name_tracker: Pubkey,
    pub launch_symbol_tracker: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub launch_pair_whitelist: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateWithBypassAccounts<'_, '_>> for CreateWithBypassKeys {
    fn from(accounts: CreateWithBypassAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            creator_cooldown: *accounts.creator_cooldown.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            creator_vault: *accounts.creator_vault.key,
            curve_vault: *accounts.curve_vault.key,
            metadata: *accounts.metadata.key,
            governance_authority: *accounts.governance_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_metadata_program: *accounts.token_metadata_program.key,
            rent: *accounts.rent.key,
            launch_name_tracker: *accounts.launch_name_tracker.key,
            launch_symbol_tracker: *accounts.launch_symbol_tracker.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            launch_pair_whitelist: *accounts.launch_pair_whitelist.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateWithBypassKeys> for [AccountMeta; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateWithBypassKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_cooldown,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.governance_authority,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.launch_name_tracker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.launch_symbol_tracker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.launch_pair_whitelist,
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
impl From<[Pubkey; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN]> for CreateWithBypassKeys {
    fn from(pubkeys: [Pubkey; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            creator_cooldown: pubkeys[3],
            mint: pubkeys[4],
            bonding_curve: pubkeys[5],
            creator_vault: pubkeys[6],
            curve_vault: pubkeys[7],
            metadata: pubkeys[8],
            governance_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            token_metadata_program: pubkeys[13],
            rent: pubkeys[14],
            launch_name_tracker: pubkeys[15],
            launch_symbol_tracker: pubkeys[16],
            protocol_fee_recipient: pubkeys[17],
            launch_pair_whitelist: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
        }
    }
}
impl<'info> From<CreateWithBypassAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateWithBypassAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.creator_cooldown.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.creator_vault.clone(),
            accounts.curve_vault.clone(),
            accounts.metadata.clone(),
            accounts.governance_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_metadata_program.clone(),
            accounts.rent.clone(),
            accounts.launch_name_tracker.clone(),
            accounts.launch_symbol_tracker.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.launch_pair_whitelist.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN]>
for CreateWithBypassAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            creator_cooldown: &arr[3],
            mint: &arr[4],
            bonding_curve: &arr[5],
            creator_vault: &arr[6],
            curve_vault: &arr[7],
            metadata: &arr[8],
            governance_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            token_metadata_program: &arr[13],
            rent: &arr[14],
            launch_name_tracker: &arr[15],
            launch_symbol_tracker: &arr[16],
            protocol_fee_recipient: &arr[17],
            launch_pair_whitelist: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
        }
    }
}
pub const CREATE_WITH_BYPASS_IX_DISCM: [u8; 8usize] = [92, 5, 53, 28, 27, 108, 61, 87];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateWithBypassIxArgs {
    pub params: CreateParams,
    pub max_bypass_fee_lamports: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateWithBypassIxData(pub CreateWithBypassIxArgs);
impl From<CreateWithBypassIxArgs> for CreateWithBypassIxData {
    fn from(args: CreateWithBypassIxArgs) -> Self {
        Self(args)
    }
}
impl CreateWithBypassIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_WITH_BYPASS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <CreateParams>::deserialize(&mut reader)?
        };
        let max_bypass_fee_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(CreateWithBypassIxArgs {
                params,
                max_bypass_fee_lamports,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_WITH_BYPASS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.0.max_bypass_fee_lamports, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_with_bypass_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateWithBypassKeys,
    args: CreateWithBypassIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_WITH_BYPASS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateWithBypassIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_with_bypass_ix(
    keys: CreateWithBypassKeys,
    args: CreateWithBypassIxArgs,
) -> std::io::Result<Instruction> {
    create_with_bypass_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn create_with_bypass_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateWithBypassAccounts<'_, '_>,
    args: CreateWithBypassIxArgs,
) -> ProgramResult {
    let keys: CreateWithBypassKeys = accounts.into();
    let ix = create_with_bypass_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_with_bypass_invoke(
    accounts: CreateWithBypassAccounts<'_, '_>,
    args: CreateWithBypassIxArgs,
) -> ProgramResult {
    create_with_bypass_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn create_with_bypass_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateWithBypassAccounts<'_, '_>,
    args: CreateWithBypassIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateWithBypassKeys = accounts.into();
    let ix = create_with_bypass_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_with_bypass_invoke_signed(
    accounts: CreateWithBypassAccounts<'_, '_>,
    args: CreateWithBypassIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_with_bypass_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_with_bypass_verify_account_keys(
    accounts: CreateWithBypassAccounts<'_, '_>,
    keys: CreateWithBypassKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.creator_cooldown.key, keys.creator_cooldown),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.curve_vault.key, keys.curve_vault),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.governance_authority.key, keys.governance_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_metadata_program.key, keys.token_metadata_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.launch_name_tracker.key, keys.launch_name_tracker),
        (*accounts.launch_symbol_tracker.key, keys.launch_symbol_tracker),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.launch_pair_whitelist.key, keys.launch_pair_whitelist),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_with_bypass_verify_writable_privileges<'me, 'info>(
    accounts: CreateWithBypassAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.creator_cooldown,
        accounts.mint,
        accounts.bonding_curve,
        accounts.creator_vault,
        accounts.curve_vault,
        accounts.metadata,
        accounts.launch_name_tracker,
        accounts.launch_symbol_tracker,
        accounts.protocol_fee_recipient,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_with_bypass_verify_signer_privileges<'me, 'info>(
    accounts: CreateWithBypassAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator, accounts.authority, accounts.mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_with_bypass_verify_account_privileges<'me, 'info>(
    accounts: CreateWithBypassAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_with_bypass_verify_writable_privileges(accounts)?;
    create_with_bypass_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeConfigAccounts<'me, 'info> {
    pub initializer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeConfigKeys {
    pub initializer: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeConfigAccounts<'_, '_>> for InitializeConfigKeys {
    fn from(accounts: InitializeConfigAccounts) -> Self {
        Self {
            initializer: *accounts.initializer.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeConfigKeys> for [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.initializer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]> for InitializeConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            initializer: pubkeys[0],
            config: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.initializer.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            initializer: &arr[0],
            config: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const INITIALIZE_CONFIG_IX_DISCM: [u8; 8usize] = [
    208, 127, 21, 1, 194, 190, 196, 70,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeConfigIxArgs {
    pub params: InitializeConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfigIxData(pub InitializeConfigIxArgs);
impl From<InitializeConfigIxArgs> for InitializeConfigIxData {
    fn from(args: InitializeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeConfigParams>::deserialize(&mut reader)?
        };
        Ok(Self(InitializeConfigIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_config_ix(
    keys: InitializeConfigKeys,
    args: InitializeConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_config_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn initialize_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_config_invoke(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
) -> ProgramResult {
    initialize_config_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn initialize_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeConfigKeys = accounts.into();
    let ix = initialize_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_config_invoke_signed(
    accounts: InitializeConfigAccounts<'_, '_>,
    args: InitializeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_config_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_config_verify_account_keys(
    accounts: InitializeConfigAccounts<'_, '_>,
    keys: InitializeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.initializer.key, keys.initializer),
        (*accounts.config.key, keys.config),
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
pub fn initialize_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.initializer, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.initializer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_config_verify_writable_privileges(accounts)?;
    initialize_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMigrationConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub migration_config: &'me AccountInfo<'info>,
    pub raydium_program: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub create_pool_fee_receiver: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeMigrationConfigKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub migration_config: Pubkey,
    pub raydium_program: Pubkey,
    pub amm_config: Pubkey,
    pub permission: Pubkey,
    pub wsol_mint: Pubkey,
    pub create_pool_fee_receiver: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeMigrationConfigAccounts<'_, '_>> for InitializeMigrationConfigKeys {
    fn from(accounts: InitializeMigrationConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            migration_config: *accounts.migration_config.key,
            raydium_program: *accounts.raydium_program.key,
            amm_config: *accounts.amm_config.key,
            permission: *accounts.permission.key,
            wsol_mint: *accounts.wsol_mint.key,
            create_pool_fee_receiver: *accounts.create_pool_fee_receiver.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeMigrationConfigKeys>
for [AccountMeta; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMigrationConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.create_pool_fee_receiver,
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
impl From<[Pubkey; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeMigrationConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            migration_config: pubkeys[2],
            raydium_program: pubkeys[3],
            amm_config: pubkeys[4],
            permission: pubkeys[5],
            wsol_mint: pubkeys[6],
            create_pool_fee_receiver: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeMigrationConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeMigrationConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.migration_config.clone(),
            accounts.raydium_program.clone(),
            accounts.amm_config.clone(),
            accounts.permission.clone(),
            accounts.wsol_mint.clone(),
            accounts.create_pool_fee_receiver.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeMigrationConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            migration_config: &arr[2],
            raydium_program: &arr[3],
            amm_config: &arr[4],
            permission: &arr[5],
            wsol_mint: &arr[6],
            create_pool_fee_receiver: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const INITIALIZE_MIGRATION_CONFIG_IX_DISCM: [u8; 8usize] = [
    149, 51, 166, 5, 251, 88, 134, 41,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeMigrationConfigIxArgs {
    pub params: InitializeMigrationConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMigrationConfigIxData(pub InitializeMigrationConfigIxArgs);
impl From<InitializeMigrationConfigIxArgs> for InitializeMigrationConfigIxData {
    fn from(args: InitializeMigrationConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMigrationConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_MIGRATION_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeMigrationConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeMigrationConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_MIGRATION_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_migration_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeMigrationConfigKeys,
    args: InitializeMigrationConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMigrationConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_migration_config_ix(
    keys: InitializeMigrationConfigKeys,
    args: InitializeMigrationConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_migration_config_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn initialize_migration_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMigrationConfigAccounts<'_, '_>,
    args: InitializeMigrationConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeMigrationConfigKeys = accounts.into();
    let ix = initialize_migration_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_migration_config_invoke(
    accounts: InitializeMigrationConfigAccounts<'_, '_>,
    args: InitializeMigrationConfigIxArgs,
) -> ProgramResult {
    initialize_migration_config_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn initialize_migration_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMigrationConfigAccounts<'_, '_>,
    args: InitializeMigrationConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMigrationConfigKeys = accounts.into();
    let ix = initialize_migration_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_migration_config_invoke_signed(
    accounts: InitializeMigrationConfigAccounts<'_, '_>,
    args: InitializeMigrationConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_migration_config_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_migration_config_verify_account_keys(
    accounts: InitializeMigrationConfigAccounts<'_, '_>,
    keys: InitializeMigrationConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.migration_config.key, keys.migration_config),
        (*accounts.raydium_program.key, keys.raydium_program),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.permission.key, keys.permission),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.create_pool_fee_receiver.key, keys.create_pool_fee_receiver),
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
pub fn initialize_migration_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMigrationConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.migration_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_migration_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeMigrationConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_migration_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeMigrationConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_migration_config_verify_writable_privileges(accounts)?;
    initialize_migration_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeReferralsAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub trader_referral: &'me AccountInfo<'info>,
    pub referrer_profile: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeReferralsKeys {
    pub user: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub trader_referral: Pubkey,
    pub referrer_profile: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeReferralsAccounts<'_, '_>> for InitializeReferralsKeys {
    fn from(accounts: InitializeReferralsAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            trader_referral: *accounts.trader_referral.key,
            referrer_profile: *accounts.referrer_profile.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeReferralsKeys>
for [AccountMeta; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeReferralsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trader_referral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_profile,
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
impl From<[Pubkey; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN]> for InitializeReferralsKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            trader_referral: pubkeys[3],
            referrer_profile: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializeReferralsAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeReferralsAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.trader_referral.clone(),
            accounts.referrer_profile.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN]>
for InitializeReferralsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            trader_referral: &arr[3],
            referrer_profile: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const INITIALIZE_REFERRALS_IX_DISCM: [u8; 8usize] = [
    78, 170, 246, 128, 248, 1, 149, 26,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeReferralsIxArgs {
    pub params: InitializeReferralsParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeReferralsIxData(pub InitializeReferralsIxArgs);
impl From<InitializeReferralsIxArgs> for InitializeReferralsIxData {
    fn from(args: InitializeReferralsIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeReferralsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REFERRALS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeReferralsParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeReferralsIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REFERRALS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_referrals_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeReferralsKeys,
    args: InitializeReferralsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REFERRALS_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeReferralsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_referrals_ix(
    keys: InitializeReferralsKeys,
    args: InitializeReferralsIxArgs,
) -> std::io::Result<Instruction> {
    initialize_referrals_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn initialize_referrals_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeReferralsAccounts<'_, '_>,
    args: InitializeReferralsIxArgs,
) -> ProgramResult {
    let keys: InitializeReferralsKeys = accounts.into();
    let ix = initialize_referrals_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_referrals_invoke(
    accounts: InitializeReferralsAccounts<'_, '_>,
    args: InitializeReferralsIxArgs,
) -> ProgramResult {
    initialize_referrals_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn initialize_referrals_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeReferralsAccounts<'_, '_>,
    args: InitializeReferralsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeReferralsKeys = accounts.into();
    let ix = initialize_referrals_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_referrals_invoke_signed(
    accounts: InitializeReferralsAccounts<'_, '_>,
    args: InitializeReferralsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_referrals_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_referrals_verify_account_keys(
    accounts: InitializeReferralsAccounts<'_, '_>,
    keys: InitializeReferralsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.trader_referral.key, keys.trader_referral),
        (*accounts.referrer_profile.key, keys.referrer_profile),
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
pub fn initialize_referrals_verify_writable_privileges<'me, 'info>(
    accounts: InitializeReferralsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.trader_referral,
        accounts.referrer_profile,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_referrals_verify_signer_privileges<'me, 'info>(
    accounts: InitializeReferralsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_referrals_verify_account_privileges<'me, 'info>(
    accounts: InitializeReferralsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_referrals_verify_writable_privileges(accounts)?;
    initialize_referrals_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRevenueConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRevenueConfigKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub revenue_config: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeRevenueConfigAccounts<'_, '_>> for InitializeRevenueConfigKeys {
    fn from(accounts: InitializeRevenueConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            revenue_config: *accounts.revenue_config.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeRevenueConfigKeys>
for [AccountMeta; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRevenueConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
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
impl From<[Pubkey; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeRevenueConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            revenue_config: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<InitializeRevenueConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRevenueConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.revenue_config.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN]>
for InitializeRevenueConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            revenue_config: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const INITIALIZE_REVENUE_CONFIG_IX_DISCM: [u8; 8usize] = [
    187, 67, 52, 161, 14, 137, 109, 6,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeRevenueConfigIxArgs {
    pub params: RevenueConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRevenueConfigIxData(pub InitializeRevenueConfigIxArgs);
impl From<InitializeRevenueConfigIxArgs> for InitializeRevenueConfigIxData {
    fn from(args: InitializeRevenueConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeRevenueConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REVENUE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RevenueConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeRevenueConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REVENUE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_revenue_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeRevenueConfigKeys,
    args: InitializeRevenueConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REVENUE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeRevenueConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_revenue_config_ix(
    keys: InitializeRevenueConfigKeys,
    args: InitializeRevenueConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_revenue_config_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn initialize_revenue_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRevenueConfigAccounts<'_, '_>,
    args: InitializeRevenueConfigIxArgs,
) -> ProgramResult {
    let keys: InitializeRevenueConfigKeys = accounts.into();
    let ix = initialize_revenue_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_revenue_config_invoke(
    accounts: InitializeRevenueConfigAccounts<'_, '_>,
    args: InitializeRevenueConfigIxArgs,
) -> ProgramResult {
    initialize_revenue_config_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn initialize_revenue_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRevenueConfigAccounts<'_, '_>,
    args: InitializeRevenueConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRevenueConfigKeys = accounts.into();
    let ix = initialize_revenue_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_revenue_config_invoke_signed(
    accounts: InitializeRevenueConfigAccounts<'_, '_>,
    args: InitializeRevenueConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_revenue_config_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_revenue_config_verify_account_keys(
    accounts: InitializeRevenueConfigAccounts<'_, '_>,
    keys: InitializeRevenueConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.revenue_config.key, keys.revenue_config),
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
pub fn initialize_revenue_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRevenueConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.revenue_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_revenue_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRevenueConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_revenue_config_verify_account_privileges<'me, 'info>(
    accounts: InitializeRevenueConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_revenue_config_verify_writable_privileges(accounts)?;
    initialize_revenue_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeTraderReferralAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub trader_referral: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeTraderReferralKeys {
    pub user: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub trader_referral: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeTraderReferralAccounts<'_, '_>> for InitializeTraderReferralKeys {
    fn from(accounts: InitializeTraderReferralAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            trader_referral: *accounts.trader_referral.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeTraderReferralKeys>
for [AccountMeta; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeTraderReferralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trader_referral,
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
impl From<[Pubkey; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN]>
for InitializeTraderReferralKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            trader_referral: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<InitializeTraderReferralAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeTraderReferralAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.trader_referral.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN]>
for InitializeTraderReferralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            trader_referral: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const INITIALIZE_TRADER_REFERRAL_IX_DISCM: [u8; 8usize] = [
    66, 172, 55, 221, 201, 210, 218, 35,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeTraderReferralIxArgs {
    pub params: InitializeTraderReferralParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTraderReferralIxData(pub InitializeTraderReferralIxArgs);
impl From<InitializeTraderReferralIxArgs> for InitializeTraderReferralIxData {
    fn from(args: InitializeTraderReferralIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeTraderReferralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_TRADER_REFERRAL_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <InitializeTraderReferralParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(InitializeTraderReferralIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_TRADER_REFERRAL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_trader_referral_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeTraderReferralKeys,
    args: InitializeTraderReferralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_TRADER_REFERRAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeTraderReferralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_trader_referral_ix(
    keys: InitializeTraderReferralKeys,
    args: InitializeTraderReferralIxArgs,
) -> std::io::Result<Instruction> {
    initialize_trader_referral_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn initialize_trader_referral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTraderReferralAccounts<'_, '_>,
    args: InitializeTraderReferralIxArgs,
) -> ProgramResult {
    let keys: InitializeTraderReferralKeys = accounts.into();
    let ix = initialize_trader_referral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_trader_referral_invoke(
    accounts: InitializeTraderReferralAccounts<'_, '_>,
    args: InitializeTraderReferralIxArgs,
) -> ProgramResult {
    initialize_trader_referral_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn initialize_trader_referral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeTraderReferralAccounts<'_, '_>,
    args: InitializeTraderReferralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeTraderReferralKeys = accounts.into();
    let ix = initialize_trader_referral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_trader_referral_invoke_signed(
    accounts: InitializeTraderReferralAccounts<'_, '_>,
    args: InitializeTraderReferralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_trader_referral_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_trader_referral_verify_account_keys(
    accounts: InitializeTraderReferralAccounts<'_, '_>,
    keys: InitializeTraderReferralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.trader_referral.key, keys.trader_referral),
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
pub fn initialize_trader_referral_verify_writable_privileges<'me, 'info>(
    accounts: InitializeTraderReferralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.trader_referral] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_trader_referral_verify_signer_privileges<'me, 'info>(
    accounts: InitializeTraderReferralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_trader_referral_verify_account_privileges<'me, 'info>(
    accounts: InitializeTraderReferralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_trader_referral_verify_writable_privileges(accounts)?;
    initialize_trader_referral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_IX_ACCOUNTS_LEN: usize = 28;
#[derive(Copy, Clone, Debug)]
pub struct MigrateAccounts<'me, 'info> {
    pub executor: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub migration_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub curve_vault: &'me AccountInfo<'info>,
    pub migration_state: &'me AccountInfo<'info>,
    pub migration_sol_staging: &'me AccountInfo<'info>,
    pub revenue_authority: &'me AccountInfo<'info>,
    pub migration_token_account: &'me AccountInfo<'info>,
    pub migration_wsol_account: &'me AccountInfo<'info>,
    pub raydium_program: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub raydium_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer_lp_token: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub observation: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub create_pool_fee_receiver: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateKeys {
    pub executor: Pubkey,
    pub config: Pubkey,
    pub migration_config: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub curve_vault: Pubkey,
    pub migration_state: Pubkey,
    pub migration_sol_staging: Pubkey,
    pub revenue_authority: Pubkey,
    pub migration_token_account: Pubkey,
    pub migration_wsol_account: Pubkey,
    pub raydium_program: Pubkey,
    pub amm_config: Pubkey,
    pub raydium_authority: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub payer_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub observation: Pubkey,
    pub permission: Pubkey,
    pub wsol_mint: Pubkey,
    pub create_pool_fee_receiver: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigrateAccounts<'_, '_>> for MigrateKeys {
    fn from(accounts: MigrateAccounts) -> Self {
        Self {
            executor: *accounts.executor.key,
            config: *accounts.config.key,
            migration_config: *accounts.migration_config.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            curve_vault: *accounts.curve_vault.key,
            migration_state: *accounts.migration_state.key,
            migration_sol_staging: *accounts.migration_sol_staging.key,
            revenue_authority: *accounts.revenue_authority.key,
            migration_token_account: *accounts.migration_token_account.key,
            migration_wsol_account: *accounts.migration_wsol_account.key,
            raydium_program: *accounts.raydium_program.key,
            amm_config: *accounts.amm_config.key,
            raydium_authority: *accounts.raydium_authority.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            payer_lp_token: *accounts.payer_lp_token.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            observation: *accounts.observation.key,
            permission: *accounts.permission.key,
            wsol_mint: *accounts.wsol_mint.key,
            create_pool_fee_receiver: *accounts.create_pool_fee_receiver.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateKeys> for [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.executor,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_config,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.migration_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.migration_sol_staging,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.migration_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_lp_token,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.observation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.permission,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.create_pool_fee_receiver,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; MIGRATE_IX_ACCOUNTS_LEN]> for MigrateKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            executor: pubkeys[0],
            config: pubkeys[1],
            migration_config: pubkeys[2],
            mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            curve_vault: pubkeys[5],
            migration_state: pubkeys[6],
            migration_sol_staging: pubkeys[7],
            revenue_authority: pubkeys[8],
            migration_token_account: pubkeys[9],
            migration_wsol_account: pubkeys[10],
            raydium_program: pubkeys[11],
            amm_config: pubkeys[12],
            raydium_authority: pubkeys[13],
            pool: pubkeys[14],
            lp_mint: pubkeys[15],
            payer_lp_token: pubkeys[16],
            token_0_vault: pubkeys[17],
            token_1_vault: pubkeys[18],
            observation: pubkeys[19],
            permission: pubkeys[20],
            wsol_mint: pubkeys[21],
            create_pool_fee_receiver: pubkeys[22],
            token_program: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
            event_authority: pubkeys[26],
            program: pubkeys[27],
        }
    }
}
impl<'info> From<MigrateAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateAccounts<'_, 'info>) -> Self {
        [
            accounts.executor.clone(),
            accounts.config.clone(),
            accounts.migration_config.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.curve_vault.clone(),
            accounts.migration_state.clone(),
            accounts.migration_sol_staging.clone(),
            accounts.revenue_authority.clone(),
            accounts.migration_token_account.clone(),
            accounts.migration_wsol_account.clone(),
            accounts.raydium_program.clone(),
            accounts.amm_config.clone(),
            accounts.raydium_authority.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.payer_lp_token.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.observation.clone(),
            accounts.permission.clone(),
            accounts.wsol_mint.clone(),
            accounts.create_pool_fee_receiver.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]>
for MigrateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            executor: &arr[0],
            config: &arr[1],
            migration_config: &arr[2],
            mint: &arr[3],
            bonding_curve: &arr[4],
            curve_vault: &arr[5],
            migration_state: &arr[6],
            migration_sol_staging: &arr[7],
            revenue_authority: &arr[8],
            migration_token_account: &arr[9],
            migration_wsol_account: &arr[10],
            raydium_program: &arr[11],
            amm_config: &arr[12],
            raydium_authority: &arr[13],
            pool: &arr[14],
            lp_mint: &arr[15],
            payer_lp_token: &arr[16],
            token_0_vault: &arr[17],
            token_1_vault: &arr[18],
            observation: &arr[19],
            permission: &arr[20],
            wsol_mint: &arr[21],
            create_pool_fee_receiver: &arr[22],
            token_program: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
            event_authority: &arr[26],
            program: &arr[27],
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
    migrate_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
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
    migrate_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
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
    migrate_invoke_signed_with_program_id(TRENCH_PROGRAM_ID, accounts, seeds)
}
pub fn migrate_verify_account_keys(
    accounts: MigrateAccounts<'_, '_>,
    keys: MigrateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.executor.key, keys.executor),
        (*accounts.config.key, keys.config),
        (*accounts.migration_config.key, keys.migration_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.curve_vault.key, keys.curve_vault),
        (*accounts.migration_state.key, keys.migration_state),
        (*accounts.migration_sol_staging.key, keys.migration_sol_staging),
        (*accounts.revenue_authority.key, keys.revenue_authority),
        (*accounts.migration_token_account.key, keys.migration_token_account),
        (*accounts.migration_wsol_account.key, keys.migration_wsol_account),
        (*accounts.raydium_program.key, keys.raydium_program),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.raydium_authority.key, keys.raydium_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer_lp_token.key, keys.payer_lp_token),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.observation.key, keys.observation),
        (*accounts.permission.key, keys.permission),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.create_pool_fee_receiver.key, keys.create_pool_fee_receiver),
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
pub fn migrate_verify_writable_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.executor,
        accounts.bonding_curve,
        accounts.curve_vault,
        accounts.migration_state,
        accounts.migration_sol_staging,
        accounts.migration_token_account,
        accounts.migration_wsol_account,
        accounts.pool,
        accounts.lp_mint,
        accounts.payer_lp_token,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.observation,
        accounts.create_pool_fee_receiver,
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
    for should_be_signer in [accounts.executor] {
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
pub const MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct MigrateTradeAuthorityWhitelistAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateTradeAuthorityWhitelistKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigrateTradeAuthorityWhitelistAccounts<'_, '_>>
for MigrateTradeAuthorityWhitelistKeys {
    fn from(accounts: MigrateTradeAuthorityWhitelistAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateTradeAuthorityWhitelistKeys>
for [AccountMeta; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateTradeAuthorityWhitelistKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN]>
for MigrateTradeAuthorityWhitelistKeys {
    fn from(
        pubkeys: [Pubkey; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<MigrateTradeAuthorityWhitelistAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateTradeAuthorityWhitelistAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN]>
for MigrateTradeAuthorityWhitelistAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_DISCM: [u8; 8usize] = [
    131, 21, 57, 20, 165, 56, 205, 123,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateTradeAuthorityWhitelistIxData;
impl MigrateTradeAuthorityWhitelistIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_trade_authority_whitelist_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateTradeAuthorityWhitelistKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_TRADE_AUTHORITY_WHITELIST_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateTradeAuthorityWhitelistIxData.try_to_vec()?,
    })
}
pub fn migrate_trade_authority_whitelist_ix(
    keys: MigrateTradeAuthorityWhitelistKeys,
) -> std::io::Result<Instruction> {
    migrate_trade_authority_whitelist_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
}
pub fn migrate_trade_authority_whitelist_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateTradeAuthorityWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateTradeAuthorityWhitelistKeys = accounts.into();
    let ix = migrate_trade_authority_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_trade_authority_whitelist_invoke(
    accounts: MigrateTradeAuthorityWhitelistAccounts<'_, '_>,
) -> ProgramResult {
    migrate_trade_authority_whitelist_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
}
pub fn migrate_trade_authority_whitelist_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateTradeAuthorityWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateTradeAuthorityWhitelistKeys = accounts.into();
    let ix = migrate_trade_authority_whitelist_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_trade_authority_whitelist_invoke_signed(
    accounts: MigrateTradeAuthorityWhitelistAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_trade_authority_whitelist_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_trade_authority_whitelist_verify_account_keys(
    accounts: MigrateTradeAuthorityWhitelistAccounts<'_, '_>,
    keys: MigrateTradeAuthorityWhitelistKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
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
pub fn migrate_trade_authority_whitelist_verify_writable_privileges<'me, 'info>(
    accounts: MigrateTradeAuthorityWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_trade_authority_whitelist_verify_signer_privileges<'me, 'info>(
    accounts: MigrateTradeAuthorityWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_trade_authority_whitelist_verify_account_privileges<'me, 'info>(
    accounts: MigrateTradeAuthorityWhitelistAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_trade_authority_whitelist_verify_writable_privileges(accounts)?;
    migrate_trade_authority_whitelist_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct PayoutCreatorRevenueAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub creator_revenue_state: &'me AccountInfo<'info>,
    pub legacy_creator_vault: &'me AccountInfo<'info>,
    pub revenue_authority: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub revenue_wsol_account: &'me AccountInfo<'info>,
    pub temporary_wsol_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayoutCreatorRevenueKeys {
    pub payer: Pubkey,
    pub revenue_config: Pubkey,
    pub mint: Pubkey,
    pub creator_revenue_state: Pubkey,
    pub legacy_creator_vault: Pubkey,
    pub revenue_authority: Pubkey,
    pub wsol_mint: Pubkey,
    pub revenue_wsol_account: Pubkey,
    pub temporary_wsol_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PayoutCreatorRevenueAccounts<'_, '_>> for PayoutCreatorRevenueKeys {
    fn from(accounts: PayoutCreatorRevenueAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            revenue_config: *accounts.revenue_config.key,
            mint: *accounts.mint.key,
            creator_revenue_state: *accounts.creator_revenue_state.key,
            legacy_creator_vault: *accounts.legacy_creator_vault.key,
            revenue_authority: *accounts.revenue_authority.key,
            wsol_mint: *accounts.wsol_mint.key,
            revenue_wsol_account: *accounts.revenue_wsol_account.key,
            temporary_wsol_account: *accounts.temporary_wsol_account.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PayoutCreatorRevenueKeys>
for [AccountMeta; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(keys: PayoutCreatorRevenueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_revenue_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.legacy_creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.temporary_wsol_account,
                is_signer: false,
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
impl From<[Pubkey; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN]>
for PayoutCreatorRevenueKeys {
    fn from(pubkeys: [Pubkey; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            revenue_config: pubkeys[1],
            mint: pubkeys[2],
            creator_revenue_state: pubkeys[3],
            legacy_creator_vault: pubkeys[4],
            revenue_authority: pubkeys[5],
            wsol_mint: pubkeys[6],
            revenue_wsol_account: pubkeys[7],
            temporary_wsol_account: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<PayoutCreatorRevenueAccounts<'_, 'info>>
for [AccountInfo<'info>; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PayoutCreatorRevenueAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.revenue_config.clone(),
            accounts.mint.clone(),
            accounts.creator_revenue_state.clone(),
            accounts.legacy_creator_vault.clone(),
            accounts.revenue_authority.clone(),
            accounts.wsol_mint.clone(),
            accounts.revenue_wsol_account.clone(),
            accounts.temporary_wsol_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN]>
for PayoutCreatorRevenueAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            revenue_config: &arr[1],
            mint: &arr[2],
            creator_revenue_state: &arr[3],
            legacy_creator_vault: &arr[4],
            revenue_authority: &arr[5],
            wsol_mint: &arr[6],
            revenue_wsol_account: &arr[7],
            temporary_wsol_account: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const PAYOUT_CREATOR_REVENUE_IX_DISCM: [u8; 8usize] = [
    136, 98, 210, 189, 153, 216, 83, 164,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayoutCreatorRevenueIxArgs {
    pub params: PayoutCreatorRevenueParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayoutCreatorRevenueIxData(pub PayoutCreatorRevenueIxArgs);
impl From<PayoutCreatorRevenueIxArgs> for PayoutCreatorRevenueIxData {
    fn from(args: PayoutCreatorRevenueIxArgs) -> Self {
        Self(args)
    }
}
impl PayoutCreatorRevenueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAYOUT_CREATOR_REVENUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <PayoutCreatorRevenueParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(PayoutCreatorRevenueIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAYOUT_CREATOR_REVENUE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn payout_creator_revenue_ix_with_program_id(
    program_id: Pubkey,
    keys: PayoutCreatorRevenueKeys,
    args: PayoutCreatorRevenueIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAYOUT_CREATOR_REVENUE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PayoutCreatorRevenueIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn payout_creator_revenue_ix(
    keys: PayoutCreatorRevenueKeys,
    args: PayoutCreatorRevenueIxArgs,
) -> std::io::Result<Instruction> {
    payout_creator_revenue_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn payout_creator_revenue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PayoutCreatorRevenueAccounts<'_, '_>,
    args: PayoutCreatorRevenueIxArgs,
) -> ProgramResult {
    let keys: PayoutCreatorRevenueKeys = accounts.into();
    let ix = payout_creator_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn payout_creator_revenue_invoke(
    accounts: PayoutCreatorRevenueAccounts<'_, '_>,
    args: PayoutCreatorRevenueIxArgs,
) -> ProgramResult {
    payout_creator_revenue_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn payout_creator_revenue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PayoutCreatorRevenueAccounts<'_, '_>,
    args: PayoutCreatorRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PayoutCreatorRevenueKeys = accounts.into();
    let ix = payout_creator_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn payout_creator_revenue_invoke_signed(
    accounts: PayoutCreatorRevenueAccounts<'_, '_>,
    args: PayoutCreatorRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    payout_creator_revenue_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn payout_creator_revenue_verify_account_keys(
    accounts: PayoutCreatorRevenueAccounts<'_, '_>,
    keys: PayoutCreatorRevenueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.revenue_config.key, keys.revenue_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.creator_revenue_state.key, keys.creator_revenue_state),
        (*accounts.legacy_creator_vault.key, keys.legacy_creator_vault),
        (*accounts.revenue_authority.key, keys.revenue_authority),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.revenue_wsol_account.key, keys.revenue_wsol_account),
        (*accounts.temporary_wsol_account.key, keys.temporary_wsol_account),
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
pub fn payout_creator_revenue_verify_writable_privileges<'me, 'info>(
    accounts: PayoutCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.creator_revenue_state,
        accounts.legacy_creator_vault,
        accounts.revenue_wsol_account,
        accounts.temporary_wsol_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn payout_creator_revenue_verify_signer_privileges<'me, 'info>(
    accounts: PayoutCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn payout_creator_revenue_verify_account_privileges<'me, 'info>(
    accounts: PayoutCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    payout_creator_revenue_verify_writable_privileges(accounts)?;
    payout_creator_revenue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct PayoutProtocolRevenueAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub creator_revenue_state: &'me AccountInfo<'info>,
    pub protocol_recipient: &'me AccountInfo<'info>,
    pub revenue_authority: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub revenue_wsol_account: &'me AccountInfo<'info>,
    pub temporary_wsol_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayoutProtocolRevenueKeys {
    pub payer: Pubkey,
    pub revenue_config: Pubkey,
    pub mint: Pubkey,
    pub creator_revenue_state: Pubkey,
    pub protocol_recipient: Pubkey,
    pub revenue_authority: Pubkey,
    pub wsol_mint: Pubkey,
    pub revenue_wsol_account: Pubkey,
    pub temporary_wsol_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PayoutProtocolRevenueAccounts<'_, '_>> for PayoutProtocolRevenueKeys {
    fn from(accounts: PayoutProtocolRevenueAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            revenue_config: *accounts.revenue_config.key,
            mint: *accounts.mint.key,
            creator_revenue_state: *accounts.creator_revenue_state.key,
            protocol_recipient: *accounts.protocol_recipient.key,
            revenue_authority: *accounts.revenue_authority.key,
            wsol_mint: *accounts.wsol_mint.key,
            revenue_wsol_account: *accounts.revenue_wsol_account.key,
            temporary_wsol_account: *accounts.temporary_wsol_account.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PayoutProtocolRevenueKeys>
for [AccountMeta; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(keys: PayoutProtocolRevenueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_revenue_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_wsol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.temporary_wsol_account,
                is_signer: false,
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
impl From<[Pubkey; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN]>
for PayoutProtocolRevenueKeys {
    fn from(pubkeys: [Pubkey; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            revenue_config: pubkeys[1],
            mint: pubkeys[2],
            creator_revenue_state: pubkeys[3],
            protocol_recipient: pubkeys[4],
            revenue_authority: pubkeys[5],
            wsol_mint: pubkeys[6],
            revenue_wsol_account: pubkeys[7],
            temporary_wsol_account: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<PayoutProtocolRevenueAccounts<'_, 'info>>
for [AccountInfo<'info>; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PayoutProtocolRevenueAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.revenue_config.clone(),
            accounts.mint.clone(),
            accounts.creator_revenue_state.clone(),
            accounts.protocol_recipient.clone(),
            accounts.revenue_authority.clone(),
            accounts.wsol_mint.clone(),
            accounts.revenue_wsol_account.clone(),
            accounts.temporary_wsol_account.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN]>
for PayoutProtocolRevenueAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            revenue_config: &arr[1],
            mint: &arr[2],
            creator_revenue_state: &arr[3],
            protocol_recipient: &arr[4],
            revenue_authority: &arr[5],
            wsol_mint: &arr[6],
            revenue_wsol_account: &arr[7],
            temporary_wsol_account: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const PAYOUT_PROTOCOL_REVENUE_IX_DISCM: [u8; 8usize] = [
    148, 103, 216, 170, 174, 251, 100, 126,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayoutProtocolRevenueIxArgs {
    pub params: PayoutProtocolRevenueParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayoutProtocolRevenueIxData(pub PayoutProtocolRevenueIxArgs);
impl From<PayoutProtocolRevenueIxArgs> for PayoutProtocolRevenueIxData {
    fn from(args: PayoutProtocolRevenueIxArgs) -> Self {
        Self(args)
    }
}
impl PayoutProtocolRevenueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAYOUT_PROTOCOL_REVENUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <PayoutProtocolRevenueParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(PayoutProtocolRevenueIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAYOUT_PROTOCOL_REVENUE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn payout_protocol_revenue_ix_with_program_id(
    program_id: Pubkey,
    keys: PayoutProtocolRevenueKeys,
    args: PayoutProtocolRevenueIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAYOUT_PROTOCOL_REVENUE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PayoutProtocolRevenueIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn payout_protocol_revenue_ix(
    keys: PayoutProtocolRevenueKeys,
    args: PayoutProtocolRevenueIxArgs,
) -> std::io::Result<Instruction> {
    payout_protocol_revenue_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn payout_protocol_revenue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PayoutProtocolRevenueAccounts<'_, '_>,
    args: PayoutProtocolRevenueIxArgs,
) -> ProgramResult {
    let keys: PayoutProtocolRevenueKeys = accounts.into();
    let ix = payout_protocol_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn payout_protocol_revenue_invoke(
    accounts: PayoutProtocolRevenueAccounts<'_, '_>,
    args: PayoutProtocolRevenueIxArgs,
) -> ProgramResult {
    payout_protocol_revenue_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn payout_protocol_revenue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PayoutProtocolRevenueAccounts<'_, '_>,
    args: PayoutProtocolRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PayoutProtocolRevenueKeys = accounts.into();
    let ix = payout_protocol_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn payout_protocol_revenue_invoke_signed(
    accounts: PayoutProtocolRevenueAccounts<'_, '_>,
    args: PayoutProtocolRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    payout_protocol_revenue_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn payout_protocol_revenue_verify_account_keys(
    accounts: PayoutProtocolRevenueAccounts<'_, '_>,
    keys: PayoutProtocolRevenueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.revenue_config.key, keys.revenue_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.creator_revenue_state.key, keys.creator_revenue_state),
        (*accounts.protocol_recipient.key, keys.protocol_recipient),
        (*accounts.revenue_authority.key, keys.revenue_authority),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.revenue_wsol_account.key, keys.revenue_wsol_account),
        (*accounts.temporary_wsol_account.key, keys.temporary_wsol_account),
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
pub fn payout_protocol_revenue_verify_writable_privileges<'me, 'info>(
    accounts: PayoutProtocolRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.creator_revenue_state,
        accounts.protocol_recipient,
        accounts.revenue_wsol_account,
        accounts.temporary_wsol_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn payout_protocol_revenue_verify_signer_privileges<'me, 'info>(
    accounts: PayoutProtocolRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn payout_protocol_revenue_verify_account_privileges<'me, 'info>(
    accounts: PayoutProtocolRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    payout_protocol_revenue_verify_writable_privileges(accounts)?;
    payout_protocol_revenue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub seller: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub curve_vault: &'me AccountInfo<'info>,
    pub seller_token_account: &'me AccountInfo<'info>,
    pub buyer_limit: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub trader_cashback_vault: &'me AccountInfo<'info>,
    pub trader_referral: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub seller: Pubkey,
    pub config: Pubkey,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub curve_vault: Pubkey,
    pub seller_token_account: Pubkey,
    pub buyer_limit: Pubkey,
    pub creator_vault: Pubkey,
    pub trader_cashback_vault: Pubkey,
    pub trader_referral: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            seller: *accounts.seller.key,
            config: *accounts.config.key,
            authority: *accounts.authority.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            curve_vault: *accounts.curve_vault.key,
            seller_token_account: *accounts.seller_token_account.key,
            buyer_limit: *accounts.buyer_limit.key,
            creator_vault: *accounts.creator_vault.key,
            trader_cashback_vault: *accounts.trader_cashback_vault.key,
            trader_referral: *accounts.trader_referral.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.seller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.curve_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seller_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_limit,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_cashback_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader_referral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
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
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seller: pubkeys[0],
            config: pubkeys[1],
            authority: pubkeys[2],
            mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            curve_vault: pubkeys[5],
            seller_token_account: pubkeys[6],
            buyer_limit: pubkeys[7],
            creator_vault: pubkeys[8],
            trader_cashback_vault: pubkeys[9],
            trader_referral: pubkeys[10],
            protocol_fee_recipient: pubkeys[11],
            token_program: pubkeys[12],
            system_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>>
for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.seller.clone(),
            accounts.config.clone(),
            accounts.authority.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.curve_vault.clone(),
            accounts.seller_token_account.clone(),
            accounts.buyer_limit.clone(),
            accounts.creator_vault.clone(),
            accounts.trader_cashback_vault.clone(),
            accounts.trader_referral.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            seller: &arr[0],
            config: &arr[1],
            authority: &arr[2],
            mint: &arr[3],
            bonding_curve: &arr[4],
            curve_vault: &arr[5],
            seller_token_account: &arr[6],
            buyer_limit: &arr[7],
            creator_vault: &arr[8],
            trader_cashback_vault: &arr[9],
            trader_referral: &arr[10],
            protocol_fee_recipient: &arr[11],
            token_program: &arr[12],
            system_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const SELL_IX_DISCM: [u8; 8usize] = [51, 230, 133, 164, 1, 127, 131, 173];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellIxArgs {
    pub params: SellParams,
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
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SellParams>::deserialize(&mut reader)?
        };
        Ok(Self(SellIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
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
    sell_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
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
    sell_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
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
    sell_invoke_signed_with_program_id(TRENCH_PROGRAM_ID, accounts, args, seeds)
}
pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.seller.key, keys.seller),
        (*accounts.config.key, keys.config),
        (*accounts.authority.key, keys.authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.curve_vault.key, keys.curve_vault),
        (*accounts.seller_token_account.key, keys.seller_token_account),
        (*accounts.buyer_limit.key, keys.buyer_limit),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.trader_cashback_vault.key, keys.trader_cashback_vault),
        (*accounts.trader_referral.key, keys.trader_referral),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
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
pub fn sell_verify_writable_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.seller,
        accounts.bonding_curve,
        accounts.curve_vault,
        accounts.seller_token_account,
        accounts.buyer_limit,
        accounts.creator_vault,
        accounts.trader_cashback_vault,
        accounts.trader_referral,
        accounts.protocol_fee_recipient,
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
    for should_be_signer in [accounts.seller, accounts.authority] {
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
pub const SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SetCreatorRevenueRecipientAccounts<'me, 'info> {
    pub governance_authority: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub creator_revenue_state: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCreatorRevenueRecipientKeys {
    pub governance_authority: Pubkey,
    pub revenue_config: Pubkey,
    pub mint: Pubkey,
    pub creator_revenue_state: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetCreatorRevenueRecipientAccounts<'_, '_>>
for SetCreatorRevenueRecipientKeys {
    fn from(accounts: SetCreatorRevenueRecipientAccounts) -> Self {
        Self {
            governance_authority: *accounts.governance_authority.key,
            revenue_config: *accounts.revenue_config.key,
            mint: *accounts.mint.key,
            creator_revenue_state: *accounts.creator_revenue_state.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetCreatorRevenueRecipientKeys>
for [AccountMeta; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCreatorRevenueRecipientKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governance_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_revenue_state,
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
impl From<[Pubkey; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN]>
for SetCreatorRevenueRecipientKeys {
    fn from(pubkeys: [Pubkey; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governance_authority: pubkeys[0],
            revenue_config: pubkeys[1],
            mint: pubkeys[2],
            creator_revenue_state: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<SetCreatorRevenueRecipientAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCreatorRevenueRecipientAccounts<'_, 'info>) -> Self {
        [
            accounts.governance_authority.clone(),
            accounts.revenue_config.clone(),
            accounts.mint.clone(),
            accounts.creator_revenue_state.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN]>
for SetCreatorRevenueRecipientAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            governance_authority: &arr[0],
            revenue_config: &arr[1],
            mint: &arr[2],
            creator_revenue_state: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const SET_CREATOR_REVENUE_RECIPIENT_IX_DISCM: [u8; 8usize] = [
    66, 159, 63, 47, 31, 117, 210, 170,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetCreatorRevenueRecipientIxArgs {
    pub new_recipient: Pubkey,
    pub expected_recipient_revision: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCreatorRevenueRecipientIxData(pub SetCreatorRevenueRecipientIxArgs);
impl From<SetCreatorRevenueRecipientIxArgs> for SetCreatorRevenueRecipientIxData {
    fn from(args: SetCreatorRevenueRecipientIxArgs) -> Self {
        Self(args)
    }
}
impl SetCreatorRevenueRecipientIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_CREATOR_REVENUE_RECIPIENT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let new_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let expected_recipient_revision: u64 = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(SetCreatorRevenueRecipientIxArgs {
                new_recipient,
                expected_recipient_revision,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CREATOR_REVENUE_RECIPIENT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.new_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.0.expected_recipient_revision,
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
pub fn set_creator_revenue_recipient_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCreatorRevenueRecipientKeys,
    args: SetCreatorRevenueRecipientIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CREATOR_REVENUE_RECIPIENT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetCreatorRevenueRecipientIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_creator_revenue_recipient_ix(
    keys: SetCreatorRevenueRecipientKeys,
    args: SetCreatorRevenueRecipientIxArgs,
) -> std::io::Result<Instruction> {
    set_creator_revenue_recipient_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn set_creator_revenue_recipient_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCreatorRevenueRecipientAccounts<'_, '_>,
    args: SetCreatorRevenueRecipientIxArgs,
) -> ProgramResult {
    let keys: SetCreatorRevenueRecipientKeys = accounts.into();
    let ix = set_creator_revenue_recipient_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_creator_revenue_recipient_invoke(
    accounts: SetCreatorRevenueRecipientAccounts<'_, '_>,
    args: SetCreatorRevenueRecipientIxArgs,
) -> ProgramResult {
    set_creator_revenue_recipient_invoke_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn set_creator_revenue_recipient_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCreatorRevenueRecipientAccounts<'_, '_>,
    args: SetCreatorRevenueRecipientIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetCreatorRevenueRecipientKeys = accounts.into();
    let ix = set_creator_revenue_recipient_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_creator_revenue_recipient_invoke_signed(
    accounts: SetCreatorRevenueRecipientAccounts<'_, '_>,
    args: SetCreatorRevenueRecipientIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_creator_revenue_recipient_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_creator_revenue_recipient_verify_account_keys(
    accounts: SetCreatorRevenueRecipientAccounts<'_, '_>,
    keys: SetCreatorRevenueRecipientKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governance_authority.key, keys.governance_authority),
        (*accounts.revenue_config.key, keys.revenue_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.creator_revenue_state.key, keys.creator_revenue_state),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_creator_revenue_recipient_verify_writable_privileges<'me, 'info>(
    accounts: SetCreatorRevenueRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.creator_revenue_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_creator_revenue_recipient_verify_signer_privileges<'me, 'info>(
    accounts: SetCreatorRevenueRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.governance_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_creator_revenue_recipient_verify_account_privileges<'me, 'info>(
    accounts: SetCreatorRevenueRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_creator_revenue_recipient_verify_writable_privileges(accounts)?;
    set_creator_revenue_recipient_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PARTNER_STATUS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetPartnerStatusAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub referrer_profile: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPartnerStatusKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub referrer_profile: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetPartnerStatusAccounts<'_, '_>> for SetPartnerStatusKeys {
    fn from(accounts: SetPartnerStatusAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            referrer_profile: *accounts.referrer_profile.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetPartnerStatusKeys> for [AccountMeta; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPartnerStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_profile,
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
impl From<[Pubkey; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN]> for SetPartnerStatusKeys {
    fn from(pubkeys: [Pubkey; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            referrer_profile: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<SetPartnerStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPartnerStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.referrer_profile.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN]>
for SetPartnerStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            referrer_profile: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const SET_PARTNER_STATUS_IX_DISCM: [u8; 8usize] = [
    39, 59, 123, 245, 108, 35, 65, 96,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPartnerStatusIxArgs {
    pub params: SetPartnerStatusParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPartnerStatusIxData(pub SetPartnerStatusIxArgs);
impl From<SetPartnerStatusIxArgs> for SetPartnerStatusIxData {
    fn from(args: SetPartnerStatusIxArgs) -> Self {
        Self(args)
    }
}
impl SetPartnerStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PARTNER_STATUS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SetPartnerStatusParams>::deserialize(&mut reader)?
        };
        Ok(Self(SetPartnerStatusIxArgs { params }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PARTNER_STATUS_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_partner_status_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPartnerStatusKeys,
    args: SetPartnerStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PARTNER_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPartnerStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_partner_status_ix(
    keys: SetPartnerStatusKeys,
    args: SetPartnerStatusIxArgs,
) -> std::io::Result<Instruction> {
    set_partner_status_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn set_partner_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPartnerStatusAccounts<'_, '_>,
    args: SetPartnerStatusIxArgs,
) -> ProgramResult {
    let keys: SetPartnerStatusKeys = accounts.into();
    let ix = set_partner_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_partner_status_invoke(
    accounts: SetPartnerStatusAccounts<'_, '_>,
    args: SetPartnerStatusIxArgs,
) -> ProgramResult {
    set_partner_status_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn set_partner_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPartnerStatusAccounts<'_, '_>,
    args: SetPartnerStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPartnerStatusKeys = accounts.into();
    let ix = set_partner_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_partner_status_invoke_signed(
    accounts: SetPartnerStatusAccounts<'_, '_>,
    args: SetPartnerStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_partner_status_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_partner_status_verify_account_keys(
    accounts: SetPartnerStatusAccounts<'_, '_>,
    keys: SetPartnerStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.referrer_profile.key, keys.referrer_profile),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_partner_status_verify_writable_privileges<'me, 'info>(
    accounts: SetPartnerStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.referrer_profile] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_partner_status_verify_signer_privileges<'me, 'info>(
    accounts: SetPartnerStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_partner_status_verify_account_privileges<'me, 'info>(
    accounts: SetPartnerStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_partner_status_verify_writable_privileges(accounts)?;
    set_partner_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct SettleCreatorRevenueAccounts<'me, 'info> {
    pub accounting_authority: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub migration_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub migration_state: &'me AccountInfo<'info>,
    pub creator_revenue_state: &'me AccountInfo<'info>,
    pub revenue_authority: &'me AccountInfo<'info>,
    pub raydium_program: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub raydium_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_0_vault: &'me AccountInfo<'info>,
    pub token_1_vault: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub revenue_token_0: &'me AccountInfo<'info>,
    pub revenue_token_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleCreatorRevenueKeys {
    pub accounting_authority: Pubkey,
    pub revenue_config: Pubkey,
    pub migration_config: Pubkey,
    pub mint: Pubkey,
    pub migration_state: Pubkey,
    pub creator_revenue_state: Pubkey,
    pub revenue_authority: Pubkey,
    pub raydium_program: Pubkey,
    pub amm_config: Pubkey,
    pub raydium_authority: Pubkey,
    pub pool: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub revenue_token_0: Pubkey,
    pub revenue_token_1: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SettleCreatorRevenueAccounts<'_, '_>> for SettleCreatorRevenueKeys {
    fn from(accounts: SettleCreatorRevenueAccounts) -> Self {
        Self {
            accounting_authority: *accounts.accounting_authority.key,
            revenue_config: *accounts.revenue_config.key,
            migration_config: *accounts.migration_config.key,
            mint: *accounts.mint.key,
            migration_state: *accounts.migration_state.key,
            creator_revenue_state: *accounts.creator_revenue_state.key,
            revenue_authority: *accounts.revenue_authority.key,
            raydium_program: *accounts.raydium_program.key,
            amm_config: *accounts.amm_config.key,
            raydium_authority: *accounts.raydium_authority.key,
            pool: *accounts.pool.key,
            token_0_vault: *accounts.token_0_vault.key,
            token_1_vault: *accounts.token_1_vault.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            revenue_token_0: *accounts.revenue_token_0.key,
            revenue_token_1: *accounts.revenue_token_1.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SettleCreatorRevenueKeys>
for [AccountMeta; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(keys: SettleCreatorRevenueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.accounting_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_revenue_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.raydium_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_token_1,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN]>
for SettleCreatorRevenueKeys {
    fn from(pubkeys: [Pubkey; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            accounting_authority: pubkeys[0],
            revenue_config: pubkeys[1],
            migration_config: pubkeys[2],
            mint: pubkeys[3],
            migration_state: pubkeys[4],
            creator_revenue_state: pubkeys[5],
            revenue_authority: pubkeys[6],
            raydium_program: pubkeys[7],
            amm_config: pubkeys[8],
            raydium_authority: pubkeys[9],
            pool: pubkeys[10],
            token_0_vault: pubkeys[11],
            token_1_vault: pubkeys[12],
            token_0_mint: pubkeys[13],
            token_1_mint: pubkeys[14],
            revenue_token_0: pubkeys[15],
            revenue_token_1: pubkeys[16],
            token_program: pubkeys[17],
            associated_token_program: pubkeys[18],
            system_program: pubkeys[19],
            event_authority: pubkeys[20],
            program: pubkeys[21],
        }
    }
}
impl<'info> From<SettleCreatorRevenueAccounts<'_, 'info>>
for [AccountInfo<'info>; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SettleCreatorRevenueAccounts<'_, 'info>) -> Self {
        [
            accounts.accounting_authority.clone(),
            accounts.revenue_config.clone(),
            accounts.migration_config.clone(),
            accounts.mint.clone(),
            accounts.migration_state.clone(),
            accounts.creator_revenue_state.clone(),
            accounts.revenue_authority.clone(),
            accounts.raydium_program.clone(),
            accounts.amm_config.clone(),
            accounts.raydium_authority.clone(),
            accounts.pool.clone(),
            accounts.token_0_vault.clone(),
            accounts.token_1_vault.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.revenue_token_0.clone(),
            accounts.revenue_token_1.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN]>
for SettleCreatorRevenueAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            accounting_authority: &arr[0],
            revenue_config: &arr[1],
            migration_config: &arr[2],
            mint: &arr[3],
            migration_state: &arr[4],
            creator_revenue_state: &arr[5],
            revenue_authority: &arr[6],
            raydium_program: &arr[7],
            amm_config: &arr[8],
            raydium_authority: &arr[9],
            pool: &arr[10],
            token_0_vault: &arr[11],
            token_1_vault: &arr[12],
            token_0_mint: &arr[13],
            token_1_mint: &arr[14],
            revenue_token_0: &arr[15],
            revenue_token_1: &arr[16],
            token_program: &arr[17],
            associated_token_program: &arr[18],
            system_program: &arr[19],
            event_authority: &arr[20],
            program: &arr[21],
        }
    }
}
pub const SETTLE_CREATOR_REVENUE_IX_DISCM: [u8; 8usize] = [
    115, 194, 6, 28, 0, 64, 42, 113,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettleCreatorRevenueIxArgs {
    pub params: SettleCreatorRevenueParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleCreatorRevenueIxData(pub SettleCreatorRevenueIxArgs);
impl From<SettleCreatorRevenueIxArgs> for SettleCreatorRevenueIxData {
    fn from(args: SettleCreatorRevenueIxArgs) -> Self {
        Self(args)
    }
}
impl SettleCreatorRevenueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_CREATOR_REVENUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <SettleCreatorRevenueParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(SettleCreatorRevenueIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_CREATOR_REVENUE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_creator_revenue_ix_with_program_id(
    program_id: Pubkey,
    keys: SettleCreatorRevenueKeys,
    args: SettleCreatorRevenueIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETTLE_CREATOR_REVENUE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SettleCreatorRevenueIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_creator_revenue_ix(
    keys: SettleCreatorRevenueKeys,
    args: SettleCreatorRevenueIxArgs,
) -> std::io::Result<Instruction> {
    settle_creator_revenue_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn settle_creator_revenue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SettleCreatorRevenueAccounts<'_, '_>,
    args: SettleCreatorRevenueIxArgs,
) -> ProgramResult {
    let keys: SettleCreatorRevenueKeys = accounts.into();
    let ix = settle_creator_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn settle_creator_revenue_invoke(
    accounts: SettleCreatorRevenueAccounts<'_, '_>,
    args: SettleCreatorRevenueIxArgs,
) -> ProgramResult {
    settle_creator_revenue_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn settle_creator_revenue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SettleCreatorRevenueAccounts<'_, '_>,
    args: SettleCreatorRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SettleCreatorRevenueKeys = accounts.into();
    let ix = settle_creator_revenue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn settle_creator_revenue_invoke_signed(
    accounts: SettleCreatorRevenueAccounts<'_, '_>,
    args: SettleCreatorRevenueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    settle_creator_revenue_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn settle_creator_revenue_verify_account_keys(
    accounts: SettleCreatorRevenueAccounts<'_, '_>,
    keys: SettleCreatorRevenueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.accounting_authority.key, keys.accounting_authority),
        (*accounts.revenue_config.key, keys.revenue_config),
        (*accounts.migration_config.key, keys.migration_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.migration_state.key, keys.migration_state),
        (*accounts.creator_revenue_state.key, keys.creator_revenue_state),
        (*accounts.revenue_authority.key, keys.revenue_authority),
        (*accounts.raydium_program.key, keys.raydium_program),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.raydium_authority.key, keys.raydium_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_0_vault.key, keys.token_0_vault),
        (*accounts.token_1_vault.key, keys.token_1_vault),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.revenue_token_0.key, keys.revenue_token_0),
        (*accounts.revenue_token_1.key, keys.revenue_token_1),
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
pub fn settle_creator_revenue_verify_writable_privileges<'me, 'info>(
    accounts: SettleCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator_revenue_state,
        accounts.revenue_authority,
        accounts.pool,
        accounts.token_0_vault,
        accounts.token_1_vault,
        accounts.revenue_token_0,
        accounts.revenue_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn settle_creator_revenue_verify_signer_privileges<'me, 'info>(
    accounts: SettleCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.accounting_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn settle_creator_revenue_verify_account_privileges<'me, 'info>(
    accounts: SettleCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    settle_creator_revenue_verify_writable_privileges(accounts)?;
    settle_creator_revenue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SetupCreatorRevenueAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub migration_state: &'me AccountInfo<'info>,
    pub governance: &'me AccountInfo<'info>,
    pub creator_revenue_state: &'me AccountInfo<'info>,
    pub revenue_authority: &'me AccountInfo<'info>,
    pub token_0_mint: &'me AccountInfo<'info>,
    pub token_1_mint: &'me AccountInfo<'info>,
    pub revenue_token_0: &'me AccountInfo<'info>,
    pub revenue_token_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetupCreatorRevenueKeys {
    pub payer: Pubkey,
    pub revenue_config: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub migration_state: Pubkey,
    pub governance: Pubkey,
    pub creator_revenue_state: Pubkey,
    pub revenue_authority: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub revenue_token_0: Pubkey,
    pub revenue_token_1: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SetupCreatorRevenueAccounts<'_, '_>> for SetupCreatorRevenueKeys {
    fn from(accounts: SetupCreatorRevenueAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            revenue_config: *accounts.revenue_config.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            migration_state: *accounts.migration_state.key,
            governance: *accounts.governance.key,
            creator_revenue_state: *accounts.creator_revenue_state.key,
            revenue_authority: *accounts.revenue_authority.key,
            token_0_mint: *accounts.token_0_mint.key,
            token_1_mint: *accounts.token_1_mint.key,
            revenue_token_0: *accounts.revenue_token_0.key,
            revenue_token_1: *accounts.revenue_token_1.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SetupCreatorRevenueKeys>
for [AccountMeta; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetupCreatorRevenueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.migration_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.governance,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_revenue_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_token_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.revenue_token_1,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN]> for SetupCreatorRevenueKeys {
    fn from(pubkeys: [Pubkey; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            revenue_config: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            migration_state: pubkeys[4],
            governance: pubkeys[5],
            creator_revenue_state: pubkeys[6],
            revenue_authority: pubkeys[7],
            token_0_mint: pubkeys[8],
            token_1_mint: pubkeys[9],
            revenue_token_0: pubkeys[10],
            revenue_token_1: pubkeys[11],
            token_program: pubkeys[12],
            associated_token_program: pubkeys[13],
            system_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<SetupCreatorRevenueAccounts<'_, 'info>>
for [AccountInfo<'info>; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetupCreatorRevenueAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.revenue_config.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.migration_state.clone(),
            accounts.governance.clone(),
            accounts.creator_revenue_state.clone(),
            accounts.revenue_authority.clone(),
            accounts.token_0_mint.clone(),
            accounts.token_1_mint.clone(),
            accounts.revenue_token_0.clone(),
            accounts.revenue_token_1.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN]>
for SetupCreatorRevenueAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            revenue_config: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            migration_state: &arr[4],
            governance: &arr[5],
            creator_revenue_state: &arr[6],
            revenue_authority: &arr[7],
            token_0_mint: &arr[8],
            token_1_mint: &arr[9],
            revenue_token_0: &arr[10],
            revenue_token_1: &arr[11],
            token_program: &arr[12],
            associated_token_program: &arr[13],
            system_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}
pub const SETUP_CREATOR_REVENUE_IX_DISCM: [u8; 8usize] = [
    123, 0, 193, 245, 228, 198, 150, 121,
];
#[derive(Clone, Debug, PartialEq)]
pub struct SetupCreatorRevenueIxData;
impl SetupCreatorRevenueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETUP_CREATOR_REVENUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETUP_CREATOR_REVENUE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn setup_creator_revenue_ix_with_program_id(
    program_id: Pubkey,
    keys: SetupCreatorRevenueKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SETUP_CREATOR_REVENUE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetupCreatorRevenueIxData.try_to_vec()?,
    })
}
pub fn setup_creator_revenue_ix(
    keys: SetupCreatorRevenueKeys,
) -> std::io::Result<Instruction> {
    setup_creator_revenue_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
}
pub fn setup_creator_revenue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetupCreatorRevenueAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetupCreatorRevenueKeys = accounts.into();
    let ix = setup_creator_revenue_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn setup_creator_revenue_invoke(
    accounts: SetupCreatorRevenueAccounts<'_, '_>,
) -> ProgramResult {
    setup_creator_revenue_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
}
pub fn setup_creator_revenue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetupCreatorRevenueAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetupCreatorRevenueKeys = accounts.into();
    let ix = setup_creator_revenue_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn setup_creator_revenue_invoke_signed(
    accounts: SetupCreatorRevenueAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    setup_creator_revenue_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn setup_creator_revenue_verify_account_keys(
    accounts: SetupCreatorRevenueAccounts<'_, '_>,
    keys: SetupCreatorRevenueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.revenue_config.key, keys.revenue_config),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.migration_state.key, keys.migration_state),
        (*accounts.governance.key, keys.governance),
        (*accounts.creator_revenue_state.key, keys.creator_revenue_state),
        (*accounts.revenue_authority.key, keys.revenue_authority),
        (*accounts.token_0_mint.key, keys.token_0_mint),
        (*accounts.token_1_mint.key, keys.token_1_mint),
        (*accounts.revenue_token_0.key, keys.revenue_token_0),
        (*accounts.revenue_token_1.key, keys.revenue_token_1),
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
pub fn setup_creator_revenue_verify_writable_privileges<'me, 'info>(
    accounts: SetupCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.creator_revenue_state,
        accounts.revenue_token_0,
        accounts.revenue_token_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn setup_creator_revenue_verify_signer_privileges<'me, 'info>(
    accounts: SetupCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn setup_creator_revenue_verify_account_privileges<'me, 'info>(
    accounts: SetupCreatorRevenueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    setup_creator_revenue_verify_writable_privileges(accounts)?;
    setup_creator_revenue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UnwhitelistLaunchPairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub launch_pair_whitelist: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnwhitelistLaunchPairKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub launch_pair_whitelist: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnwhitelistLaunchPairAccounts<'_, '_>> for UnwhitelistLaunchPairKeys {
    fn from(accounts: UnwhitelistLaunchPairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            launch_pair_whitelist: *accounts.launch_pair_whitelist.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnwhitelistLaunchPairKeys>
for [AccountMeta; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: UnwhitelistLaunchPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.launch_pair_whitelist,
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
impl From<[Pubkey; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN]>
for UnwhitelistLaunchPairKeys {
    fn from(pubkeys: [Pubkey; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            launch_pair_whitelist: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UnwhitelistLaunchPairAccounts<'_, 'info>>
for [AccountInfo<'info>; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnwhitelistLaunchPairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.launch_pair_whitelist.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN]>
for UnwhitelistLaunchPairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            launch_pair_whitelist: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UNWHITELIST_LAUNCH_PAIR_IX_DISCM: [u8; 8usize] = [
    75, 178, 5, 46, 95, 214, 39, 82,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnwhitelistLaunchPairIxArgs {
    pub params: WhitelistLaunchPairParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnwhitelistLaunchPairIxData(pub UnwhitelistLaunchPairIxArgs);
impl From<UnwhitelistLaunchPairIxArgs> for UnwhitelistLaunchPairIxData {
    fn from(args: UnwhitelistLaunchPairIxArgs) -> Self {
        Self(args)
    }
}
impl UnwhitelistLaunchPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNWHITELIST_LAUNCH_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <WhitelistLaunchPairParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UnwhitelistLaunchPairIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNWHITELIST_LAUNCH_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unwhitelist_launch_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: UnwhitelistLaunchPairKeys,
    args: UnwhitelistLaunchPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNWHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnwhitelistLaunchPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unwhitelist_launch_pair_ix(
    keys: UnwhitelistLaunchPairKeys,
    args: UnwhitelistLaunchPairIxArgs,
) -> std::io::Result<Instruction> {
    unwhitelist_launch_pair_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn unwhitelist_launch_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnwhitelistLaunchPairAccounts<'_, '_>,
    args: UnwhitelistLaunchPairIxArgs,
) -> ProgramResult {
    let keys: UnwhitelistLaunchPairKeys = accounts.into();
    let ix = unwhitelist_launch_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unwhitelist_launch_pair_invoke(
    accounts: UnwhitelistLaunchPairAccounts<'_, '_>,
    args: UnwhitelistLaunchPairIxArgs,
) -> ProgramResult {
    unwhitelist_launch_pair_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn unwhitelist_launch_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnwhitelistLaunchPairAccounts<'_, '_>,
    args: UnwhitelistLaunchPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnwhitelistLaunchPairKeys = accounts.into();
    let ix = unwhitelist_launch_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unwhitelist_launch_pair_invoke_signed(
    accounts: UnwhitelistLaunchPairAccounts<'_, '_>,
    args: UnwhitelistLaunchPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unwhitelist_launch_pair_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn unwhitelist_launch_pair_verify_account_keys(
    accounts: UnwhitelistLaunchPairAccounts<'_, '_>,
    keys: UnwhitelistLaunchPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.launch_pair_whitelist.key, keys.launch_pair_whitelist),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unwhitelist_launch_pair_verify_writable_privileges<'me, 'info>(
    accounts: UnwhitelistLaunchPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.launch_pair_whitelist] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unwhitelist_launch_pair_verify_signer_privileges<'me, 'info>(
    accounts: UnwhitelistLaunchPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unwhitelist_launch_pair_verify_account_privileges<'me, 'info>(
    accounts: UnwhitelistLaunchPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unwhitelist_launch_pair_verify_writable_privileges(accounts)?;
    unwhitelist_launch_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UnwhitelistTradeAuthorityAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnwhitelistTradeAuthorityKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UnwhitelistTradeAuthorityAccounts<'_, '_>> for UnwhitelistTradeAuthorityKeys {
    fn from(accounts: UnwhitelistTradeAuthorityAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UnwhitelistTradeAuthorityKeys>
for [AccountMeta; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UnwhitelistTradeAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN]>
for UnwhitelistTradeAuthorityKeys {
    fn from(pubkeys: [Pubkey; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UnwhitelistTradeAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnwhitelistTradeAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN]>
for UnwhitelistTradeAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UNWHITELIST_TRADE_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    111, 228, 112, 9, 11, 200, 131, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnwhitelistTradeAuthorityIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnwhitelistTradeAuthorityIxData(pub UnwhitelistTradeAuthorityIxArgs);
impl From<UnwhitelistTradeAuthorityIxArgs> for UnwhitelistTradeAuthorityIxData {
    fn from(args: UnwhitelistTradeAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl UnwhitelistTradeAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNWHITELIST_TRADE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(UnwhitelistTradeAuthorityIxArgs {
                authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNWHITELIST_TRADE_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unwhitelist_trade_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: UnwhitelistTradeAuthorityKeys,
    args: UnwhitelistTradeAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNWHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UnwhitelistTradeAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unwhitelist_trade_authority_ix(
    keys: UnwhitelistTradeAuthorityKeys,
    args: UnwhitelistTradeAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    unwhitelist_trade_authority_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn unwhitelist_trade_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnwhitelistTradeAuthorityAccounts<'_, '_>,
    args: UnwhitelistTradeAuthorityIxArgs,
) -> ProgramResult {
    let keys: UnwhitelistTradeAuthorityKeys = accounts.into();
    let ix = unwhitelist_trade_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn unwhitelist_trade_authority_invoke(
    accounts: UnwhitelistTradeAuthorityAccounts<'_, '_>,
    args: UnwhitelistTradeAuthorityIxArgs,
) -> ProgramResult {
    unwhitelist_trade_authority_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn unwhitelist_trade_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnwhitelistTradeAuthorityAccounts<'_, '_>,
    args: UnwhitelistTradeAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnwhitelistTradeAuthorityKeys = accounts.into();
    let ix = unwhitelist_trade_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unwhitelist_trade_authority_invoke_signed(
    accounts: UnwhitelistTradeAuthorityAccounts<'_, '_>,
    args: UnwhitelistTradeAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unwhitelist_trade_authority_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn unwhitelist_trade_authority_verify_account_keys(
    accounts: UnwhitelistTradeAuthorityAccounts<'_, '_>,
    keys: UnwhitelistTradeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unwhitelist_trade_authority_verify_writable_privileges<'me, 'info>(
    accounts: UnwhitelistTradeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unwhitelist_trade_authority_verify_signer_privileges<'me, 'info>(
    accounts: UnwhitelistTradeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unwhitelist_trade_authority_verify_account_privileges<'me, 'info>(
    accounts: UnwhitelistTradeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unwhitelist_trade_authority_verify_writable_privileges(accounts)?;
    unwhitelist_trade_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMigrationConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub migration_config: &'me AccountInfo<'info>,
    pub raydium_program: &'me AccountInfo<'info>,
    pub permission: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateMigrationConfigKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub migration_config: Pubkey,
    pub raydium_program: Pubkey,
    pub permission: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateMigrationConfigAccounts<'_, '_>> for UpdateMigrationConfigKeys {
    fn from(accounts: UpdateMigrationConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            migration_config: *accounts.migration_config.key,
            raydium_program: *accounts.raydium_program.key,
            permission: *accounts.permission.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateMigrationConfigKeys>
for [AccountMeta; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateMigrationConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.raydium_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.permission,
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
impl From<[Pubkey; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateMigrationConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            migration_config: pubkeys[2],
            raydium_program: pubkeys[3],
            permission: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateMigrationConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateMigrationConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.migration_config.clone(),
            accounts.raydium_program.clone(),
            accounts.permission.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateMigrationConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            migration_config: &arr[2],
            raydium_program: &arr[3],
            permission: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const UPDATE_MIGRATION_CONFIG_IX_DISCM: [u8; 8usize] = [
    179, 59, 183, 71, 93, 71, 9, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateMigrationConfigIxArgs {
    pub params: UpdateMigrationConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMigrationConfigIxData(pub UpdateMigrationConfigIxArgs);
impl From<UpdateMigrationConfigIxArgs> for UpdateMigrationConfigIxData {
    fn from(args: UpdateMigrationConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMigrationConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_MIGRATION_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateMigrationConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateMigrationConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_MIGRATION_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_migration_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateMigrationConfigKeys,
    args: UpdateMigrationConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_MIGRATION_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateMigrationConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_migration_config_ix(
    keys: UpdateMigrationConfigKeys,
    args: UpdateMigrationConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_migration_config_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn update_migration_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMigrationConfigAccounts<'_, '_>,
    args: UpdateMigrationConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateMigrationConfigKeys = accounts.into();
    let ix = update_migration_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_migration_config_invoke(
    accounts: UpdateMigrationConfigAccounts<'_, '_>,
    args: UpdateMigrationConfigIxArgs,
) -> ProgramResult {
    update_migration_config_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn update_migration_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateMigrationConfigAccounts<'_, '_>,
    args: UpdateMigrationConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateMigrationConfigKeys = accounts.into();
    let ix = update_migration_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_migration_config_invoke_signed(
    accounts: UpdateMigrationConfigAccounts<'_, '_>,
    args: UpdateMigrationConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_migration_config_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_migration_config_verify_account_keys(
    accounts: UpdateMigrationConfigAccounts<'_, '_>,
    keys: UpdateMigrationConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.migration_config.key, keys.migration_config),
        (*accounts.raydium_program.key, keys.raydium_program),
        (*accounts.permission.key, keys.permission),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_migration_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateMigrationConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.migration_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_migration_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateMigrationConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_migration_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateMigrationConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_migration_config_verify_writable_privileges(accounts)?;
    update_migration_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRevenueConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub revenue_config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRevenueConfigKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub revenue_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateRevenueConfigAccounts<'_, '_>> for UpdateRevenueConfigKeys {
    fn from(accounts: UpdateRevenueConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            revenue_config: *accounts.revenue_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateRevenueConfigKeys>
for [AccountMeta; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRevenueConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.revenue_config,
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
impl From<[Pubkey; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateRevenueConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            revenue_config: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateRevenueConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRevenueConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.revenue_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateRevenueConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            revenue_config: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_REVENUE_CONFIG_IX_DISCM: [u8; 8usize] = [
    125, 182, 255, 247, 206, 187, 195, 146,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRevenueConfigIxArgs {
    pub params: RevenueConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRevenueConfigIxData(pub UpdateRevenueConfigIxArgs);
impl From<UpdateRevenueConfigIxArgs> for UpdateRevenueConfigIxData {
    fn from(args: UpdateRevenueConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRevenueConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REVENUE_CONFIG_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <RevenueConfigParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(UpdateRevenueConfigIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REVENUE_CONFIG_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_revenue_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRevenueConfigKeys,
    args: UpdateRevenueConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REVENUE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRevenueConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_revenue_config_ix(
    keys: UpdateRevenueConfigKeys,
    args: UpdateRevenueConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_revenue_config_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn update_revenue_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRevenueConfigAccounts<'_, '_>,
    args: UpdateRevenueConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateRevenueConfigKeys = accounts.into();
    let ix = update_revenue_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_revenue_config_invoke(
    accounts: UpdateRevenueConfigAccounts<'_, '_>,
    args: UpdateRevenueConfigIxArgs,
) -> ProgramResult {
    update_revenue_config_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn update_revenue_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRevenueConfigAccounts<'_, '_>,
    args: UpdateRevenueConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRevenueConfigKeys = accounts.into();
    let ix = update_revenue_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_revenue_config_invoke_signed(
    accounts: UpdateRevenueConfigAccounts<'_, '_>,
    args: UpdateRevenueConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_revenue_config_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_revenue_config_verify_account_keys(
    accounts: UpdateRevenueConfigAccounts<'_, '_>,
    keys: UpdateRevenueConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.revenue_config.key, keys.revenue_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_revenue_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRevenueConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.revenue_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_revenue_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRevenueConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_revenue_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateRevenueConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_revenue_config_verify_writable_privileges(accounts)?;
    update_revenue_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct ValidateLaunchLimitTrackerAccounts<'me, 'info> {
    pub launch_limit_tracker: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ValidateLaunchLimitTrackerKeys {
    pub launch_limit_tracker: Pubkey,
}
impl From<ValidateLaunchLimitTrackerAccounts<'_, '_>>
for ValidateLaunchLimitTrackerKeys {
    fn from(accounts: ValidateLaunchLimitTrackerAccounts) -> Self {
        Self {
            launch_limit_tracker: *accounts.launch_limit_tracker.key,
        }
    }
}
impl From<ValidateLaunchLimitTrackerKeys>
for [AccountMeta; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(keys: ValidateLaunchLimitTrackerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.launch_limit_tracker,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN]>
for ValidateLaunchLimitTrackerKeys {
    fn from(pubkeys: [Pubkey; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            launch_limit_tracker: pubkeys[0],
        }
    }
}
impl<'info> From<ValidateLaunchLimitTrackerAccounts<'_, 'info>>
for [AccountInfo<'info>; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ValidateLaunchLimitTrackerAccounts<'_, 'info>) -> Self {
        [accounts.launch_limit_tracker.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN]>
for ValidateLaunchLimitTrackerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            launch_limit_tracker: &arr[0],
        }
    }
}
pub const VALIDATE_LAUNCH_LIMIT_TRACKER_IX_DISCM: [u8; 8usize] = [
    198, 32, 26, 51, 147, 85, 2, 93,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ValidateLaunchLimitTrackerIxData;
impl ValidateLaunchLimitTrackerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VALIDATE_LAUNCH_LIMIT_TRACKER_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VALIDATE_LAUNCH_LIMIT_TRACKER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn validate_launch_limit_tracker_ix_with_program_id(
    program_id: Pubkey,
    keys: ValidateLaunchLimitTrackerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; VALIDATE_LAUNCH_LIMIT_TRACKER_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ValidateLaunchLimitTrackerIxData.try_to_vec()?,
    })
}
pub fn validate_launch_limit_tracker_ix(
    keys: ValidateLaunchLimitTrackerKeys,
) -> std::io::Result<Instruction> {
    validate_launch_limit_tracker_ix_with_program_id(TRENCH_PROGRAM_ID, keys)
}
pub fn validate_launch_limit_tracker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ValidateLaunchLimitTrackerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ValidateLaunchLimitTrackerKeys = accounts.into();
    let ix = validate_launch_limit_tracker_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn validate_launch_limit_tracker_invoke(
    accounts: ValidateLaunchLimitTrackerAccounts<'_, '_>,
) -> ProgramResult {
    validate_launch_limit_tracker_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts)
}
pub fn validate_launch_limit_tracker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ValidateLaunchLimitTrackerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ValidateLaunchLimitTrackerKeys = accounts.into();
    let ix = validate_launch_limit_tracker_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn validate_launch_limit_tracker_invoke_signed(
    accounts: ValidateLaunchLimitTrackerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    validate_launch_limit_tracker_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn validate_launch_limit_tracker_verify_account_keys(
    accounts: ValidateLaunchLimitTrackerAccounts<'_, '_>,
    keys: ValidateLaunchLimitTrackerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.launch_limit_tracker.key, keys.launch_limit_tracker),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WhitelistLaunchPairAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub launch_pair_whitelist: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WhitelistLaunchPairKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub launch_pair_whitelist: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WhitelistLaunchPairAccounts<'_, '_>> for WhitelistLaunchPairKeys {
    fn from(accounts: WhitelistLaunchPairAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            launch_pair_whitelist: *accounts.launch_pair_whitelist.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WhitelistLaunchPairKeys>
for [AccountMeta; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: WhitelistLaunchPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.launch_pair_whitelist,
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
impl From<[Pubkey; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN]> for WhitelistLaunchPairKeys {
    fn from(pubkeys: [Pubkey; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            launch_pair_whitelist: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<WhitelistLaunchPairAccounts<'_, 'info>>
for [AccountInfo<'info>; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: WhitelistLaunchPairAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.launch_pair_whitelist.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN]>
for WhitelistLaunchPairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            launch_pair_whitelist: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const WHITELIST_LAUNCH_PAIR_IX_DISCM: [u8; 8usize] = [
    35, 72, 49, 59, 219, 160, 150, 115,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WhitelistLaunchPairIxArgs {
    pub params: WhitelistLaunchPairParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhitelistLaunchPairIxData(pub WhitelistLaunchPairIxArgs);
impl From<WhitelistLaunchPairIxArgs> for WhitelistLaunchPairIxData {
    fn from(args: WhitelistLaunchPairIxArgs) -> Self {
        Self(args)
    }
}
impl WhitelistLaunchPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHITELIST_LAUNCH_PAIR_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let params = if reader.is_empty() {
            Default::default()
        } else {
            <WhitelistLaunchPairParams>::deserialize(&mut reader)?
        };
        Ok(
            Self(WhitelistLaunchPairIxArgs {
                params,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHITELIST_LAUNCH_PAIR_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.params, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn whitelist_launch_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: WhitelistLaunchPairKeys,
    args: WhitelistLaunchPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WHITELIST_LAUNCH_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: WhitelistLaunchPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn whitelist_launch_pair_ix(
    keys: WhitelistLaunchPairKeys,
    args: WhitelistLaunchPairIxArgs,
) -> std::io::Result<Instruction> {
    whitelist_launch_pair_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn whitelist_launch_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistLaunchPairAccounts<'_, '_>,
    args: WhitelistLaunchPairIxArgs,
) -> ProgramResult {
    let keys: WhitelistLaunchPairKeys = accounts.into();
    let ix = whitelist_launch_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn whitelist_launch_pair_invoke(
    accounts: WhitelistLaunchPairAccounts<'_, '_>,
    args: WhitelistLaunchPairIxArgs,
) -> ProgramResult {
    whitelist_launch_pair_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn whitelist_launch_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistLaunchPairAccounts<'_, '_>,
    args: WhitelistLaunchPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WhitelistLaunchPairKeys = accounts.into();
    let ix = whitelist_launch_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn whitelist_launch_pair_invoke_signed(
    accounts: WhitelistLaunchPairAccounts<'_, '_>,
    args: WhitelistLaunchPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    whitelist_launch_pair_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn whitelist_launch_pair_verify_account_keys(
    accounts: WhitelistLaunchPairAccounts<'_, '_>,
    keys: WhitelistLaunchPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.launch_pair_whitelist.key, keys.launch_pair_whitelist),
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
pub fn whitelist_launch_pair_verify_writable_privileges<'me, 'info>(
    accounts: WhitelistLaunchPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.launch_pair_whitelist] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn whitelist_launch_pair_verify_signer_privileges<'me, 'info>(
    accounts: WhitelistLaunchPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn whitelist_launch_pair_verify_account_privileges<'me, 'info>(
    accounts: WhitelistLaunchPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    whitelist_launch_pair_verify_writable_privileges(accounts)?;
    whitelist_launch_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct WhitelistTradeAuthorityAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WhitelistTradeAuthorityKeys {
    pub admin: Pubkey,
    pub config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WhitelistTradeAuthorityAccounts<'_, '_>> for WhitelistTradeAuthorityKeys {
    fn from(accounts: WhitelistTradeAuthorityAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            config: *accounts.config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WhitelistTradeAuthorityKeys>
for [AccountMeta; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: WhitelistTradeAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN]>
for WhitelistTradeAuthorityKeys {
    fn from(pubkeys: [Pubkey; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            config: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<WhitelistTradeAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: WhitelistTradeAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN]>
for WhitelistTradeAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            config: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const WHITELIST_TRADE_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    38, 0, 80, 103, 155, 45, 65, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WhitelistTradeAuthorityIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhitelistTradeAuthorityIxData(pub WhitelistTradeAuthorityIxArgs);
impl From<WhitelistTradeAuthorityIxArgs> for WhitelistTradeAuthorityIxData {
    fn from(args: WhitelistTradeAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl WhitelistTradeAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHITELIST_TRADE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        Ok(
            Self(WhitelistTradeAuthorityIxArgs {
                authority,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHITELIST_TRADE_AUTHORITY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.authority, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn whitelist_trade_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: WhitelistTradeAuthorityKeys,
    args: WhitelistTradeAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WHITELIST_TRADE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: WhitelistTradeAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn whitelist_trade_authority_ix(
    keys: WhitelistTradeAuthorityKeys,
    args: WhitelistTradeAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    whitelist_trade_authority_ix_with_program_id(TRENCH_PROGRAM_ID, keys, args)
}
pub fn whitelist_trade_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistTradeAuthorityAccounts<'_, '_>,
    args: WhitelistTradeAuthorityIxArgs,
) -> ProgramResult {
    let keys: WhitelistTradeAuthorityKeys = accounts.into();
    let ix = whitelist_trade_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn whitelist_trade_authority_invoke(
    accounts: WhitelistTradeAuthorityAccounts<'_, '_>,
    args: WhitelistTradeAuthorityIxArgs,
) -> ProgramResult {
    whitelist_trade_authority_invoke_with_program_id(TRENCH_PROGRAM_ID, accounts, args)
}
pub fn whitelist_trade_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WhitelistTradeAuthorityAccounts<'_, '_>,
    args: WhitelistTradeAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WhitelistTradeAuthorityKeys = accounts.into();
    let ix = whitelist_trade_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn whitelist_trade_authority_invoke_signed(
    accounts: WhitelistTradeAuthorityAccounts<'_, '_>,
    args: WhitelistTradeAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    whitelist_trade_authority_invoke_signed_with_program_id(
        TRENCH_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn whitelist_trade_authority_verify_account_keys(
    accounts: WhitelistTradeAuthorityAccounts<'_, '_>,
    keys: WhitelistTradeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.config.key, keys.config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn whitelist_trade_authority_verify_writable_privileges<'me, 'info>(
    accounts: WhitelistTradeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn whitelist_trade_authority_verify_signer_privileges<'me, 'info>(
    accounts: WhitelistTradeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn whitelist_trade_authority_verify_account_privileges<'me, 'info>(
    accounts: WhitelistTradeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    whitelist_trade_authority_verify_writable_privileges(accounts)?;
    whitelist_trade_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
