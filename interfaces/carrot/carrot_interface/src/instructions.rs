use solana_pubkey::Pubkey;
use solana_cpi::{invoke, invoke_signed};
use solana_instruction::{AccountMeta, Instruction};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramError;
use std::io::Read;
#[allow(unused_imports)]
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum CarrotProgramIx {
    InitVault(InitVaultIxArgs),
    TransferVaultAuthority,
    PauseVault(PauseVaultIxArgs),
    MigrateVaultAccount,
    DistributeVaultFees,
    UpdateFee(UpdateFeeIxArgs),
    Issue(IssueIxArgs),
    Redeem(RedeemIxArgs),
    AddAsset,
    UpdateAsset(UpdateAssetIxArgs),
    RemoveAsset,
    RemoveStrategy,
    UpdateStrategy(UpdateStrategyIxArgs),
    MarginfiSupplyStrategyInit(MarginfiSupplyStrategyInitIxArgs),
    MarginfiSupplyStrategyDeposit(MarginfiSupplyStrategyDepositIxArgs),
    MarginfiSupplyStrategyWithdraw(MarginfiSupplyStrategyWithdrawIxArgs),
    MarginfiSupplyStrategyClaimEmissions,
    KlendSupplyStrategyInit(KlendSupplyStrategyInitIxArgs),
    KlendSupplyStrategyDeposit(KlendSupplyStrategyDepositIxArgs),
    KlendSupplyStrategyWithdraw(KlendSupplyStrategyWithdrawIxArgs),
    KlendSupplyStrategyClaimEmissions,
    SolendSupplyStrategyInit(SolendSupplyStrategyInitIxArgs),
    SolendSupplyStrategyDeposit(SolendSupplyStrategyDepositIxArgs),
    SolendSupplyStrategyWithdraw(SolendSupplyStrategyWithdrawIxArgs),
    MangoSupplyStrategyInit(MangoSupplyStrategyInitIxArgs),
    MangoSupplyStrategyDeposit(MangoSupplyStrategyDepositIxArgs),
    MangoSupplyStrategyWithdraw(MangoSupplyStrategyWithdrawIxArgs),
    DriftSupplyStrategyInit(DriftSupplyStrategyInitIxArgs),
    DriftSupplyStrategyDeposit(DriftSupplyStrategyDepositIxArgs),
    DriftSupplyStrategyWithdraw(DriftSupplyStrategyWithdrawIxArgs),
    DriftInsuranceFundStrategyInit(DriftInsuranceFundStrategyInitIxArgs),
    DriftInsuranceFundStrategyStake(DriftInsuranceFundStrategyStakeIxArgs),
    DriftInsuranceFundStrategyUnstake(DriftInsuranceFundStrategyUnstakeIxArgs),
    DriftInsuranceFundStrategyWithdraw,
    ClendSupplyStrategyInit(ClendSupplyStrategyInitIxArgs),
    ClendSupplyStrategyDeposit(ClendSupplyStrategyDepositIxArgs),
    ClendSupplyStrategyWithdraw(ClendSupplyStrategyWithdrawIxArgs),
    ChestStrategyInit(ChestStrategyInitIxArgs),
    ChestStrategyDeposit(ChestStrategyDepositIxArgs),
    ChestStrategyRequestWithdraw(ChestStrategyRequestWithdrawIxArgs),
    ChestStrategyWithdraw(ChestStrategyWithdrawIxArgs),
    UpdateSwitchboardOraclePrice,
}
impl CarrotProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.starts_with(&INIT_VAULT_IX_DISCM) {
            let mut reader = &buf[INIT_VAULT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <InitVaultArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::InitVault(InitVaultIxArgs { args }));
        }
        if buf.starts_with(&TRANSFER_VAULT_AUTHORITY_IX_DISCM) {
            return Ok(Self::TransferVaultAuthority);
        }
        if buf.starts_with(&PAUSE_VAULT_IX_DISCM) {
            let mut reader = &buf[PAUSE_VAULT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <PauseVaultArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::PauseVault(PauseVaultIxArgs { args }));
        }
        if buf.starts_with(&MIGRATE_VAULT_ACCOUNT_IX_DISCM) {
            return Ok(Self::MigrateVaultAccount);
        }
        if buf.starts_with(&DISTRIBUTE_VAULT_FEES_IX_DISCM) {
            return Ok(Self::DistributeVaultFees);
        }
        if buf.starts_with(&UPDATE_FEE_IX_DISCM) {
            let mut reader = &buf[UPDATE_FEE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateFeeArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::UpdateFee(UpdateFeeIxArgs { args }));
        }
        if buf.starts_with(&ISSUE_IX_DISCM) {
            let mut reader = &buf[ISSUE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <IssueArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Issue(IssueIxArgs { args }));
        }
        if buf.starts_with(&REDEEM_IX_DISCM) {
            let mut reader = &buf[REDEEM_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <RedeemArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::Redeem(RedeemIxArgs { args }));
        }
        if buf.starts_with(&ADD_ASSET_IX_DISCM) {
            return Ok(Self::AddAsset);
        }
        if buf.starts_with(&UPDATE_ASSET_IX_DISCM) {
            let mut reader = &buf[UPDATE_ASSET_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <UpdateAssetArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::UpdateAsset(UpdateAssetIxArgs { args }));
        }
        if buf.starts_with(&REMOVE_ASSET_IX_DISCM) {
            return Ok(Self::RemoveAsset);
        }
        if buf.starts_with(&REMOVE_STRATEGY_IX_DISCM) {
            return Ok(Self::RemoveStrategy);
        }
        if buf.starts_with(&UPDATE_STRATEGY_IX_DISCM) {
            let mut reader = &buf[UPDATE_STRATEGY_IX_DISCM.len()..];
            let args = <UpdateStrategyArgs>::deserialize(&mut reader)?;
            return Ok(Self::UpdateStrategy(UpdateStrategyIxArgs { args }));
        }
        if buf.starts_with(&MARGINFI_SUPPLY_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[MARGINFI_SUPPLY_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MarginfiSupplyStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::MarginfiSupplyStrategyInit(MarginfiSupplyStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MarginfiSupplyStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::MarginfiSupplyStrategyDeposit(MarginfiSupplyStrategyDepositIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MarginfiSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::MarginfiSupplyStrategyWithdraw(MarginfiSupplyStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM) {
            return Ok(Self::MarginfiSupplyStrategyClaimEmissions);
        }
        if buf.starts_with(&KLEND_SUPPLY_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[KLEND_SUPPLY_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <KlendSupplyStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::KlendSupplyStrategyInit(KlendSupplyStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <KlendSupplyStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::KlendSupplyStrategyDeposit(KlendSupplyStrategyDepositIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <KlendSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::KlendSupplyStrategyWithdraw(KlendSupplyStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM) {
            return Ok(Self::KlendSupplyStrategyClaimEmissions);
        }
        if buf.starts_with(&SOLEND_SUPPLY_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[SOLEND_SUPPLY_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SolendSupplyStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SolendSupplyStrategyInit(SolendSupplyStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SolendSupplyStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SolendSupplyStrategyDeposit(SolendSupplyStrategyDepositIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <SolendSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::SolendSupplyStrategyWithdraw(SolendSupplyStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MANGO_SUPPLY_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[MANGO_SUPPLY_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MangoSupplyStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::MangoSupplyStrategyInit(MangoSupplyStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MangoSupplyStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::MangoSupplyStrategyDeposit(MangoSupplyStrategyDepositIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <MangoSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::MangoSupplyStrategyWithdraw(MangoSupplyStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_SUPPLY_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[DRIFT_SUPPLY_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <DriftSupplyStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DriftSupplyStrategyInit(DriftSupplyStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <DriftSupplyStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DriftSupplyStrategyDeposit(DriftSupplyStrategyDepositIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <DriftSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DriftSupplyStrategyWithdraw(DriftSupplyStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <DriftInsuranceFundStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DriftInsuranceFundStrategyInit(DriftInsuranceFundStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_DISCM) {
            let mut reader = &buf[DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <DriftInsuranceFundStrategyStakeArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DriftInsuranceFundStrategyStake(DriftInsuranceFundStrategyStakeIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_DISCM) {
            let mut reader = &buf[DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_DISCM
                .len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <DriftInsuranceFundStrategyUnstakeArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::DriftInsuranceFundStrategyUnstake(DriftInsuranceFundStrategyUnstakeIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_DISCM) {
            return Ok(Self::DriftInsuranceFundStrategyWithdraw);
        }
        if buf.starts_with(&CLEND_SUPPLY_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[CLEND_SUPPLY_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ClendSupplyStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ClendSupplyStrategyInit(ClendSupplyStrategyInitIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ClendSupplyStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ClendSupplyStrategyDeposit(ClendSupplyStrategyDepositIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ClendSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ClendSupplyStrategyWithdraw(ClendSupplyStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&CHEST_STRATEGY_INIT_IX_DISCM) {
            let mut reader = &buf[CHEST_STRATEGY_INIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ChestStrategyInitArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::ChestStrategyInit(ChestStrategyInitIxArgs { args }));
        }
        if buf.starts_with(&CHEST_STRATEGY_DEPOSIT_IX_DISCM) {
            let mut reader = &buf[CHEST_STRATEGY_DEPOSIT_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ChestStrategyDepositArgs>::deserialize(&mut reader)?
            };
            return Ok(Self::ChestStrategyDeposit(ChestStrategyDepositIxArgs { args }));
        }
        if buf.starts_with(&CHEST_STRATEGY_REQUEST_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[CHEST_STRATEGY_REQUEST_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ChestStrategyRequestWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ChestStrategyRequestWithdraw(ChestStrategyRequestWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&CHEST_STRATEGY_WITHDRAW_IX_DISCM) {
            let mut reader = &buf[CHEST_STRATEGY_WITHDRAW_IX_DISCM.len()..];
            let args = if reader.is_empty() {
                Default::default()
            } else {
                <ChestStrategyWithdrawArgs>::deserialize(&mut reader)?
            };
            return Ok(
                Self::ChestStrategyWithdraw(ChestStrategyWithdrawIxArgs {
                    args,
                }),
            );
        }
        if buf.starts_with(&UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_DISCM) {
            return Ok(Self::UpdateSwitchboardOraclePrice);
        }
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitVault(args) => {
                writer.write_all(&INIT_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::TransferVaultAuthority => {
                writer.write_all(&TRANSFER_VAULT_AUTHORITY_IX_DISCM)
            }
            Self::PauseVault(args) => {
                writer.write_all(&PAUSE_VAULT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MigrateVaultAccount => {
                writer.write_all(&MIGRATE_VAULT_ACCOUNT_IX_DISCM)
            }
            Self::DistributeVaultFees => {
                writer.write_all(&DISTRIBUTE_VAULT_FEES_IX_DISCM)
            }
            Self::UpdateFee(args) => {
                writer.write_all(&UPDATE_FEE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Issue(args) => {
                writer.write_all(&ISSUE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::Redeem(args) => {
                writer.write_all(&REDEEM_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::AddAsset => writer.write_all(&ADD_ASSET_IX_DISCM),
            Self::UpdateAsset(args) => {
                writer.write_all(&UPDATE_ASSET_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::RemoveAsset => writer.write_all(&REMOVE_ASSET_IX_DISCM),
            Self::RemoveStrategy => writer.write_all(&REMOVE_STRATEGY_IX_DISCM),
            Self::UpdateStrategy(args) => {
                writer.write_all(&UPDATE_STRATEGY_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MarginfiSupplyStrategyInit(args) => {
                writer.write_all(&MARGINFI_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MarginfiSupplyStrategyDeposit(args) => {
                writer.write_all(&MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MarginfiSupplyStrategyWithdraw(args) => {
                writer.write_all(&MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MarginfiSupplyStrategyClaimEmissions => {
                writer.write_all(&MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM)
            }
            Self::KlendSupplyStrategyInit(args) => {
                writer.write_all(&KLEND_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::KlendSupplyStrategyDeposit(args) => {
                writer.write_all(&KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::KlendSupplyStrategyWithdraw(args) => {
                writer.write_all(&KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::KlendSupplyStrategyClaimEmissions => {
                writer.write_all(&KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM)
            }
            Self::SolendSupplyStrategyInit(args) => {
                writer.write_all(&SOLEND_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SolendSupplyStrategyDeposit(args) => {
                writer.write_all(&SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::SolendSupplyStrategyWithdraw(args) => {
                writer.write_all(&SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MangoSupplyStrategyInit(args) => {
                writer.write_all(&MANGO_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MangoSupplyStrategyDeposit(args) => {
                writer.write_all(&MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::MangoSupplyStrategyWithdraw(args) => {
                writer.write_all(&MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftSupplyStrategyInit(args) => {
                writer.write_all(&DRIFT_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftSupplyStrategyDeposit(args) => {
                writer.write_all(&DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftSupplyStrategyWithdraw(args) => {
                writer.write_all(&DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftInsuranceFundStrategyInit(args) => {
                writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftInsuranceFundStrategyStake(args) => {
                writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftInsuranceFundStrategyUnstake(args) => {
                writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::DriftInsuranceFundStrategyWithdraw => {
                writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_DISCM)
            }
            Self::ClendSupplyStrategyInit(args) => {
                writer.write_all(&CLEND_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ClendSupplyStrategyDeposit(args) => {
                writer.write_all(&CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ClendSupplyStrategyWithdraw(args) => {
                writer.write_all(&CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ChestStrategyInit(args) => {
                writer.write_all(&CHEST_STRATEGY_INIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ChestStrategyDeposit(args) => {
                writer.write_all(&CHEST_STRATEGY_DEPOSIT_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ChestStrategyRequestWithdraw(args) => {
                writer.write_all(&CHEST_STRATEGY_REQUEST_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::ChestStrategyWithdraw(args) => {
                writer.write_all(&CHEST_STRATEGY_WITHDRAW_IX_DISCM)?;
                borsh::BorshSerialize::serialize(&args.args, &mut writer)?;
                Ok(())
            }
            Self::UpdateSwitchboardOraclePrice => {
                writer.write_all(&UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_DISCM)
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
pub const INIT_VAULT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitVaultAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub shares: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitVaultKeys {
    pub vault: Pubkey,
    pub shares: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitVaultAccounts<'_, '_>> for InitVaultKeys {
    fn from(accounts: InitVaultAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            shares: *accounts.shares.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitVaultKeys> for [AccountMeta; INIT_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; INIT_VAULT_IX_ACCOUNTS_LEN]> for InitVaultKeys {
    fn from(pubkeys: [Pubkey; INIT_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            shares: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.shares.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_VAULT_IX_ACCOUNTS_LEN]>
for InitVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            shares: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INIT_VAULT_IX_DISCM: [u8; 8usize] = [77, 79, 85, 150, 33, 217, 52, 106];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitVaultIxArgs {
    pub args: InitVaultArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitVaultIxData(pub InitVaultIxArgs);
impl From<InitVaultIxArgs> for InitVaultIxData {
    fn from(args: InitVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InitVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <InitVaultArgs>::deserialize(&mut reader)?
        };
        Ok(Self(InitVaultIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitVaultKeys,
    args: InitVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_vault_ix(
    keys: InitVaultKeys,
    args: InitVaultIxArgs,
) -> std::io::Result<Instruction> {
    init_vault_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn init_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitVaultAccounts<'_, '_>,
    args: InitVaultIxArgs,
) -> ProgramResult {
    let keys: InitVaultKeys = accounts.into();
    let ix = init_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_vault_invoke(
    accounts: InitVaultAccounts<'_, '_>,
    args: InitVaultIxArgs,
) -> ProgramResult {
    init_vault_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn init_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitVaultAccounts<'_, '_>,
    args: InitVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitVaultKeys = accounts.into();
    let ix = init_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_vault_invoke_signed(
    accounts: InitVaultAccounts<'_, '_>,
    args: InitVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_vault_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, args, seeds)
}
pub fn init_vault_verify_account_keys(
    accounts: InitVaultAccounts<'_, '_>,
    keys: InitVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.shares.key, keys.shares),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_vault_verify_account_privileges<'me, 'info>(
    accounts: InitVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_vault_verify_writable_privileges(accounts)?;
    init_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TransferVaultAuthorityAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
    pub old_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferVaultAuthorityKeys {
    pub vault: Pubkey,
    pub new_authority: Pubkey,
    pub old_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<TransferVaultAuthorityAccounts<'_, '_>> for TransferVaultAuthorityKeys {
    fn from(accounts: TransferVaultAuthorityAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            new_authority: *accounts.new_authority.key,
            old_authority: *accounts.old_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TransferVaultAuthorityKeys>
for [AccountMeta; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferVaultAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.old_authority,
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
impl From<[Pubkey; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferVaultAuthorityKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            new_authority: pubkeys[1],
            old_authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<TransferVaultAuthorityAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferVaultAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.new_authority.clone(),
            accounts.old_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN]>
for TransferVaultAuthorityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            new_authority: &arr[1],
            old_authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const TRANSFER_VAULT_AUTHORITY_IX_DISCM: [u8; 8usize] = [
    139, 35, 83, 88, 52, 186, 162, 110,
];
#[derive(Clone, Debug, PartialEq)]
pub struct TransferVaultAuthorityIxData;
impl TransferVaultAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_VAULT_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_VAULT_AUTHORITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_vault_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferVaultAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_VAULT_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TransferVaultAuthorityIxData.try_to_vec()?,
    })
}
pub fn transfer_vault_authority_ix(
    keys: TransferVaultAuthorityKeys,
) -> std::io::Result<Instruction> {
    transfer_vault_authority_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn transfer_vault_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TransferVaultAuthorityKeys = accounts.into();
    let ix = transfer_vault_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_vault_authority_invoke(
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    transfer_vault_authority_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn transfer_vault_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferVaultAuthorityKeys = accounts.into();
    let ix = transfer_vault_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_vault_authority_invoke_signed(
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_vault_authority_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn transfer_vault_authority_verify_account_keys(
    accounts: TransferVaultAuthorityAccounts<'_, '_>,
    keys: TransferVaultAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.old_authority.key, keys.old_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_vault_authority_verify_writable_privileges<'me, 'info>(
    accounts: TransferVaultAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.old_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_vault_authority_verify_signer_privileges<'me, 'info>(
    accounts: TransferVaultAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.old_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_vault_authority_verify_account_privileges<'me, 'info>(
    accounts: TransferVaultAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_vault_authority_verify_writable_privileges(accounts)?;
    transfer_vault_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PAUSE_VAULT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PauseVaultAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PauseVaultKeys {
    pub vault: Pubkey,
    pub authority: Pubkey,
}
impl From<PauseVaultAccounts<'_, '_>> for PauseVaultKeys {
    fn from(accounts: PauseVaultAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<PauseVaultKeys> for [AccountMeta; PAUSE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: PauseVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PAUSE_VAULT_IX_ACCOUNTS_LEN]> for PauseVaultKeys {
    fn from(pubkeys: [Pubkey; PAUSE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<PauseVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; PAUSE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: PauseVaultAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PAUSE_VAULT_IX_ACCOUNTS_LEN]>
for PauseVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PAUSE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const PAUSE_VAULT_IX_DISCM: [u8; 8usize] = [250, 6, 228, 57, 6, 104, 19, 210];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PauseVaultIxArgs {
    pub args: PauseVaultArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PauseVaultIxData(pub PauseVaultIxArgs);
impl From<PauseVaultIxArgs> for PauseVaultIxData {
    fn from(args: PauseVaultIxArgs) -> Self {
        Self(args)
    }
}
impl PauseVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAUSE_VAULT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <PauseVaultArgs>::deserialize(&mut reader)?
        };
        Ok(Self(PauseVaultIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAUSE_VAULT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pause_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: PauseVaultKeys,
    args: PauseVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PAUSE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: PauseVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pause_vault_ix(
    keys: PauseVaultKeys,
    args: PauseVaultIxArgs,
) -> std::io::Result<Instruction> {
    pause_vault_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn pause_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PauseVaultAccounts<'_, '_>,
    args: PauseVaultIxArgs,
) -> ProgramResult {
    let keys: PauseVaultKeys = accounts.into();
    let ix = pause_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pause_vault_invoke(
    accounts: PauseVaultAccounts<'_, '_>,
    args: PauseVaultIxArgs,
) -> ProgramResult {
    pause_vault_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn pause_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PauseVaultAccounts<'_, '_>,
    args: PauseVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PauseVaultKeys = accounts.into();
    let ix = pause_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pause_vault_invoke_signed(
    accounts: PauseVaultAccounts<'_, '_>,
    args: PauseVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pause_vault_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, args, seeds)
}
pub fn pause_vault_verify_account_keys(
    accounts: PauseVaultAccounts<'_, '_>,
    keys: PauseVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pause_vault_verify_writable_privileges<'me, 'info>(
    accounts: PauseVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pause_vault_verify_signer_privileges<'me, 'info>(
    accounts: PauseVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pause_vault_verify_account_privileges<'me, 'info>(
    accounts: PauseVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pause_vault_verify_writable_privileges(accounts)?;
    pause_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MigrateVaultAccountAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateVaultAccountKeys {
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<MigrateVaultAccountAccounts<'_, '_>> for MigrateVaultAccountKeys {
    fn from(accounts: MigrateVaultAccountAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<MigrateVaultAccountKeys>
for [AccountMeta; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateVaultAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN]> for MigrateVaultAccountKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<MigrateVaultAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateVaultAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN]>
for MigrateVaultAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const MIGRATE_VAULT_ACCOUNT_IX_DISCM: [u8; 8usize] = [
    136, 92, 166, 245, 217, 170, 73, 108,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateVaultAccountIxData;
impl MigrateVaultAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_VAULT_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_VAULT_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_vault_account_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateVaultAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_VAULT_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateVaultAccountIxData.try_to_vec()?,
    })
}
pub fn migrate_vault_account_ix(
    keys: MigrateVaultAccountKeys,
) -> std::io::Result<Instruction> {
    migrate_vault_account_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn migrate_vault_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateVaultAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateVaultAccountKeys = accounts.into();
    let ix = migrate_vault_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_vault_account_invoke(
    accounts: MigrateVaultAccountAccounts<'_, '_>,
) -> ProgramResult {
    migrate_vault_account_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn migrate_vault_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateVaultAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateVaultAccountKeys = accounts.into();
    let ix = migrate_vault_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_vault_account_invoke_signed(
    accounts: MigrateVaultAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_vault_account_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn migrate_vault_account_verify_account_keys(
    accounts: MigrateVaultAccountAccounts<'_, '_>,
    keys: MigrateVaultAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_vault_account_verify_writable_privileges<'me, 'info>(
    accounts: MigrateVaultAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_vault_account_verify_signer_privileges<'me, 'info>(
    accounts: MigrateVaultAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_vault_account_verify_account_privileges<'me, 'info>(
    accounts: MigrateVaultAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_vault_account_verify_writable_privileges(accounts)?;
    migrate_vault_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DistributeVaultFeesAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub shares: &'me AccountInfo<'info>,
    pub shares_destination: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DistributeVaultFeesKeys {
    pub vault: Pubkey,
    pub shares: Pubkey,
    pub shares_destination: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<DistributeVaultFeesAccounts<'_, '_>> for DistributeVaultFeesKeys {
    fn from(accounts: DistributeVaultFeesAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            shares: *accounts.shares.key,
            shares_destination: *accounts.shares_destination.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<DistributeVaultFeesKeys>
for [AccountMeta; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: DistributeVaultFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN]> for DistributeVaultFeesKeys {
    fn from(pubkeys: [Pubkey; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            shares: pubkeys[1],
            shares_destination: pubkeys[2],
            authority: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
            log_program: pubkeys[6],
        }
    }
}
impl<'info> From<DistributeVaultFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: DistributeVaultFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.shares.clone(),
            accounts.shares_destination.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN]>
for DistributeVaultFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            shares: &arr[1],
            shares_destination: &arr[2],
            authority: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
            log_program: &arr[6],
        }
    }
}
pub const DISTRIBUTE_VAULT_FEES_IX_DISCM: [u8; 8usize] = [
    158, 43, 226, 51, 80, 42, 40, 67,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeVaultFeesIxData;
impl DistributeVaultFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISTRIBUTE_VAULT_FEES_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISTRIBUTE_VAULT_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn distribute_vault_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: DistributeVaultFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISTRIBUTE_VAULT_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DistributeVaultFeesIxData.try_to_vec()?,
    })
}
pub fn distribute_vault_fees_ix(
    keys: DistributeVaultFeesKeys,
) -> std::io::Result<Instruction> {
    distribute_vault_fees_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn distribute_vault_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DistributeVaultFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DistributeVaultFeesKeys = accounts.into();
    let ix = distribute_vault_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn distribute_vault_fees_invoke(
    accounts: DistributeVaultFeesAccounts<'_, '_>,
) -> ProgramResult {
    distribute_vault_fees_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn distribute_vault_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DistributeVaultFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DistributeVaultFeesKeys = accounts.into();
    let ix = distribute_vault_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn distribute_vault_fees_invoke_signed(
    accounts: DistributeVaultFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    distribute_vault_fees_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn distribute_vault_fees_verify_account_keys(
    accounts: DistributeVaultFeesAccounts<'_, '_>,
    keys: DistributeVaultFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.shares.key, keys.shares),
        (*accounts.shares_destination.key, keys.shares_destination),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn distribute_vault_fees_verify_writable_privileges<'me, 'info>(
    accounts: DistributeVaultFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.shares,
        accounts.shares_destination,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn distribute_vault_fees_verify_signer_privileges<'me, 'info>(
    accounts: DistributeVaultFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn distribute_vault_fees_verify_account_privileges<'me, 'info>(
    accounts: DistributeVaultFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    distribute_vault_fees_verify_writable_privileges(accounts)?;
    distribute_vault_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeKeys {
    pub vault: Pubkey,
    pub authority: Pubkey,
}
impl From<UpdateFeeAccounts<'_, '_>> for UpdateFeeKeys {
    fn from(accounts: UpdateFeeAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<UpdateFeeKeys> for [AccountMeta; UPDATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEE_IX_ACCOUNTS_LEN]> for UpdateFeeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEE_IX_ACCOUNTS_LEN]>
for UpdateFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const UPDATE_FEE_IX_DISCM: [u8; 8usize] = [232, 253, 195, 247, 148, 212, 73, 222];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeIxArgs {
    pub args: UpdateFeeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeIxData(pub UpdateFeeIxArgs);
impl From<UpdateFeeIxArgs> for UpdateFeeIxData {
    fn from(args: UpdateFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateFeeArgs>::deserialize(&mut reader)?
        };
        Ok(Self(UpdateFeeIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeKeys,
    args: UpdateFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fee_ix(
    keys: UpdateFeeKeys,
    args: UpdateFeeIxArgs,
) -> std::io::Result<Instruction> {
    update_fee_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn update_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
) -> ProgramResult {
    let keys: UpdateFeeKeys = accounts.into();
    let ix = update_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_invoke(
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
) -> ProgramResult {
    update_fee_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn update_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeKeys = accounts.into();
    let ix = update_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_invoke_signed(
    accounts: UpdateFeeAccounts<'_, '_>,
    args: UpdateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_fee_verify_account_keys(
    accounts: UpdateFeeAccounts<'_, '_>,
    keys: UpdateFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fee_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_verify_writable_privileges(accounts)?;
    update_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ISSUE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct IssueAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub shares: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub asset: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub shares_token_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IssueKeys {
    pub vault: Pubkey,
    pub shares: Pubkey,
    pub user_shares_ata: Pubkey,
    pub asset: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub user_asset_ata: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub shares_token_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<IssueAccounts<'_, '_>> for IssueKeys {
    fn from(accounts: IssueAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            shares: *accounts.shares.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            asset: *accounts.asset.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            shares_token_program: *accounts.shares_token_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<IssueKeys> for [AccountMeta; ISSUE_IX_ACCOUNTS_LEN] {
    fn from(keys: IssueKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
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
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.shares_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ISSUE_IX_ACCOUNTS_LEN]> for IssueKeys {
    fn from(pubkeys: [Pubkey; ISSUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            shares: pubkeys[1],
            user_shares_ata: pubkeys[2],
            asset: pubkeys[3],
            vault_asset_ata: pubkeys[4],
            user_asset_ata: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            asset_token_program: pubkeys[8],
            shares_token_program: pubkeys[9],
            log_program: pubkeys[10],
        }
    }
}
impl<'info> From<IssueAccounts<'_, 'info>>
for [AccountInfo<'info>; ISSUE_IX_ACCOUNTS_LEN] {
    fn from(accounts: IssueAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.shares.clone(),
            accounts.user_shares_ata.clone(),
            accounts.asset.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.user_asset_ata.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.shares_token_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ISSUE_IX_ACCOUNTS_LEN]>
for IssueAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ISSUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            shares: &arr[1],
            user_shares_ata: &arr[2],
            asset: &arr[3],
            vault_asset_ata: &arr[4],
            user_asset_ata: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            asset_token_program: &arr[8],
            shares_token_program: &arr[9],
            log_program: &arr[10],
        }
    }
}
pub const ISSUE_IX_DISCM: [u8; 8usize] = [190, 1, 98, 214, 81, 99, 222, 247];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueIxArgs {
    pub args: IssueArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IssueIxData(pub IssueIxArgs);
impl From<IssueIxArgs> for IssueIxData {
    fn from(args: IssueIxArgs) -> Self {
        Self(args)
    }
}
impl IssueIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ISSUE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <IssueArgs>::deserialize(&mut reader)?
        };
        Ok(Self(IssueIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ISSUE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn issue_ix_with_program_id(
    program_id: Pubkey,
    keys: IssueKeys,
    args: IssueIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ISSUE_IX_ACCOUNTS_LEN] = keys.into();
    let data: IssueIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn issue_ix(keys: IssueKeys, args: IssueIxArgs) -> std::io::Result<Instruction> {
    issue_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn issue_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IssueAccounts<'_, '_>,
    args: IssueIxArgs,
) -> ProgramResult {
    let keys: IssueKeys = accounts.into();
    let ix = issue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn issue_invoke(
    accounts: IssueAccounts<'_, '_>,
    args: IssueIxArgs,
) -> ProgramResult {
    issue_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn issue_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IssueAccounts<'_, '_>,
    args: IssueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IssueKeys = accounts.into();
    let ix = issue_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn issue_invoke_signed(
    accounts: IssueAccounts<'_, '_>,
    args: IssueIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    issue_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, args, seeds)
}
pub fn issue_verify_account_keys(
    accounts: IssueAccounts<'_, '_>,
    keys: IssueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.shares.key, keys.shares),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.asset.key, keys.asset),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.shares_token_program.key, keys.shares_token_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn issue_verify_writable_privileges<'me, 'info>(
    accounts: IssueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.shares,
        accounts.user_shares_ata,
        accounts.vault_asset_ata,
        accounts.user_asset_ata,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn issue_verify_signer_privileges<'me, 'info>(
    accounts: IssueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn issue_verify_account_privileges<'me, 'info>(
    accounts: IssueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    issue_verify_writable_privileges(accounts)?;
    issue_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RedeemAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub shares: &'me AccountInfo<'info>,
    pub user_shares_ata: &'me AccountInfo<'info>,
    pub asset: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub user_asset_ata: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub shares_token_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemKeys {
    pub vault: Pubkey,
    pub shares: Pubkey,
    pub user_shares_ata: Pubkey,
    pub asset: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub user_asset_ata: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub shares_token_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<RedeemAccounts<'_, '_>> for RedeemKeys {
    fn from(accounts: RedeemAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            shares: *accounts.shares.key,
            user_shares_ata: *accounts.user_shares_ata.key,
            asset: *accounts.asset.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            user_asset_ata: *accounts.user_asset_ata.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            shares_token_program: *accounts.shares_token_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<RedeemKeys> for [AccountMeta; REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_shares_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_asset_ata,
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
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.shares_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_IX_ACCOUNTS_LEN]> for RedeemKeys {
    fn from(pubkeys: [Pubkey; REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            shares: pubkeys[1],
            user_shares_ata: pubkeys[2],
            asset: pubkeys[3],
            vault_asset_ata: pubkeys[4],
            user_asset_ata: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            asset_token_program: pubkeys[8],
            shares_token_program: pubkeys[9],
            log_program: pubkeys[10],
        }
    }
}
impl<'info> From<RedeemAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.shares.clone(),
            accounts.user_shares_ata.clone(),
            accounts.asset.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.user_asset_ata.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.shares_token_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN]>
for RedeemAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            shares: &arr[1],
            user_shares_ata: &arr[2],
            asset: &arr[3],
            vault_asset_ata: &arr[4],
            user_asset_ata: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            asset_token_program: &arr[8],
            shares_token_program: &arr[9],
            log_program: &arr[10],
        }
    }
}
pub const REDEEM_IX_DISCM: [u8; 8usize] = [184, 12, 86, 149, 70, 196, 97, 225];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RedeemIxArgs {
    pub args: RedeemArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemIxData(pub RedeemIxArgs);
impl From<RedeemIxArgs> for RedeemIxData {
    fn from(args: RedeemIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <RedeemArgs>::deserialize(&mut reader)?
        };
        Ok(Self(RedeemIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemKeys,
    args: RedeemIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_ix(keys: RedeemKeys, args: RedeemIxArgs) -> std::io::Result<Instruction> {
    redeem_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn redeem_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
) -> ProgramResult {
    let keys: RedeemKeys = accounts.into();
    let ix = redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_invoke(
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
) -> ProgramResult {
    redeem_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn redeem_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemKeys = accounts.into();
    let ix = redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_invoke_signed(
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, args, seeds)
}
pub fn redeem_verify_account_keys(
    accounts: RedeemAccounts<'_, '_>,
    keys: RedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.shares.key, keys.shares),
        (*accounts.user_shares_ata.key, keys.user_shares_ata),
        (*accounts.asset.key, keys.asset),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.user_asset_ata.key, keys.user_asset_ata),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.shares_token_program.key, keys.shares_token_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_verify_writable_privileges<'me, 'info>(
    accounts: RedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.shares,
        accounts.user_shares_ata,
        accounts.vault_asset_ata,
        accounts.user_asset_ata,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_verify_signer_privileges<'me, 'info>(
    accounts: RedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_verify_account_privileges<'me, 'info>(
    accounts: RedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_verify_writable_privileges(accounts)?;
    redeem_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_ASSET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddAssetAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub asset_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddAssetKeys {
    pub vault: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<AddAssetAccounts<'_, '_>> for AddAssetKeys {
    fn from(accounts: AddAssetAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            asset_mint: *accounts.asset_mint.key,
            asset_oracle: *accounts.asset_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddAssetKeys> for [AccountMeta; ADD_ASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: AddAssetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; ADD_ASSET_IX_ACCOUNTS_LEN]> for AddAssetKeys {
    fn from(pubkeys: [Pubkey; ADD_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            vault_asset_ata: pubkeys[1],
            asset_mint: pubkeys[2],
            asset_oracle: pubkeys[3],
            authority: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddAssetAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_ASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddAssetAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.asset_mint.clone(),
            accounts.asset_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_ASSET_IX_ACCOUNTS_LEN]>
for AddAssetAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            vault_asset_ata: &arr[1],
            asset_mint: &arr[2],
            asset_oracle: &arr[3],
            authority: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const ADD_ASSET_IX_DISCM: [u8; 8usize] = [81, 53, 134, 142, 243, 73, 42, 179];
#[derive(Clone, Debug, PartialEq)]
pub struct AddAssetIxData;
impl AddAssetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_ASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_ASSET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_asset_ix_with_program_id(
    program_id: Pubkey,
    keys: AddAssetKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_ASSET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AddAssetIxData.try_to_vec()?,
    })
}
pub fn add_asset_ix(keys: AddAssetKeys) -> std::io::Result<Instruction> {
    add_asset_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn add_asset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddAssetAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AddAssetKeys = accounts.into();
    let ix = add_asset_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_asset_invoke(accounts: AddAssetAccounts<'_, '_>) -> ProgramResult {
    add_asset_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn add_asset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddAssetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddAssetKeys = accounts.into();
    let ix = add_asset_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_asset_invoke_signed(
    accounts: AddAssetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_asset_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, seeds)
}
pub fn add_asset_verify_account_keys(
    accounts: AddAssetAccounts<'_, '_>,
    keys: AddAssetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.asset_oracle.key, keys.asset_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_asset_verify_writable_privileges<'me, 'info>(
    accounts: AddAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_asset_verify_signer_privileges<'me, 'info>(
    accounts: AddAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_asset_verify_account_privileges<'me, 'info>(
    accounts: AddAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_asset_verify_writable_privileges(accounts)?;
    add_asset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ASSET_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAssetAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAssetKeys {
    pub vault: Pubkey,
    pub authority: Pubkey,
}
impl From<UpdateAssetAccounts<'_, '_>> for UpdateAssetKeys {
    fn from(accounts: UpdateAssetAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<UpdateAssetKeys> for [AccountMeta; UPDATE_ASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAssetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_ASSET_IX_ACCOUNTS_LEN]> for UpdateAssetKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAssetAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAssetAccounts<'_, 'info>) -> Self {
        [accounts.vault.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ASSET_IX_ACCOUNTS_LEN]>
for UpdateAssetAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const UPDATE_ASSET_IX_DISCM: [u8; 8usize] = [56, 126, 238, 138, 192, 118, 228, 172];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAssetIxArgs {
    pub args: UpdateAssetArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAssetIxData(pub UpdateAssetIxArgs);
impl From<UpdateAssetIxArgs> for UpdateAssetIxData {
    fn from(args: UpdateAssetIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAssetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <UpdateAssetArgs>::deserialize(&mut reader)?
        };
        Ok(Self(UpdateAssetIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ASSET_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_asset_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAssetKeys,
    args: UpdateAssetIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ASSET_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAssetIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_asset_ix(
    keys: UpdateAssetKeys,
    args: UpdateAssetIxArgs,
) -> std::io::Result<Instruction> {
    update_asset_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn update_asset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAssetAccounts<'_, '_>,
    args: UpdateAssetIxArgs,
) -> ProgramResult {
    let keys: UpdateAssetKeys = accounts.into();
    let ix = update_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_asset_invoke(
    accounts: UpdateAssetAccounts<'_, '_>,
    args: UpdateAssetIxArgs,
) -> ProgramResult {
    update_asset_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn update_asset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAssetAccounts<'_, '_>,
    args: UpdateAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAssetKeys = accounts.into();
    let ix = update_asset_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_asset_invoke_signed(
    accounts: UpdateAssetAccounts<'_, '_>,
    args: UpdateAssetIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_asset_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, args, seeds)
}
pub fn update_asset_verify_account_keys(
    accounts: UpdateAssetAccounts<'_, '_>,
    keys: UpdateAssetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.authority.key, keys.authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_asset_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_asset_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_asset_verify_account_privileges<'me, 'info>(
    accounts: UpdateAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_asset_verify_writable_privileges(accounts)?;
    update_asset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ASSET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAssetAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAssetKeys {
    pub vault: Pubkey,
    pub asset: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemoveAssetAccounts<'_, '_>> for RemoveAssetKeys {
    fn from(accounts: RemoveAssetAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset: *accounts.asset.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemoveAssetKeys> for [AccountMeta; REMOVE_ASSET_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAssetKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; REMOVE_ASSET_IX_ACCOUNTS_LEN]> for RemoveAssetKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            authority: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<RemoveAssetAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ASSET_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAssetAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ASSET_IX_ACCOUNTS_LEN]>
for RemoveAssetAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_ASSET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            asset: &arr[1],
            vault_asset_ata: &arr[2],
            authority: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const REMOVE_ASSET_IX_DISCM: [u8; 8usize] = [139, 243, 2, 142, 50, 197, 54, 181];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAssetIxData;
impl RemoveAssetIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ASSET_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ASSET_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_asset_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAssetKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ASSET_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAssetIxData.try_to_vec()?,
    })
}
pub fn remove_asset_ix(keys: RemoveAssetKeys) -> std::io::Result<Instruction> {
    remove_asset_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn remove_asset_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAssetAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAssetKeys = accounts.into();
    let ix = remove_asset_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_asset_invoke(accounts: RemoveAssetAccounts<'_, '_>) -> ProgramResult {
    remove_asset_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn remove_asset_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAssetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAssetKeys = accounts.into();
    let ix = remove_asset_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_asset_invoke_signed(
    accounts: RemoveAssetAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_asset_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, seeds)
}
pub fn remove_asset_verify_account_keys(
    accounts: RemoveAssetAccounts<'_, '_>,
    keys: RemoveAssetKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset.key, keys.asset),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_asset_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_asset_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_asset_verify_account_privileges<'me, 'info>(
    accounts: RemoveAssetAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_asset_verify_writable_privileges(accounts)?;
    remove_asset_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_STRATEGY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RemoveStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<RemoveStrategyAccounts<'_, '_>> for RemoveStrategyKeys {
    fn from(accounts: RemoveStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RemoveStrategyKeys> for [AccountMeta; REMOVE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; REMOVE_STRATEGY_IX_ACCOUNTS_LEN]> for RemoveStrategyKeys {
    fn from(pubkeys: [Pubkey; REMOVE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<RemoveStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_STRATEGY_IX_ACCOUNTS_LEN]>
for RemoveStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const REMOVE_STRATEGY_IX_DISCM: [u8; 8usize] = [185, 238, 33, 91, 134, 210, 97, 26];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveStrategyIxData;
impl RemoveStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_STRATEGY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveStrategyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveStrategyIxData.try_to_vec()?,
    })
}
pub fn remove_strategy_ix(keys: RemoveStrategyKeys) -> std::io::Result<Instruction> {
    remove_strategy_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn remove_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveStrategyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveStrategyKeys = accounts.into();
    let ix = remove_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_strategy_invoke(
    accounts: RemoveStrategyAccounts<'_, '_>,
) -> ProgramResult {
    remove_strategy_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn remove_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveStrategyKeys = accounts.into();
    let ix = remove_strategy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_strategy_invoke_signed(
    accounts: RemoveStrategyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_strategy_invoke_signed_with_program_id(CARROT_PROGRAM_ID, accounts, seeds)
}
pub fn remove_strategy_verify_account_keys(
    accounts: RemoveStrategyAccounts<'_, '_>,
    keys: RemoveStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_strategy_verify_writable_privileges<'me, 'info>(
    accounts: RemoveStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.strategy, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_strategy_verify_signer_privileges<'me, 'info>(
    accounts: RemoveStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_strategy_verify_account_privileges<'me, 'info>(
    accounts: RemoveStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_strategy_verify_writable_privileges(accounts)?;
    remove_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_STRATEGY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateStrategyAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateStrategyKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateStrategyAccounts<'_, '_>> for UpdateStrategyKeys {
    fn from(accounts: UpdateStrategyAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateStrategyKeys> for [AccountMeta; UPDATE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; UPDATE_STRATEGY_IX_ACCOUNTS_LEN]> for UpdateStrategyKeys {
    fn from(pubkeys: [Pubkey; UPDATE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_STRATEGY_IX_ACCOUNTS_LEN]>
for UpdateStrategyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const UPDATE_STRATEGY_IX_DISCM: [u8; 8usize] = [16, 76, 138, 179, 171, 112, 196, 21];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateStrategyIxArgs {
    pub args: UpdateStrategyArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStrategyIxData(pub UpdateStrategyIxArgs);
impl From<UpdateStrategyIxArgs> for UpdateStrategyIxData {
    fn from(args: UpdateStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_STRATEGY_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = <UpdateStrategyArgs>::deserialize(&mut reader)?;
        Ok(Self(UpdateStrategyIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_STRATEGY_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateStrategyKeys,
    args: UpdateStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_strategy_ix(
    keys: UpdateStrategyKeys,
    args: UpdateStrategyIxArgs,
) -> std::io::Result<Instruction> {
    update_strategy_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn update_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStrategyAccounts<'_, '_>,
    args: UpdateStrategyIxArgs,
) -> ProgramResult {
    let keys: UpdateStrategyKeys = accounts.into();
    let ix = update_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_strategy_invoke(
    accounts: UpdateStrategyAccounts<'_, '_>,
    args: UpdateStrategyIxArgs,
) -> ProgramResult {
    update_strategy_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn update_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStrategyAccounts<'_, '_>,
    args: UpdateStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateStrategyKeys = accounts.into();
    let ix = update_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_strategy_invoke_signed(
    accounts: UpdateStrategyAccounts<'_, '_>,
    args: UpdateStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_strategy_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_strategy_verify_account_keys(
    accounts: UpdateStrategyAccounts<'_, '_>,
    keys: UpdateStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_strategy_verify_writable_privileges<'me, 'info>(
    accounts: UpdateStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.strategy, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_strategy_verify_signer_privileges<'me, 'info>(
    accounts: UpdateStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_strategy_verify_account_privileges<'me, 'info>(
    accounts: UpdateStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_strategy_verify_writable_privileges(accounts)?;
    update_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiSupplyStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub marginfi_group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub marginfi_bank: &'me AccountInfo<'info>,
    pub marginfi_bank_liquidity_vault: &'me AccountInfo<'info>,
    pub marginfi_bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub marginfi_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub marginfi_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub marginfi_group: Pubkey,
    pub marginfi_account: Pubkey,
    pub marginfi_bank: Pubkey,
    pub marginfi_bank_liquidity_vault: Pubkey,
    pub marginfi_bank_liquidity_vault_authority: Pubkey,
    pub marginfi_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub marginfi_program: Pubkey,
}
impl From<MarginfiSupplyStrategyInitAccounts<'_, '_>>
for MarginfiSupplyStrategyInitKeys {
    fn from(accounts: MarginfiSupplyStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            marginfi_group: *accounts.marginfi_group.key,
            marginfi_account: *accounts.marginfi_account.key,
            marginfi_bank: *accounts.marginfi_bank.key,
            marginfi_bank_liquidity_vault: *accounts.marginfi_bank_liquidity_vault.key,
            marginfi_bank_liquidity_vault_authority: *accounts
                .marginfi_bank_liquidity_vault_authority
                .key,
            marginfi_oracle: *accounts.marginfi_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            marginfi_program: *accounts.marginfi_program.key,
        }
    }
}
impl From<MarginfiSupplyStrategyInitKeys>
for [AccountMeta; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiSupplyStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank_liquidity_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyInitKeys {
    fn from(pubkeys: [Pubkey; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            marginfi_group: pubkeys[3],
            marginfi_account: pubkeys[4],
            marginfi_bank: pubkeys[5],
            marginfi_bank_liquidity_vault: pubkeys[6],
            marginfi_bank_liquidity_vault_authority: pubkeys[7],
            marginfi_oracle: pubkeys[8],
            authority: pubkeys[9],
            system_program: pubkeys[10],
            marginfi_program: pubkeys[11],
        }
    }
}
impl<'info> From<MarginfiSupplyStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiSupplyStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.marginfi_group.clone(),
            accounts.marginfi_account.clone(),
            accounts.marginfi_bank.clone(),
            accounts.marginfi_bank_liquidity_vault.clone(),
            accounts.marginfi_bank_liquidity_vault_authority.clone(),
            accounts.marginfi_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.marginfi_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            marginfi_group: &arr[3],
            marginfi_account: &arr[4],
            marginfi_bank: &arr[5],
            marginfi_bank_liquidity_vault: &arr[6],
            marginfi_bank_liquidity_vault_authority: &arr[7],
            marginfi_oracle: &arr[8],
            authority: &arr[9],
            system_program: &arr[10],
            marginfi_program: &arr[11],
        }
    }
}
pub const MARGINFI_SUPPLY_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    177, 175, 81, 239, 57, 30, 251, 160,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiSupplyStrategyInitIxArgs {
    pub args: MarginfiSupplyStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyInitIxData(pub MarginfiSupplyStrategyInitIxArgs);
impl From<MarginfiSupplyStrategyInitIxArgs> for MarginfiSupplyStrategyInitIxData {
    fn from(args: MarginfiSupplyStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiSupplyStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_SUPPLY_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MarginfiSupplyStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(MarginfiSupplyStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_supply_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiSupplyStrategyInitKeys,
    args: MarginfiSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MarginfiSupplyStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_supply_strategy_init_ix(
    keys: MarginfiSupplyStrategyInitKeys,
    args: MarginfiSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_supply_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn marginfi_supply_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyInitAccounts<'_, '_>,
    args: MarginfiSupplyStrategyInitIxArgs,
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyInitKeys = accounts.into();
    let ix = marginfi_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_supply_strategy_init_invoke(
    accounts: MarginfiSupplyStrategyInitAccounts<'_, '_>,
    args: MarginfiSupplyStrategyInitIxArgs,
) -> ProgramResult {
    marginfi_supply_strategy_init_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_supply_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyInitAccounts<'_, '_>,
    args: MarginfiSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyInitKeys = accounts.into();
    let ix = marginfi_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_supply_strategy_init_invoke_signed(
    accounts: MarginfiSupplyStrategyInitAccounts<'_, '_>,
    args: MarginfiSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_supply_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_supply_strategy_init_verify_account_keys(
    accounts: MarginfiSupplyStrategyInitAccounts<'_, '_>,
    keys: MarginfiSupplyStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.marginfi_bank.key, keys.marginfi_bank),
        (
            *accounts.marginfi_bank_liquidity_vault.key,
            keys.marginfi_bank_liquidity_vault,
        ),
        (
            *accounts.marginfi_bank_liquidity_vault_authority.key,
            keys.marginfi_bank_liquidity_vault_authority,
        ),
        (*accounts.marginfi_oracle.key, keys.marginfi_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.marginfi_program.key, keys.marginfi_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.marginfi_account,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.marginfi_account, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_supply_strategy_init_verify_writable_privileges(accounts)?;
    marginfi_supply_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiSupplyStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub marginfi_group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub marginfi_bank: &'me AccountInfo<'info>,
    pub marginfi_bank_liquidity_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub marginfi_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyDepositKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub marginfi_group: Pubkey,
    pub marginfi_account: Pubkey,
    pub marginfi_bank: Pubkey,
    pub marginfi_bank_liquidity_vault: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub marginfi_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<MarginfiSupplyStrategyDepositAccounts<'_, '_>>
for MarginfiSupplyStrategyDepositKeys {
    fn from(accounts: MarginfiSupplyStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            marginfi_group: *accounts.marginfi_group.key,
            marginfi_account: *accounts.marginfi_account.key,
            marginfi_bank: *accounts.marginfi_bank.key,
            marginfi_bank_liquidity_vault: *accounts.marginfi_bank_liquidity_vault.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            marginfi_program: *accounts.marginfi_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<MarginfiSupplyStrategyDepositKeys>
for [AccountMeta; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiSupplyStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank_liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.marginfi_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyDepositKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            marginfi_group: pubkeys[4],
            marginfi_account: pubkeys[5],
            marginfi_bank: pubkeys[6],
            marginfi_bank_liquidity_vault: pubkeys[7],
            authority: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
            marginfi_program: pubkeys[11],
            log_program: pubkeys[12],
        }
    }
}
impl<'info> From<MarginfiSupplyStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiSupplyStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.marginfi_group.clone(),
            accounts.marginfi_account.clone(),
            accounts.marginfi_bank.clone(),
            accounts.marginfi_bank_liquidity_vault.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.marginfi_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            marginfi_group: &arr[4],
            marginfi_account: &arr[5],
            marginfi_bank: &arr[6],
            marginfi_bank_liquidity_vault: &arr[7],
            authority: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
            marginfi_program: &arr[11],
            log_program: &arr[12],
        }
    }
}
pub const MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    186, 12, 46, 101, 237, 152, 72, 252,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiSupplyStrategyDepositIxArgs {
    pub args: MarginfiSupplyStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyDepositIxData(pub MarginfiSupplyStrategyDepositIxArgs);
impl From<MarginfiSupplyStrategyDepositIxArgs> for MarginfiSupplyStrategyDepositIxData {
    fn from(args: MarginfiSupplyStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiSupplyStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MarginfiSupplyStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(MarginfiSupplyStrategyDepositIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_supply_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiSupplyStrategyDepositKeys,
    args: MarginfiSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MarginfiSupplyStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_supply_strategy_deposit_ix(
    keys: MarginfiSupplyStrategyDepositKeys,
    args: MarginfiSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_supply_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn marginfi_supply_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyDepositAccounts<'_, '_>,
    args: MarginfiSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyDepositKeys = accounts.into();
    let ix = marginfi_supply_strategy_deposit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_supply_strategy_deposit_invoke(
    accounts: MarginfiSupplyStrategyDepositAccounts<'_, '_>,
    args: MarginfiSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    marginfi_supply_strategy_deposit_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_supply_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyDepositAccounts<'_, '_>,
    args: MarginfiSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyDepositKeys = accounts.into();
    let ix = marginfi_supply_strategy_deposit_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_supply_strategy_deposit_invoke_signed(
    accounts: MarginfiSupplyStrategyDepositAccounts<'_, '_>,
    args: MarginfiSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_supply_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_supply_strategy_deposit_verify_account_keys(
    accounts: MarginfiSupplyStrategyDepositAccounts<'_, '_>,
    keys: MarginfiSupplyStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.marginfi_bank.key, keys.marginfi_bank),
        (
            *accounts.marginfi_bank_liquidity_vault.key,
            keys.marginfi_bank_liquidity_vault,
        ),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.marginfi_program.key, keys.marginfi_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.marginfi_account,
        accounts.marginfi_bank,
        accounts.marginfi_bank_liquidity_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_supply_strategy_deposit_verify_writable_privileges(accounts)?;
    marginfi_supply_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiSupplyStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub marginfi_group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub marginfi_bank: &'me AccountInfo<'info>,
    pub marginfi_bank_liquidity_vault: &'me AccountInfo<'info>,
    pub marginfi_bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub marginfi_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub marginfi_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub marginfi_group: Pubkey,
    pub marginfi_account: Pubkey,
    pub marginfi_bank: Pubkey,
    pub marginfi_bank_liquidity_vault: Pubkey,
    pub marginfi_bank_liquidity_vault_authority: Pubkey,
    pub marginfi_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub marginfi_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<MarginfiSupplyStrategyWithdrawAccounts<'_, '_>>
for MarginfiSupplyStrategyWithdrawKeys {
    fn from(accounts: MarginfiSupplyStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            marginfi_group: *accounts.marginfi_group.key,
            marginfi_account: *accounts.marginfi_account.key,
            marginfi_bank: *accounts.marginfi_bank.key,
            marginfi_bank_liquidity_vault: *accounts.marginfi_bank_liquidity_vault.key,
            marginfi_bank_liquidity_vault_authority: *accounts
                .marginfi_bank_liquidity_vault_authority
                .key,
            marginfi_oracle: *accounts.marginfi_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            marginfi_program: *accounts.marginfi_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<MarginfiSupplyStrategyWithdrawKeys>
for [AccountMeta; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiSupplyStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank_liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.marginfi_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyWithdrawKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            marginfi_group: pubkeys[4],
            marginfi_account: pubkeys[5],
            marginfi_bank: pubkeys[6],
            marginfi_bank_liquidity_vault: pubkeys[7],
            marginfi_bank_liquidity_vault_authority: pubkeys[8],
            marginfi_oracle: pubkeys[9],
            authority: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            marginfi_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<MarginfiSupplyStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiSupplyStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.marginfi_group.clone(),
            accounts.marginfi_account.clone(),
            accounts.marginfi_bank.clone(),
            accounts.marginfi_bank_liquidity_vault.clone(),
            accounts.marginfi_bank_liquidity_vault_authority.clone(),
            accounts.marginfi_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.marginfi_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            marginfi_group: &arr[4],
            marginfi_account: &arr[5],
            marginfi_bank: &arr[6],
            marginfi_bank_liquidity_vault: &arr[7],
            marginfi_bank_liquidity_vault_authority: &arr[8],
            marginfi_oracle: &arr[9],
            authority: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            marginfi_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    217, 67, 169, 9, 14, 55, 71, 97,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarginfiSupplyStrategyWithdrawIxArgs {
    pub args: MarginfiSupplyStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyWithdrawIxData(
    pub MarginfiSupplyStrategyWithdrawIxArgs,
);
impl From<MarginfiSupplyStrategyWithdrawIxArgs>
for MarginfiSupplyStrategyWithdrawIxData {
    fn from(args: MarginfiSupplyStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl MarginfiSupplyStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MarginfiSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(MarginfiSupplyStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_supply_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiSupplyStrategyWithdrawKeys,
    args: MarginfiSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MarginfiSupplyStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn marginfi_supply_strategy_withdraw_ix(
    keys: MarginfiSupplyStrategyWithdrawKeys,
    args: MarginfiSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    marginfi_supply_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn marginfi_supply_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MarginfiSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyWithdrawKeys = accounts.into();
    let ix = marginfi_supply_strategy_withdraw_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_supply_strategy_withdraw_invoke(
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MarginfiSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    marginfi_supply_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn marginfi_supply_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MarginfiSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyWithdrawKeys = accounts.into();
    let ix = marginfi_supply_strategy_withdraw_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_supply_strategy_withdraw_invoke_signed(
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MarginfiSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_supply_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn marginfi_supply_strategy_withdraw_verify_account_keys(
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'_, '_>,
    keys: MarginfiSupplyStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.marginfi_bank.key, keys.marginfi_bank),
        (
            *accounts.marginfi_bank_liquidity_vault.key,
            keys.marginfi_bank_liquidity_vault,
        ),
        (
            *accounts.marginfi_bank_liquidity_vault_authority.key,
            keys.marginfi_bank_liquidity_vault_authority,
        ),
        (*accounts.marginfi_oracle.key, keys.marginfi_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.marginfi_program.key, keys.marginfi_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.marginfi_account,
        accounts.marginfi_bank,
        accounts.marginfi_bank_liquidity_vault,
        accounts.marginfi_bank_liquidity_vault_authority,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_supply_strategy_withdraw_verify_writable_privileges(accounts)?;
    marginfi_supply_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct MarginfiSupplyStrategyClaimEmissionsAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub marginfi_group: &'me AccountInfo<'info>,
    pub marginfi_account: &'me AccountInfo<'info>,
    pub marginfi_bank: &'me AccountInfo<'info>,
    pub marginfi_emissions_vault: &'me AccountInfo<'info>,
    pub marginfi_emissions_auth: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub marginfi_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyClaimEmissionsKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub marginfi_group: Pubkey,
    pub marginfi_account: Pubkey,
    pub marginfi_bank: Pubkey,
    pub marginfi_emissions_vault: Pubkey,
    pub marginfi_emissions_auth: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub marginfi_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<MarginfiSupplyStrategyClaimEmissionsAccounts<'_, '_>>
for MarginfiSupplyStrategyClaimEmissionsKeys {
    fn from(accounts: MarginfiSupplyStrategyClaimEmissionsAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            marginfi_group: *accounts.marginfi_group.key,
            marginfi_account: *accounts.marginfi_account.key,
            marginfi_bank: *accounts.marginfi_bank.key,
            marginfi_emissions_vault: *accounts.marginfi_emissions_vault.key,
            marginfi_emissions_auth: *accounts.marginfi_emissions_auth.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            marginfi_program: *accounts.marginfi_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<MarginfiSupplyStrategyClaimEmissionsKeys>
for [AccountMeta; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(keys: MarginfiSupplyStrategyClaimEmissionsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_emissions_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.marginfi_emissions_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.marginfi_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN]>
for MarginfiSupplyStrategyClaimEmissionsKeys {
    fn from(
        pubkeys: [Pubkey; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            marginfi_group: pubkeys[4],
            marginfi_account: pubkeys[5],
            marginfi_bank: pubkeys[6],
            marginfi_emissions_vault: pubkeys[7],
            marginfi_emissions_auth: pubkeys[8],
            authority: pubkeys[9],
            token_program: pubkeys[10],
            marginfi_program: pubkeys[11],
            log_program: pubkeys[12],
        }
    }
}
impl<'info> From<MarginfiSupplyStrategyClaimEmissionsAccounts<'_, 'info>>
for [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.marginfi_group.clone(),
            accounts.marginfi_account.clone(),
            accounts.marginfi_bank.clone(),
            accounts.marginfi_emissions_vault.clone(),
            accounts.marginfi_emissions_auth.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.marginfi_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<'info>; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN],
> for MarginfiSupplyStrategyClaimEmissionsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            marginfi_group: &arr[4],
            marginfi_account: &arr[5],
            marginfi_bank: &arr[6],
            marginfi_emissions_vault: &arr[7],
            marginfi_emissions_auth: &arr[8],
            authority: &arr[9],
            token_program: &arr[10],
            marginfi_program: &arr[11],
            log_program: &arr[12],
        }
    }
}
pub const MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM: [u8; 8usize] = [
    168, 249, 106, 188, 219, 36, 101, 132,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MarginfiSupplyStrategyClaimEmissionsIxData;
impl MarginfiSupplyStrategyClaimEmissionsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marginfi_supply_strategy_claim_emissions_ix_with_program_id(
    program_id: Pubkey,
    keys: MarginfiSupplyStrategyClaimEmissionsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARGINFI_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarginfiSupplyStrategyClaimEmissionsIxData.try_to_vec()?,
    })
}
pub fn marginfi_supply_strategy_claim_emissions_ix(
    keys: MarginfiSupplyStrategyClaimEmissionsKeys,
) -> std::io::Result<Instruction> {
    marginfi_supply_strategy_claim_emissions_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn marginfi_supply_strategy_claim_emissions_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyClaimEmissionsKeys = accounts.into();
    let ix = marginfi_supply_strategy_claim_emissions_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn marginfi_supply_strategy_claim_emissions_invoke(
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'_, '_>,
) -> ProgramResult {
    marginfi_supply_strategy_claim_emissions_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
    )
}
pub fn marginfi_supply_strategy_claim_emissions_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarginfiSupplyStrategyClaimEmissionsKeys = accounts.into();
    let ix = marginfi_supply_strategy_claim_emissions_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marginfi_supply_strategy_claim_emissions_invoke_signed(
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marginfi_supply_strategy_claim_emissions_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn marginfi_supply_strategy_claim_emissions_verify_account_keys(
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'_, '_>,
    keys: MarginfiSupplyStrategyClaimEmissionsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.marginfi_group.key, keys.marginfi_group),
        (*accounts.marginfi_account.key, keys.marginfi_account),
        (*accounts.marginfi_bank.key, keys.marginfi_bank),
        (*accounts.marginfi_emissions_vault.key, keys.marginfi_emissions_vault),
        (*accounts.marginfi_emissions_auth.key, keys.marginfi_emissions_auth),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.marginfi_program.key, keys.marginfi_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_claim_emissions_verify_writable_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault_asset_ata,
        accounts.marginfi_account,
        accounts.marginfi_bank,
        accounts.marginfi_emissions_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_claim_emissions_verify_signer_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn marginfi_supply_strategy_claim_emissions_verify_account_privileges<'me, 'info>(
    accounts: MarginfiSupplyStrategyClaimEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marginfi_supply_strategy_claim_emissions_verify_writable_privileges(accounts)?;
    marginfi_supply_strategy_claim_emissions_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct KlendSupplyStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub asset_pyth_oracle: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub user_metadata: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub obligation_farm: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub reserve_farm_state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub klend_program: &'me AccountInfo<'info>,
    pub kfarms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_pyth_oracle: Pubkey,
    pub scope_prices: Pubkey,
    pub user_metadata: Pubkey,
    pub obligation: Pubkey,
    pub obligation_farm: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
    pub reserve_farm_state: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub klend_program: Pubkey,
    pub kfarms_program: Pubkey,
}
impl From<KlendSupplyStrategyInitAccounts<'_, '_>> for KlendSupplyStrategyInitKeys {
    fn from(accounts: KlendSupplyStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            asset_pyth_oracle: *accounts.asset_pyth_oracle.key,
            scope_prices: *accounts.scope_prices.key,
            user_metadata: *accounts.user_metadata.key,
            obligation: *accounts.obligation.key,
            obligation_farm: *accounts.obligation_farm.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_destination_deposit_collateral: *accounts
                .reserve_destination_deposit_collateral
                .key,
            reserve_farm_state: *accounts.reserve_farm_state.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            klend_program: *accounts.klend_program.key,
            kfarms_program: *accounts.kfarms_program.key,
        }
    }
}
impl From<KlendSupplyStrategyInitKeys>
for [AccountMeta; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: KlendSupplyStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_pyth_oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation_farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.klend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kfarms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyInitKeys {
    fn from(pubkeys: [Pubkey; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            asset_pyth_oracle: pubkeys[3],
            scope_prices: pubkeys[4],
            user_metadata: pubkeys[5],
            obligation: pubkeys[6],
            obligation_farm: pubkeys[7],
            lending_market: pubkeys[8],
            lending_market_authority: pubkeys[9],
            reserve: pubkeys[10],
            reserve_collateral_mint: pubkeys[11],
            reserve_liquidity_supply: pubkeys[12],
            reserve_destination_deposit_collateral: pubkeys[13],
            reserve_farm_state: pubkeys[14],
            authority: pubkeys[15],
            rent: pubkeys[16],
            system_program: pubkeys[17],
            token_program: pubkeys[18],
            klend_program: pubkeys[19],
            kfarms_program: pubkeys[20],
        }
    }
}
impl<'info> From<KlendSupplyStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: KlendSupplyStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.asset_pyth_oracle.clone(),
            accounts.scope_prices.clone(),
            accounts.user_metadata.clone(),
            accounts.obligation.clone(),
            accounts.obligation_farm.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_destination_deposit_collateral.clone(),
            accounts.reserve_farm_state.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.klend_program.clone(),
            accounts.kfarms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            asset_pyth_oracle: &arr[3],
            scope_prices: &arr[4],
            user_metadata: &arr[5],
            obligation: &arr[6],
            obligation_farm: &arr[7],
            lending_market: &arr[8],
            lending_market_authority: &arr[9],
            reserve: &arr[10],
            reserve_collateral_mint: &arr[11],
            reserve_liquidity_supply: &arr[12],
            reserve_destination_deposit_collateral: &arr[13],
            reserve_farm_state: &arr[14],
            authority: &arr[15],
            rent: &arr[16],
            system_program: &arr[17],
            token_program: &arr[18],
            klend_program: &arr[19],
            kfarms_program: &arr[20],
        }
    }
}
pub const KLEND_SUPPLY_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    158, 15, 27, 241, 53, 22, 117, 239,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KlendSupplyStrategyInitIxArgs {
    pub args: KlendSupplyStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyInitIxData(pub KlendSupplyStrategyInitIxArgs);
impl From<KlendSupplyStrategyInitIxArgs> for KlendSupplyStrategyInitIxData {
    fn from(args: KlendSupplyStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl KlendSupplyStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KLEND_SUPPLY_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <KlendSupplyStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(KlendSupplyStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KLEND_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn klend_supply_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: KlendSupplyStrategyInitKeys,
    args: KlendSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: KlendSupplyStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn klend_supply_strategy_init_ix(
    keys: KlendSupplyStrategyInitKeys,
    args: KlendSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    klend_supply_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn klend_supply_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyInitAccounts<'_, '_>,
    args: KlendSupplyStrategyInitIxArgs,
) -> ProgramResult {
    let keys: KlendSupplyStrategyInitKeys = accounts.into();
    let ix = klend_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn klend_supply_strategy_init_invoke(
    accounts: KlendSupplyStrategyInitAccounts<'_, '_>,
    args: KlendSupplyStrategyInitIxArgs,
) -> ProgramResult {
    klend_supply_strategy_init_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn klend_supply_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyInitAccounts<'_, '_>,
    args: KlendSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KlendSupplyStrategyInitKeys = accounts.into();
    let ix = klend_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn klend_supply_strategy_init_invoke_signed(
    accounts: KlendSupplyStrategyInitAccounts<'_, '_>,
    args: KlendSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    klend_supply_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn klend_supply_strategy_init_verify_account_keys(
    accounts: KlendSupplyStrategyInitAccounts<'_, '_>,
    keys: KlendSupplyStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.asset_pyth_oracle.key, keys.asset_pyth_oracle),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.user_metadata.key, keys.user_metadata),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.obligation_farm.key, keys.obligation_farm),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (
            *accounts.reserve_destination_deposit_collateral.key,
            keys.reserve_destination_deposit_collateral,
        ),
        (*accounts.reserve_farm_state.key, keys.reserve_farm_state),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.klend_program.key, keys.klend_program),
        (*accounts.kfarms_program.key, keys.kfarms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.asset_pyth_oracle,
        accounts.scope_prices,
        accounts.user_metadata,
        accounts.obligation,
        accounts.obligation_farm,
        accounts.lending_market,
        accounts.lending_market_authority,
        accounts.reserve,
        accounts.reserve_farm_state,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    klend_supply_strategy_init_verify_writable_privileges(accounts)?;
    klend_supply_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct KlendSupplyStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub vault_collateral_ata: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub klend_program: &'me AccountInfo<'info>,
    pub kfarms_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyDepositKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
    pub vault_collateral_ata: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub oracle: Pubkey,
    pub scope_prices: Pubkey,
    pub obligation: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub klend_program: Pubkey,
    pub kfarms_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<KlendSupplyStrategyDepositAccounts<'_, '_>>
for KlendSupplyStrategyDepositKeys {
    fn from(accounts: KlendSupplyStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_destination_deposit_collateral: *accounts
                .reserve_destination_deposit_collateral
                .key,
            vault_collateral_ata: *accounts.vault_collateral_ata.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            oracle: *accounts.oracle.key,
            scope_prices: *accounts.scope_prices.key,
            obligation: *accounts.obligation.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            klend_program: *accounts.klend_program.key,
            kfarms_program: *accounts.kfarms_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<KlendSupplyStrategyDepositKeys>
for [AccountMeta; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: KlendSupplyStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_collateral_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.klend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kfarms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyDepositKeys {
    fn from(pubkeys: [Pubkey; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            reserve: pubkeys[4],
            reserve_liquidity_supply: pubkeys[5],
            reserve_collateral_mint: pubkeys[6],
            reserve_destination_deposit_collateral: pubkeys[7],
            vault_collateral_ata: pubkeys[8],
            lending_market: pubkeys[9],
            lending_market_authority: pubkeys[10],
            oracle: pubkeys[11],
            scope_prices: pubkeys[12],
            obligation: pubkeys[13],
            authority: pubkeys[14],
            rent: pubkeys[15],
            instructions_sysvar: pubkeys[16],
            system_program: pubkeys[17],
            collateral_token_program: pubkeys[18],
            asset_token_program: pubkeys[19],
            klend_program: pubkeys[20],
            kfarms_program: pubkeys[21],
            log_program: pubkeys[22],
        }
    }
}
impl<'info> From<KlendSupplyStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: KlendSupplyStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_destination_deposit_collateral.clone(),
            accounts.vault_collateral_ata.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.oracle.clone(),
            accounts.scope_prices.clone(),
            accounts.obligation.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.klend_program.clone(),
            accounts.kfarms_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            reserve: &arr[4],
            reserve_liquidity_supply: &arr[5],
            reserve_collateral_mint: &arr[6],
            reserve_destination_deposit_collateral: &arr[7],
            vault_collateral_ata: &arr[8],
            lending_market: &arr[9],
            lending_market_authority: &arr[10],
            oracle: &arr[11],
            scope_prices: &arr[12],
            obligation: &arr[13],
            authority: &arr[14],
            rent: &arr[15],
            instructions_sysvar: &arr[16],
            system_program: &arr[17],
            collateral_token_program: &arr[18],
            asset_token_program: &arr[19],
            klend_program: &arr[20],
            kfarms_program: &arr[21],
            log_program: &arr[22],
        }
    }
}
pub const KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    112, 30, 167, 72, 223, 248, 56, 192,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KlendSupplyStrategyDepositIxArgs {
    pub args: KlendSupplyStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyDepositIxData(pub KlendSupplyStrategyDepositIxArgs);
impl From<KlendSupplyStrategyDepositIxArgs> for KlendSupplyStrategyDepositIxData {
    fn from(args: KlendSupplyStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl KlendSupplyStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <KlendSupplyStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(KlendSupplyStrategyDepositIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn klend_supply_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: KlendSupplyStrategyDepositKeys,
    args: KlendSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: KlendSupplyStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn klend_supply_strategy_deposit_ix(
    keys: KlendSupplyStrategyDepositKeys,
    args: KlendSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    klend_supply_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn klend_supply_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyDepositAccounts<'_, '_>,
    args: KlendSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: KlendSupplyStrategyDepositKeys = accounts.into();
    let ix = klend_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn klend_supply_strategy_deposit_invoke(
    accounts: KlendSupplyStrategyDepositAccounts<'_, '_>,
    args: KlendSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    klend_supply_strategy_deposit_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn klend_supply_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyDepositAccounts<'_, '_>,
    args: KlendSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KlendSupplyStrategyDepositKeys = accounts.into();
    let ix = klend_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn klend_supply_strategy_deposit_invoke_signed(
    accounts: KlendSupplyStrategyDepositAccounts<'_, '_>,
    args: KlendSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    klend_supply_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn klend_supply_strategy_deposit_verify_account_keys(
    accounts: KlendSupplyStrategyDepositAccounts<'_, '_>,
    keys: KlendSupplyStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (
            *accounts.reserve_destination_deposit_collateral.key,
            keys.reserve_destination_deposit_collateral,
        ),
        (*accounts.vault_collateral_ata.key, keys.vault_collateral_ata),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.klend_program.key, keys.klend_program),
        (*accounts.kfarms_program.key, keys.kfarms_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.asset_mint,
        accounts.vault_asset_ata,
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_destination_deposit_collateral,
        accounts.vault_collateral_ata,
        accounts.lending_market_authority,
        accounts.obligation,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    klend_supply_strategy_deposit_verify_writable_privileges(accounts)?;
    klend_supply_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 23;
#[derive(Copy, Clone, Debug)]
pub struct KlendSupplyStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub vault_collateral_ata: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub asset_token_program: &'me AccountInfo<'info>,
    pub klend_program: &'me AccountInfo<'info>,
    pub kfarms_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
    pub vault_collateral_ata: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub oracle: Pubkey,
    pub scope_prices: Pubkey,
    pub obligation: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub asset_token_program: Pubkey,
    pub klend_program: Pubkey,
    pub kfarms_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<KlendSupplyStrategyWithdrawAccounts<'_, '_>>
for KlendSupplyStrategyWithdrawKeys {
    fn from(accounts: KlendSupplyStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_destination_deposit_collateral: *accounts
                .reserve_destination_deposit_collateral
                .key,
            vault_collateral_ata: *accounts.vault_collateral_ata.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            oracle: *accounts.oracle.key,
            scope_prices: *accounts.scope_prices.key,
            obligation: *accounts.obligation.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            system_program: *accounts.system_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            asset_token_program: *accounts.asset_token_program.key,
            klend_program: *accounts.klend_program.key,
            kfarms_program: *accounts.kfarms_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<KlendSupplyStrategyWithdrawKeys>
for [AccountMeta; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: KlendSupplyStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_collateral_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.klend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kfarms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyWithdrawKeys {
    fn from(pubkeys: [Pubkey; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            reserve: pubkeys[4],
            reserve_liquidity_supply: pubkeys[5],
            reserve_collateral_mint: pubkeys[6],
            reserve_destination_deposit_collateral: pubkeys[7],
            vault_collateral_ata: pubkeys[8],
            lending_market: pubkeys[9],
            lending_market_authority: pubkeys[10],
            oracle: pubkeys[11],
            scope_prices: pubkeys[12],
            obligation: pubkeys[13],
            authority: pubkeys[14],
            rent: pubkeys[15],
            instructions_sysvar: pubkeys[16],
            system_program: pubkeys[17],
            collateral_token_program: pubkeys[18],
            asset_token_program: pubkeys[19],
            klend_program: pubkeys[20],
            kfarms_program: pubkeys[21],
            log_program: pubkeys[22],
        }
    }
}
impl<'info> From<KlendSupplyStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: KlendSupplyStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_destination_deposit_collateral.clone(),
            accounts.vault_collateral_ata.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.oracle.clone(),
            accounts.scope_prices.clone(),
            accounts.obligation.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.system_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.asset_token_program.clone(),
            accounts.klend_program.clone(),
            accounts.kfarms_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            reserve: &arr[4],
            reserve_liquidity_supply: &arr[5],
            reserve_collateral_mint: &arr[6],
            reserve_destination_deposit_collateral: &arr[7],
            vault_collateral_ata: &arr[8],
            lending_market: &arr[9],
            lending_market_authority: &arr[10],
            oracle: &arr[11],
            scope_prices: &arr[12],
            obligation: &arr[13],
            authority: &arr[14],
            rent: &arr[15],
            instructions_sysvar: &arr[16],
            system_program: &arr[17],
            collateral_token_program: &arr[18],
            asset_token_program: &arr[19],
            klend_program: &arr[20],
            kfarms_program: &arr[21],
            log_program: &arr[22],
        }
    }
}
pub const KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    198, 165, 26, 210, 209, 50, 56, 192,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KlendSupplyStrategyWithdrawIxArgs {
    pub args: KlendSupplyStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyWithdrawIxData(pub KlendSupplyStrategyWithdrawIxArgs);
impl From<KlendSupplyStrategyWithdrawIxArgs> for KlendSupplyStrategyWithdrawIxData {
    fn from(args: KlendSupplyStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl KlendSupplyStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <KlendSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(KlendSupplyStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn klend_supply_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: KlendSupplyStrategyWithdrawKeys,
    args: KlendSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: KlendSupplyStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn klend_supply_strategy_withdraw_ix(
    keys: KlendSupplyStrategyWithdrawKeys,
    args: KlendSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    klend_supply_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn klend_supply_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: KlendSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: KlendSupplyStrategyWithdrawKeys = accounts.into();
    let ix = klend_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn klend_supply_strategy_withdraw_invoke(
    accounts: KlendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: KlendSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    klend_supply_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn klend_supply_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: KlendSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KlendSupplyStrategyWithdrawKeys = accounts.into();
    let ix = klend_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn klend_supply_strategy_withdraw_invoke_signed(
    accounts: KlendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: KlendSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    klend_supply_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn klend_supply_strategy_withdraw_verify_account_keys(
    accounts: KlendSupplyStrategyWithdrawAccounts<'_, '_>,
    keys: KlendSupplyStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (
            *accounts.reserve_destination_deposit_collateral.key,
            keys.reserve_destination_deposit_collateral,
        ),
        (*accounts.vault_collateral_ata.key, keys.vault_collateral_ata),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.asset_token_program.key, keys.asset_token_program),
        (*accounts.klend_program.key, keys.klend_program),
        (*accounts.kfarms_program.key, keys.kfarms_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.asset_mint,
        accounts.vault_asset_ata,
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_destination_deposit_collateral,
        accounts.vault_collateral_ata,
        accounts.lending_market_authority,
        accounts.obligation,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    klend_supply_strategy_withdraw_verify_writable_privileges(accounts)?;
    klend_supply_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct KlendSupplyStrategyClaimEmissionsAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
    pub user_state: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub kfarms_global_config: &'me AccountInfo<'info>,
    pub kfarms_vaults_authority: &'me AccountInfo<'info>,
    pub kfarms_rewards_vault: &'me AccountInfo<'info>,
    pub kfarms_rewards_treasury_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub kfarms_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyClaimEmissionsKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub scope_prices: Pubkey,
    pub user_state: Pubkey,
    pub farm_state: Pubkey,
    pub kfarms_global_config: Pubkey,
    pub kfarms_vaults_authority: Pubkey,
    pub kfarms_rewards_vault: Pubkey,
    pub kfarms_rewards_treasury_vault: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub kfarms_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<KlendSupplyStrategyClaimEmissionsAccounts<'_, '_>>
for KlendSupplyStrategyClaimEmissionsKeys {
    fn from(accounts: KlendSupplyStrategyClaimEmissionsAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            scope_prices: *accounts.scope_prices.key,
            user_state: *accounts.user_state.key,
            farm_state: *accounts.farm_state.key,
            kfarms_global_config: *accounts.kfarms_global_config.key,
            kfarms_vaults_authority: *accounts.kfarms_vaults_authority.key,
            kfarms_rewards_vault: *accounts.kfarms_rewards_vault.key,
            kfarms_rewards_treasury_vault: *accounts.kfarms_rewards_treasury_vault.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            kfarms_program: *accounts.kfarms_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<KlendSupplyStrategyClaimEmissionsKeys>
for [AccountMeta; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(keys: KlendSupplyStrategyClaimEmissionsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kfarms_global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kfarms_vaults_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kfarms_rewards_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.kfarms_rewards_treasury_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.kfarms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyClaimEmissionsKeys {
    fn from(
        pubkeys: [Pubkey; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            scope_prices: pubkeys[4],
            user_state: pubkeys[5],
            farm_state: pubkeys[6],
            kfarms_global_config: pubkeys[7],
            kfarms_vaults_authority: pubkeys[8],
            kfarms_rewards_vault: pubkeys[9],
            kfarms_rewards_treasury_vault: pubkeys[10],
            authority: pubkeys[11],
            token_program: pubkeys[12],
            kfarms_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<KlendSupplyStrategyClaimEmissionsAccounts<'_, 'info>>
for [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN] {
    fn from(accounts: KlendSupplyStrategyClaimEmissionsAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.scope_prices.clone(),
            accounts.user_state.clone(),
            accounts.farm_state.clone(),
            accounts.kfarms_global_config.clone(),
            accounts.kfarms_vaults_authority.clone(),
            accounts.kfarms_rewards_vault.clone(),
            accounts.kfarms_rewards_treasury_vault.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.kfarms_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN]>
for KlendSupplyStrategyClaimEmissionsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            scope_prices: &arr[4],
            user_state: &arr[5],
            farm_state: &arr[6],
            kfarms_global_config: &arr[7],
            kfarms_vaults_authority: &arr[8],
            kfarms_rewards_vault: &arr[9],
            kfarms_rewards_treasury_vault: &arr[10],
            authority: &arr[11],
            token_program: &arr[12],
            kfarms_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM: [u8; 8usize] = [
    134, 218, 152, 220, 186, 129, 52, 175,
];
#[derive(Clone, Debug, PartialEq)]
pub struct KlendSupplyStrategyClaimEmissionsIxData;
impl KlendSupplyStrategyClaimEmissionsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn klend_supply_strategy_claim_emissions_ix_with_program_id(
    program_id: Pubkey,
    keys: KlendSupplyStrategyClaimEmissionsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; KLEND_SUPPLY_STRATEGY_CLAIM_EMISSIONS_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: KlendSupplyStrategyClaimEmissionsIxData.try_to_vec()?,
    })
}
pub fn klend_supply_strategy_claim_emissions_ix(
    keys: KlendSupplyStrategyClaimEmissionsKeys,
) -> std::io::Result<Instruction> {
    klend_supply_strategy_claim_emissions_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn klend_supply_strategy_claim_emissions_invoke_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: KlendSupplyStrategyClaimEmissionsKeys = accounts.into();
    let ix = klend_supply_strategy_claim_emissions_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn klend_supply_strategy_claim_emissions_invoke(
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'_, '_>,
) -> ProgramResult {
    klend_supply_strategy_claim_emissions_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
    )
}
pub fn klend_supply_strategy_claim_emissions_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: KlendSupplyStrategyClaimEmissionsKeys = accounts.into();
    let ix = klend_supply_strategy_claim_emissions_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn klend_supply_strategy_claim_emissions_invoke_signed(
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    klend_supply_strategy_claim_emissions_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn klend_supply_strategy_claim_emissions_verify_account_keys(
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'_, '_>,
    keys: KlendSupplyStrategyClaimEmissionsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.scope_prices.key, keys.scope_prices),
        (*accounts.user_state.key, keys.user_state),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.kfarms_global_config.key, keys.kfarms_global_config),
        (*accounts.kfarms_vaults_authority.key, keys.kfarms_vaults_authority),
        (*accounts.kfarms_rewards_vault.key, keys.kfarms_rewards_vault),
        (
            *accounts.kfarms_rewards_treasury_vault.key,
            keys.kfarms_rewards_treasury_vault,
        ),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.kfarms_program.key, keys.kfarms_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_claim_emissions_verify_writable_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.user_state,
        accounts.farm_state,
        accounts.kfarms_rewards_vault,
        accounts.kfarms_rewards_treasury_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_claim_emissions_verify_signer_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn klend_supply_strategy_claim_emissions_verify_account_privileges<'me, 'info>(
    accounts: KlendSupplyStrategyClaimEmissionsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    klend_supply_strategy_claim_emissions_verify_writable_privileges(accounts)?;
    klend_supply_strategy_claim_emissions_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SolendSupplyStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub asset_pyth_oracle: &'me AccountInfo<'info>,
    pub asset_switchboard_oracle: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_deposit_collateral_ata: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub solend_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolendSupplyStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_pyth_oracle: Pubkey,
    pub asset_switchboard_oracle: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_deposit_collateral_ata: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub solend_program: Pubkey,
}
impl From<SolendSupplyStrategyInitAccounts<'_, '_>> for SolendSupplyStrategyInitKeys {
    fn from(accounts: SolendSupplyStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            asset_pyth_oracle: *accounts.asset_pyth_oracle.key,
            asset_switchboard_oracle: *accounts.asset_switchboard_oracle.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_deposit_collateral_ata: *accounts.reserve_deposit_collateral_ata.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            solend_program: *accounts.solend_program.key,
        }
    }
}
impl From<SolendSupplyStrategyInitKeys>
for [AccountMeta; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SolendSupplyStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_switchboard_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_deposit_collateral_ata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.solend_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for SolendSupplyStrategyInitKeys {
    fn from(pubkeys: [Pubkey; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            asset_pyth_oracle: pubkeys[3],
            asset_switchboard_oracle: pubkeys[4],
            obligation: pubkeys[5],
            lending_market: pubkeys[6],
            lending_market_authority: pubkeys[7],
            reserve: pubkeys[8],
            reserve_collateral_mint: pubkeys[9],
            reserve_liquidity_supply: pubkeys[10],
            reserve_deposit_collateral_ata: pubkeys[11],
            authority: pubkeys[12],
            rent: pubkeys[13],
            system_program: pubkeys[14],
            token_program: pubkeys[15],
            solend_program: pubkeys[16],
        }
    }
}
impl<'info> From<SolendSupplyStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolendSupplyStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.asset_pyth_oracle.clone(),
            accounts.asset_switchboard_oracle.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_deposit_collateral_ata.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.solend_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for SolendSupplyStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            asset_pyth_oracle: &arr[3],
            asset_switchboard_oracle: &arr[4],
            obligation: &arr[5],
            lending_market: &arr[6],
            lending_market_authority: &arr[7],
            reserve: &arr[8],
            reserve_collateral_mint: &arr[9],
            reserve_liquidity_supply: &arr[10],
            reserve_deposit_collateral_ata: &arr[11],
            authority: &arr[12],
            rent: &arr[13],
            system_program: &arr[14],
            token_program: &arr[15],
            solend_program: &arr[16],
        }
    }
}
pub const SOLEND_SUPPLY_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    96, 112, 209, 132, 136, 45, 137, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SolendSupplyStrategyInitIxArgs {
    pub args: SolendSupplyStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolendSupplyStrategyInitIxData(pub SolendSupplyStrategyInitIxArgs);
impl From<SolendSupplyStrategyInitIxArgs> for SolendSupplyStrategyInitIxData {
    fn from(args: SolendSupplyStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl SolendSupplyStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOLEND_SUPPLY_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SolendSupplyStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(SolendSupplyStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOLEND_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn solend_supply_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: SolendSupplyStrategyInitKeys,
    args: SolendSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SolendSupplyStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn solend_supply_strategy_init_ix(
    keys: SolendSupplyStrategyInitKeys,
    args: SolendSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    solend_supply_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn solend_supply_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolendSupplyStrategyInitAccounts<'_, '_>,
    args: SolendSupplyStrategyInitIxArgs,
) -> ProgramResult {
    let keys: SolendSupplyStrategyInitKeys = accounts.into();
    let ix = solend_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn solend_supply_strategy_init_invoke(
    accounts: SolendSupplyStrategyInitAccounts<'_, '_>,
    args: SolendSupplyStrategyInitIxArgs,
) -> ProgramResult {
    solend_supply_strategy_init_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn solend_supply_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolendSupplyStrategyInitAccounts<'_, '_>,
    args: SolendSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolendSupplyStrategyInitKeys = accounts.into();
    let ix = solend_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn solend_supply_strategy_init_invoke_signed(
    accounts: SolendSupplyStrategyInitAccounts<'_, '_>,
    args: SolendSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    solend_supply_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn solend_supply_strategy_init_verify_account_keys(
    accounts: SolendSupplyStrategyInitAccounts<'_, '_>,
    keys: SolendSupplyStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.asset_pyth_oracle.key, keys.asset_pyth_oracle),
        (*accounts.asset_switchboard_oracle.key, keys.asset_switchboard_oracle),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (
            *accounts.reserve_deposit_collateral_ata.key,
            keys.reserve_deposit_collateral_ata,
        ),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.solend_program.key, keys.solend_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.obligation,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    solend_supply_strategy_init_verify_writable_privileges(accounts)?;
    solend_supply_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct SolendSupplyStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_pyth_oracle: &'me AccountInfo<'info>,
    pub asset_switchboard_oracle: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub vault_collateral_ata: &'me AccountInfo<'info>,
    pub deposit_collateral_ata: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub solend_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolendSupplyStrategyDepositKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub asset_pyth_oracle: Pubkey,
    pub asset_switchboard_oracle: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub vault_collateral_ata: Pubkey,
    pub deposit_collateral_ata: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub solend_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<SolendSupplyStrategyDepositAccounts<'_, '_>>
for SolendSupplyStrategyDepositKeys {
    fn from(accounts: SolendSupplyStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            asset_pyth_oracle: *accounts.asset_pyth_oracle.key,
            asset_switchboard_oracle: *accounts.asset_switchboard_oracle.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            vault_collateral_ata: *accounts.vault_collateral_ata.key,
            deposit_collateral_ata: *accounts.deposit_collateral_ata.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            solend_program: *accounts.solend_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<SolendSupplyStrategyDepositKeys>
for [AccountMeta; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SolendSupplyStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_switchboard_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_collateral_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_collateral_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.solend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for SolendSupplyStrategyDepositKeys {
    fn from(pubkeys: [Pubkey; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            asset_pyth_oracle: pubkeys[4],
            asset_switchboard_oracle: pubkeys[5],
            obligation: pubkeys[6],
            lending_market: pubkeys[7],
            lending_market_authority: pubkeys[8],
            reserve: pubkeys[9],
            reserve_liquidity_supply: pubkeys[10],
            reserve_collateral_mint: pubkeys[11],
            vault_collateral_ata: pubkeys[12],
            deposit_collateral_ata: pubkeys[13],
            authority: pubkeys[14],
            rent: pubkeys[15],
            system_program: pubkeys[16],
            token_program: pubkeys[17],
            solend_program: pubkeys[18],
            log_program: pubkeys[19],
        }
    }
}
impl<'info> From<SolendSupplyStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolendSupplyStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.asset_pyth_oracle.clone(),
            accounts.asset_switchboard_oracle.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.vault_collateral_ata.clone(),
            accounts.deposit_collateral_ata.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.solend_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for SolendSupplyStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            asset_pyth_oracle: &arr[4],
            asset_switchboard_oracle: &arr[5],
            obligation: &arr[6],
            lending_market: &arr[7],
            lending_market_authority: &arr[8],
            reserve: &arr[9],
            reserve_liquidity_supply: &arr[10],
            reserve_collateral_mint: &arr[11],
            vault_collateral_ata: &arr[12],
            deposit_collateral_ata: &arr[13],
            authority: &arr[14],
            rent: &arr[15],
            system_program: &arr[16],
            token_program: &arr[17],
            solend_program: &arr[18],
            log_program: &arr[19],
        }
    }
}
pub const SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    174, 98, 61, 32, 29, 132, 212, 159,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SolendSupplyStrategyDepositIxArgs {
    pub args: SolendSupplyStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolendSupplyStrategyDepositIxData(pub SolendSupplyStrategyDepositIxArgs);
impl From<SolendSupplyStrategyDepositIxArgs> for SolendSupplyStrategyDepositIxData {
    fn from(args: SolendSupplyStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl SolendSupplyStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SolendSupplyStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(SolendSupplyStrategyDepositIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn solend_supply_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: SolendSupplyStrategyDepositKeys,
    args: SolendSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SolendSupplyStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn solend_supply_strategy_deposit_ix(
    keys: SolendSupplyStrategyDepositKeys,
    args: SolendSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    solend_supply_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn solend_supply_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolendSupplyStrategyDepositAccounts<'_, '_>,
    args: SolendSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: SolendSupplyStrategyDepositKeys = accounts.into();
    let ix = solend_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn solend_supply_strategy_deposit_invoke(
    accounts: SolendSupplyStrategyDepositAccounts<'_, '_>,
    args: SolendSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    solend_supply_strategy_deposit_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn solend_supply_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolendSupplyStrategyDepositAccounts<'_, '_>,
    args: SolendSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolendSupplyStrategyDepositKeys = accounts.into();
    let ix = solend_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn solend_supply_strategy_deposit_invoke_signed(
    accounts: SolendSupplyStrategyDepositAccounts<'_, '_>,
    args: SolendSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    solend_supply_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn solend_supply_strategy_deposit_verify_account_keys(
    accounts: SolendSupplyStrategyDepositAccounts<'_, '_>,
    keys: SolendSupplyStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_pyth_oracle.key, keys.asset_pyth_oracle),
        (*accounts.asset_switchboard_oracle.key, keys.asset_switchboard_oracle),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.vault_collateral_ata.key, keys.vault_collateral_ata),
        (*accounts.deposit_collateral_ata.key, keys.deposit_collateral_ata),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.solend_program.key, keys.solend_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.obligation,
        accounts.lending_market,
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.vault_collateral_ata,
        accounts.deposit_collateral_ata,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    solend_supply_strategy_deposit_verify_writable_privileges(accounts)?;
    solend_supply_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct SolendSupplyStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_pyth_oracle: &'me AccountInfo<'info>,
    pub asset_switchboard_oracle: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub vault_collateral_ata: &'me AccountInfo<'info>,
    pub withdraw_collateral_ata: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub solend_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolendSupplyStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub asset_pyth_oracle: Pubkey,
    pub asset_switchboard_oracle: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub vault_collateral_ata: Pubkey,
    pub withdraw_collateral_ata: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub solend_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<SolendSupplyStrategyWithdrawAccounts<'_, '_>>
for SolendSupplyStrategyWithdrawKeys {
    fn from(accounts: SolendSupplyStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            asset_pyth_oracle: *accounts.asset_pyth_oracle.key,
            asset_switchboard_oracle: *accounts.asset_switchboard_oracle.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            vault_collateral_ata: *accounts.vault_collateral_ata.key,
            withdraw_collateral_ata: *accounts.withdraw_collateral_ata.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            solend_program: *accounts.solend_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<SolendSupplyStrategyWithdrawKeys>
for [AccountMeta; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: SolendSupplyStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_switchboard_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_collateral_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_collateral_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.solend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for SolendSupplyStrategyWithdrawKeys {
    fn from(pubkeys: [Pubkey; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            asset_pyth_oracle: pubkeys[4],
            asset_switchboard_oracle: pubkeys[5],
            obligation: pubkeys[6],
            lending_market: pubkeys[7],
            lending_market_authority: pubkeys[8],
            reserve: pubkeys[9],
            reserve_liquidity_supply: pubkeys[10],
            reserve_collateral_mint: pubkeys[11],
            vault_collateral_ata: pubkeys[12],
            withdraw_collateral_ata: pubkeys[13],
            authority: pubkeys[14],
            rent: pubkeys[15],
            system_program: pubkeys[16],
            token_program: pubkeys[17],
            solend_program: pubkeys[18],
            log_program: pubkeys[19],
        }
    }
}
impl<'info> From<SolendSupplyStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolendSupplyStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.asset_pyth_oracle.clone(),
            accounts.asset_switchboard_oracle.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.vault_collateral_ata.clone(),
            accounts.withdraw_collateral_ata.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.solend_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for SolendSupplyStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            asset_pyth_oracle: &arr[4],
            asset_switchboard_oracle: &arr[5],
            obligation: &arr[6],
            lending_market: &arr[7],
            lending_market_authority: &arr[8],
            reserve: &arr[9],
            reserve_liquidity_supply: &arr[10],
            reserve_collateral_mint: &arr[11],
            vault_collateral_ata: &arr[12],
            withdraw_collateral_ata: &arr[13],
            authority: &arr[14],
            rent: &arr[15],
            system_program: &arr[16],
            token_program: &arr[17],
            solend_program: &arr[18],
            log_program: &arr[19],
        }
    }
}
pub const SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    116, 25, 66, 127, 158, 245, 152, 244,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SolendSupplyStrategyWithdrawIxArgs {
    pub args: SolendSupplyStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolendSupplyStrategyWithdrawIxData(pub SolendSupplyStrategyWithdrawIxArgs);
impl From<SolendSupplyStrategyWithdrawIxArgs> for SolendSupplyStrategyWithdrawIxData {
    fn from(args: SolendSupplyStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl SolendSupplyStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <SolendSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(SolendSupplyStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn solend_supply_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: SolendSupplyStrategyWithdrawKeys,
    args: SolendSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SolendSupplyStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn solend_supply_strategy_withdraw_ix(
    keys: SolendSupplyStrategyWithdrawKeys,
    args: SolendSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    solend_supply_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn solend_supply_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: SolendSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: SolendSupplyStrategyWithdrawKeys = accounts.into();
    let ix = solend_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn solend_supply_strategy_withdraw_invoke(
    accounts: SolendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: SolendSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    solend_supply_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn solend_supply_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: SolendSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolendSupplyStrategyWithdrawKeys = accounts.into();
    let ix = solend_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn solend_supply_strategy_withdraw_invoke_signed(
    accounts: SolendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: SolendSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    solend_supply_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn solend_supply_strategy_withdraw_verify_account_keys(
    accounts: SolendSupplyStrategyWithdrawAccounts<'_, '_>,
    keys: SolendSupplyStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_pyth_oracle.key, keys.asset_pyth_oracle),
        (*accounts.asset_switchboard_oracle.key, keys.asset_switchboard_oracle),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.vault_collateral_ata.key, keys.vault_collateral_ata),
        (*accounts.withdraw_collateral_ata.key, keys.withdraw_collateral_ata),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.solend_program.key, keys.solend_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.obligation,
        accounts.lending_market,
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.vault_collateral_ata,
        accounts.withdraw_collateral_ata,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn solend_supply_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: SolendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    solend_supply_strategy_withdraw_verify_writable_privileges(accounts)?;
    solend_supply_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct MangoSupplyStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub mango_group: &'me AccountInfo<'info>,
    pub mango_account: &'me AccountInfo<'info>,
    pub mango_bank: &'me AccountInfo<'info>,
    pub mango_vault: &'me AccountInfo<'info>,
    pub pyth_oracle: &'me AccountInfo<'info>,
    pub switchboard_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub mango_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MangoSupplyStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub mango_group: Pubkey,
    pub mango_account: Pubkey,
    pub mango_bank: Pubkey,
    pub mango_vault: Pubkey,
    pub pyth_oracle: Pubkey,
    pub switchboard_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub mango_program: Pubkey,
}
impl From<MangoSupplyStrategyInitAccounts<'_, '_>> for MangoSupplyStrategyInitKeys {
    fn from(accounts: MangoSupplyStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            mango_group: *accounts.mango_group.key,
            mango_account: *accounts.mango_account.key,
            mango_bank: *accounts.mango_bank.key,
            mango_vault: *accounts.mango_vault.key,
            pyth_oracle: *accounts.pyth_oracle.key,
            switchboard_oracle: *accounts.switchboard_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            mango_program: *accounts.mango_program.key,
        }
    }
}
impl From<MangoSupplyStrategyInitKeys>
for [AccountMeta; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: MangoSupplyStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mango_bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for MangoSupplyStrategyInitKeys {
    fn from(pubkeys: [Pubkey; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            mango_group: pubkeys[3],
            mango_account: pubkeys[4],
            mango_bank: pubkeys[5],
            mango_vault: pubkeys[6],
            pyth_oracle: pubkeys[7],
            switchboard_oracle: pubkeys[8],
            authority: pubkeys[9],
            system_program: pubkeys[10],
            mango_program: pubkeys[11],
        }
    }
}
impl<'info> From<MangoSupplyStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MangoSupplyStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.mango_group.clone(),
            accounts.mango_account.clone(),
            accounts.mango_bank.clone(),
            accounts.mango_vault.clone(),
            accounts.pyth_oracle.clone(),
            accounts.switchboard_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.mango_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for MangoSupplyStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            mango_group: &arr[3],
            mango_account: &arr[4],
            mango_bank: &arr[5],
            mango_vault: &arr[6],
            pyth_oracle: &arr[7],
            switchboard_oracle: &arr[8],
            authority: &arr[9],
            system_program: &arr[10],
            mango_program: &arr[11],
        }
    }
}
pub const MANGO_SUPPLY_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    171, 196, 79, 146, 67, 46, 23, 124,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MangoSupplyStrategyInitIxArgs {
    pub args: MangoSupplyStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MangoSupplyStrategyInitIxData(pub MangoSupplyStrategyInitIxArgs);
impl From<MangoSupplyStrategyInitIxArgs> for MangoSupplyStrategyInitIxData {
    fn from(args: MangoSupplyStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl MangoSupplyStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANGO_SUPPLY_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MangoSupplyStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(MangoSupplyStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANGO_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mango_supply_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: MangoSupplyStrategyInitKeys,
    args: MangoSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MANGO_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: MangoSupplyStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mango_supply_strategy_init_ix(
    keys: MangoSupplyStrategyInitKeys,
    args: MangoSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    mango_supply_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn mango_supply_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MangoSupplyStrategyInitAccounts<'_, '_>,
    args: MangoSupplyStrategyInitIxArgs,
) -> ProgramResult {
    let keys: MangoSupplyStrategyInitKeys = accounts.into();
    let ix = mango_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mango_supply_strategy_init_invoke(
    accounts: MangoSupplyStrategyInitAccounts<'_, '_>,
    args: MangoSupplyStrategyInitIxArgs,
) -> ProgramResult {
    mango_supply_strategy_init_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn mango_supply_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MangoSupplyStrategyInitAccounts<'_, '_>,
    args: MangoSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MangoSupplyStrategyInitKeys = accounts.into();
    let ix = mango_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mango_supply_strategy_init_invoke_signed(
    accounts: MangoSupplyStrategyInitAccounts<'_, '_>,
    args: MangoSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mango_supply_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mango_supply_strategy_init_verify_account_keys(
    accounts: MangoSupplyStrategyInitAccounts<'_, '_>,
    keys: MangoSupplyStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.mango_group.key, keys.mango_group),
        (*accounts.mango_account.key, keys.mango_account),
        (*accounts.mango_bank.key, keys.mango_bank),
        (*accounts.mango_vault.key, keys.mango_vault),
        (*accounts.pyth_oracle.key, keys.pyth_oracle),
        (*accounts.switchboard_oracle.key, keys.switchboard_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.mango_program.key, keys.mango_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.mango_account,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mango_supply_strategy_init_verify_writable_privileges(accounts)?;
    mango_supply_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MangoSupplyStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub mango_group: &'me AccountInfo<'info>,
    pub mango_account: &'me AccountInfo<'info>,
    pub mango_bank: &'me AccountInfo<'info>,
    pub mango_vault: &'me AccountInfo<'info>,
    pub pyth_oracle: &'me AccountInfo<'info>,
    pub switchboard_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub mango_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MangoSupplyStrategyDepositKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub mango_group: Pubkey,
    pub mango_account: Pubkey,
    pub mango_bank: Pubkey,
    pub mango_vault: Pubkey,
    pub pyth_oracle: Pubkey,
    pub switchboard_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub mango_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<MangoSupplyStrategyDepositAccounts<'_, '_>>
for MangoSupplyStrategyDepositKeys {
    fn from(accounts: MangoSupplyStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            mango_group: *accounts.mango_group.key,
            mango_account: *accounts.mango_account.key,
            mango_bank: *accounts.mango_bank.key,
            mango_vault: *accounts.mango_vault.key,
            pyth_oracle: *accounts.pyth_oracle.key,
            switchboard_oracle: *accounts.switchboard_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            mango_program: *accounts.mango_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<MangoSupplyStrategyDepositKeys>
for [AccountMeta; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: MangoSupplyStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mango_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mango_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.mango_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for MangoSupplyStrategyDepositKeys {
    fn from(pubkeys: [Pubkey; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            mango_group: pubkeys[4],
            mango_account: pubkeys[5],
            mango_bank: pubkeys[6],
            mango_vault: pubkeys[7],
            pyth_oracle: pubkeys[8],
            switchboard_oracle: pubkeys[9],
            authority: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            mango_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<MangoSupplyStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: MangoSupplyStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.mango_group.clone(),
            accounts.mango_account.clone(),
            accounts.mango_bank.clone(),
            accounts.mango_vault.clone(),
            accounts.pyth_oracle.clone(),
            accounts.switchboard_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.mango_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for MangoSupplyStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            mango_group: &arr[4],
            mango_account: &arr[5],
            mango_bank: &arr[6],
            mango_vault: &arr[7],
            pyth_oracle: &arr[8],
            switchboard_oracle: &arr[9],
            authority: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            mango_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    145, 134, 116, 16, 173, 216, 144, 179,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MangoSupplyStrategyDepositIxArgs {
    pub args: MangoSupplyStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MangoSupplyStrategyDepositIxData(pub MangoSupplyStrategyDepositIxArgs);
impl From<MangoSupplyStrategyDepositIxArgs> for MangoSupplyStrategyDepositIxData {
    fn from(args: MangoSupplyStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl MangoSupplyStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MangoSupplyStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(MangoSupplyStrategyDepositIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mango_supply_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: MangoSupplyStrategyDepositKeys,
    args: MangoSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MANGO_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MangoSupplyStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mango_supply_strategy_deposit_ix(
    keys: MangoSupplyStrategyDepositKeys,
    args: MangoSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    mango_supply_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn mango_supply_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MangoSupplyStrategyDepositAccounts<'_, '_>,
    args: MangoSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: MangoSupplyStrategyDepositKeys = accounts.into();
    let ix = mango_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mango_supply_strategy_deposit_invoke(
    accounts: MangoSupplyStrategyDepositAccounts<'_, '_>,
    args: MangoSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    mango_supply_strategy_deposit_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn mango_supply_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MangoSupplyStrategyDepositAccounts<'_, '_>,
    args: MangoSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MangoSupplyStrategyDepositKeys = accounts.into();
    let ix = mango_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mango_supply_strategy_deposit_invoke_signed(
    accounts: MangoSupplyStrategyDepositAccounts<'_, '_>,
    args: MangoSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mango_supply_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mango_supply_strategy_deposit_verify_account_keys(
    accounts: MangoSupplyStrategyDepositAccounts<'_, '_>,
    keys: MangoSupplyStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.mango_group.key, keys.mango_group),
        (*accounts.mango_account.key, keys.mango_account),
        (*accounts.mango_bank.key, keys.mango_bank),
        (*accounts.mango_vault.key, keys.mango_vault),
        (*accounts.pyth_oracle.key, keys.pyth_oracle),
        (*accounts.switchboard_oracle.key, keys.switchboard_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.mango_program.key, keys.mango_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.mango_account,
        accounts.mango_bank,
        accounts.mango_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mango_supply_strategy_deposit_verify_writable_privileges(accounts)?;
    mango_supply_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MangoSupplyStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub mango_group: &'me AccountInfo<'info>,
    pub mango_account: &'me AccountInfo<'info>,
    pub mango_bank: &'me AccountInfo<'info>,
    pub mango_vault: &'me AccountInfo<'info>,
    pub pyth_oracle: &'me AccountInfo<'info>,
    pub switchboard_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub mango_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MangoSupplyStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub mango_group: Pubkey,
    pub mango_account: Pubkey,
    pub mango_bank: Pubkey,
    pub mango_vault: Pubkey,
    pub pyth_oracle: Pubkey,
    pub switchboard_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub mango_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<MangoSupplyStrategyWithdrawAccounts<'_, '_>>
for MangoSupplyStrategyWithdrawKeys {
    fn from(accounts: MangoSupplyStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            mango_group: *accounts.mango_group.key,
            mango_account: *accounts.mango_account.key,
            mango_bank: *accounts.mango_bank.key,
            mango_vault: *accounts.mango_vault.key,
            pyth_oracle: *accounts.pyth_oracle.key,
            switchboard_oracle: *accounts.switchboard_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            mango_program: *accounts.mango_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<MangoSupplyStrategyWithdrawKeys>
for [AccountMeta; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: MangoSupplyStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mango_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mango_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mango_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.mango_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for MangoSupplyStrategyWithdrawKeys {
    fn from(pubkeys: [Pubkey; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            mango_group: pubkeys[4],
            mango_account: pubkeys[5],
            mango_bank: pubkeys[6],
            mango_vault: pubkeys[7],
            pyth_oracle: pubkeys[8],
            switchboard_oracle: pubkeys[9],
            authority: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            mango_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<MangoSupplyStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: MangoSupplyStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.mango_group.clone(),
            accounts.mango_account.clone(),
            accounts.mango_bank.clone(),
            accounts.mango_vault.clone(),
            accounts.pyth_oracle.clone(),
            accounts.switchboard_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.mango_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for MangoSupplyStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            mango_group: &arr[4],
            mango_account: &arr[5],
            mango_bank: &arr[6],
            mango_vault: &arr[7],
            pyth_oracle: &arr[8],
            switchboard_oracle: &arr[9],
            authority: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            mango_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    246, 92, 37, 254, 180, 99, 171, 77,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MangoSupplyStrategyWithdrawIxArgs {
    pub args: MangoSupplyStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MangoSupplyStrategyWithdrawIxData(pub MangoSupplyStrategyWithdrawIxArgs);
impl From<MangoSupplyStrategyWithdrawIxArgs> for MangoSupplyStrategyWithdrawIxData {
    fn from(args: MangoSupplyStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl MangoSupplyStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <MangoSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(MangoSupplyStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mango_supply_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: MangoSupplyStrategyWithdrawKeys,
    args: MangoSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MANGO_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MangoSupplyStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mango_supply_strategy_withdraw_ix(
    keys: MangoSupplyStrategyWithdrawKeys,
    args: MangoSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    mango_supply_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn mango_supply_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MangoSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MangoSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: MangoSupplyStrategyWithdrawKeys = accounts.into();
    let ix = mango_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn mango_supply_strategy_withdraw_invoke(
    accounts: MangoSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MangoSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    mango_supply_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn mango_supply_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MangoSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MangoSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MangoSupplyStrategyWithdrawKeys = accounts.into();
    let ix = mango_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mango_supply_strategy_withdraw_invoke_signed(
    accounts: MangoSupplyStrategyWithdrawAccounts<'_, '_>,
    args: MangoSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mango_supply_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mango_supply_strategy_withdraw_verify_account_keys(
    accounts: MangoSupplyStrategyWithdrawAccounts<'_, '_>,
    keys: MangoSupplyStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.mango_group.key, keys.mango_group),
        (*accounts.mango_account.key, keys.mango_account),
        (*accounts.mango_bank.key, keys.mango_bank),
        (*accounts.mango_vault.key, keys.mango_vault),
        (*accounts.pyth_oracle.key, keys.pyth_oracle),
        (*accounts.switchboard_oracle.key, keys.switchboard_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.mango_program.key, keys.mango_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.mango_account,
        accounts.mango_bank,
        accounts.mango_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mango_supply_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: MangoSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mango_supply_strategy_withdraw_verify_writable_privileges(accounts)?;
    mango_supply_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct DriftSupplyStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub drift_user: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_perp_market: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub spot_pyth_oracle: &'me AccountInfo<'info>,
    pub perp_pyth_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftSupplyStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub drift_user: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_state: Pubkey,
    pub drift_signer: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_perp_market: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub spot_pyth_oracle: Pubkey,
    pub perp_pyth_oracle: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub drift_program: Pubkey,
}
impl From<DriftSupplyStrategyInitAccounts<'_, '_>> for DriftSupplyStrategyInitKeys {
    fn from(accounts: DriftSupplyStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            drift_user: *accounts.drift_user.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_state: *accounts.drift_state.key,
            drift_signer: *accounts.drift_signer.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_perp_market: *accounts.drift_perp_market.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            spot_pyth_oracle: *accounts.spot_pyth_oracle.key,
            perp_pyth_oracle: *accounts.perp_pyth_oracle.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            drift_program: *accounts.drift_program.key,
        }
    }
}
impl From<DriftSupplyStrategyInitKeys>
for [AccountMeta; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftSupplyStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_perp_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.spot_pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perp_pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for DriftSupplyStrategyInitKeys {
    fn from(pubkeys: [Pubkey; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            drift_user: pubkeys[3],
            drift_user_stats: pubkeys[4],
            drift_state: pubkeys[5],
            drift_signer: pubkeys[6],
            drift_spot_market: pubkeys[7],
            drift_perp_market: pubkeys[8],
            drift_spot_market_vault: pubkeys[9],
            spot_pyth_oracle: pubkeys[10],
            perp_pyth_oracle: pubkeys[11],
            authority: pubkeys[12],
            rent: pubkeys[13],
            system_program: pubkeys[14],
            drift_program: pubkeys[15],
        }
    }
}
impl<'info> From<DriftSupplyStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftSupplyStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.drift_user.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_state.clone(),
            accounts.drift_signer.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_perp_market.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.spot_pyth_oracle.clone(),
            accounts.perp_pyth_oracle.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.drift_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for DriftSupplyStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            drift_user: &arr[3],
            drift_user_stats: &arr[4],
            drift_state: &arr[5],
            drift_signer: &arr[6],
            drift_spot_market: &arr[7],
            drift_perp_market: &arr[8],
            drift_spot_market_vault: &arr[9],
            spot_pyth_oracle: &arr[10],
            perp_pyth_oracle: &arr[11],
            authority: &arr[12],
            rent: &arr[13],
            system_program: &arr[14],
            drift_program: &arr[15],
        }
    }
}
pub const DRIFT_SUPPLY_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    113, 229, 159, 10, 39, 204, 102, 77,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftSupplyStrategyInitIxArgs {
    pub args: DriftSupplyStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftSupplyStrategyInitIxData(pub DriftSupplyStrategyInitIxArgs);
impl From<DriftSupplyStrategyInitIxArgs> for DriftSupplyStrategyInitIxData {
    fn from(args: DriftSupplyStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl DriftSupplyStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_SUPPLY_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <DriftSupplyStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(DriftSupplyStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_supply_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftSupplyStrategyInitKeys,
    args: DriftSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DriftSupplyStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_supply_strategy_init_ix(
    keys: DriftSupplyStrategyInitKeys,
    args: DriftSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    drift_supply_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn drift_supply_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftSupplyStrategyInitAccounts<'_, '_>,
    args: DriftSupplyStrategyInitIxArgs,
) -> ProgramResult {
    let keys: DriftSupplyStrategyInitKeys = accounts.into();
    let ix = drift_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_supply_strategy_init_invoke(
    accounts: DriftSupplyStrategyInitAccounts<'_, '_>,
    args: DriftSupplyStrategyInitIxArgs,
) -> ProgramResult {
    drift_supply_strategy_init_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn drift_supply_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftSupplyStrategyInitAccounts<'_, '_>,
    args: DriftSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftSupplyStrategyInitKeys = accounts.into();
    let ix = drift_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_supply_strategy_init_invoke_signed(
    accounts: DriftSupplyStrategyInitAccounts<'_, '_>,
    args: DriftSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_supply_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_supply_strategy_init_verify_account_keys(
    accounts: DriftSupplyStrategyInitAccounts<'_, '_>,
    keys: DriftSupplyStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.drift_user.key, keys.drift_user),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_perp_market.key, keys.drift_perp_market),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.spot_pyth_oracle.key, keys.spot_pyth_oracle),
        (*accounts.perp_pyth_oracle.key, keys.perp_pyth_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.drift_program.key, keys.drift_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.drift_user,
        accounts.drift_user_stats,
        accounts.drift_state,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_supply_strategy_init_verify_writable_privileges(accounts)?;
    drift_supply_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DriftSupplyStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub drift_user: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftSupplyStrategyDepositKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub drift_user: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_state: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub drift_program: Pubkey,
    pub token_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<DriftSupplyStrategyDepositAccounts<'_, '_>>
for DriftSupplyStrategyDepositKeys {
    fn from(accounts: DriftSupplyStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            drift_user: *accounts.drift_user.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_state: *accounts.drift_state.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            drift_program: *accounts.drift_program.key,
            token_program: *accounts.token_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<DriftSupplyStrategyDepositKeys>
for [AccountMeta; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftSupplyStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for DriftSupplyStrategyDepositKeys {
    fn from(pubkeys: [Pubkey; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            drift_user: pubkeys[4],
            drift_user_stats: pubkeys[5],
            drift_state: pubkeys[6],
            drift_spot_market: pubkeys[7],
            drift_spot_market_vault: pubkeys[8],
            authority: pubkeys[9],
            system_program: pubkeys[10],
            drift_program: pubkeys[11],
            token_program: pubkeys[12],
            log_program: pubkeys[13],
        }
    }
}
impl<'info> From<DriftSupplyStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftSupplyStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.drift_user.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_state.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.drift_program.clone(),
            accounts.token_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for DriftSupplyStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            drift_user: &arr[4],
            drift_user_stats: &arr[5],
            drift_state: &arr[6],
            drift_spot_market: &arr[7],
            drift_spot_market_vault: &arr[8],
            authority: &arr[9],
            system_program: &arr[10],
            drift_program: &arr[11],
            token_program: &arr[12],
            log_program: &arr[13],
        }
    }
}
pub const DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    223, 216, 94, 16, 168, 109, 107, 212,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftSupplyStrategyDepositIxArgs {
    pub args: DriftSupplyStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftSupplyStrategyDepositIxData(pub DriftSupplyStrategyDepositIxArgs);
impl From<DriftSupplyStrategyDepositIxArgs> for DriftSupplyStrategyDepositIxData {
    fn from(args: DriftSupplyStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl DriftSupplyStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <DriftSupplyStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(DriftSupplyStrategyDepositIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_supply_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftSupplyStrategyDepositKeys,
    args: DriftSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DriftSupplyStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_supply_strategy_deposit_ix(
    keys: DriftSupplyStrategyDepositKeys,
    args: DriftSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    drift_supply_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn drift_supply_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftSupplyStrategyDepositAccounts<'_, '_>,
    args: DriftSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: DriftSupplyStrategyDepositKeys = accounts.into();
    let ix = drift_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_supply_strategy_deposit_invoke(
    accounts: DriftSupplyStrategyDepositAccounts<'_, '_>,
    args: DriftSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    drift_supply_strategy_deposit_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn drift_supply_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftSupplyStrategyDepositAccounts<'_, '_>,
    args: DriftSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftSupplyStrategyDepositKeys = accounts.into();
    let ix = drift_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_supply_strategy_deposit_invoke_signed(
    accounts: DriftSupplyStrategyDepositAccounts<'_, '_>,
    args: DriftSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_supply_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_supply_strategy_deposit_verify_account_keys(
    accounts: DriftSupplyStrategyDepositAccounts<'_, '_>,
    keys: DriftSupplyStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.drift_user.key, keys.drift_user),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.drift_user,
        accounts.drift_user_stats,
        accounts.drift_state,
        accounts.drift_spot_market,
        accounts.drift_spot_market_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_supply_strategy_deposit_verify_writable_privileges(accounts)?;
    drift_supply_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct DriftSupplyStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub drift_user: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftSupplyStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub drift_user: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_state: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_signer: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub drift_program: Pubkey,
    pub token_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<DriftSupplyStrategyWithdrawAccounts<'_, '_>>
for DriftSupplyStrategyWithdrawKeys {
    fn from(accounts: DriftSupplyStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            drift_user: *accounts.drift_user.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_state: *accounts.drift_state.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_signer: *accounts.drift_signer.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            drift_program: *accounts.drift_program.key,
            token_program: *accounts.token_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<DriftSupplyStrategyWithdrawKeys>
for [AccountMeta; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftSupplyStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for DriftSupplyStrategyWithdrawKeys {
    fn from(pubkeys: [Pubkey; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            drift_user: pubkeys[4],
            drift_user_stats: pubkeys[5],
            drift_state: pubkeys[6],
            drift_spot_market_vault: pubkeys[7],
            drift_spot_market: pubkeys[8],
            drift_signer: pubkeys[9],
            authority: pubkeys[10],
            system_program: pubkeys[11],
            drift_program: pubkeys[12],
            token_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<DriftSupplyStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftSupplyStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.drift_user.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_state.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_signer.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.drift_program.clone(),
            accounts.token_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for DriftSupplyStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            drift_user: &arr[4],
            drift_user_stats: &arr[5],
            drift_state: &arr[6],
            drift_spot_market_vault: &arr[7],
            drift_spot_market: &arr[8],
            drift_signer: &arr[9],
            authority: &arr[10],
            system_program: &arr[11],
            drift_program: &arr[12],
            token_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    227, 37, 134, 61, 133, 58, 72, 28,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftSupplyStrategyWithdrawIxArgs {
    pub args: DriftSupplyStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftSupplyStrategyWithdrawIxData(pub DriftSupplyStrategyWithdrawIxArgs);
impl From<DriftSupplyStrategyWithdrawIxArgs> for DriftSupplyStrategyWithdrawIxData {
    fn from(args: DriftSupplyStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl DriftSupplyStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <DriftSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(DriftSupplyStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_supply_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftSupplyStrategyWithdrawKeys,
    args: DriftSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DriftSupplyStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_supply_strategy_withdraw_ix(
    keys: DriftSupplyStrategyWithdrawKeys,
    args: DriftSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    drift_supply_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn drift_supply_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftSupplyStrategyWithdrawAccounts<'_, '_>,
    args: DriftSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: DriftSupplyStrategyWithdrawKeys = accounts.into();
    let ix = drift_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_supply_strategy_withdraw_invoke(
    accounts: DriftSupplyStrategyWithdrawAccounts<'_, '_>,
    args: DriftSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    drift_supply_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn drift_supply_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftSupplyStrategyWithdrawAccounts<'_, '_>,
    args: DriftSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftSupplyStrategyWithdrawKeys = accounts.into();
    let ix = drift_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_supply_strategy_withdraw_invoke_signed(
    accounts: DriftSupplyStrategyWithdrawAccounts<'_, '_>,
    args: DriftSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_supply_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_supply_strategy_withdraw_verify_account_keys(
    accounts: DriftSupplyStrategyWithdrawAccounts<'_, '_>,
    keys: DriftSupplyStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.drift_user.key, keys.drift_user),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.drift_user,
        accounts.drift_user_stats,
        accounts.drift_state,
        accounts.drift_spot_market_vault,
        accounts.drift_spot_market,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_supply_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: DriftSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_supply_strategy_withdraw_verify_writable_privileges(accounts)?;
    drift_supply_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct DriftInsuranceFundStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub drift_insurance_fund_stake: &'me AccountInfo<'info>,
    pub drift_insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub drift_state: Pubkey,
    pub drift_insurance_fund_stake: Pubkey,
    pub drift_insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub drift_program: Pubkey,
}
impl From<DriftInsuranceFundStrategyInitAccounts<'_, '_>>
for DriftInsuranceFundStrategyInitKeys {
    fn from(accounts: DriftInsuranceFundStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            drift_state: *accounts.drift_state.key,
            drift_insurance_fund_stake: *accounts.drift_insurance_fund_stake.key,
            drift_insurance_fund_vault: *accounts.drift_insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            drift_program: *accounts.drift_program.key,
        }
    }
}
impl From<DriftInsuranceFundStrategyInitKeys>
for [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftInsuranceFundStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyInitKeys {
    fn from(
        pubkeys: [Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            drift_user_stats: pubkeys[3],
            drift_spot_market: pubkeys[4],
            drift_spot_market_vault: pubkeys[5],
            drift_state: pubkeys[6],
            drift_insurance_fund_stake: pubkeys[7],
            drift_insurance_fund_vault: pubkeys[8],
            drift_signer: pubkeys[9],
            authority: pubkeys[10],
            rent: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
            drift_program: pubkeys[14],
        }
    }
}
impl<'info> From<DriftInsuranceFundStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftInsuranceFundStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.drift_state.clone(),
            accounts.drift_insurance_fund_stake.clone(),
            accounts.drift_insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.drift_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            drift_user_stats: &arr[3],
            drift_spot_market: &arr[4],
            drift_spot_market_vault: &arr[5],
            drift_state: &arr[6],
            drift_insurance_fund_stake: &arr[7],
            drift_insurance_fund_vault: &arr[8],
            drift_signer: &arr[9],
            authority: &arr[10],
            rent: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
            drift_program: &arr[14],
        }
    }
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    134, 134, 2, 180, 247, 232, 205, 33,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftInsuranceFundStrategyInitIxArgs {
    pub args: DriftInsuranceFundStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyInitIxData(
    pub DriftInsuranceFundStrategyInitIxArgs,
);
impl From<DriftInsuranceFundStrategyInitIxArgs>
for DriftInsuranceFundStrategyInitIxData {
    fn from(args: DriftInsuranceFundStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl DriftInsuranceFundStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <DriftInsuranceFundStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(DriftInsuranceFundStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_insurance_fund_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftInsuranceFundStrategyInitKeys,
    args: DriftInsuranceFundStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DriftInsuranceFundStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_insurance_fund_strategy_init_ix(
    keys: DriftInsuranceFundStrategyInitKeys,
    args: DriftInsuranceFundStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    drift_insurance_fund_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn drift_insurance_fund_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyInitAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyInitIxArgs,
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyInitKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_init_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_insurance_fund_strategy_init_invoke(
    accounts: DriftInsuranceFundStrategyInitAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyInitIxArgs,
) -> ProgramResult {
    drift_insurance_fund_strategy_init_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn drift_insurance_fund_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyInitAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyInitKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_init_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_insurance_fund_strategy_init_invoke_signed(
    accounts: DriftInsuranceFundStrategyInitAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_insurance_fund_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_insurance_fund_strategy_init_verify_account_keys(
    accounts: DriftInsuranceFundStrategyInitAccounts<'_, '_>,
    keys: DriftInsuranceFundStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.drift_insurance_fund_stake.key, keys.drift_insurance_fund_stake),
        (*accounts.drift_insurance_fund_vault.key, keys.drift_insurance_fund_vault),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.authority.key, keys.authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.drift_program.key, keys.drift_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.drift_user_stats,
        accounts.drift_spot_market,
        accounts.drift_state,
        accounts.drift_insurance_fund_stake,
        accounts.drift_signer,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_insurance_fund_strategy_init_verify_writable_privileges(accounts)?;
    drift_insurance_fund_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct DriftInsuranceFundStrategyStakeAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub drift_insurance_fund_stake: &'me AccountInfo<'info>,
    pub drift_insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyStakeKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub drift_state: Pubkey,
    pub drift_insurance_fund_stake: Pubkey,
    pub drift_insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub drift_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<DriftInsuranceFundStrategyStakeAccounts<'_, '_>>
for DriftInsuranceFundStrategyStakeKeys {
    fn from(accounts: DriftInsuranceFundStrategyStakeAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            drift_state: *accounts.drift_state.key,
            drift_insurance_fund_stake: *accounts.drift_insurance_fund_stake.key,
            drift_insurance_fund_vault: *accounts.drift_insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            drift_program: *accounts.drift_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<DriftInsuranceFundStrategyStakeKeys>
for [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftInsuranceFundStrategyStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyStakeKeys {
    fn from(
        pubkeys: [Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            vault_asset_ata: pubkeys[3],
            drift_user_stats: pubkeys[4],
            drift_spot_market: pubkeys[5],
            drift_spot_market_vault: pubkeys[6],
            drift_state: pubkeys[7],
            drift_insurance_fund_stake: pubkeys[8],
            drift_insurance_fund_vault: pubkeys[9],
            drift_signer: pubkeys[10],
            authority: pubkeys[11],
            token_program: pubkeys[12],
            drift_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<DriftInsuranceFundStrategyStakeAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftInsuranceFundStrategyStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.drift_state.clone(),
            accounts.drift_insurance_fund_stake.clone(),
            accounts.drift_insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.drift_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyStakeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            vault_asset_ata: &arr[3],
            drift_user_stats: &arr[4],
            drift_spot_market: &arr[5],
            drift_spot_market_vault: &arr[6],
            drift_state: &arr[7],
            drift_insurance_fund_stake: &arr[8],
            drift_insurance_fund_vault: &arr[9],
            drift_signer: &arr[10],
            authority: &arr[11],
            token_program: &arr[12],
            drift_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_DISCM: [u8; 8usize] = [
    182, 76, 176, 27, 160, 47, 58, 179,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftInsuranceFundStrategyStakeIxArgs {
    pub args: DriftInsuranceFundStrategyStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyStakeIxData(
    pub DriftInsuranceFundStrategyStakeIxArgs,
);
impl From<DriftInsuranceFundStrategyStakeIxArgs>
for DriftInsuranceFundStrategyStakeIxData {
    fn from(args: DriftInsuranceFundStrategyStakeIxArgs) -> Self {
        Self(args)
    }
}
impl DriftInsuranceFundStrategyStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <DriftInsuranceFundStrategyStakeArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(DriftInsuranceFundStrategyStakeIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_insurance_fund_strategy_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftInsuranceFundStrategyStakeKeys,
    args: DriftInsuranceFundStrategyStakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_STAKE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DriftInsuranceFundStrategyStakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_insurance_fund_strategy_stake_ix(
    keys: DriftInsuranceFundStrategyStakeKeys,
    args: DriftInsuranceFundStrategyStakeIxArgs,
) -> std::io::Result<Instruction> {
    drift_insurance_fund_strategy_stake_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn drift_insurance_fund_strategy_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyStakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyStakeIxArgs,
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyStakeKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_stake_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_insurance_fund_strategy_stake_invoke(
    accounts: DriftInsuranceFundStrategyStakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyStakeIxArgs,
) -> ProgramResult {
    drift_insurance_fund_strategy_stake_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn drift_insurance_fund_strategy_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyStakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyStakeKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_stake_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_insurance_fund_strategy_stake_invoke_signed(
    accounts: DriftInsuranceFundStrategyStakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_insurance_fund_strategy_stake_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_insurance_fund_strategy_stake_verify_account_keys(
    accounts: DriftInsuranceFundStrategyStakeAccounts<'_, '_>,
    keys: DriftInsuranceFundStrategyStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.drift_insurance_fund_stake.key, keys.drift_insurance_fund_stake),
        (*accounts.drift_insurance_fund_vault.key, keys.drift_insurance_fund_vault),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_stake_verify_writable_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.vault_asset_ata,
        accounts.drift_user_stats,
        accounts.drift_spot_market,
        accounts.drift_spot_market_vault,
        accounts.drift_insurance_fund_stake,
        accounts.drift_insurance_fund_vault,
        accounts.drift_signer,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_stake_verify_signer_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_stake_verify_account_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_insurance_fund_strategy_stake_verify_writable_privileges(accounts)?;
    drift_insurance_fund_strategy_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DriftInsuranceFundStrategyUnstakeAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_insurance_fund_stake: &'me AccountInfo<'info>,
    pub drift_insurance_fund_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyUnstakeKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_insurance_fund_stake: Pubkey,
    pub drift_insurance_fund_vault: Pubkey,
    pub authority: Pubkey,
    pub drift_program: Pubkey,
}
impl From<DriftInsuranceFundStrategyUnstakeAccounts<'_, '_>>
for DriftInsuranceFundStrategyUnstakeKeys {
    fn from(accounts: DriftInsuranceFundStrategyUnstakeAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_insurance_fund_stake: *accounts.drift_insurance_fund_stake.key,
            drift_insurance_fund_vault: *accounts.drift_insurance_fund_vault.key,
            authority: *accounts.authority.key,
            drift_program: *accounts.drift_program.key,
        }
    }
}
impl From<DriftInsuranceFundStrategyUnstakeKeys>
for [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftInsuranceFundStrategyUnstakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyUnstakeKeys {
    fn from(
        pubkeys: [Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            drift_user_stats: pubkeys[2],
            drift_spot_market: pubkeys[3],
            drift_insurance_fund_stake: pubkeys[4],
            drift_insurance_fund_vault: pubkeys[5],
            authority: pubkeys[6],
            drift_program: pubkeys[7],
        }
    }
}
impl<'info> From<DriftInsuranceFundStrategyUnstakeAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftInsuranceFundStrategyUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_insurance_fund_stake.clone(),
            accounts.drift_insurance_fund_vault.clone(),
            accounts.authority.clone(),
            accounts.drift_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyUnstakeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            drift_user_stats: &arr[2],
            drift_spot_market: &arr[3],
            drift_insurance_fund_stake: &arr[4],
            drift_insurance_fund_vault: &arr[5],
            authority: &arr[6],
            drift_program: &arr[7],
        }
    }
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_DISCM: [u8; 8usize] = [
    91, 150, 31, 34, 58, 35, 194, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DriftInsuranceFundStrategyUnstakeIxArgs {
    pub args: DriftInsuranceFundStrategyUnstakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyUnstakeIxData(
    pub DriftInsuranceFundStrategyUnstakeIxArgs,
);
impl From<DriftInsuranceFundStrategyUnstakeIxArgs>
for DriftInsuranceFundStrategyUnstakeIxData {
    fn from(args: DriftInsuranceFundStrategyUnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl DriftInsuranceFundStrategyUnstakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <DriftInsuranceFundStrategyUnstakeArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(DriftInsuranceFundStrategyUnstakeIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_insurance_fund_strategy_unstake_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftInsuranceFundStrategyUnstakeKeys,
    args: DriftInsuranceFundStrategyUnstakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_UNSTAKE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DriftInsuranceFundStrategyUnstakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn drift_insurance_fund_strategy_unstake_ix(
    keys: DriftInsuranceFundStrategyUnstakeKeys,
    args: DriftInsuranceFundStrategyUnstakeIxArgs,
) -> std::io::Result<Instruction> {
    drift_insurance_fund_strategy_unstake_ix_with_program_id(
        CARROT_PROGRAM_ID,
        keys,
        args,
    )
}
pub fn drift_insurance_fund_strategy_unstake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyUnstakeIxArgs,
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyUnstakeKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_unstake_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_insurance_fund_strategy_unstake_invoke(
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyUnstakeIxArgs,
) -> ProgramResult {
    drift_insurance_fund_strategy_unstake_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn drift_insurance_fund_strategy_unstake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyUnstakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyUnstakeKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_unstake_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_insurance_fund_strategy_unstake_invoke_signed(
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'_, '_>,
    args: DriftInsuranceFundStrategyUnstakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_insurance_fund_strategy_unstake_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn drift_insurance_fund_strategy_unstake_verify_account_keys(
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'_, '_>,
    keys: DriftInsuranceFundStrategyUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_insurance_fund_stake.key, keys.drift_insurance_fund_stake),
        (*accounts.drift_insurance_fund_vault.key, keys.drift_insurance_fund_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.drift_program.key, keys.drift_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_unstake_verify_writable_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.drift_user_stats,
        accounts.drift_spot_market,
        accounts.drift_insurance_fund_stake,
        accounts.drift_insurance_fund_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_unstake_verify_signer_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_unstake_verify_account_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_insurance_fund_strategy_unstake_verify_writable_privileges(accounts)?;
    drift_insurance_fund_strategy_unstake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DriftInsuranceFundStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub drift_insurance_fund_stake: &'me AccountInfo<'info>,
    pub drift_insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_state: Pubkey,
    pub drift_insurance_fund_stake: Pubkey,
    pub drift_insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub drift_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<DriftInsuranceFundStrategyWithdrawAccounts<'_, '_>>
for DriftInsuranceFundStrategyWithdrawKeys {
    fn from(accounts: DriftInsuranceFundStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_state: *accounts.drift_state.key,
            drift_insurance_fund_stake: *accounts.drift_insurance_fund_stake.key,
            drift_insurance_fund_vault: *accounts.drift_insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            drift_program: *accounts.drift_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<DriftInsuranceFundStrategyWithdrawKeys>
for [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: DriftInsuranceFundStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_insurance_fund_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_signer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyWithdrawKeys {
    fn from(
        pubkeys: [Pubkey; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            vault_asset_ata: pubkeys[3],
            drift_user_stats: pubkeys[4],
            drift_spot_market: pubkeys[5],
            drift_state: pubkeys[6],
            drift_insurance_fund_stake: pubkeys[7],
            drift_insurance_fund_vault: pubkeys[8],
            drift_signer: pubkeys[9],
            authority: pubkeys[10],
            token_program: pubkeys[11],
            drift_program: pubkeys[12],
            log_program: pubkeys[13],
        }
    }
}
impl<'info> From<DriftInsuranceFundStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: DriftInsuranceFundStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_state.clone(),
            accounts.drift_insurance_fund_stake.clone(),
            accounts.drift_insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.drift_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for DriftInsuranceFundStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            vault_asset_ata: &arr[3],
            drift_user_stats: &arr[4],
            drift_spot_market: &arr[5],
            drift_state: &arr[6],
            drift_insurance_fund_stake: &arr[7],
            drift_insurance_fund_vault: &arr[8],
            drift_signer: &arr[9],
            authority: &arr[10],
            token_program: &arr[11],
            drift_program: &arr[12],
            log_program: &arr[13],
        }
    }
}
pub const DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    28, 209, 176, 53, 103, 64, 71, 173,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DriftInsuranceFundStrategyWithdrawIxData;
impl DriftInsuranceFundStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn drift_insurance_fund_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: DriftInsuranceFundStrategyWithdrawKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRIFT_INSURANCE_FUND_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DriftInsuranceFundStrategyWithdrawIxData.try_to_vec()?,
    })
}
pub fn drift_insurance_fund_strategy_withdraw_ix(
    keys: DriftInsuranceFundStrategyWithdrawKeys,
) -> std::io::Result<Instruction> {
    drift_insurance_fund_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn drift_insurance_fund_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyWithdrawKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_withdraw_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn drift_insurance_fund_strategy_withdraw_invoke(
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'_, '_>,
) -> ProgramResult {
    drift_insurance_fund_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
    )
}
pub fn drift_insurance_fund_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DriftInsuranceFundStrategyWithdrawKeys = accounts.into();
    let ix = drift_insurance_fund_strategy_withdraw_ix_with_program_id(
        program_id,
        keys,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn drift_insurance_fund_strategy_withdraw_invoke_signed(
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    drift_insurance_fund_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn drift_insurance_fund_strategy_withdraw_verify_account_keys(
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'_, '_>,
    keys: DriftInsuranceFundStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.drift_insurance_fund_stake.key, keys.drift_insurance_fund_stake),
        (*accounts.drift_insurance_fund_vault.key, keys.drift_insurance_fund_vault),
        (*accounts.drift_signer.key, keys.drift_signer),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.vault_asset_ata,
        accounts.drift_user_stats,
        accounts.drift_spot_market,
        accounts.drift_insurance_fund_stake,
        accounts.drift_insurance_fund_vault,
        accounts.drift_signer,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn drift_insurance_fund_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: DriftInsuranceFundStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    drift_insurance_fund_strategy_withdraw_verify_writable_privileges(accounts)?;
    drift_insurance_fund_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct ClendSupplyStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub clend_group: &'me AccountInfo<'info>,
    pub clend_account: &'me AccountInfo<'info>,
    pub clend_bank: &'me AccountInfo<'info>,
    pub clend_bank_liquidity_vault: &'me AccountInfo<'info>,
    pub clend_bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub clend_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub clend_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClendSupplyStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub clend_group: Pubkey,
    pub clend_account: Pubkey,
    pub clend_bank: Pubkey,
    pub clend_bank_liquidity_vault: Pubkey,
    pub clend_bank_liquidity_vault_authority: Pubkey,
    pub clend_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub clend_program: Pubkey,
}
impl From<ClendSupplyStrategyInitAccounts<'_, '_>> for ClendSupplyStrategyInitKeys {
    fn from(accounts: ClendSupplyStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            clend_group: *accounts.clend_group.key,
            clend_account: *accounts.clend_account.key,
            clend_bank: *accounts.clend_bank.key,
            clend_bank_liquidity_vault: *accounts.clend_bank_liquidity_vault.key,
            clend_bank_liquidity_vault_authority: *accounts
                .clend_bank_liquidity_vault_authority
                .key,
            clend_oracle: *accounts.clend_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            clend_program: *accounts.clend_program.key,
        }
    }
}
impl From<ClendSupplyStrategyInitKeys>
for [AccountMeta; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ClendSupplyStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_bank,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_bank_liquidity_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for ClendSupplyStrategyInitKeys {
    fn from(pubkeys: [Pubkey; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            clend_group: pubkeys[3],
            clend_account: pubkeys[4],
            clend_bank: pubkeys[5],
            clend_bank_liquidity_vault: pubkeys[6],
            clend_bank_liquidity_vault_authority: pubkeys[7],
            clend_oracle: pubkeys[8],
            authority: pubkeys[9],
            system_program: pubkeys[10],
            clend_program: pubkeys[11],
        }
    }
}
impl<'info> From<ClendSupplyStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClendSupplyStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.clend_group.clone(),
            accounts.clend_account.clone(),
            accounts.clend_bank.clone(),
            accounts.clend_bank_liquidity_vault.clone(),
            accounts.clend_bank_liquidity_vault_authority.clone(),
            accounts.clend_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.clend_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for ClendSupplyStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            clend_group: &arr[3],
            clend_account: &arr[4],
            clend_bank: &arr[5],
            clend_bank_liquidity_vault: &arr[6],
            clend_bank_liquidity_vault_authority: &arr[7],
            clend_oracle: &arr[8],
            authority: &arr[9],
            system_program: &arr[10],
            clend_program: &arr[11],
        }
    }
}
pub const CLEND_SUPPLY_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    223, 209, 52, 97, 27, 23, 98, 190,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClendSupplyStrategyInitIxArgs {
    pub args: ClendSupplyStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClendSupplyStrategyInitIxData(pub ClendSupplyStrategyInitIxArgs);
impl From<ClendSupplyStrategyInitIxArgs> for ClendSupplyStrategyInitIxData {
    fn from(args: ClendSupplyStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl ClendSupplyStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLEND_SUPPLY_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ClendSupplyStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ClendSupplyStrategyInitIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLEND_SUPPLY_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clend_supply_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: ClendSupplyStrategyInitKeys,
    args: ClendSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEND_SUPPLY_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClendSupplyStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn clend_supply_strategy_init_ix(
    keys: ClendSupplyStrategyInitKeys,
    args: ClendSupplyStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    clend_supply_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn clend_supply_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClendSupplyStrategyInitAccounts<'_, '_>,
    args: ClendSupplyStrategyInitIxArgs,
) -> ProgramResult {
    let keys: ClendSupplyStrategyInitKeys = accounts.into();
    let ix = clend_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn clend_supply_strategy_init_invoke(
    accounts: ClendSupplyStrategyInitAccounts<'_, '_>,
    args: ClendSupplyStrategyInitIxArgs,
) -> ProgramResult {
    clend_supply_strategy_init_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn clend_supply_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClendSupplyStrategyInitAccounts<'_, '_>,
    args: ClendSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClendSupplyStrategyInitKeys = accounts.into();
    let ix = clend_supply_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clend_supply_strategy_init_invoke_signed(
    accounts: ClendSupplyStrategyInitAccounts<'_, '_>,
    args: ClendSupplyStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clend_supply_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn clend_supply_strategy_init_verify_account_keys(
    accounts: ClendSupplyStrategyInitAccounts<'_, '_>,
    keys: ClendSupplyStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.clend_group.key, keys.clend_group),
        (*accounts.clend_account.key, keys.clend_account),
        (*accounts.clend_bank.key, keys.clend_bank),
        (*accounts.clend_bank_liquidity_vault.key, keys.clend_bank_liquidity_vault),
        (
            *accounts.clend_bank_liquidity_vault_authority.key,
            keys.clend_bank_liquidity_vault_authority,
        ),
        (*accounts.clend_oracle.key, keys.clend_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.clend_program.key, keys.clend_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.clend_account,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.clend_account, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clend_supply_strategy_init_verify_writable_privileges(accounts)?;
    clend_supply_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ClendSupplyStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub clend_group: &'me AccountInfo<'info>,
    pub clend_account: &'me AccountInfo<'info>,
    pub clend_bank: &'me AccountInfo<'info>,
    pub clend_bank_liquidity_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub clend_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClendSupplyStrategyDepositKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub clend_group: Pubkey,
    pub clend_account: Pubkey,
    pub clend_bank: Pubkey,
    pub clend_bank_liquidity_vault: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub clend_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<ClendSupplyStrategyDepositAccounts<'_, '_>>
for ClendSupplyStrategyDepositKeys {
    fn from(accounts: ClendSupplyStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            clend_group: *accounts.clend_group.key,
            clend_account: *accounts.clend_account.key,
            clend_bank: *accounts.clend_bank.key,
            clend_bank_liquidity_vault: *accounts.clend_bank_liquidity_vault.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            clend_program: *accounts.clend_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<ClendSupplyStrategyDepositKeys>
for [AccountMeta; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ClendSupplyStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_bank_liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.clend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for ClendSupplyStrategyDepositKeys {
    fn from(pubkeys: [Pubkey; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            clend_group: pubkeys[4],
            clend_account: pubkeys[5],
            clend_bank: pubkeys[6],
            clend_bank_liquidity_vault: pubkeys[7],
            authority: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
            clend_program: pubkeys[11],
            log_program: pubkeys[12],
        }
    }
}
impl<'info> From<ClendSupplyStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClendSupplyStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.clend_group.clone(),
            accounts.clend_account.clone(),
            accounts.clend_bank.clone(),
            accounts.clend_bank_liquidity_vault.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.clend_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for ClendSupplyStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            clend_group: &arr[4],
            clend_account: &arr[5],
            clend_bank: &arr[6],
            clend_bank_liquidity_vault: &arr[7],
            authority: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
            clend_program: &arr[11],
            log_program: &arr[12],
        }
    }
}
pub const CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    188, 224, 139, 151, 152, 156, 115, 243,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClendSupplyStrategyDepositIxArgs {
    pub args: ClendSupplyStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClendSupplyStrategyDepositIxData(pub ClendSupplyStrategyDepositIxArgs);
impl From<ClendSupplyStrategyDepositIxArgs> for ClendSupplyStrategyDepositIxData {
    fn from(args: ClendSupplyStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl ClendSupplyStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ClendSupplyStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ClendSupplyStrategyDepositIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clend_supply_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: ClendSupplyStrategyDepositKeys,
    args: ClendSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEND_SUPPLY_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ClendSupplyStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn clend_supply_strategy_deposit_ix(
    keys: ClendSupplyStrategyDepositKeys,
    args: ClendSupplyStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    clend_supply_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn clend_supply_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClendSupplyStrategyDepositAccounts<'_, '_>,
    args: ClendSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: ClendSupplyStrategyDepositKeys = accounts.into();
    let ix = clend_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn clend_supply_strategy_deposit_invoke(
    accounts: ClendSupplyStrategyDepositAccounts<'_, '_>,
    args: ClendSupplyStrategyDepositIxArgs,
) -> ProgramResult {
    clend_supply_strategy_deposit_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn clend_supply_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClendSupplyStrategyDepositAccounts<'_, '_>,
    args: ClendSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClendSupplyStrategyDepositKeys = accounts.into();
    let ix = clend_supply_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clend_supply_strategy_deposit_invoke_signed(
    accounts: ClendSupplyStrategyDepositAccounts<'_, '_>,
    args: ClendSupplyStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clend_supply_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn clend_supply_strategy_deposit_verify_account_keys(
    accounts: ClendSupplyStrategyDepositAccounts<'_, '_>,
    keys: ClendSupplyStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.clend_group.key, keys.clend_group),
        (*accounts.clend_account.key, keys.clend_account),
        (*accounts.clend_bank.key, keys.clend_bank),
        (*accounts.clend_bank_liquidity_vault.key, keys.clend_bank_liquidity_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.clend_program.key, keys.clend_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.clend_account,
        accounts.clend_bank,
        accounts.clend_bank_liquidity_vault,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clend_supply_strategy_deposit_verify_writable_privileges(accounts)?;
    clend_supply_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct ClendSupplyStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub clend_group: &'me AccountInfo<'info>,
    pub clend_account: &'me AccountInfo<'info>,
    pub clend_bank: &'me AccountInfo<'info>,
    pub clend_bank_liquidity_vault: &'me AccountInfo<'info>,
    pub clend_bank_liquidity_vault_authority: &'me AccountInfo<'info>,
    pub clend_oracle: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub clend_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClendSupplyStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy: Pubkey,
    pub clend_group: Pubkey,
    pub clend_account: Pubkey,
    pub clend_bank: Pubkey,
    pub clend_bank_liquidity_vault: Pubkey,
    pub clend_bank_liquidity_vault_authority: Pubkey,
    pub clend_oracle: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub clend_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<ClendSupplyStrategyWithdrawAccounts<'_, '_>>
for ClendSupplyStrategyWithdrawKeys {
    fn from(accounts: ClendSupplyStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy: *accounts.strategy.key,
            clend_group: *accounts.clend_group.key,
            clend_account: *accounts.clend_account.key,
            clend_bank: *accounts.clend_bank.key,
            clend_bank_liquidity_vault: *accounts.clend_bank_liquidity_vault.key,
            clend_bank_liquidity_vault_authority: *accounts
                .clend_bank_liquidity_vault_authority
                .key,
            clend_oracle: *accounts.clend_oracle.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            clend_program: *accounts.clend_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<ClendSupplyStrategyWithdrawKeys>
for [AccountMeta; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: ClendSupplyStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_group,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clend_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_bank,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_bank_liquidity_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_bank_liquidity_vault_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clend_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.clend_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for ClendSupplyStrategyWithdrawKeys {
    fn from(pubkeys: [Pubkey; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            asset_mint: pubkeys[1],
            vault_asset_ata: pubkeys[2],
            strategy: pubkeys[3],
            clend_group: pubkeys[4],
            clend_account: pubkeys[5],
            clend_bank: pubkeys[6],
            clend_bank_liquidity_vault: pubkeys[7],
            clend_bank_liquidity_vault_authority: pubkeys[8],
            clend_oracle: pubkeys[9],
            authority: pubkeys[10],
            system_program: pubkeys[11],
            token_program: pubkeys[12],
            clend_program: pubkeys[13],
            log_program: pubkeys[14],
        }
    }
}
impl<'info> From<ClendSupplyStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClendSupplyStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy.clone(),
            accounts.clend_group.clone(),
            accounts.clend_account.clone(),
            accounts.clend_bank.clone(),
            accounts.clend_bank_liquidity_vault.clone(),
            accounts.clend_bank_liquidity_vault_authority.clone(),
            accounts.clend_oracle.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.clend_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for ClendSupplyStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            asset_mint: &arr[1],
            vault_asset_ata: &arr[2],
            strategy: &arr[3],
            clend_group: &arr[4],
            clend_account: &arr[5],
            clend_bank: &arr[6],
            clend_bank_liquidity_vault: &arr[7],
            clend_bank_liquidity_vault_authority: &arr[8],
            clend_oracle: &arr[9],
            authority: &arr[10],
            system_program: &arr[11],
            token_program: &arr[12],
            clend_program: &arr[13],
            log_program: &arr[14],
        }
    }
}
pub const CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    219, 130, 60, 235, 151, 226, 46, 21,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClendSupplyStrategyWithdrawIxArgs {
    pub args: ClendSupplyStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClendSupplyStrategyWithdrawIxData(pub ClendSupplyStrategyWithdrawIxArgs);
impl From<ClendSupplyStrategyWithdrawIxArgs> for ClendSupplyStrategyWithdrawIxData {
    fn from(args: ClendSupplyStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl ClendSupplyStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ClendSupplyStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ClendSupplyStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clend_supply_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: ClendSupplyStrategyWithdrawKeys,
    args: ClendSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEND_SUPPLY_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ClendSupplyStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn clend_supply_strategy_withdraw_ix(
    keys: ClendSupplyStrategyWithdrawKeys,
    args: ClendSupplyStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    clend_supply_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn clend_supply_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: ClendSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: ClendSupplyStrategyWithdrawKeys = accounts.into();
    let ix = clend_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn clend_supply_strategy_withdraw_invoke(
    accounts: ClendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: ClendSupplyStrategyWithdrawIxArgs,
) -> ProgramResult {
    clend_supply_strategy_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn clend_supply_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: ClendSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClendSupplyStrategyWithdrawKeys = accounts.into();
    let ix = clend_supply_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clend_supply_strategy_withdraw_invoke_signed(
    accounts: ClendSupplyStrategyWithdrawAccounts<'_, '_>,
    args: ClendSupplyStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clend_supply_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn clend_supply_strategy_withdraw_verify_account_keys(
    accounts: ClendSupplyStrategyWithdrawAccounts<'_, '_>,
    keys: ClendSupplyStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.clend_group.key, keys.clend_group),
        (*accounts.clend_account.key, keys.clend_account),
        (*accounts.clend_bank.key, keys.clend_bank),
        (*accounts.clend_bank_liquidity_vault.key, keys.clend_bank_liquidity_vault),
        (
            *accounts.clend_bank_liquidity_vault_authority.key,
            keys.clend_bank_liquidity_vault_authority,
        ),
        (*accounts.clend_oracle.key, keys.clend_oracle),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.clend_program.key, keys.clend_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.vault_asset_ata,
        accounts.clend_account,
        accounts.clend_bank,
        accounts.clend_bank_liquidity_vault,
        accounts.clend_bank_liquidity_vault_authority,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn clend_supply_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: ClendSupplyStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clend_supply_strategy_withdraw_verify_writable_privileges(accounts)?;
    clend_supply_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ChestStrategyInitAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub asset_token_account: &'me AccountInfo<'info>,
    pub chest: &'me AccountInfo<'info>,
    pub coin_token_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChestStrategyInitKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_token_account: Pubkey,
    pub chest: Pubkey,
    pub coin_token_account: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<ChestStrategyInitAccounts<'_, '_>> for ChestStrategyInitKeys {
    fn from(accounts: ChestStrategyInitAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            asset_token_account: *accounts.asset_token_account.key,
            chest: *accounts.chest.key,
            coin_token_account: *accounts.coin_token_account.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ChestStrategyInitKeys> for [AccountMeta; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ChestStrategyInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.asset_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.chest,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
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
impl From<[Pubkey; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN]> for ChestStrategyInitKeys {
    fn from(pubkeys: [Pubkey; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            asset_token_account: pubkeys[3],
            chest: pubkeys[4],
            coin_token_account: pubkeys[5],
            authority: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<ChestStrategyInitAccounts<'_, 'info>>
for [AccountInfo<'info>; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChestStrategyInitAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.asset_token_account.clone(),
            accounts.chest.clone(),
            accounts.coin_token_account.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN]>
for ChestStrategyInitAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            asset_token_account: &arr[3],
            chest: &arr[4],
            coin_token_account: &arr[5],
            authority: &arr[6],
            system_program: &arr[7],
        }
    }
}
pub const CHEST_STRATEGY_INIT_IX_DISCM: [u8; 8usize] = [
    47, 225, 103, 29, 28, 239, 246, 216,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChestStrategyInitIxArgs {
    pub args: ChestStrategyInitArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChestStrategyInitIxData(pub ChestStrategyInitIxArgs);
impl From<ChestStrategyInitIxArgs> for ChestStrategyInitIxData {
    fn from(args: ChestStrategyInitIxArgs) -> Self {
        Self(args)
    }
}
impl ChestStrategyInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHEST_STRATEGY_INIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ChestStrategyInitArgs>::deserialize(&mut reader)?
        };
        Ok(Self(ChestStrategyInitIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHEST_STRATEGY_INIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn chest_strategy_init_ix_with_program_id(
    program_id: Pubkey,
    keys: ChestStrategyInitKeys,
    args: ChestStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHEST_STRATEGY_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChestStrategyInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn chest_strategy_init_ix(
    keys: ChestStrategyInitKeys,
    args: ChestStrategyInitIxArgs,
) -> std::io::Result<Instruction> {
    chest_strategy_init_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn chest_strategy_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyInitAccounts<'_, '_>,
    args: ChestStrategyInitIxArgs,
) -> ProgramResult {
    let keys: ChestStrategyInitKeys = accounts.into();
    let ix = chest_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn chest_strategy_init_invoke(
    accounts: ChestStrategyInitAccounts<'_, '_>,
    args: ChestStrategyInitIxArgs,
) -> ProgramResult {
    chest_strategy_init_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn chest_strategy_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyInitAccounts<'_, '_>,
    args: ChestStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChestStrategyInitKeys = accounts.into();
    let ix = chest_strategy_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn chest_strategy_init_invoke_signed(
    accounts: ChestStrategyInitAccounts<'_, '_>,
    args: ChestStrategyInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    chest_strategy_init_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn chest_strategy_init_verify_account_keys(
    accounts: ChestStrategyInitAccounts<'_, '_>,
    keys: ChestStrategyInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.asset_token_account.key, keys.asset_token_account),
        (*accounts.chest.key, keys.chest),
        (*accounts.coin_token_account.key, keys.coin_token_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn chest_strategy_init_verify_writable_privileges<'me, 'info>(
    accounts: ChestStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.strategy, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn chest_strategy_init_verify_signer_privileges<'me, 'info>(
    accounts: ChestStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn chest_strategy_init_verify_account_privileges<'me, 'info>(
    accounts: ChestStrategyInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    chest_strategy_init_verify_writable_privileges(accounts)?;
    chest_strategy_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct ChestStrategyDepositAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub chest: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub chest_asset_reserve: &'me AccountInfo<'info>,
    pub drift_vault: &'me AccountInfo<'info>,
    pub drift_vault_depositor: &'me AccountInfo<'info>,
    pub drift_vault_token_account: &'me AccountInfo<'info>,
    pub drift_user: &'me AccountInfo<'info>,
    pub drift_user_stats: &'me AccountInfo<'info>,
    pub drift_spot_market: &'me AccountInfo<'info>,
    pub drift_spot_market_vault: &'me AccountInfo<'info>,
    pub drift_state: &'me AccountInfo<'info>,
    pub vault_asset_token_account: &'me AccountInfo<'info>,
    pub vault_coin_token_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub chest_program: &'me AccountInfo<'info>,
    pub drift_program: &'me AccountInfo<'info>,
    pub drift_vaults_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChestStrategyDepositKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub chest: Pubkey,
    pub coin: Pubkey,
    pub chest_asset_reserve: Pubkey,
    pub drift_vault: Pubkey,
    pub drift_vault_depositor: Pubkey,
    pub drift_vault_token_account: Pubkey,
    pub drift_user: Pubkey,
    pub drift_user_stats: Pubkey,
    pub drift_spot_market: Pubkey,
    pub drift_spot_market_vault: Pubkey,
    pub drift_state: Pubkey,
    pub vault_asset_token_account: Pubkey,
    pub vault_coin_token_account: Pubkey,
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub chest_program: Pubkey,
    pub drift_program: Pubkey,
    pub drift_vaults_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<ChestStrategyDepositAccounts<'_, '_>> for ChestStrategyDepositKeys {
    fn from(accounts: ChestStrategyDepositAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            chest: *accounts.chest.key,
            coin: *accounts.coin.key,
            chest_asset_reserve: *accounts.chest_asset_reserve.key,
            drift_vault: *accounts.drift_vault.key,
            drift_vault_depositor: *accounts.drift_vault_depositor.key,
            drift_vault_token_account: *accounts.drift_vault_token_account.key,
            drift_user: *accounts.drift_user.key,
            drift_user_stats: *accounts.drift_user_stats.key,
            drift_spot_market: *accounts.drift_spot_market.key,
            drift_spot_market_vault: *accounts.drift_spot_market_vault.key,
            drift_state: *accounts.drift_state.key,
            vault_asset_token_account: *accounts.vault_asset_token_account.key,
            vault_coin_token_account: *accounts.vault_coin_token_account.key,
            authority: *accounts.authority.key,
            token_program: *accounts.token_program.key,
            chest_program: *accounts.chest_program.key,
            drift_program: *accounts.drift_program.key,
            drift_vaults_program: *accounts.drift_vaults_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<ChestStrategyDepositKeys>
for [AccountMeta; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ChestStrategyDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.chest,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.chest_asset_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault_depositor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user_stats,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_spot_market_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.chest_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.drift_vaults_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for ChestStrategyDepositKeys {
    fn from(pubkeys: [Pubkey; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            chest: pubkeys[3],
            coin: pubkeys[4],
            chest_asset_reserve: pubkeys[5],
            drift_vault: pubkeys[6],
            drift_vault_depositor: pubkeys[7],
            drift_vault_token_account: pubkeys[8],
            drift_user: pubkeys[9],
            drift_user_stats: pubkeys[10],
            drift_spot_market: pubkeys[11],
            drift_spot_market_vault: pubkeys[12],
            drift_state: pubkeys[13],
            vault_asset_token_account: pubkeys[14],
            vault_coin_token_account: pubkeys[15],
            authority: pubkeys[16],
            token_program: pubkeys[17],
            chest_program: pubkeys[18],
            drift_program: pubkeys[19],
            drift_vaults_program: pubkeys[20],
            log_program: pubkeys[21],
        }
    }
}
impl<'info> From<ChestStrategyDepositAccounts<'_, 'info>>
for [AccountInfo<'info>; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChestStrategyDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.chest.clone(),
            accounts.coin.clone(),
            accounts.chest_asset_reserve.clone(),
            accounts.drift_vault.clone(),
            accounts.drift_vault_depositor.clone(),
            accounts.drift_vault_token_account.clone(),
            accounts.drift_user.clone(),
            accounts.drift_user_stats.clone(),
            accounts.drift_spot_market.clone(),
            accounts.drift_spot_market_vault.clone(),
            accounts.drift_state.clone(),
            accounts.vault_asset_token_account.clone(),
            accounts.vault_coin_token_account.clone(),
            accounts.authority.clone(),
            accounts.token_program.clone(),
            accounts.chest_program.clone(),
            accounts.drift_program.clone(),
            accounts.drift_vaults_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN]>
for ChestStrategyDepositAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            chest: &arr[3],
            coin: &arr[4],
            chest_asset_reserve: &arr[5],
            drift_vault: &arr[6],
            drift_vault_depositor: &arr[7],
            drift_vault_token_account: &arr[8],
            drift_user: &arr[9],
            drift_user_stats: &arr[10],
            drift_spot_market: &arr[11],
            drift_spot_market_vault: &arr[12],
            drift_state: &arr[13],
            vault_asset_token_account: &arr[14],
            vault_coin_token_account: &arr[15],
            authority: &arr[16],
            token_program: &arr[17],
            chest_program: &arr[18],
            drift_program: &arr[19],
            drift_vaults_program: &arr[20],
            log_program: &arr[21],
        }
    }
}
pub const CHEST_STRATEGY_DEPOSIT_IX_DISCM: [u8; 8usize] = [
    67, 239, 123, 240, 182, 31, 21, 248,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChestStrategyDepositIxArgs {
    pub args: ChestStrategyDepositArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChestStrategyDepositIxData(pub ChestStrategyDepositIxArgs);
impl From<ChestStrategyDepositIxArgs> for ChestStrategyDepositIxData {
    fn from(args: ChestStrategyDepositIxArgs) -> Self {
        Self(args)
    }
}
impl ChestStrategyDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHEST_STRATEGY_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ChestStrategyDepositArgs>::deserialize(&mut reader)?
        };
        Ok(Self(ChestStrategyDepositIxArgs { args }))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHEST_STRATEGY_DEPOSIT_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn chest_strategy_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: ChestStrategyDepositKeys,
    args: ChestStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHEST_STRATEGY_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChestStrategyDepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn chest_strategy_deposit_ix(
    keys: ChestStrategyDepositKeys,
    args: ChestStrategyDepositIxArgs,
) -> std::io::Result<Instruction> {
    chest_strategy_deposit_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn chest_strategy_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyDepositAccounts<'_, '_>,
    args: ChestStrategyDepositIxArgs,
) -> ProgramResult {
    let keys: ChestStrategyDepositKeys = accounts.into();
    let ix = chest_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn chest_strategy_deposit_invoke(
    accounts: ChestStrategyDepositAccounts<'_, '_>,
    args: ChestStrategyDepositIxArgs,
) -> ProgramResult {
    chest_strategy_deposit_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn chest_strategy_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyDepositAccounts<'_, '_>,
    args: ChestStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChestStrategyDepositKeys = accounts.into();
    let ix = chest_strategy_deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn chest_strategy_deposit_invoke_signed(
    accounts: ChestStrategyDepositAccounts<'_, '_>,
    args: ChestStrategyDepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    chest_strategy_deposit_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn chest_strategy_deposit_verify_account_keys(
    accounts: ChestStrategyDepositAccounts<'_, '_>,
    keys: ChestStrategyDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.chest.key, keys.chest),
        (*accounts.coin.key, keys.coin),
        (*accounts.chest_asset_reserve.key, keys.chest_asset_reserve),
        (*accounts.drift_vault.key, keys.drift_vault),
        (*accounts.drift_vault_depositor.key, keys.drift_vault_depositor),
        (*accounts.drift_vault_token_account.key, keys.drift_vault_token_account),
        (*accounts.drift_user.key, keys.drift_user),
        (*accounts.drift_user_stats.key, keys.drift_user_stats),
        (*accounts.drift_spot_market.key, keys.drift_spot_market),
        (*accounts.drift_spot_market_vault.key, keys.drift_spot_market_vault),
        (*accounts.drift_state.key, keys.drift_state),
        (*accounts.vault_asset_token_account.key, keys.vault_asset_token_account),
        (*accounts.vault_coin_token_account.key, keys.vault_coin_token_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.chest_program.key, keys.chest_program),
        (*accounts.drift_program.key, keys.drift_program),
        (*accounts.drift_vaults_program.key, keys.drift_vaults_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn chest_strategy_deposit_verify_writable_privileges<'me, 'info>(
    accounts: ChestStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.chest,
        accounts.coin,
        accounts.chest_asset_reserve,
        accounts.drift_vault,
        accounts.drift_vault_depositor,
        accounts.drift_vault_token_account,
        accounts.drift_user,
        accounts.drift_user_stats,
        accounts.drift_spot_market,
        accounts.drift_spot_market_vault,
        accounts.drift_state,
        accounts.vault_asset_token_account,
        accounts.vault_coin_token_account,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn chest_strategy_deposit_verify_signer_privileges<'me, 'info>(
    accounts: ChestStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn chest_strategy_deposit_verify_account_privileges<'me, 'info>(
    accounts: ChestStrategyDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    chest_strategy_deposit_verify_writable_privileges(accounts)?;
    chest_strategy_deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct ChestStrategyRequestWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub chest: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub chest_coin_reserve: &'me AccountInfo<'info>,
    pub redemption_request: &'me AccountInfo<'info>,
    pub drift_vault: &'me AccountInfo<'info>,
    pub drift_vault_depositor: &'me AccountInfo<'info>,
    pub drift_user: &'me AccountInfo<'info>,
    pub asset_destination: &'me AccountInfo<'info>,
    pub vault_coin_token_account: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub chest_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChestStrategyRequestWithdrawKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub chest: Pubkey,
    pub coin: Pubkey,
    pub chest_coin_reserve: Pubkey,
    pub redemption_request: Pubkey,
    pub drift_vault: Pubkey,
    pub drift_vault_depositor: Pubkey,
    pub drift_user: Pubkey,
    pub asset_destination: Pubkey,
    pub vault_coin_token_account: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub chest_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<ChestStrategyRequestWithdrawAccounts<'_, '_>>
for ChestStrategyRequestWithdrawKeys {
    fn from(accounts: ChestStrategyRequestWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            chest: *accounts.chest.key,
            coin: *accounts.coin.key,
            chest_coin_reserve: *accounts.chest_coin_reserve.key,
            redemption_request: *accounts.redemption_request.key,
            drift_vault: *accounts.drift_vault.key,
            drift_vault_depositor: *accounts.drift_vault_depositor.key,
            drift_user: *accounts.drift_user.key,
            asset_destination: *accounts.asset_destination.key,
            vault_coin_token_account: *accounts.vault_coin_token_account.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            chest_program: *accounts.chest_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<ChestStrategyRequestWithdrawKeys>
for [AccountMeta; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: ChestStrategyRequestWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.chest,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.chest_coin_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redemption_request,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault_depositor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_destination,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.chest_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN]>
for ChestStrategyRequestWithdrawKeys {
    fn from(pubkeys: [Pubkey; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            chest: pubkeys[3],
            coin: pubkeys[4],
            chest_coin_reserve: pubkeys[5],
            redemption_request: pubkeys[6],
            drift_vault: pubkeys[7],
            drift_vault_depositor: pubkeys[8],
            drift_user: pubkeys[9],
            asset_destination: pubkeys[10],
            vault_coin_token_account: pubkeys[11],
            authority: pubkeys[12],
            system_program: pubkeys[13],
            token_program: pubkeys[14],
            chest_program: pubkeys[15],
            log_program: pubkeys[16],
        }
    }
}
impl<'info> From<ChestStrategyRequestWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChestStrategyRequestWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.chest.clone(),
            accounts.coin.clone(),
            accounts.chest_coin_reserve.clone(),
            accounts.redemption_request.clone(),
            accounts.drift_vault.clone(),
            accounts.drift_vault_depositor.clone(),
            accounts.drift_user.clone(),
            accounts.asset_destination.clone(),
            accounts.vault_coin_token_account.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.chest_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN]>
for ChestStrategyRequestWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            chest: &arr[3],
            coin: &arr[4],
            chest_coin_reserve: &arr[5],
            redemption_request: &arr[6],
            drift_vault: &arr[7],
            drift_vault_depositor: &arr[8],
            drift_user: &arr[9],
            asset_destination: &arr[10],
            vault_coin_token_account: &arr[11],
            authority: &arr[12],
            system_program: &arr[13],
            token_program: &arr[14],
            chest_program: &arr[15],
            log_program: &arr[16],
        }
    }
}
pub const CHEST_STRATEGY_REQUEST_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    3, 235, 227, 14, 76, 245, 237, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChestStrategyRequestWithdrawIxArgs {
    pub args: ChestStrategyRequestWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChestStrategyRequestWithdrawIxData(pub ChestStrategyRequestWithdrawIxArgs);
impl From<ChestStrategyRequestWithdrawIxArgs> for ChestStrategyRequestWithdrawIxData {
    fn from(args: ChestStrategyRequestWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl ChestStrategyRequestWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHEST_STRATEGY_REQUEST_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ChestStrategyRequestWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ChestStrategyRequestWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHEST_STRATEGY_REQUEST_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn chest_strategy_request_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: ChestStrategyRequestWithdrawKeys,
    args: ChestStrategyRequestWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHEST_STRATEGY_REQUEST_WITHDRAW_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: ChestStrategyRequestWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn chest_strategy_request_withdraw_ix(
    keys: ChestStrategyRequestWithdrawKeys,
    args: ChestStrategyRequestWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    chest_strategy_request_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn chest_strategy_request_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyRequestWithdrawAccounts<'_, '_>,
    args: ChestStrategyRequestWithdrawIxArgs,
) -> ProgramResult {
    let keys: ChestStrategyRequestWithdrawKeys = accounts.into();
    let ix = chest_strategy_request_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn chest_strategy_request_withdraw_invoke(
    accounts: ChestStrategyRequestWithdrawAccounts<'_, '_>,
    args: ChestStrategyRequestWithdrawIxArgs,
) -> ProgramResult {
    chest_strategy_request_withdraw_invoke_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
    )
}
pub fn chest_strategy_request_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyRequestWithdrawAccounts<'_, '_>,
    args: ChestStrategyRequestWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChestStrategyRequestWithdrawKeys = accounts.into();
    let ix = chest_strategy_request_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn chest_strategy_request_withdraw_invoke_signed(
    accounts: ChestStrategyRequestWithdrawAccounts<'_, '_>,
    args: ChestStrategyRequestWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    chest_strategy_request_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn chest_strategy_request_withdraw_verify_account_keys(
    accounts: ChestStrategyRequestWithdrawAccounts<'_, '_>,
    keys: ChestStrategyRequestWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.chest.key, keys.chest),
        (*accounts.coin.key, keys.coin),
        (*accounts.chest_coin_reserve.key, keys.chest_coin_reserve),
        (*accounts.redemption_request.key, keys.redemption_request),
        (*accounts.drift_vault.key, keys.drift_vault),
        (*accounts.drift_vault_depositor.key, keys.drift_vault_depositor),
        (*accounts.drift_user.key, keys.drift_user),
        (*accounts.asset_destination.key, keys.asset_destination),
        (*accounts.vault_coin_token_account.key, keys.vault_coin_token_account),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.chest_program.key, keys.chest_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn chest_strategy_request_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: ChestStrategyRequestWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.chest,
        accounts.coin,
        accounts.chest_coin_reserve,
        accounts.redemption_request,
        accounts.drift_vault,
        accounts.drift_vault_depositor,
        accounts.drift_user,
        accounts.vault_coin_token_account,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn chest_strategy_request_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: ChestStrategyRequestWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn chest_strategy_request_withdraw_verify_account_privileges<'me, 'info>(
    accounts: ChestStrategyRequestWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    chest_strategy_request_withdraw_verify_writable_privileges(accounts)?;
    chest_strategy_request_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct ChestStrategyWithdrawAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub strategy: &'me AccountInfo<'info>,
    pub asset_mint: &'me AccountInfo<'info>,
    pub vault_asset_ata: &'me AccountInfo<'info>,
    pub strategy_asset_token_account: &'me AccountInfo<'info>,
    pub chest: &'me AccountInfo<'info>,
    pub chest_redemption_request: &'me AccountInfo<'info>,
    pub coin: &'me AccountInfo<'info>,
    pub vault_coin_token_account: &'me AccountInfo<'info>,
    pub drift_vault: &'me AccountInfo<'info>,
    pub drift_vault_depositor: &'me AccountInfo<'info>,
    pub drift_user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub chest_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChestStrategyWithdrawKeys {
    pub vault: Pubkey,
    pub strategy: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_asset_ata: Pubkey,
    pub strategy_asset_token_account: Pubkey,
    pub chest: Pubkey,
    pub chest_redemption_request: Pubkey,
    pub coin: Pubkey,
    pub vault_coin_token_account: Pubkey,
    pub drift_vault: Pubkey,
    pub drift_vault_depositor: Pubkey,
    pub drift_user: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub chest_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<ChestStrategyWithdrawAccounts<'_, '_>> for ChestStrategyWithdrawKeys {
    fn from(accounts: ChestStrategyWithdrawAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            strategy: *accounts.strategy.key,
            asset_mint: *accounts.asset_mint.key,
            vault_asset_ata: *accounts.vault_asset_ata.key,
            strategy_asset_token_account: *accounts.strategy_asset_token_account.key,
            chest: *accounts.chest.key,
            chest_redemption_request: *accounts.chest_redemption_request.key,
            coin: *accounts.coin.key,
            vault_coin_token_account: *accounts.vault_coin_token_account.key,
            drift_vault: *accounts.drift_vault.key,
            drift_vault_depositor: *accounts.drift_vault_depositor.key,
            drift_user: *accounts.drift_user.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            chest_program: *accounts.chest_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<ChestStrategyWithdrawKeys>
for [AccountMeta; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: ChestStrategyWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asset_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_asset_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.strategy_asset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.chest,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.chest_redemption_request,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_vault_depositor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.drift_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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
                pubkey: keys.chest_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for ChestStrategyWithdrawKeys {
    fn from(pubkeys: [Pubkey; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            strategy: pubkeys[1],
            asset_mint: pubkeys[2],
            vault_asset_ata: pubkeys[3],
            strategy_asset_token_account: pubkeys[4],
            chest: pubkeys[5],
            chest_redemption_request: pubkeys[6],
            coin: pubkeys[7],
            vault_coin_token_account: pubkeys[8],
            drift_vault: pubkeys[9],
            drift_vault_depositor: pubkeys[10],
            drift_user: pubkeys[11],
            authority: pubkeys[12],
            system_program: pubkeys[13],
            token_program: pubkeys[14],
            chest_program: pubkeys[15],
            log_program: pubkeys[16],
        }
    }
}
impl<'info> From<ChestStrategyWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChestStrategyWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.strategy.clone(),
            accounts.asset_mint.clone(),
            accounts.vault_asset_ata.clone(),
            accounts.strategy_asset_token_account.clone(),
            accounts.chest.clone(),
            accounts.chest_redemption_request.clone(),
            accounts.coin.clone(),
            accounts.vault_coin_token_account.clone(),
            accounts.drift_vault.clone(),
            accounts.drift_vault_depositor.clone(),
            accounts.drift_user.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.chest_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN]>
for ChestStrategyWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            strategy: &arr[1],
            asset_mint: &arr[2],
            vault_asset_ata: &arr[3],
            strategy_asset_token_account: &arr[4],
            chest: &arr[5],
            chest_redemption_request: &arr[6],
            coin: &arr[7],
            vault_coin_token_account: &arr[8],
            drift_vault: &arr[9],
            drift_vault_depositor: &arr[10],
            drift_user: &arr[11],
            authority: &arr[12],
            system_program: &arr[13],
            token_program: &arr[14],
            chest_program: &arr[15],
            log_program: &arr[16],
        }
    }
}
pub const CHEST_STRATEGY_WITHDRAW_IX_DISCM: [u8; 8usize] = [
    225, 169, 142, 80, 186, 56, 87, 136,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChestStrategyWithdrawIxArgs {
    pub args: ChestStrategyWithdrawArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChestStrategyWithdrawIxData(pub ChestStrategyWithdrawIxArgs);
impl From<ChestStrategyWithdrawIxArgs> for ChestStrategyWithdrawIxData {
    fn from(args: ChestStrategyWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl ChestStrategyWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHEST_STRATEGY_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let args = if reader.is_empty() {
            Default::default()
        } else {
            <ChestStrategyWithdrawArgs>::deserialize(&mut reader)?
        };
        Ok(
            Self(ChestStrategyWithdrawIxArgs {
                args,
            }),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHEST_STRATEGY_WITHDRAW_IX_DISCM)?;
        borsh::BorshSerialize::serialize(&self.0.args, &mut writer)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn chest_strategy_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: ChestStrategyWithdrawKeys,
    args: ChestStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHEST_STRATEGY_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChestStrategyWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn chest_strategy_withdraw_ix(
    keys: ChestStrategyWithdrawKeys,
    args: ChestStrategyWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    chest_strategy_withdraw_ix_with_program_id(CARROT_PROGRAM_ID, keys, args)
}
pub fn chest_strategy_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyWithdrawAccounts<'_, '_>,
    args: ChestStrategyWithdrawIxArgs,
) -> ProgramResult {
    let keys: ChestStrategyWithdrawKeys = accounts.into();
    let ix = chest_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn chest_strategy_withdraw_invoke(
    accounts: ChestStrategyWithdrawAccounts<'_, '_>,
    args: ChestStrategyWithdrawIxArgs,
) -> ProgramResult {
    chest_strategy_withdraw_invoke_with_program_id(CARROT_PROGRAM_ID, accounts, args)
}
pub fn chest_strategy_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChestStrategyWithdrawAccounts<'_, '_>,
    args: ChestStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChestStrategyWithdrawKeys = accounts.into();
    let ix = chest_strategy_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn chest_strategy_withdraw_invoke_signed(
    accounts: ChestStrategyWithdrawAccounts<'_, '_>,
    args: ChestStrategyWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    chest_strategy_withdraw_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        args,
        seeds,
    )
}
pub fn chest_strategy_withdraw_verify_account_keys(
    accounts: ChestStrategyWithdrawAccounts<'_, '_>,
    keys: ChestStrategyWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.strategy.key, keys.strategy),
        (*accounts.asset_mint.key, keys.asset_mint),
        (*accounts.vault_asset_ata.key, keys.vault_asset_ata),
        (*accounts.strategy_asset_token_account.key, keys.strategy_asset_token_account),
        (*accounts.chest.key, keys.chest),
        (*accounts.chest_redemption_request.key, keys.chest_redemption_request),
        (*accounts.coin.key, keys.coin),
        (*accounts.vault_coin_token_account.key, keys.vault_coin_token_account),
        (*accounts.drift_vault.key, keys.drift_vault),
        (*accounts.drift_vault_depositor.key, keys.drift_vault_depositor),
        (*accounts.drift_user.key, keys.drift_user),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.chest_program.key, keys.chest_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn chest_strategy_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: ChestStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.strategy,
        accounts.vault_asset_ata,
        accounts.strategy_asset_token_account,
        accounts.chest,
        accounts.coin,
        accounts.vault_coin_token_account,
        accounts.drift_vault,
        accounts.drift_vault_depositor,
        accounts.drift_user,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn chest_strategy_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: ChestStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn chest_strategy_withdraw_verify_account_privileges<'me, 'info>(
    accounts: ChestStrategyWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    chest_strategy_withdraw_verify_writable_privileges(accounts)?;
    chest_strategy_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSwitchboardOraclePriceAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub shares: &'me AccountInfo<'info>,
    pub feed: &'me AccountInfo<'info>,
    pub queue: &'me AccountInfo<'info>,
    pub program_state: &'me AccountInfo<'info>,
    pub recent_slothashes: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub switchboard_on_demand_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub log_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSwitchboardOraclePriceKeys {
    pub vault: Pubkey,
    pub shares: Pubkey,
    pub feed: Pubkey,
    pub queue: Pubkey,
    pub program_state: Pubkey,
    pub recent_slothashes: Pubkey,
    pub reward_vault: Pubkey,
    pub token_mint: Pubkey,
    pub switchboard_on_demand_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub log_program: Pubkey,
}
impl From<UpdateSwitchboardOraclePriceAccounts<'_, '_>>
for UpdateSwitchboardOraclePriceKeys {
    fn from(accounts: UpdateSwitchboardOraclePriceAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            shares: *accounts.shares.key,
            feed: *accounts.feed.key,
            queue: *accounts.queue.key,
            program_state: *accounts.program_state.key,
            recent_slothashes: *accounts.recent_slothashes.key,
            reward_vault: *accounts.reward_vault.key,
            token_mint: *accounts.token_mint.key,
            switchboard_on_demand_program: *accounts.switchboard_on_demand_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            log_program: *accounts.log_program.key,
        }
    }
}
impl From<UpdateSwitchboardOraclePriceKeys>
for [AccountMeta; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSwitchboardOraclePriceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.shares,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.feed,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.queue,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recent_slothashes,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_on_demand_program,
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
                pubkey: keys.log_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN]>
for UpdateSwitchboardOraclePriceKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            shares: pubkeys[1],
            feed: pubkeys[2],
            queue: pubkeys[3],
            program_state: pubkeys[4],
            recent_slothashes: pubkeys[5],
            reward_vault: pubkeys[6],
            token_mint: pubkeys[7],
            switchboard_on_demand_program: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            log_program: pubkeys[11],
        }
    }
}
impl<'info> From<UpdateSwitchboardOraclePriceAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSwitchboardOraclePriceAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.shares.clone(),
            accounts.feed.clone(),
            accounts.queue.clone(),
            accounts.program_state.clone(),
            accounts.recent_slothashes.clone(),
            accounts.reward_vault.clone(),
            accounts.token_mint.clone(),
            accounts.switchboard_on_demand_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.log_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN]>
for UpdateSwitchboardOraclePriceAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            shares: &arr[1],
            feed: &arr[2],
            queue: &arr[3],
            program_state: &arr[4],
            recent_slothashes: &arr[5],
            reward_vault: &arr[6],
            token_mint: &arr[7],
            switchboard_on_demand_program: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            log_program: &arr[11],
        }
    }
}
pub const UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_DISCM: [u8; 8usize] = [
    117, 128, 6, 136, 244, 23, 83, 10,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSwitchboardOraclePriceIxData;
impl UpdateSwitchboardOraclePriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8usize];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_switchboard_oracle_price_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSwitchboardOraclePriceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SWITCHBOARD_ORACLE_PRICE_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateSwitchboardOraclePriceIxData.try_to_vec()?,
    })
}
pub fn update_switchboard_oracle_price_ix(
    keys: UpdateSwitchboardOraclePriceKeys,
) -> std::io::Result<Instruction> {
    update_switchboard_oracle_price_ix_with_program_id(CARROT_PROGRAM_ID, keys)
}
pub fn update_switchboard_oracle_price_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSwitchboardOraclePriceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateSwitchboardOraclePriceKeys = accounts.into();
    let ix = update_switchboard_oracle_price_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_switchboard_oracle_price_invoke(
    accounts: UpdateSwitchboardOraclePriceAccounts<'_, '_>,
) -> ProgramResult {
    update_switchboard_oracle_price_invoke_with_program_id(CARROT_PROGRAM_ID, accounts)
}
pub fn update_switchboard_oracle_price_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSwitchboardOraclePriceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSwitchboardOraclePriceKeys = accounts.into();
    let ix = update_switchboard_oracle_price_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_switchboard_oracle_price_invoke_signed(
    accounts: UpdateSwitchboardOraclePriceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_switchboard_oracle_price_invoke_signed_with_program_id(
        CARROT_PROGRAM_ID,
        accounts,
        seeds,
    )
}
pub fn update_switchboard_oracle_price_verify_account_keys(
    accounts: UpdateSwitchboardOraclePriceAccounts<'_, '_>,
    keys: UpdateSwitchboardOraclePriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.shares.key, keys.shares),
        (*accounts.feed.key, keys.feed),
        (*accounts.queue.key, keys.queue),
        (*accounts.program_state.key, keys.program_state),
        (*accounts.recent_slothashes.key, keys.recent_slothashes),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.token_mint.key, keys.token_mint),
        (
            *accounts.switchboard_on_demand_program.key,
            keys.switchboard_on_demand_program,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.log_program.key, keys.log_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_switchboard_oracle_price_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSwitchboardOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault, accounts.feed, accounts.reward_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_switchboard_oracle_price_verify_account_privileges<'me, 'info>(
    accounts: UpdateSwitchboardOraclePriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_switchboard_oracle_price_verify_writable_privileges(accounts)?;
    Ok(())
}
